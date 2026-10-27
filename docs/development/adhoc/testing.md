---
position: 11
---

# Testing

The behaviour in these pages is pinned down by pspautotests recorded on real PSPs. This page lists them,
says how to run them on hardware and on PPSSPP, and how PPSSPP currently does.

## The tests

Single console (`tests/net/adhoc`, the console creates its own group; needs the WLAN switch on, nothing
else):

| Test | Covers |
|---|---|
| `pdp` | PDP argument checks, return values, timeouts, receiving from yourself, `GetPdpStat`, socket IDs |
| `ptp` | PTP argument checks, a connection to yourself, data, close, refusal, `GetPtpStat` |
| `adhocctl` | Init, handlers, group names, create/connect/scan/disconnect, events, parameters |
| `matching` | matching Init and Create limits, calls on stopped and started contexts, hello data |
| `poll` | `PollSocket` events per socket state, socket alerts, waking blocked calls |
| `alerts` | which alert flag stops which PTP call, and waking threads already blocked |
| `gamemode` | game mode entry checks, members not turning up, the data calls outside game mode |
| `discover` | the sceNetAdhocDiscover sequence with nobody around |
| `self` | what one console can't do with itself (broadcasts, two matching contexts) |
| `loopback` | sceNetInet on 127.0.0.1 in every ad hoc state |
| `limits` | socket and context limits, port 0 |
| `apctl` | infrastructure and ad hoc at once; needs an access point in profile 1 (not for `test.py`) |

Single console, sceNetInet over loopback (`tests/net/inet`): `errors`, `options`, `addr`, `udp`, `tcp`,
`bind`, `blocking`, `poll`, `stats` (the control block lists, `SocketAbort`, `sendmsg`/`recvmsg`), and
`offline`: the sceNet utilities, apctl with no access point (handlers, scanning, missing profiles) and the
resolver with no network. `offline`'s scan finds whatever access points are around, so it only prints
whether there were any.

Every test links `tests/net/net-imports.S`, which declares every known function of the networking
libraries, so a new test can call any of them.

Two consoles (`tests/net/adhoc/pair`, `tests/net/inet/pair`), each a `_host` and `_client` program:

| Test | Covers |
|---|---|
| `basic` | scanning and joining, peer list and lookups, PDP sizes and loss, broadcast, PTP transfer, leaving |
| `matching` | a full matching session: hello, join, accept, data and acks, cancel, bye |
| `matching2` | reject with opt, cancel before accept, hello changes, a full host, kicking, P2P, aborting data |
| `vanish` | a peer leaving the group with connections and a matching session open |
| `lifecycle` | the group outliving its creator, rejoining, two groups with the same name |
| `flow` | PTP buffering with a reader that doesn't read, PDP receive buffer sizes |
| `gamemode` | game mode between two consoles: entering, members, replicas updating |
| `inet/pair/lan` | two consoles on an access point: apctl info, broadcast discovery, TCP, UDP, DNS |

## Running them on PSPs

See [pspautotests-hardware.md](https://github.com/hrydgard/ppsspp/blob/master/docs/pspautotests-hardware.md)
in the PPSSPP repo: single-console tests run with `gentest.py`; two-console tests with
`tests/net/adhoc/pair/record.py`, which runs both sides at once through two USB bridges. All the results
were recorded on PSP-1000s with firmware 6.60 and 6.61, which gave the same output.

## Running them on PPSSPP

- Single-console tests: `python3 test.py net/adhoc/pdp` (they're in `tests_next`, since PPSSPP doesn't pass
  them yet). For the ad hoc ones, PPSSPPHeadless needs `--adhoc-server --realtime` to get anywhere.
- Two-console tests: `python3 Tools/adhoc_multi.py net/adhoc/pair/basic` runs one PPSSPPHeadless per role.
  In P2P mode (the default), instance *N* uses 127.0.0.*N*, which on macOS needs
  `sudo ifconfig lo0 alias 127.0.0.2 up`. With `--relay`, the instances use a local aemu_postoffice
  server (built from `ext/aemu_postoffice/server_cpp`) in relay mode instead, and need no aliases.

## Where PPSSPP stands

As of October 2026, lines that differ from the PSP's output (or where the run stops early):

| Test | Result |
|---|---|
| `adhoc/pdp` | runs to the end; argument checks, broadcasts to itself, partial copies differ |
| `adhoc/ptp` | runs to the end; argument checks, connect results, stats differ |
| `adhoc/adhocctl` | stops after the Init checks: Init blocks for the server and returns BUSY |
| `adhoc/matching` | 52 of 100 lines: Create checks, Start's return value, hello data |
| `adhoc/poll` | 14 of 65: most of the per-state events |
| `adhoc/alerts` | 3 of 31: connect and flush alerts, the poll alert's `revents` |
| `adhoc/gamemode` | stops in game mode, stuck BUSY |
| `adhoc/discover` | 9 of 29: it's a stub |
| `adhoc/self` | 3 of 27: Start's return value |
| `adhoc/loopback` | 4 of 31: TCP over loopback fails |
| `adhoc/limits` | 8 of 26: limits, port 0, error codes |
| `inet/errors` | 35 of 76: errno values, return values, bind/listen rules |
| `inet/options` | 27 of 89: option sizes and values |
| `inet/addr` | 10 of 33: only dotted quads parsed |
| `inet/udp` | 3 of 61 |
| `inet/tcp` | stops at the first TCP connection over loopback |
| `inet/offline` | most lines: apctl and resolver error codes, limits, `ThreadAbort` |
| `inet/stats` | the control block lists aren't implemented; `recvmsg` and `SocketAbort` fail |

The two-console tests, run on two PPSSPPs in relay mode, all fail; the differences are listed under
[Known issues](/docs/development/adhoc/improving).
