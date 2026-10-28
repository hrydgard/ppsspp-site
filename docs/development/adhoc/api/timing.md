---
position: 8
---

# Timing

How long the calls take on a PSP, how long until their events arrive, and how fast data moves between
two consoles. Measured on PSP-1000s (firmware 6.60 and 6.61) side by side, by the pspautotests
`net/adhoc/timing` and `net/adhoc/pair/timing`, which print these numbers rather than check them.
Medians unless a range is given.

Games get these timings without thinking about them, and many depend on them: an emulator that
delivers an event much sooner or later, or blocks where the PSP returns, breaks some game.

## adhocctl

| | Returns after | Event after |
|---|---|---|
| `sceNetAdhocctlInit` | 14 ms | |
| `sceNetAdhocctlCreate` | 30 µs | CONNECT after 1.1 s |
| `sceNetAdhocctlJoin` | | CONNECT after 0.53 s; the creator is in the peer list after about 1.2 s |
| `sceNetAdhocctlConnect` | 20 µs | CONNECT after 3.3-3.8 s to an existing group, about 4.0 s (3.8-4.4) when it creates one |
| `sceNetAdhocctlScan` | 30 µs | SCAN after 1.0 s, with or without groups around |
| `sceNetAdhocctlDisconnect` | 20 µs | DISCONNECT after 105 ms |
| `sceNetAdhocctlCreateEnterGameMode`, `JoinEnterGameMode` | | GAMEMODE on all members at the same moment, once the last one has joined (3-4 s after a join a second after the create) |
| `sceNetAdhocctlExitGameMode` | | after 0.64 s |
| `sceNetAdhocctlTerm` | 0.25 ms | |

The state (`GetState`) changes when the event is delivered, not before, except that Scan goes to
SCANNING at once (right after Init, a moment later). **`sceNetAdhocctlGetParameter`
takes about 14 ms in a group**, every call; `GetState` takes 0.4 µs, `GetPeerList` and `GetScanInfo`
2-6 µs.

## Blocking calls and timeouts

Receives, accepts and polls with a timeout:

| Timeout | Waits |
|---|---|
| 1 µs | returns at once (about 8 µs) |
| 50 or 100 µs | about 380 µs (about 420 µs for `PollSocket`) |
| 500 µs and up | the timeout plus about 150 µs (190 µs for `PollSocket`) |

`sceKernelDelayThread` behaves the same way (anything under about 230 µs takes 230 µs), so this is the
kernel's thread wake-up granularity.

**Polling has to yield.** The networking threads run at the priority given to `sceNetInit`; a thread of
the same priority that loops on non-blocking calls without ever waiting starves them, and nothing is
sent or received. A wait of any length in the loop (even `sceKernelDelayThread(0)`) is enough.

## Call costs

| Call | Cost |
|---|---|
| `PdpCreate` / `PdpDelete` | 36 / 21 µs |
| `PdpSend`, non-blocking, unicast / broadcast | 38 / 12 µs |
| `PdpRecv` or `PtpRecv`, non-blocking, nothing there | 3.6 µs |
| `PtpSend`, non-blocking, 16 bytes | 184 µs |
| `PollSocket`, non-blocking | 23 µs |
| `PtpOpen` / `PtpListen` / `PtpClose` | 120 / 38 / 91 µs |
| `GetPdpStat`, `GetPtpStat` | 2-4 µs |
| `MatchingCreate` / `Start` / `Stop` / `Delete` | 93 / 580 / 530 / 22 µs |
| `MatchingSetHelloOpt` / `GetHelloOpt` / `GetMembers` | 27 / 7 / 1.5 µs |
| `GameModeUpdateMaster` / `UpdateReplica` | 64-98 / 7-11 µs |

## Between two consoles

| | Time |
|---|---|
| PDP round trip, 16 / 256 / 1024 / 1444 / 4096 / 8192 bytes | 4.1 / 4.9 / 8.0 / 9.4 / 19 / 33 ms |
| PTP connect (Open to established) | 4-8 ms |
| PTP round trip, 16 / 1024 / 8192 bytes | 6.3 / 10 / 44 ms |
| `PtpFlush` after sending 8192 bytes | 40 ms |
| Matching hellos | every `hello_int`, to within 2% |
| Matching join, `SelectTarget` to ESTABLISHED | 25 ms, plus `rexmt_int` for each lost packet |
| Matching `SendData` to DATA_ACK | 4.5 ms |
| Game mode: a replica changes | every 12.4 ms (6-9 ms at the shortest, 220 ms at worst), however often the master updates |

PDP takes about 2 ms each way plus about 1.8 µs per byte, about 4.5 Mbit/s of payload. A PDP round trip
measured with a blocking receive and with a polling loop gave the same times.
