---
position: 3
---

# sceNetAdhocctl: groups and peers

Adhocctl decides which network the console is on and who else is in it. A game uses it to scan for groups,
create or join one, list the peers, and set up game mode. This page describes the API, then how PPSSPP
emulates it on top of an [adhoc server](/docs/development/adhoc/adhoc-server).

## API

### States and events

`sceNetAdhocctlGetState` returns one of:

| Value | State |
|---|---|
| 0 | DISCONNECTED |
| 1 | CONNECTED |
| 2 | SCANNING |
| 3 | GAMEMODE |
| 4 | DISCOVER |
| 5 | WOL (wake on LAN) |

Handlers registered with `sceNetAdhocctlAddHandler(handler, arg)` are called as
`handler(int event, int error, void *arg)` with one of these events:

| Value | Event | When |
|---|---|---|
| 0 | ERROR | An operation failed; `error` has the code |
| 1 | CONNECT | The console is now in the group (after Connect, Create or Join) |
| 2 | DISCONNECT | The console left the group |
| 3 | SCAN | A scan finished; the results are ready for `GetScanInfo` |
| 4 | GAME | Game mode was entered: every required member is present |
| 5 | DISCOVER | Discover mode finished |
| 6 | WOL | Wake on LAN |
| 7 | WOL_INTERRUPTED | |

Almost all adhocctl operations are asynchronous: the call starts the operation and returns 0, and the
result arrives later through the handler. Games expect the state to have changed by the time the handler
runs, so a handler that checks `GetState` sees the new state.

### Functions

| Function | Does |
|---|---|
| `Init(stacksize, prio, adhocId)` | Starts adhocctl and sets the product code. See [On the PSP](/docs/development/adhoc/on-the-psp#initialization). |
| `Term()` | Stops it. |
| `AddHandler(handler, arg)` / `DelHandler(id)` | Register and remove event handlers. |
| `Scan()` | Starts a scan. State SCANNING, then SCAN event and back to DISCONNECTED. |
| `GetScanInfo(&size, buf)` | Copies the scan results. With a NULL buffer, returns the size needed. |
| `Connect(name)` | Joins the group with that name if a scan finds it, otherwise creates it. |
| `Create(name)` | Creates a new group. |
| `Join(scaninfo)` | Joins the group described by a scan result. |
| `Disconnect()` | Leaves the group. DISCONNECT event. |
| `GetPeerList(&size, buf)` | The other consoles in the group. NULL buffer returns the size needed. |
| `GetPeerInfo(mac, size, info)` | One peer's entry. |
| `GetNameByAddr(mac, nick)` / `GetAddrByName(nick, &size, buf)` | Nickname to MAC address and back. |
| `GetParameter(param)` | The current channel, group name, own nickname and BSSID. |
| `GetAdhocId(id)` | The product code passed to Init. |
| `CreateEnterGameMode`, `CreateEnterGameModeMin`, `JoinEnterGameMode`, `ExitGameMode`, `GetGameModeInfo` | See [Game mode](/docs/development/adhoc/game-mode). |

### Structs

`SceNetAdhocctlScanInfo`, one per group found:

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | `next` (pointer to the next entry) |
| 4 | 4 | channel |
| 8 | 8 | group name |
| 16 | 6 | BSSID |
| 22 | 2 | padding |
| 24 | 4 | mode: 0 = normal, 1 = game mode |

`SceNetAdhocctlPeerInfo`, one per peer (152 bytes):

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | `next` |
| 4 | 128 | nickname (NUL-terminated) |
| 132 | 6 | MAC address |
| 138 | 6 | padding / flags |
| 144 | 8 | `last_recv`: time (in microseconds) the peer was last heard from |

Games use `last_recv` to judge whether a peer is still alive.

`SceNetAdhocctlParameter` (148 bytes): channel (4), group name (8), nickname (128), BSSID (6), padding (2).

### Behaviour on the PSP

Confirmed with the `net/adhoc/adhocctl` test in pspautotests, on one console:

- **Init**: the stack size must be at least 0xC00 (`STACKSIZE_TOO_SHORT` below that); a priority outside
  the thread priority range fails with the kernel's illegal-priority error (0x80020193); a NULL ad hoc ID
  or a type above 2 gives `INVALID_ARG`. The product code isn't checked.
- **A second Init** returns `ALREADY_INITIALIZED`, *and* leaves adhocctl uninitialized. After that, Init
  fails with 0x80410601 (the auth module's ALREADY_INITIALIZED) until the modules are reloaded.
- **Term** returns 0 even when not initialized. It returns before adhocctl's thread has stopped, so an
  Init right after it can be undone.
- **Handlers**: at most 4 (`TOO_MANY_HANDLERS` after that). The same function can be registered more than
  once, under a new ID, and is then called once per registration. NULL gives `INVALID_ARG`; deleting an
  unknown ID gives `ID_NOT_FOUND`.
- **Group names**: letters and digits only (a dash, space or underscore gives `INVALID_ARG`). A NULL name is
  accepted.
- **While connected**, Create, Connect and Scan all return 0, then call the handler with ERROR and
  `ALREADY_CONNECTED`.
- **While disconnected**, `GetParameter` gives `DISCONNECTED`, `GetPeerList` gives 0x80410682, and
  `Disconnect` returns 0.
- **`GetPeerInfo`** for the console's own MAC gives `NO_ENTRY`: the console isn't its own peer.
- **`sceNetGetLocalEtherAddr`** fails with 0x80410180 until a scan or group operation has started.
- **Between two consoles** (the `net/adhoc/pair/basic` test): the client's scan finds the host's group;
  after joining, each lists the other in `GetPeerList` with its nickname and a non-zero `last_recv`, and
  both report the same BSSID. `GetPeerInfo` with a buffer smaller than the struct gives `INVALID_BUFLEN`
  (0x80410704). `GetNameByAddr` and `GetAddrByName` find the peer; `GetNameByAddr` for the console's own
  MAC gives `NO_ENTRY`.
- **When a peer leaves**, it disappears from the others' peer lists within a few seconds, without any
  handler event.
- Creating a group alone, scanning, connecting and disconnecting each raise their event once. Scanning
  with nobody around finds nothing, not even the group the console just left.

### Limits

- Group name: up to 8 characters, letters and digits.
- Nickname: 128 bytes including the NUL. It's the system nickname from the PSP's settings.
- Game mode supports up to 16 members.

### Error codes

| Code | Name |
|---|---|
| 0x80410B01 | NOT_LEFT_IBSS |
| 0x80410B02 | ALREADY_CONNECTED |
| 0x80410B03 | WLAN_SWITCH_OFF |
| 0x80410B04 | INVALID_ARG |
| 0x80410B05 | TIMEOUT |
| 0x80410B06 | ID_NOT_FOUND |
| 0x80410B07 | ALREADY_INITIALIZED |
| 0x80410B08 | NOT_INITIALIZED |
| 0x80410B09 | DISCONNECTED |
| 0x80410B0A | NO_SCAN_INFO |
| 0x80410B0B | INVALID_IBSS |
| 0x80410B0C | NOT_ENTER_GAMEMODE |
| 0x80410B0D | CHANNEL_NOT_AVAILABLE |
| 0x80410B0E | WLAN_BEACON_LOST |
| 0x80410B0F | WLAN_SUSPENDED |
| 0x80410B10 | BUSY |
| 0x80410B11 | CHANNEL_NOT_MATCH |
| 0x80410B12 | TOO_MANY_HANDLERS |
| 0x80410B13 | STACKSIZE_TOO_SHORT |

## The netconf dialog

Instead of driving adhocctl itself, a game can show the system's network dialog:
`sceUtilityNetconfInitStart(params)`, then `sceUtilityNetconfUpdate` every frame until
`sceUtilityNetconfGetStatus` reports FINISHED (3), then `sceUtilityNetconfShutdownStart`. For ad hoc, the
`action` field is:

| Action | Meaning |
|---|---|
| 2 | CONNECT_ADHOC: connect to the group, creating it if needed |
| 4 | CREATE_ADHOC: create the group |
| 5 | JOIN_ADHOC: scan for the group and join it |

The parameters point to an `adhocparam` struct with the group name (8 bytes) and a timeout. The dialog
calls the adhocctl functions itself and finishes once the console is connected (or the user cancels).

## How PPSSPP emulates it

There's no radio to scan, so everything adhocctl would learn from the air comes from the
[adhoc server](/docs/development/adhoc/adhoc-server) instead. The pieces:

| Piece | Kind | Does |
|---|---|---|
| `friendFinder` | host thread | Logs in to the adhoc server, pings it, receives its messages and updates the peer list, the group list and the BSSID. Queues events for the game. |
| `AdhocThread` | PSP thread, created by `sceNetAdhocctlInit` with the game's priority and stack size | Calls the game's handlers. |
| `peerlock` | mutex | Guards the peer (`friends`) and group lists. |

### Identity

- **Nickname**: the Nickname setting (default "PPSSPP").
- **MAC address**: the MAC address setting, randomly generated on first use, with the low two bits of the
  first byte cleared (a unicast, globally administered address). Second and later instances running on
  the same machine get `00:02:02:02:02:02`, `00:03:03:03:03:03` and so on.
- **Product code**: taken from `sceNetAdhocctlInit` as-is. The server groups players by it. Servers can
  *crosslink* regional product codes so that, for example, the European and American versions of a game
  meet.
- **Channel**: the WLAN channel setting. "Automatic" is reported to the game as channel 11, because some
  games ignore channel 0.

### Logging in

`sceNetAdhocctlInit` starts `AdhocThread` and `friendFinder`. If WLAN is enabled, it then blocks the
calling thread until `friendFinder` has logged in to the server, or 5 seconds have passed (it then returns
BUSY). Some games need the server connection to be up by the time Init returns.

`friendFinder` resolves the server name, connects to port 27312 (TCP, or UDP with ENet if "Connect to adhoc
server using UDP" is on), and sends LOGIN with the MAC address, nickname and product code. Every 2 seconds
it sends a PING; the server logs out clients that stay silent for 15 seconds. It handles one message from
the server every 10 ms.

If the connection to the server drops, an on-screen message says so and all peers are marked as gone. The
connection is re-established the next time the game scans or creates a group.

### Operation by operation

| Game calls | PPSSPP does |
|---|---|
| `Scan` | Sets state SCANNING and sends SCAN. The server answers with one entry per group of this product code (name and host MAC), then SCAN_COMPLETE, which raises the SCAN event. The channel and mode in the scan results come from local settings, because the server doesn't track them. |
| `Connect` / `Create` / `Join` | Sends CONNECT(group name). The server sends this player the nickname, MAC and IP address of every member already in the group, sends every member the same for this player, and finally sends CONNECT_BSSID with the group creator's MAC, which raises the CONNECT event. Join only uses the group name from the scan result: the server can't tell the three apart. |
| `Disconnect` | Sends DISCONNECT. The server tells the remaining members (by IP address). The DISCONNECT event is raised even if the server can't be reached. |
| `GetPeerList`, `GetPeerInfo`, `GetNameByAddr`, `GetAddrByName` | Answered from the local peer list. |
| `GetScanInfo` | Answered from the last scan's group list. |

The calls that talk to the server put the calling thread to sleep until the request is sent (not until
it's answered), retrying every 500 µs of emulated time for up to 5 seconds of real time. If another
request is already pending they return BUSY.

### Peers

The peer list holds every player the server has reported in the group, with their nickname, MAC, IP
address as the server saw it, and `last_recv`.

- `last_recv` is updated whenever data arrives from that peer.
- Peers are never removed from the list during a session. A peer that left (or a server disconnect) only
  sets its `last_recv` to 0, and such peers are skipped by `GetPeerList` and by broadcasts. Matching code
  may still need their data.
- **`GetPeerList` reports `last_recv` as at most 10 ms old** for every live peer. Several games (Falcom
  titles, Dissidia 012) drop peers they think have gone quiet, and on the internet, quiet gaps are normal.
- The player's own console isn't in the list, as on the PSP.

### Event delivery and delays

Events from `friendFinder` and from the HLE functions go into a queue. `AdhocThread` runs a loop that
takes one event at a time, waits a while, changes the adhocctl state, and then calls every registered
handler with the event:

| Event | Delay |
|---|---|
| CONNECT after Create | 2 s |
| CONNECT after Connect | 1 s |
| CONNECT after Join | 100 ms (but held back until at least one peer is known, for up to 5 s) |
| GAME | 2 s |
| DISCONNECT | 10 ms |
| ERROR | 30 ms |
| anything else | 100 ms |

The delays stand in for how long the real operations take. Several games break if CONNECT arrives too
early, before the other side is ready, or too late. The values are tuned against games, not measured.

### Game-specific behaviour

These are hard-coded rather than per-game flags:

- `Scan` or `Create` while already connected raise an ERROR(ALREADY_CONNECTED) event and return 0
  (Valhalla Knights 2).
- `Create` without a running `friendFinder` (offline) fakes success and goes straight to CONNECTED
  (Full Auto 2).
- `Disconnect` always raises the DISCONNECT event (God Eater freezes otherwise) and takes about a frame
  (16.7 ms) to return (Tekken 6).
- An invalid group name returns `0x80410711`, the sceNetAdhoc INVALID_ARG code.

### Not emulated

- DISCOVER and WOL: never entered, and `sceNetAdhocDiscover*` is a stub that never finds anyone.
- Channels: there's no channel to match, so CHANNEL_NOT_MATCH and similar never occur.

### The netconf dialog in PPSSPP

`Core/Dialog/PSPNetconfDialog.cpp` shows "Connecting..." with the channel, then calls `sceNetAdhocctlCreate`
(CREATE_ADHOC), `sceNetAdhocctlConnect` (CONNECT_ADHOC), or, for JOIN_ADHOC, scans, looks for the group in
the results (scanning again if it isn't there yet) and calls `sceNetAdhocctlJoin`. It finishes when the
state reaches CONNECTED. The timeout is fixed at 15 seconds regardless of what the game asks for; Cancel
is offered while joining and after a timeout.
