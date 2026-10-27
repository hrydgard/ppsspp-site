---
position: 6
---

# sceNetAdhocMatching

The matching library is a ready-made lobby protocol on top of PDP. A host advertises itself, clients see
the advertisement, ask to join, and are accepted or rejected; the library then keeps the member list alive
and can send data to members. Plenty of games use it (Patapon 3, Gran Turismo, Dissidia 012, GTA: Vice
City Stories, ...); others build their own lobby over PDP broadcasts.

## API

### Modes

| Mode | Value | |
|---|---|---|
| HOST (parent) | 1 | Advertises, accepts up to `maxnum - 1` children |
| CLIENT (child) | 2 | Looks for hosts, joins one |
| P2P | 3 | Both advertise and look; pairs up with exactly one partner |

### Functions

| Function | Does |
|---|---|
| `Init(poolsize)` / `Term()` | Sets up the library's own memory pool. |
| `Create(mode, maxnum, port, rxbuflen, hello_int, keepalive_int, init_count, rexmt_int, callback)` | Creates a matching context on a PDP port. Intervals in µs. Returns its ID. |
| `Start(id, evthPrio, evthStack, inthPrio, inthStack, optlen, optdata)` | Starts the context's event and input threads. `optdata` is the "hello" data a host advertises (for example the room's name and settings). |
| `Start2(...)` | The same, with extra thread partition arguments. |
| `Stop(id)` / `Delete(id)` | |
| `SelectTarget(id, mac, optlen, opt)` | Client: ask to join that host. Host: accept that client's request. |
| `CancelTarget(id, mac)` / `CancelTargetWithOpt(id, mac, optlen, opt)` | Reject a request, withdraw one, or leave / kick. |
| `SetHelloOpt(id, optlen, opt)` / `GetHelloOpt(id, &optlen, buf)` | Change or read the hello data. |
| `GetMembers(id, &size, buf)` | The members: `{ next; mac[6]; pad[2] }`, 12 bytes each. |
| `SendData(id, mac, len, data)` / `AbortSendData(id, mac)` | Send data to an established member. |
| `GetPoolStat(stat)` / `GetPoolMaxAlloc()` | Memory pool statistics. |

The callback is `callback(int id, int event, u8 *mac, int optlen, void *opt)`:

| Event | Value | Meaning |
|---|---|---|
| HELLO | 1 | A host's advertisement was received; `opt` is its hello data |
| REQUEST (JOIN) | 2 | A client asks to join (host side) |
| LEAVE (LEFT) | 3 | A member left |
| DENY (REJECT) | 4 | The host rejected our request |
| CANCEL | 5 | A client withdrew its request |
| ACCEPT | 6 | The host accepted our request |
| ESTABLISHED (COMPLETE) | 7 | Matching with that peer is complete |
| TIMEOUT | 8 | A peer stopped responding |
| ERROR | 9 | |
| BYE (DISCONNECT) | 10 | A peer stopped matching |
| DATA | 11 | Data from `SendData` arrived |
| DATA_ACK (DATA_CONFIRM) | 12 | Our `SendData` was acknowledged |
| DATA_TIMEOUT | 13 | Our `SendData` was never acknowledged |

### A typical session

```
  host                                 client
  Create(HOST) + Start(hello data)     Create(CLIENT) + Start
  ---- HELLO (broadcast, every hello_int) ---->   event HELLO
                                       SelectTarget(host)
  event REQUEST  <---- join request ----------
  SelectTarget(client)
  ---- accept ------------------------------->   event ACCEPT
  event ESTABLISHED                              event ESTABLISHED
  <---- keep-alive, both ways, every keepalive_int ---->
  SendData / event DATA, both ways
```

### Error codes

| Code | Name | Code | Name |
|---|---|---|---|
| 0x80410801 | INVALID_MODE | 0x8041080D | TARGET_NOT_READY |
| 0x80410802 | INVALID_PORT | 0x8041080E | EXCEED_MAXNUM |
| 0x80410803 | INVALID_MAXNUM | 0x8041080F | REQUEST_IN_PROGRESS |
| 0x80410804 | RXBUF_TOO_SHORT | 0x80410810 | ALREADY_ESTABLISHED |
| 0x80410805 | INVALID_OPTLEN | 0x80410811 | BUSY |
| 0x80410806 | INVALID_ARG | 0x80410812 | ALREADY_INITIALIZED |
| 0x80410807 | INVALID_ID | 0x80410813 | NOT_INITIALIZED |
| 0x80410808 | ID_NOT_AVAIL | 0x80410814 | PORT_IN_USE |
| 0x80410809 | NO_SPACE | 0x80410815 | STACKSIZE_TOO_SHORT |
| 0x8041080A | IS_RUNNING | 0x80410816 | INVALID_DATALEN |
| 0x8041080B | NOT_RUNNING | 0x80410817 | NOT_ESTABLISHED |
| 0x8041080C | UNKNOWN_TARGET | 0x80410818 | DATA_BUSY |

## Sony's protocol

The firmware's matching module sends these packets over PDP on the context's port. Every packet starts with
a 4-byte header, and all integers are big-endian:

| Offset | Size | Field |
|---|---|---|
| 0 | 1 | version, always 1 (other versions are dropped) |
| 1 | 1 | opcode |
| 2 | 2 | length of the rest of the packet |

| Opcode | Packet | Notes |
|---|---|---|
| 1 | Hello | Broadcast. Carries `hello_int` and `keepalive_int`, then the hello data, then a trailer. Sent every `hello_int` while the host has room (members + 1 < `maxnum`); a full host stops advertising. Hosts ignore hellos. |
| 2 | Join request | Sent by `SelectTarget`. Opt data followed by a 16-byte trailer with a session ID. |
| 3 | Accept | Sent by the host's (or P2P partner's) `SelectTarget`, with the same kind of trailer. Raises ACCEPT and ESTABLISHED. |
| 4 | Accept confirmation | The client's reply to an accept |
| 5 | Cancel | Reject, withdraw, leave, or "you timed out". Raises CANCEL, DENY or LEAVE depending on the peer's state. |
| 6 | Member list | Host to each child: the host's MAC and every member's MAC. Resent every `keepalive_int`; this is the keep-alive. |
| 7 | Member list reply | Child to host |
| 8 | Bye | Broadcast when matching stops. Raises BYE. |
| 10 | Data | Session ID, sequence number, then the data |
| 11 | Data acknowledgement | Echoes the sequence number. Raises DATA_ACK. |

### Timers

- `Create` checks: mode 1-3, `maxnum` 2-16 (a client always uses 2), `rxbuflen` at least
  `6 * maxnum + 4`, and in HOST and P2P mode non-zero `hello_int` and `keepalive_int`; `rexmt_int` must be
  non-zero and `init_count` not negative. No defaults are filled in.
- Each established peer has a counter, reset to `init_count` whenever anything arrives from it and
  decremented every `keepalive_int`. When it reaches 0 the peer is sent a cancel and the game gets
  TIMEOUT. So a peer times out after about `keepalive_int * init_count` of silence.
- Join requests and accepts are resent every `rexmt_int`, up to `init_count` times, then time out.
- Data is resent every `rexmt_int` until acknowledged, up to `init_count` times, then DATA_TIMEOUT.

## The aemu protocol (what PPSSPP sends)

PPSSPP implements the matching library itself (the firmware module is never loaded), using the packet
format of the aemu plugin. This is a different protocol from Sony's, so **PPSSPP's matching works with
other PPSSPPs and with PSPs running aemu, but not with the real module.** The event numbers the game sees
are the same.

Packets are an opcode byte followed by little-endian 32-bit lengths, unaligned:

| Opcode | Packet | Layout |
|---|---|---|
| 0 | PING | `[0]` |
| 1 | HELLO | `[1] [s32 len] [hello data]` |
| 2 | JOIN | `[2] [s32 len] [opt]` |
| 3 | ACCEPT | `[3] [s32 optlen] [s32 siblingcount] [opt] [6-byte MAC x siblingcount]` |
| 4 | CANCEL | `[4] [s32 len] [opt]` |
| 5 | BULK (data) | `[5] [s32 len] [data]` |
| 6 | BULK_ABORT | never sent |
| 7 | BIRTH | `[7] [6-byte MAC]`: host to children, a new child joined |
| 8 | DEATH | `[8] [6-byte MAC]`: host to children, a child left |
| 9 | BYE | `[9]` |

Differences from Sony's protocol:

- No version byte, no length header, no session IDs or sequence numbers.
- Keep-alive is a 1-byte PING, not the member list. The member list is sent once in the ACCEPT, then
  maintained with BIRTH and DEATH.
- **Nothing is retransmitted.** JOIN and ACCEPT are sent once, data isn't acknowledged by the peer:
  DATA_ACK is generated locally right after `SendData`, and DATA_TIMEOUT never happens.
- HELLO and PING go to every peer in the adhocctl peer list, as individual datagrams, rather than as one
  broadcast.

## How PPSSPP implements it

### Threads

```
  game thread            host thread             host thread             PSP thread
  SelectTarget etc. ---> MatchingInput           MatchingEvent           MatchingThr<id>
  (pushes messages)      sends HELLO/PING,       every 10 ms, moves      loops: take one event,
                         sends queued packets,   events to the global    call the game's callback,
                         receives and handles    callback queue          sleep 3 ms
                         packets, timeouts
```

- **MatchingInput** (a host thread per context) sends HELLO every `hello_int` (host with room, or P2P
  without a partner) and PING every `keepalive_int`, checks timeouts, sends the packets queued by the HLE
  functions, and reads and handles incoming packets. It loops every 10 ms.
- **MatchingEvent** (a host thread per context) moves the events produced by MatchingInput into a global
  queue, every 10 ms, with the opt data copied into PSP memory.
- **MatchingThr&lt;id&gt;** (a PSP thread per context, created by `Start` with the game's event thread
  priority and stack size) takes one event at a time and calls the game's callback with it, then sleeps
  3 ms. Callbacks are never run concurrently. The 3 ms spacing was tuned: Super Pocket Tennis and
  Thrillville need about 5 ms or less, Gundam: Senjou no Kizuna Portable with 4 players needs about 3 ms.
- `SendData` sends directly from the calling thread.

The PDP socket is created by `Start`, not `Create`; creating it earlier left Dissidia 012 unable to see
any rooms. In relay mode, it's a reliable relay socket.

### Peer states

| State | |
|---|---|
| OFFER (1) | We've seen this host's (or P2P peer's) HELLO |
| PARENT (2) | Our host (client side) |
| CHILD (3) | An accepted child (host side), or a sibling (client side) |
| P2P (4) | Our P2P partner |
| INCOMING_REQUEST (5) | They sent us JOIN |
| OUTGOING_REQUEST (6) | We sent them JOIN |
| CANCEL_IN_PROGRESS (7) | We cancelled |

### Timeouts

A peer times out when nothing has been heard from it for
`(keepalive_int + rexmt_int) * init_count + 500 ms`, with the extra 500 ms for internet latency. Only PING
(and the peer's creation) counts as hearing from it in matching. `GetMembers` additionally refreshes each
member's timer from the adhocctl peer list, so a member that is still sending other traffic doesn't time
out while the game keeps polling it.

A `keepalive_int` below 1 is replaced with 2 seconds. (On the PSP it's an error in HOST and P2P mode.)

### Member order

`GetMembers` returns the host first, then the children in the order they joined, matching the host's own
view. Gran Turismo and Patapon 3 rely on all members agreeing on the order.

### Other game-driven behaviour

- `Start` on an unknown ID returns 0 (GTA: Vice City Stories).
- `CancelTarget` on a peer in a state that can't be cancelled returns 0 (Soul Calibur).
- A client ignores HELLOs from a host while a join is in progress (Super Pocket Tennis).
- In P2P mode, a rejected request leaves the peer in the list so it times out normally (Crazy Taxi).
- `Stop` sends BYE to all members. (The firmware may send DEATH-like packets instead, but that breaks
  Dragon Ball Z Tag Team.)
- `GetMembers` counts one extra member in client mode (Ys vs. Sora no Kiseki).

### Savestates

Matching contexts aren't saved. A savestate taken during matching loads with no contexts, and the game's
next matching call on its old ID fails. Only the library's initialized flag and its thread IDs are saved.
