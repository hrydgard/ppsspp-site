---
position: 2
---

# Ad hoc on a real PSP

What the PSP itself does when a game plays ad hoc. PPSSPP doesn't reproduce any of this on the wire (see
[the overview](/docs/development/adhoc/overview) for why), but it's what the emulated API is modelled on, and
it explains several of the limits games run into.

## Modules

The ad hoc stack is a set of firmware modules in `flash0:/kd/`:

| Module | Library | Role |
|---|---|---|
| `wlan.prx` | sceWlan_Driver | Driver for the Wi-Fi chip. Joins and creates 802.11 networks, sends and receives raw frames, runs the game mode send/receive slots. |
| `ifhandle.prx` | sceNet_Service | Network interfaces and packet buffers (mbufs). Sorts incoming frames: ordinary ones go to the IP stack, ad hoc ones to the ad hoc modules. |
| `pspnet.prx` | sceNet_Library | `sceNetInit`: the shared memory pool, and the callout and interrupt threads (`SceNetCallout`, `SceNetNetintr`). |
| `memab.prx`, `pspnet_adhoc_auth.prx` | | Ad hoc authentication and encryption. Every ad hoc frame passes through these. |
| `pspnet_adhoc.prx` | sceNetAdhoc_Library | PDP, PTP and the game mode data calls |
| `pspnet_adhocctl.prx` | sceNetAdhocctl_Library | Groups, scanning, peers, game mode setup |
| `pspnet_adhoc_matching.prx` | sceNetAdhocMatching_Library | Matchmaking over PDP |
| `pspnet_adhoc_download.prx`, `pspnet_adhoc_discover.prx`, `pspnet_adhoc_transfer_int.prx` | | Game sharing download, "discover" mode, and the adhoc transfer library. Rarely used by games. |

Games load them with `sceUtilityLoadNetModule` (or the newer `sceUtilityLoadModule`):

| `LoadNetModule` | `LoadModule` | Loads |
|---|---|---|
| 1 (`COMMON`) | 0x100 | The common networking modules (ifhandle, pspnet) |
| 2 (`ADHOC`) | 0x101 | pspnet_adhoc, pspnet_adhocctl, pspnet_adhoc_matching |
| 3 (`INET`) | 0x102 | The TCP/IP stack, apctl, resolver (infrastructure mode) |

`COMMON` has to be loaded first.

## Initialization

- `sceNetInit(poolsize, calloutprio, calloutstack, netintrprio, netintrstack)` creates the memory pool all
  of networking allocates from, and the two internal threads.
- `sceNetAdhocInit()` needs `sceNetInit` first.
- `sceNetAdhocctlInit(stacksize, priority, adhocId)` starts adhocctl's own thread. `adhocId` is 16 bytes:

  | Offset | Size | Field |
  |---|---|---|
  | 0 | 4 | type: 0 = commercial product (the normal case), 1 = reserved, for debugging, 2 = system |
  | 4 | 9 | product code, e.g. `ULUS10336` (not NUL-terminated) |
  | 13 | 3 | padding |

  The product code decides who can see whom: consoles only find groups with the same type and product
  code. Regional versions of a game have different product codes, so they don't meet unless the game was
  built to use a shared code.

### Error codes

All ad hoc error codes have the form `0x8041MMEE`: facility `0x041` (networking), a module number `MM`,
and the error. The ad hoc modules are:

| `MM` | Module |
|---|---|
| `06` | adhoc auth |
| `07` | sceNetAdhoc (PDP, PTP, game mode) |
| `08` | sceNetAdhocMatching |
| `0B` | sceNetAdhocctl |
| `0D` | wlan |
| `12` | adhoc transfer |
| `13` | adhoc discover |

The individual codes are listed on the pages for each library.

## The network: 802.11b IBSS

A group is an 802.11 IBSS ("independent BSS", the standard's name for ad hoc mode):

- **Channel**: 1, 6 or 11. The system setting "Ad Hoc Mode Channel" (system parameter `ADHOC_CHANNEL`)
  picks one of those or "Automatic" (0).
- **BSSID**: a random, locally administered address (the low two bits of the first byte are `10`), as
  802.11 prescribes for an IBSS. It is *not* the creator's MAC address. `sceNetAdhocctlGetScanInfo` and
  `sceNetAdhocctlGetParameter` report it. (The adhoc server protocol, and so PPSSPP, use the creator's MAC
  instead, which games can't tell apart.)
- **SSID**: built from the ad hoc type, the product code, the mode and the group name:

  ```
  PSP_ + A|X|S + product code (9) + _ + L|G + _ + group name (0-8)
  ```

  `A` is a commercial product, `X` the reserved (debug) type, and `S` the system type. `L` is a normal
  network and `G` a game mode one. For example, group `MyGroup` of `ULUS10336` is
  `PSP_AULUS10336_L_MyGroup`. The `PSP` prefix can be changed through the registry key
  `/CONFIG/NETWORK/ADHOC/ssid_prefix`, but never is in practice.
- Beacons advertise 1, 2, 5.5 and 11 Mbps (802.11b rates). In game mode, the master's beacons also carry a
  Sony vendor-specific element (OUI `00:04:1F`).

Scanning is an ordinary 802.11 scan for `PSP_` networks with the right type and product code.
`sceNetAdhocctlConnect(name)` scans for a network with that group name and joins it if one exists, otherwise
creates it. `Create` always creates one and `Join` joins the network of a given scan result.

The group name is limited to 8 characters, and PPSSPP (like the aemu server) accepts only letters and digits.

## On the air

None of the ad hoc protocols use IP. Everything is sent as raw frames with an Ethernet-style header and
Sony's ethertype `0x88C8`, followed by a 16-bit sub-type:

| Offset | Size | Field |
|---|---|---|
| 0 | 6 | destination MAC |
| 6 | 6 | source MAC |
| 12 | 2 | ethertype `0x88C8` (big-endian) |
| 14 | 2 | sub-type (big-endian): 0 = game mode, 1 = control, 2 = data (PDP/PTP) |
| 16 | ... | sub-type specific |

The rest of the frame is **encrypted and authenticated**. Control frames (sub-type 1) carry a
keep-alive and a certificate exchange through which each pair of consoles sets up keys; unicast data is
encrypted with a per-peer key, broadcast data with a group key. Only consoles with genuine PSP keys can take
part. This, more than anything, is why an emulator can't join a real ad hoc network.

### Data frames

A data frame has a 32-byte crypto header after the sub-type, then a protocol header, then the payload,
padded to a multiple of 16 bytes:

| Offset | Size | Field |
|---|---|---|
| 0 | 14 | Ethernet-style header (above) |
| 14 | 2 | sub-type `2` |
| 16 | 32 | crypto header |
| 48 | 12 | protocol header (PDP: below) |
| 60 | n | payload |

The PDP header (all fields big-endian):

| Offset | Size | Field |
|---|---|---|
| 0 | 1 | protocol: 1 = PDP, 2 = PTP |
| 1 | 1 | 1 if more fragments of this datagram follow, 0 on the last fragment |
| 2 | 2 | length of this fragment's payload + 12 |
| 4 | 2 | source port |
| 6 | 2 | destination port |
| 8 | 2 | datagram sequence number |
| 10 | 2 | offset of this fragment in the datagram |

PTP uses the same kind of frame with protocol 2 and its own header.

### Sizes

- **PDP datagrams can be up to 65523 bytes** (`PSP_ADHOC_PDP_MTU`, 0xFFF3). Longer sends fail with
  `INVALID_DATALEN`.
- The sender **fragments** datagrams into pieces of at most **1444 bytes** (`PSP_ADHOC_PDP_MFS`), so only
  a datagram of 1444 bytes or less goes out in a single frame. Losing any fragment loses the whole
  datagram, so games that care about loss keep their datagrams small.
- PTP segments are at most 1444 bytes too (`PSP_ADHOC_PTP_MSS`).

### Addresses and broadcast

- Sockets are addressed by MAC address and 16-bit port. Ports 0 and 65535 aren't valid destinations.
- **The broadcast address is `FF:FF:FF:FF:FF:FF`.** A PDP datagram sent there reaches every console in the
  group.
- `00:00:00:00:00:00` is not a broadcast address. `sceNetAdhocPdpSend` rejects it, like any other multicast
  address, with `INVALID_ADDR` (0x80410702).

### Infrastructure and ad hoc exclude each other

The radio is either on an access point or in an ad hoc network, never both, and whichever got there first
keeps it (the `net/adhoc/apctl` test, with a phone hotspot as the access point):

- `sceNetApctlConnect` while the console is in an ad hoc group returns 0, goes to JOINING, then falls
  back to DISCONNECTED without associating. The ad hoc group carries on.
- `sceNetAdhocctlCreate` while apctl has an IP returns 0, then the handler gets ERROR with 0x80410D01 (a
  wlan driver error), and adhocctl stays DISCONNECTED. Apctl stays connected.

Neither call fails up front; the failure only shows in the state or the handler.

### Talking to itself

A console can send PDP datagrams and open PTP connections to its own MAC address, and they're delivered
locally. Its broadcasts, though, never reach any of its own sockets. So two matching contexts in one
process can't find each other, and a matching session always needs a second console.

## Game mode on the air

Game mode (see [Game mode](/docs/development/adhoc/game-mode)) uses its own network (`_G_` in the SSID) and
its own frames (sub-type 0, encrypted with a session key). The Wi-Fi chip, not the CPU, sends the master's
data in fixed time slots to a multicast address shared by the group, and each console receives the latest
copy of every other member's data. The game only ever sees "the latest version of the master's buffer".

## The matching protocol

Sony's sceNetAdhocMatching runs entirely over PDP. Its packet format is described on the
[matching page](/docs/development/adhoc/matching#sony-s-protocol). It is not the format PPSSPP (or aemu) sends.

## What isn't known

- How each peer's nickname reaches the others in a group. (A peer that leaves is dropped from the
  others' lists within a few seconds; how long one that just goes silent lasts hasn't been measured.)
- How long a peer that just goes silent stays in the others' peer lists.
- The PTP protocol: its handshake, window, acknowledgements and retransmit timers.
- How "Automatic" picks a channel.
- Game mode's slot timing and maximum data size.
