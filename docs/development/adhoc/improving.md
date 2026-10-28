---
position: 12
---

# Known issues and improvements

Where PPSSPP's ad hoc and inet emulation differs from the PSP, measured by running the pspautotests
recorded on PSPs (see [Testing](/docs/development/adhoc/testing)). Every function's own differences are
listed in the [API reference](/docs/development/adhoc/api/conventions), under **PPSSPP**; this page groups
them by kind, adds the structural issues, and suggests an order to fix them.

Some differences are deliberate, to make games work over the internet. Changes to anything that goes on
the wire have to stay compatible with the aemu plugin on PSPs and Vitas, and with older PPSSPP versions,
since players mix them; the aemu connection part would keep using aemu_postoffice in any rewrite.

## What can't be the same

- **No radio.** Real ad hoc traffic is encrypted and authenticated with console keys, and PCs and phones
  can't send raw 802.11 IBSS frames anyway. Interoperating with unmodified PSPs is out of reach; the common
  ground is the aemu protocols.
- **Latency.** Ad hoc was designed for consoles a few meters apart, with latencies around a millisecond.
  Over the internet it's tens of milliseconds, and games with tight timing need slack. Some of PPSSPP's
  differences (longer matching timeouts, the faked peer `last_recv`, PTP connects that keep retrying instead
  of being refused) exist for that, and should stay, or become optional.
- **Savestates.** The other players' state can't be saved.

## Argument checking and error codes

The most widespread kind of difference, and the easiest to fix one function at a time. Typical cases:

- **Checks the PSP makes and PPSSPP doesn't:** PDP flags, port 65535, zero and multicast destinations, a
  foreign local MAC (PDP and PTP); adhocctl's Init stack size, priority and ad hoc ID, its group name rules
  and the 4-handler limit; matching's Create limits (`rxbuflen`, intervals, `init_count`, `rexmt_int`, the
  15-context limit); game mode's minimum member count.
- **Checks PPSSPP makes and the PSP doesn't:** PTP's zero sizes, backlog and broadcast destination; PDP's
  `bufsize` 0 or negative.
- **The wrong error code:** sceNetAdhoc codes where adhocctl ones belong (group names, game mode
  timeouts), `INVALID_ADDR` for NULL pointers that should be `INVALID_ARG`, `BUSY` for a refused relay
  connect.
- **The wrong order:** PPSSPP often checks the socket ID or the group before NULL pointers and flags, so a
  call with several bad arguments gets a different error.
- **Calls that succeed when they should fail**, or the other way round: `GetParameter`, `GetPeerList`,
  `GetGameModeInfo` and `GetHelloOpt` outside their state; `GetPeerInfo` and `GetNameByAddr` answering for
  the console's own MAC; matching `Stop`/`Delete`/`CancelTarget` on unknown IDs or peers returning 0;
  matching `Start` returning a thread ID.

## Behaviour

- **PDP:** PPSSPP receives its own broadcasts (the PSP never does), copies part of a datagram into a buffer
  that's too small (the PSP copies nothing), holds about ten times more in a receive buffer, and can't send
  65523-byte datagrams. Socket IDs are reused, and port 0 picks ports from 0x8000 up.
- **PTP:** connects to a port nobody listens on aren't refused (and can even report success); unaccepted
  connections don't show up in `GetPtpStat`; buffers are far larger, so a sender blocks much later.
- **Polling:** most of the PSP's per-state events are missing (FLUSH never, DISCONNECT and
  `EXCEPTION_EVENT` not for closed or refused sockets, INVALID not for bad IDs, ALERT never).
- **adhocctl:** `sceNetAdhocctlInit` blocks for up to 5 seconds logging in to the server and returns `BUSY`
  if it can't; a second Init doesn't break adhocctl as it does on the PSP. In the single-console test, the
  CONNECT event after Create didn't arrive within 5 seconds. Two groups created with the same name merge.
  The BSSID is the creator's MAC, not a random address.
- **Matching:** no retransmission (JOIN and ACCEPT are sent once), DATA_ACK generated locally (so no
  `DATA_BUSY` and no `DATA_TIMEOUT`), data events in reverse order (the event queue is a stack), a client
  lists the host first in `GetMembers`, members that left can stay listed, and the timeout is
  `(keepalive_int + rexmt_int) * init_count + 0.5 s` instead of about `keepalive_int * init_count`.
- **Game mode:** with two instances, the test hangs at buffer setup; a master of size 0 is accepted.
- **Discover:** a stub that never enters adhocctl's DISCOVER state.
- **Leaving:** in relay mode, PTP connections aren't broken when a peer leaves the group.
- **Infrastructure and ad hoc** can be used at the same time, which the PSP never allows. PPSSPP relies on
  that for a few games (`connect_adhoc_for_grouping`), so it should stay.

### sceNetInet

PPSSPP passes sceNetInet straight to the host's sockets, which shows:

- **errno values are the host's** (macOS: ECONNREFUSED is 61, not 111), and several failures return the
  errno value itself instead of -1.
- **TCP over loopback doesn't work**: the connection is made but never accepted, which stops the `tcp` and
  `blocking` tests.
- **Socket options:** booleans, SO_NBIO and the timeouts aren't written back by `getsockopt`; timeouts are
  host `timeval`s instead of the PSP's 4-byte microseconds; buffer sizes and limits are the host's.
- **Rules that differ:** `listen` on an unbound socket succeeds, binding an address in use succeeds,
  `shutdown` on an unconnected socket fails, descriptor numbers are reused, raw sockets are refused.
- **Address parsing** only accepts dotted quads, and reads `010` as decimal.

### Fixed

- `sceNetAdhocPdpSend` with NULL data, `sceNetAdhocPdpRecv` with a NULL sender, and `sceNetAdhocPtpSend`
  with a NULL length pointer stopped the game with a memory exception. They're valid calls on a PSP.

## Timing

Measured with the same timing programs as the [Timing](/docs/development/adhoc/api/timing) page, on
PPSSPPHeadless (the pairs over a local aemu_postoffice relay). Internet latency can justify PPSSPP being
*slower* than the PSP, and some of the differences are that, on purpose. The rest go the other way, or
have nothing to do with the network.

Allowances for latency:

- Matching timeouts get 500 ms extra, peers' `last_recv` is faked as fresh, PTP connects that would be
  refused retry with WOULD_BLOCK, and "Forced first connect" makes the first PtpConnect block. The
  minimum socket timeout setting is another, but it defaults to 0.
- A joiner's CONNECT waits until the server has reported a peer (or a timeout passes).
- **Blocking timeouts are measured on the host clock**, while the checks are scheduled in emulated time,
  which advances in bursts against it. Short timeouts get rounded up: in headless every blocking wait
  takes at least 16.5 ms, where the PSP's shortest is about 0.4 ms, and round trips between two
  instances come out at 31 ms for any size, against 4-33 ms on PSPs. Timing network waits against real
  time is right; the rounding isn't.

Not explained by latency:

- **CONNECT comes 2 s after Create** (PSP 1.1 s), a fixed delay tuned for Dissidia 012 and GTA: VCS.
- **Events come earlier than on the PSP:** SCAN after 0.13 s (PSP 1.0 s), Join's CONNECT after 0.15 s
  (PSP 0.53 s), DISCONNECT after 27 ms (PSP 105 ms), ExitGameMode's after 27 ms (PSP 0.64 s). Game mode
  members get GAMEMODE at different times; on the PSP all at once.
- **Calls that block and shouldn't:** Create, Scan and Disconnect take 10-17 ms to return (PSP
  20-30 µs), MatchingStart 30 ms (PSP 0.6 ms). The server round trips already happen on another thread.
- **Game mode replicas update every 0.65 ms**; on the PSP every 12 ms or so. That's 25 times the traffic.

The PSP's timings are the floor to model: nothing should arrive sooner than on a PSP, and the network's
latency adds to that. Several game-specific delays in the code exist to make up for these differences;
with the PSP's timings modelled, most should go.

## Structural issues

- **Players are identified by IP address** in P2P mode. Two players behind one public address can't both
  play, and a DISCONNECT from the server (which only carries the IP) removes the wrong one. Relay mode with
  a relay-only server doesn't have this problem.
- **Received ports with "UPnP use original port"**: sends to players on public addresses use no port
  offset, but received source ports always have the offset subtracted, so those players' ports are
  reported wrong by the offset.
- **The relay's PDP limit**: aemu_postoffice's client refuses PDP datagrams over 10 KiB, while a PSP
  delivers up to about 16 KB across the air. Raising it needs a change in aemu_postoffice.
- **Port offset in relay mode**: the relay doesn't need an offset, but PPSSPP applies one anyway, so all
  players still have to agree on it, and a mismatch silently loses all traffic.
- **Relay detection** depends on the server list's `data_mode`; the relay protocol has no way to ask a
  server what it supports.
- **Thread safety**: the adhocctl state and busy flags, the game mode member list and parts of the peer
  list are shared between `friendFinder` and the CPU thread without locks. The matching threads call HLE
  functions (PDP send and receive) directly from host threads.

## Where to go from here

Roughly in order of value:

1. **Make the single-console tests pass**, function by function, starting with the argument checks and
   error codes. Each fix is small and the tests say exactly what's expected. Keep the deliberate
   differences behind the internet-latency reasoning above.
2. **Run the adhocctl client on the CPU thread.** Replace the `friendFinder` thread with a CoreTiming
   event that polls the server connection (fetching events doesn't block) and owns the adhocctl state,
   the peer list and the game mode member list, so they're no longer shared without locks. Events are
   then scheduled in emulated time at the PSP's delays (see [Timing](/docs/development/adhoc/api/timing))
   instead of whenever a host thread wakes up, which replaces the tuned per-game delays. Connecting and
   logging in to the server can take seconds, so that stays on a worker thread: `sceNetAdhocctlInit`
   starts it and returns at once, and the poll event picks up the session when it's ready. Pings keep
   their interval in real time, so slow emulation doesn't get the player dropped. The built-in server and
   relay stay host threads, since they serve other machines.
3. **Polling events**, since games use `PollSocket` to drive their state machines.
4. **Matching**: a real event queue, real data acknowledgement (`DATA_ACK` only when the peer answers,
   `DATA_BUSY`, `DATA_TIMEOUT`), and retransmission of JOIN and ACCEPT, degrading gracefully against aemu
   clients that don't acknowledge.
5. **sceNetInet**: translate errno values and the return convention, socket options, and fix TCP over
   loopback.
6. **Game mode**: make two instances get through buffer setup.
7. **Make the two-console tests pass** on two instances, in both P2P and relay mode.
8. The structural issues: thread safety (what step 2 doesn't cover), IP identity in P2P mode, the original-port bug.

## Testing

- Two or more PPSSPP instances on one machine: `Tools/adhoc_multi.py` (see
  [Testing](/docs/development/adhoc/testing)).
- `sceNet` logging at debug level shows every ad hoc call with its arguments and result.
