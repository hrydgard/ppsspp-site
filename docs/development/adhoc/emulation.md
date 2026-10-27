---
position: 10
---

# Emulation details

Cross-cutting parts of PPSSPP's ad hoc emulation: threads, how blocking calls work, timing, ports and UPnP,
running several instances, settings, and savestates.

## Threads

| Thread | Kind | Work |
|---|---|---|
| Emulator CPU thread | | All HLE calls and timer callbacks, including every socket operation on game sockets (always non-blocking). |
| `friendFinder` | host | Talks to the adhoc server: login, ping, scans, peer joins and leaves, chat. Updates the peer and group lists, queues adhocctl events. |
| `AdhocThread` | PSP | Calls the game's adhocctl handlers. Created by `sceNetAdhocctlInit`. |
| `MatchingInput`, `MatchingEvent` | host, per matching context | See [matching](/docs/development/adhoc/matching#how-ppsspp-implements-it). |
| `MatchingThr<id>` | PSP, per matching context | Calls the game's matching callback. |
| Built-in server | host | Only if enabled. |
| ENet pump | host | Only with UDP on: services all ENet connections ~120 times a second. |
| Relay connect | host, per connecting PTP socket | The relay library's blocking connect. |
| `UPnPService` | host | Router discovery and port mappings. |

Handlers and callbacks run on PSP threads created for the purpose, because the game expects them to run in
a thread context with the priority and stack size it asked for. The PSP threads run small MIPS loops that
call a fake syscall (`__NetTriggerCallbacks`, `__NetMatchingCallbacks`), which picks the next event and
queues the game's function to be called.

## Blocking calls

Every host socket is non-blocking. When the game makes a blocking call (no `ADHOC_F_NONBLOCK`) that
can't complete immediately, PPSSPP:

1. Records the request (which socket, what operation, the buffers, the timeout, the start time).
2. Puts the calling PSP thread to sleep.
3. Schedules a timer event that retries the operation, first after one CPU cycle, then every **500 µs of
   emulated time**.
4. When the operation succeeds, fails, or the timeout passes, wakes the thread with the result.

Each retry also checks whether the socket was deleted (`SOCKET_DELETED`) or alerted (`SOCKET_ALERTED`). An
alert set while a thread is already blocked takes effect at the next retry.

The adhocctl calls that need the server (Scan, Create, Connect, Join, Disconnect, and Init's login) block in
the same way, until the request has been sent.

### Timeouts

- Timeouts are measured in **real (wall-clock) time**, not emulated time. A game running slower than real
  time still sees its network timeouts in real seconds, which keeps it in step with the other players.
- The "Minimum timeout" setting raises any non-zero blocking timeout to at least that many milliseconds,
  for games whose timeouts are too short for internet latency.
- A timeout of 0 waits forever.

### Call costs

Most socket calls take 50 µs of emulated time; a few have longer costs or delays to match what games expect
(`Disconnect` ~16.7 ms, `GetScanInfo` 200 µs, Init/Term 10 ms).

## Two clocks

- **Emulated time**: peers' `last_recv`, PTP connect attempt timing, game mode timestamps, matching
  timeouts.
- **Wall-clock time**: blocking timeouts, the server ping, the game mode join timeout, the adhocctl request
  timeout.

## Ports

- **Port offset** (default 10000): added to every port PPSSPP binds or sends to, in both P2P and relay
  mode, and subtracted from every port it receives from. Games use low ports, which need root on Linux and
  Android (below 1024), and the offset moves them out of that range. **All players must use the same
  offset.**
- **UPnP**: with "Enable UPnP" on, PPSSPP asks the router to forward each PDP socket's port, each PTP
  listener's port, and each PTP socket with a fixed local port, plus 27312 when running the built-in server.
  Mappings are named `PPSSPP:<game ID>:<nickname>`, and are only removed when UPnP is turned off or PPSSPP
  exits (removing them as sockets close made slow routers stall).
- **"UPnP use original port"**: asks the router to map the original (un-offset) port to port + offset, and
  sends to players on public IP addresses without the offset. This allows playing with real PSPs, which
  don't use an offset.

## Several instances on one machine

For testing, several PPSSPP instances can play together on one computer:

1. Enable the built-in server in one of them, and set the server address to `localhost` in all of them.
2. Each instance gets its own address, 127.0.0.*N* for instance *N*, and binds all its sockets to it, so
   they can all use the same ports. The second and later instances also get a MAC address derived from
   *N*.

On macOS, only 127.0.0.1 exists by default; the others need adding (`sudo ifconfig lo0 alias 127.0.0.2 up`).

In relay mode the instances don't bind per-instance addresses, so with a local aemu_postoffice server they
can all share 127.0.0.1.

PPSSPPHeadless has options for this, used by `Tools/adhoc_multi.py` to run the two-console pspautotests:
`--instance=N` (which instance this is), `--realtime` (run at PSP speed; otherwise headless runs as fast as
it can and the instances' clocks drift apart from the network's), and `--adhoc-server`.

## Settings

All are in Settings > Networking, and most are per-game.

| Setting | Effect |
|---|---|
| Enable networking/WLAN | Master switch. Off: most ad hoc calls return -1 or WLAN_SWITCH_OFF, nothing connects. |
| Ad hoc server address | The adhoc server (and relay) host. `localhost` or `127.*` for the built-in server. |
| Try to use server-provided packet relay | Auto / Yes / No. See [The relay](/docs/development/adhoc/relay#choosing-the-mode). |
| Connect to adhoc server using UDP (ENet) instead of TCP | ENet for both the adhoc server and relay connections. |
| Reliable datagram packet delivery on packet relay in UDP mode | Off: relayed PDP may lose packets, as on a PSP. |
| Enable built-in ad hoc server | Run the built-in server on port 27312. |
| Nickname | The nickname other players see. |
| MAC address | The virtual MAC address. |
| WLAN channel | Reported to games; Automatic is reported as 11. |
| Port offset | See above. |
| Minimum timeout | See above. |
| Forced first connect | See [PDP and PTP](/docs/development/adhoc/pdp-ptp#the-forced-first-connect-setting). |
| Enable UPnP, UPnP use original port | See above. |
| Allow speed control / savestates while connected | Normally blocked while connected, because they break the session. |

## Savestates

Network state mostly can't be saved: sockets, connections to the server and the relay, the peer list, the
group, game mode buffers and matching contexts all live outside the emulated PSP. Savestates are therefore
blocked while connected, unless explicitly allowed.

What is saved: which libraries were initialized, the handler list, the product code, the adhocctl state and
connection type, the IDs of the helper threads and timer events, and pending adhocctl requests.

On load, PPSSPP closes every socket and brings the host side (relay setup, `friendFinder`) in line with the
loaded initialized flags, so that a state saved in a game's network menu can at least log in again. After
loading, the game's old socket IDs are invalid, blocked calls return `TIMEOUT`, and the server has to be
rejoined.
