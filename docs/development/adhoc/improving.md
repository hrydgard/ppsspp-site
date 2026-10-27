---
position: 10
---

# Known issues and improvements

Where PPSSPP's ad hoc emulation differs from the PSP, and what could make it better. Some differences are
deliberate and needed to make games work over the internet; others are simply not done yet. Changes to
anything that goes on the wire have to stay compatible with the aemu plugin on PSPs and Vitas, and with
older PPSSPP versions, since players mix them.

## What can't be the same

- **No radio.** Real ad hoc traffic is encrypted and authenticated with console keys, and PCs and phones
  can't send raw 802.11 IBSS frames anyway. Interoperating with unmodified PSPs is out of reach; the
  common ground is the aemu protocols.
- **Latency.** Ad hoc was designed for consoles a few meters apart, with latencies around a millisecond.
  Over the internet it's tens of milliseconds, and games with tight timing (game mode in particular, or PTP
  connects with short timeouts) need slack. The hard-coded event delays, the minimum timeout setting, the
  extra 500 ms on matching timeouts and the faked `last_recv` all exist for this.
- **Savestates.** The other players' state can't be saved, so savestates while connected will never work.

## Differences in the APIs

### PDP

| | PSP | PPSSPP |
|---|---|---|
| Send to a NULL destination | `INVALID_ARG` | `INVALID_ADDR` |
| Send flag bits other than `NONBLOCK` | `INVALID_ARG` | accepted |
| Send to port 65535 | `INVALID_PORT` | accepted |
| Datagram over 65523 bytes | `INVALID_DATALEN` | sent (if the host allows it) |
| Multicast destination other than broadcast | `INVALID_ADDR` | treated as an unknown peer: returns 0, nothing sent |
| Unknown destination MAC | sent into the air | returns 0, nothing sent |
| Create with flag 1 | `INVALID_ARG` | accepted |
| Create with `bufsize` 0 or negative | accepted | `INVALID_ARG` |
| Create on port 65535 | `INVALID_PORT` | accepted |
| Socket IDs | always increasing | lowest free slot |
| Return value of Send and Recv | 0 | 0 (same) |
| `PdpCreate` before joining a group | `INVALID_ADDR` | `INVALID_ADDR` (same) |
| Sender receives its own broadcast | no | no (same) |

The PSP checks the arguments in a fixed order (stack, then argument/flag, initialized, socket ID, address,
port, length); PPSSPP's order differs, which matters when a game passes several bad arguments at once.

### PTP

- P2P mode never returns `CONNECTION_REFUSED`; non-blocking connects return `WOULD_BLOCK` both while
  connecting and after a refusal, and a blocking connect times out with `TIMEOUT`. On the PSP a refused
  connect fails at once with `CONNECTION_REFUSED`. (Over the internet, "nobody is listening yet" and "the
  packet hasn't arrived yet" look alike, which is why PPSSPP keeps retrying.)
- `PtpOpen` and `PtpListen` reject 0 for `bufsize`, `rexmt_int`, `rexmt_cnt` and `backlog`, and `PtpOpen`
  rejects the broadcast address; the PSP accepts all of these.
- `PtpFlush` returns at once with 0.
- `SYN_RCVD` is never reported, and the send buffer count in `GetPtpStat` is always 0.
- The retransmit arguments only drive TCP keep-alive, and only in whole seconds, which usually rounds to 0.

### Adhocctl

- Scan results carry the channel and mode from local settings, not the group's.
- Peers that leave stay in the list (marked as gone), and live peers' `last_recv` is faked to be at most
  10 ms old.
- A dropped server connection isn't re-established until the game scans or creates a group again.
- DISCOVER and WOL aren't implemented.
- Event delays (1-2 s for CONNECT and GAME) are tuned, not measured.
- Up to 32 handlers instead of 4, and registering the same function again returns 0 instead of a new ID.
- A second `sceNetAdhocctlInit` leaves adhocctl initialized; on the PSP it de-initializes it.
- `GetPeerInfo` answers for the console's own MAC, which the PSP doesn't.

### Matching

The biggest gap. Compared with Sony's module:

- **No retransmission.** JOIN and ACCEPT are sent once; on a lossy P2P link a lost packet stalls matching
  until a timeout. (In relay mode the transport is reliable, so this mostly doesn't matter there.)
- **No real data acknowledgement.** `DATA_ACK` fires locally right after `SendData`, and `DATA_TIMEOUT`
  never fires.
- **Argument checks differ.** The PSP requires `rxbuflen >= 6 * maxnum + 4` (PPSSPP: at least 1), in
  HOST and P2P mode non-zero hello and keep-alive intervals (PPSSPP substitutes 2 s), a non-negative
  `init_count` and a non-zero `rexmt_int` (PPSSPP checks neither).
- **`Delete` and `Start` on a running context** give `IS_RUNNING` on the PSP; PPSSPP stops and deletes it,
  or starts it again.
- **`GetHelloOpt`** on the PSP copies at most the game's buffer size and leaves the length alone; PPSSPP
  writes the full length and copies all of it.
- **Timeouts** are `(keepalive_int + rexmt_int) * init_count + 500 ms` instead of about
  `keepalive_int * init_count`, which is more forgiving.
- **Event order**: the internal event and message queues are stacks (last in, first out), and the code
  pushes pairs in reverse to compensate. A real queue would be simpler and race-free.
- **`GetHelloOpt`** copies the hello data without checking the size of the game's buffer.
- The hello send buffer is shared between all matching contexts.
- A timed-out P2P peer can produce repeated TIMEOUT events, and cancelled peers linger in host mode.

### Game mode

- No time slots or lockstep, no sequence numbers, no retransmission after start-up; replicas just see the
  last datagram that arrived.
- The firmware's timing synchronization isn't emulated.
- `CreateEnterGameModeMin` ignores its minimum, and the joiner's GAME event doesn't wait for everyone.

## Structural issues

- **Players are identified by IP address** in P2P mode. Two players behind one public address can't both
  play, and a DISCONNECT from the server (which only carries the IP) removes the wrong one. Relay mode with
  a relay-only server doesn't have this problem.
- **Received ports with "UPnP use original port"**: sends to players on public addresses use no port offset,
  but received source ports always have the offset subtracted, so those players' ports are reported wrong
  by the offset.
- **Port offset in relay mode**: the relay doesn't need an offset, but PPSSPP applies one anyway, so all
  players still have to agree on it, and a mismatch silently loses all traffic. Dropping it would need
  coordinating with the aemu plugin and older PPSSPP versions.
- **Relay detection** depends on the server list's `data_mode`: in Auto mode, a relay-capable server that
  isn't in the list is used in P2P mode (and with a relay-only server, nothing connects). The relay
  protocol has no way to ask a server what it supports.
- **Thread safety**: the adhocctl state and busy flags, the game mode member list and parts of the peer list
  are shared between `friendFinder` and the CPU thread without locks. The matching threads call HLE
  functions (PDP send and receive) directly from host threads.
- **Relay mode details**: `PollSocket` ignores its timeout and never reports ALERT; accepted relay sockets
  get their internal ID from the listener's slot.
- **Socket IDs** are the lowest free slot, up to 255; the firmware hands out increasing IDs.

## Where to go from here

Roughly in order of value:

1. **Hardware tests.** pspautotests has single-console tests in `tests/net/adhoc` (`pdp`, `ptp`,
   `adhocctl`, `matching`, `self`), in `tests_next` for now. Fixing the differences they show (argument
   checks, socket IDs, handler limits) is the obvious next step. Still to cover: alerts and
   `PollSocket`, game mode, and, with a second PSP, scanning for another console, peer lists, and a real
   matching session with its event order. Whether apctl and adhocctl can be connected at the same time
   (they're said not to be) is worth a test too.
2. **Matching retransmission and acknowledgement**, implemented so that it degrades gracefully against
   aemu clients (for example, resending JOIN and ACCEPT on the existing opcodes, and treating a peer's data
   ack as optional). This matters most in P2P mode.
3. **Fix the matching code smells**: real queues instead of stacks, the `GetHelloOpt` bounds check, the
   shared hello buffer, the repeated P2P timeout.
4. **Make `friendFinder` reconnect on its own** after losing the server, instead of waiting for the game to
   scan or create a group.
5. **Move host-thread HLE calls** (matching's PDP traffic) onto the CPU thread, and lock the shared adhocctl
   state.
6. **Game mode over the relay**: the relay already makes PDP reliable; adding a sequence number to game
   mode datagrams would let replicas drop stale updates.
7. **Fix the original-port receive bug** and the relay `PollSocket` timeout.

## Testing

- Two or more PPSSPP instances on one machine with the built-in server on `localhost` is the quickest
  setup (see [Emulation details](/docs/development/adhoc/emulation#several-instances-on-one-machine)).
- For relay mode, run [aemu_postoffice](https://github.com/Kethen/aemu_postoffice/releases) locally and point
  PPSSPP at `127.0.0.1` with relay mode set to Yes.
- `sceNet` logging at debug level shows every ad hoc call with its arguments and result.
