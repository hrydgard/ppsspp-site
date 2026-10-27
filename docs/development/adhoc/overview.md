---
position: 1
---

# Ad hoc networking: overview

This section documents how PSP ad hoc multiplayer works: what the games ask the system for, what a real PSP
does on the radio, and how PPSSPP fakes all of it over the internet, both peer-to-peer and through the
aemu_postoffice relay. This page is the overview. The pages after it go into detail:

| Page | Covers |
|---|---|
| [Writing an ad hoc game](/docs/development/adhoc/writing-a-game) | How a game is structured for ad hoc: setup, the lobby screens, the game loop, noticing players leave |
| [On the PSP](/docs/development/adhoc/on-the-psp) | The firmware modules, initialization, the network, and what goes out on the air |
| [sceNetAdhocctl](/docs/development/adhoc/adhocctl) | Groups: scanning, creating, joining, peers, handlers, the netconf dialog |
| [PDP and PTP](/docs/development/adhoc/pdp-ptp) | Datagram and stream sockets, and how PPSSPP maps them to UDP and TCP |
| [Game mode](/docs/development/adhoc/game-mode) | The shared-memory "game mode" replication |
| [sceNetAdhocMatching](/docs/development/adhoc/matching) | The lobby / matchmaking library, Sony's protocol and the aemu one |
| [The adhoc server](/docs/development/adhoc/adhoc-server) | The aemu adhocctl server protocol, the built-in server, the server list |
| [The relay (aemu_postoffice)](/docs/development/adhoc/relay) | Relayed PDP and PTP through a central server |
| [Emulation details](/docs/development/adhoc/emulation) | Threads, blocking, timing, ports, UPnP, configuration, savestates |
| [Testing](/docs/development/adhoc/testing) | The pspautotests behind these pages, running them on PSPs and on PPSSPP, and how PPSSPP does |
| [Known issues and improvements](/docs/development/adhoc/improving) | Where PPSSPP differs from the hardware, and what to fix first |
| [API reference](/docs/development/adhoc/api/conventions) | Every function of sceNetAdhocctl, sceNetAdhoc, sceNetAdhocMatching, sceNetAdhocDiscover, sceNetInet, sceNetApctl and sceNetResolver, and all the error codes |

For how to *play*, see [Ad hoc multiplayer](/docs/multiplayer/how-to-play) instead.

## What ad hoc is

"Ad hoc" is the PSP's local wireless multiplayer. PSPs in the same room form a small 802.11b network among
themselves, with no access point, and the game talks directly to the other consoles. There is no IP
involved at all: packets are addressed by the other console's MAC address plus a 16-bit port.

The other mode, "infrastructure", is ordinary internet play through an access point using TCP/IP
(`sceNetInet`, `sceNetApctl`), and is not covered here; see
[Infrastructure mode](/docs/multiplayer/infrastructure-servers).

## The libraries

A game sees ad hoc as four libraries, loaded together with `sceUtilityLoadNetModule`:

```
  +------------------------------------------------------------+
  |                          the game                          |
  +------------+----------------------+-----------+------------+
  | netconf    | sceNetAdhocMatching  |           |            |
  | dialog     | (lobby, over PDP)    |           |            |
  +------------+----------------------+           |            |
  | sceNetAdhocctl                    | sceNetAdhoc            |
  | (groups, peers, game mode setup)  | (PDP, PTP, game mode   |
  |                                   |  data)                 |
  +-----------------------------------+------------------------+
  | sceNet (memory pool, mbufs, interface), crypto/auth        |
  +------------------------------------------------------------+
  | wlan driver: 802.11b IBSS, raw frames, ethertype 0x88C8    |
  +------------------------------------------------------------+
```

- **sceNetAdhocctl** ("adhoc control") manages *membership*. A group of PSPs playing together is an ad hoc
  network with a name of up to 8 characters. Adhocctl scans for groups, creates and joins them, keeps the
  list of peers (their MAC addresses and nicknames) and reports changes to the game through handlers. It
  also sets up game mode.
- **sceNetAdhoc** moves *data*. It offers PDP ("Peer Datagram Protocol", unreliable datagrams, like UDP),
  PTP ("Peer Transport Protocol", reliable streams, like TCP), and the game mode data calls. Sockets are
  addressed by MAC address and port.
- **sceNetAdhocMatching** is a matchmaking library built on top of PDP: hosts advertise themselves, clients
  ask to join, hosts accept or reject them, and the library keeps the member list alive. Many games use it
  for their lobbies; many others do their own matchmaking over PDP.
- **The netconf utility dialog** (`sceUtilityNetconf*`) is the system's "Connecting..." screen. A game can
  use it to create or join a group instead of driving adhocctl itself.

## What a game does

### Initializing

1. `sceUtilityLoadNetModule(PSP_NET_MODULE_COMMON)` then `sceUtilityLoadNetModule(PSP_NET_MODULE_ADHOC)`
   (modules 1 and 2) to load the networking modules.
2. `sceNetInit()` with the size of the networking memory pool and the priorities and stack sizes of its
   two internal threads.
3. `sceNetAdhocInit()`.
4. `sceNetAdhocctlInit(stacksize, priority, adhocId)`, where `adhocId` holds the game's product code (for
   example `ULUS10336`). Only PSPs running the same product code see each other.
5. Optionally `sceNetAdhocMatchingInit(poolsize)`.

### Listing the available groups ("rooms")

1. `sceNetAdhocctlAddHandler()` to register a handler function.
2. `sceNetAdhocctlScan()` to start a scan. When the scan finishes (or fails), the handler is called with
   the SCAN event (or ERROR).
3. `sceNetAdhocctlGetScanInfo()` to fetch the list of groups found, each with its name, BSSID (an address
   identifying the network) and channel.

### Joining a group with peers

1. `sceNetAdhocctlAddHandler()` to register a handler, if not done already.
2. `sceNetAdhocctlConnect(name)`, `sceNetAdhocctlCreate(name)` or `sceNetAdhocctlJoin(scaninfo)` to become
   part of the network. The call returns at once; the handler is called with CONNECT when the console is in
   the group, or ERROR if it failed.

   Or, instead of the two steps above, run the netconf dialog in "connect", "create" or "join" mode, which
   draws the system UI and calls adhocctl itself.
3. `sceNetAdhocctlGetPeerList()` returns the MAC address and nickname of every other console in the group.

### Talking to peers

| Call | Conceptually |
|---|---|
| `sceNetAdhocPdpCreate(mac, port, bufsize, flag)` | `socket` + `bind` for a datagram socket |
| `sceNetAdhocPdpSend(id, dstmac, dstport, data, len, timeout, flag)` | `sendto` |
| `sceNetAdhocPdpRecv(id, &srcmac, &srcport, buf, &len, timeout, flag)` | `recvfrom` |
| `sceNetAdhocPtpListen(mac, port, bufsize, rexmt_int, rexmt_cnt, backlog, flag)` | `socket` + `bind` + `listen` |
| `sceNetAdhocPtpAccept(id, &peermac, &peerport, timeout, flag)` | `accept` |
| `sceNetAdhocPtpOpen(mac, port, dstmac, dstport, bufsize, rexmt_int, rexmt_cnt, flag)` | `socket` + `bind`, and start connecting |
| `sceNetAdhocPtpConnect(id, timeout, flag)` | wait for (or poll) the result of the connect started by Open |
| `sceNetAdhocPtpSend` / `sceNetAdhocPtpRecv` | `send` / `recv` |
| `sceNetAdhocPollSocket(sds, count, timeout, flag)` | `poll` |

A PDP datagram sent to `FF:FF:FF:FF:FF:FF` is broadcast to every console in the group, which is how most
games do discovery and lobby traffic without the matching library.

### Shutting down

Disconnect with `sceNetAdhocctlDisconnect()`, delete the sockets, then the `...Term()` functions in the
reverse order of the `...Init()` calls.

## How PPSSPP does it

PPSSPP has no access to a Wi-Fi radio in ad hoc mode, and real ad hoc traffic is encrypted with keys only
genuine PSPs hold, so it can't talk to unmodified PSPs over the air. Instead it emulates ad hoc over the
internet, using the scheme designed by the PRO Online / aemu plugin for custom firmware, which lets real PSPs
(and PS Vitas running Adrenaline) do the same thing. PPSSPP's code started as a port of that plugin.

```
                        +-------------------------+
     adhocctl (TCP      |       adhoc server      |     The server knows, per game,
     27312): groups,    |  (aemu protocol, also   |     which groups exist and who is
     peers, game mode   |  built into PPSSPP)     |     in them, with their IP address.
                        +-----+-------------+-----+
                              |             |
                         +----+----+   +----+----+
                         | PPSSPP  |   | PSP w/  |
                         |  (A)    |   | aemu (B)|
                         +----+----+   +----+----+
                              |             |
       P2P data mode:         +---- UDP ----+   PDP datagrams, port + offset
       direct between peers   +---- TCP ----+   PTP streams, port + offset
                              |             |
       Relay data mode:  +----+-------------+-----+
       (alternative)     | aemu_postoffice (TCP   |
                         | 27313), forwards by    |
                         | MAC address and port   |
                         +------------------------+
```

The work splits into two parts:

- **Control.** Everything adhocctl would learn from the radio is provided by an *adhoc server*. Each player
  logs in to it with their MAC address, nickname and product code. Scanning asks the server for the groups
  of that product code. Creating or joining a group tells the server, which then sends each member the
  nickname, MAC and IP address of every other member, and keeps them updated as players come and go. The
  game only ever sees MAC addresses; the IP addresses stay inside the emulator.
- **Data.** PDP and PTP are mapped onto the host's sockets in one of two ways:
  - **P2P mode** (the original scheme): a PDP socket becomes a UDP socket and a PTP socket becomes a TCP
    socket, bound to the PSP port plus a configurable *port offset*. Sending to a MAC address looks up that
    peer's IP address in the list from the server; receiving maps the sender's IP address back to a MAC.
    Players must be able to reach each other directly, which usually means port forwarding, UPnP or a
    virtual LAN.
  - **Relay mode**: every PDP and PTP socket instead becomes a connection to an aemu_postoffice relay,
    which forwards packets between players by MAC address and port. No ports need to be opened, at the cost
    of the extra hop through the server.

In P2P mode, the calls map like this:

| PSP call | What PPSSPP does |
|---|---|
| `sceNetAdhocctlConnect` / `Create` / `Join` | Sends CONNECT(group) to the adhoc server. The server answers with the members of the group (nickname, MAC, IP), then a CONNECT_BSSID packet that means "you're in"; that raises the CONNECT event. |
| `sceNetAdhocPdpCreate` | Creates a UDP socket and binds it to the port plus the port offset. |
| `sceNetAdhocPdpSend` | Looks up the IP address of the destination MAC in the peer list and sends a UDP datagram there. A broadcast is sent separately to each peer. |
| `sceNetAdhocPdpRecv` | Receives a UDP datagram, maps the sender's IP address back to a MAC address and its port back to a PSP port, and hands those to the game. |
| `sceNetAdhocPtpListen` | Creates a TCP socket, binds it to port plus offset, listens. |
| `sceNetAdhocPtpAccept` | `accept`, then maps the connecting IP address back to a MAC address. |
| `sceNetAdhocPtpOpen` | Maps the destination MAC to an IP address, creates a TCP socket and starts a non-blocking `connect`. |
| `sceNetAdhocPtpConnect` | Checks (or waits for) the result of that `connect`. |

All host sockets are non-blocking. When the game makes a blocking call, PPSSPP puts the calling PSP thread
to sleep and keeps retrying the operation on a timer until it succeeds or the game's timeout runs out. See
[Emulation details](/docs/development/adhoc/emulation).

### Who can play with whom

- PPSSPP with PPSSPP, on the same adhoc server, in the same data mode, with the same port offset (P2P
  mode).
- PPSSPP with PSPs and PS Vitas running the aemu / PRO Online plugin, on the same server. The plugin
  speaks the same server, PDP/PTP, game mode and matching protocols.
- Several PPSSPP instances on one computer, using the built-in server on `localhost`; each instance gets
  its own 127.0.0.x address and MAC address.
- Not unmodified PSPs over real Wi-Fi.

### The matching library

Sony's sceNetAdhocMatching has its own packet format, which nothing outside the firmware speaks. PPSSPP
implements the library itself, using the aemu plugin's packet format over PDP. That means it works between
PPSSPP and aemu clients, but the timing and retransmission behaviour differ from Sony's. See
[sceNetAdhocMatching](/docs/development/adhoc/matching).

### Source files

| File | Contents |
|---|---|
| `Core/HLE/sceNetAdhoc.cpp` | All sceNetAdhoc and sceNetAdhocctl functions, blocking emulation, game mode, relay hooks |
| `Core/HLE/proAdhoc.cpp/.h` | Shared state, structs, peer list, MAC/IP resolution, the `friendFinder` thread that talks to the adhoc server |
| `Core/HLE/sceNetAdhocMatching.cpp` | sceNetAdhocMatching |
| `Core/HLE/proAdhocServer.cpp` | The built-in adhoc server |
| `Core/HLE/NetAdhocCommon.*` | Shared timing constants and the event queue |
| `Core/HLE/sceNet.cpp` | `sceNetInit` / `sceNetTerm`, networking setup |
| `Core/Dialog/PSPNetconfDialog.cpp` | The netconf dialog |
| `Core/Util/PortManager.cpp` | UPnP port mapping |
| `ext/aemu_postoffice/` | The adhocctl client library (`adhocctl/`), the postoffice client (`client/`) and reference servers |
| `UI/AdhocServerScreen.cpp` | The server browser |

## History

- PPSSPP's ad hoc support started in late 2013 as a port of Coldbird's PRO Online client and server from
  the aemu project, which brought ad hoc over the internet to real PSPs running custom firmware.
- From 2020 to 2022, ANR2ME reworked most of it: blocking calls on emulated threads, PTP connect behaviour,
  game mode, matching fixes, UPnP, and a long list of game-specific fixes.
- In 2026, Kethen (the current aemu maintainer) added relay mode using aemu_postoffice ([#21116]), and
  later moved the adhocctl client protocol into the shared `aemu_postoffice` library, with an optional UDP
  (ENet) transport.
