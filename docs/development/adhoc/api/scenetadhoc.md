---
position: 4
---

# sceNetAdhoc

PDP (datagram) and PTP (stream) sockets, polling, socket alerts, and game mode's shared buffers. For
the concepts, see [PDP and PTP](/docs/development/adhoc/pdp-ptp) and
[Game mode](/docs/development/adhoc/game-mode). Tests: `net/adhoc/pdp`, `ptp`, `poll`, `self`, `limits`,
`gamemode`, and the `pair` tests.

## Common rules

- Sockets can only be created once the console is in a group (or game mode). Before that, creating one
  gives `INVALID_ADDR`, even with the console's own MAC.
- The local MAC must be the console's own (`INVALID_ADDR` otherwise, `INVALID_ARG` for NULL).
- PDP and PTP sockets share one ID space and one limit: **255 sockets** in all, then
  `SOCKET_ID_NOT_AVAIL` (0x8041070F).
- **Socket IDs always increase**; a freed ID isn't reused. A create that fails on the address, the port or
  a port in use uses up an ID too; one that fails on NULL or a bad flag doesn't.
- Local port 0 picks a free port, from 0x1000-0x7FFF.
- See [Conventions](/docs/development/adhoc/api/conventions) for `timeout` and `flag`.

**PPSSPP:** socket IDs are the lowest free slot; port 0 picks a port from 0x8000 up (the host OS's
choice minus the port offset); hitting the socket limit gives 0x80410001 or 0x80410710.

## sceNetAdhocInit / Term

```c
int sceNetAdhocInit(void); // NID 0xE1D621D7
int sceNetAdhocTerm(void); // NID 0xA62C6F57
```

`Init` needs `sceNetInit` first; twice gives `ALREADY_INITIALIZED` (0x80410713). `Term` returns 0, also
when called twice. Every other call before Init (or after Term) gives `NOT_INITIALIZED` (0x80410712).

## PDP

```c
typedef struct SceNetAdhocPdpStat {   // 20 bytes
	struct SceNetAdhocPdpStat *next;
	int id;
	SceNetEtherAddr laddr;
	u16 lport;
	u32 rcv_sb_cc;                     // bytes waiting in the receive buffer
} SceNetAdhocPdpStat;
```

### sceNetAdhocPdpCreate

```c
int sceNetAdhocPdpCreate(const SceNetEtherAddr *mac, u16 port, int bufsize, int flag); // NID 0x6F92741B
```

Creates a datagram socket on `port`. `bufsize` is the receive buffer in bytes: datagrams that don't fit
are dropped. Any value is accepted, including 0 (which receives nothing) and negative.

| Check | Error |
|---|---|
| `mac` NULL, `flag` not 0 | `INVALID_ARG` |
| not initialized | `NOT_INITIALIZED` |
| not in a group, `mac` not the console's own | `INVALID_ADDR` |
| `port` 65535 | `INVALID_PORT` |
| `port` in use | `PORT_IN_USE` (0x8041070A) |

**PPSSPP:** accepts a zero, broadcast or foreign MAC as the local address, port 65535 and any `flag`;
rejects `bufsize` 0 or negative with `INVALID_ARG`. Outside a group it checks the address before the flag.
The host buffer is 10 times `bufsize`, so a small buffer holds far more than on a PSP.

### sceNetAdhocPdpSend

```c
int sceNetAdhocPdpSend(int id, const SceNetEtherAddr *dst, u16 dport, const void *data, int len,
	u32 timeout, int flag); // NID 0xABED3790
```

Sends one datagram. **Returns 0 on success**, not the length.

| Check, in order | Error |
|---|---|
| `dst` NULL, `flag` not 0 or 1 | `INVALID_ARG` |
| not initialized | `NOT_INITIALIZED` |
| bad `id` | `INVALID_SOCKET_ID` |
| `dst` zero, or multicast other than `FF:FF:FF:FF:FF:FF` | `INVALID_ADDR` |
| `dport` 0 or 65535 | `INVALID_PORT` |
| `len` above 65523, negative, or non-zero with NULL `data` | `INVALID_DATALEN` |

- `len` 0 is fine, even with NULL `data`.
- Datagrams over 1444 bytes are fragmented on the air. Between two consoles, 16000 bytes arrive intact,
  32000 don't arrive at all; a 65523-byte send sometimes fails with 0x80410001.
- Sending to a MAC that isn't in the group, or to a port nobody has open, succeeds.
- Sending to the console's own MAC is delivered locally. **Its own broadcasts are not**, to any of its
  sockets.
- Datagrams can be lost: of 50 small ones sent back to back, a few usually are.
- Send queue full: `WOULD_BLOCK`, or a blocking send waits.

**PPSSPP:** NULL `dst` gives `INVALID_ADDR`; a zero or multicast destination and port 65535 are accepted;
NULL `data` with `len` 0 gives `INVALID_ARG`; the flag isn't checked; 65523 bytes give `WOULD_BLOCK` (the
host's UDP limit). An unknown MAC returns 0 and sends nothing. In relay mode, datagrams over 10 KiB can't
be sent.

### sceNetAdhocPdpRecv

```c
int sceNetAdhocPdpRecv(int id, SceNetEtherAddr *src, u16 *sport, void *buf, int *len,
	u32 timeout, int flag); // NID 0xDFE53E03
```

Receives one datagram. `*len` is the buffer size going in. **Returns 0 on success**, with `*len` the
datagram's size and the sender in `*src`/`*sport`.

- `src` and `sport` may be NULL. A NULL `buf` or `len`, or a bad flag, gives `INVALID_ARG` (before the ID
  is checked).
- Nothing waiting: `WOULD_BLOCK` or `TIMEOUT`, with `*len` left alone.
- **Buffer too small: `NOT_ENOUGH_SPACE` (0x80400706)**, with `*len` set to the datagram's size and the
  sender filled in. Nothing is copied, and the datagram stays queued for the next call.

**PPSSPP:** receives its own broadcasts; copies what fits when the buffer is too small; checks the socket
ID before NULL pointers; doesn't check the flag; a length of 0 or less with data waiting gives
`NOT_ENOUGH_SPACE`.

### sceNetAdhocPdpDelete

```c
int sceNetAdhocPdpDelete(int id, int flag); // NID 0x7F27BB5E
```

Deletes the socket. `flag` must be 0 (`INVALID_ARG`). An unknown ID, or deleting twice:
`INVALID_SOCKET_ID`. A thread blocked receiving on it wakes with `SOCKET_DELETED` (0x80410707).

**PPSSPP:** ignores the flag.

### sceNetAdhocGetPdpStat

```c
int sceNetAdhocGetPdpStat(int *size, SceNetAdhocPdpStat *buf); // NID 0xC7C1FC57
```

All PDP sockets, 20 bytes each. NULL `buf`: `*size` = bytes needed. NULL `size`: `INVALID_ARG`. A `*size`
too small for one entry returns 0 with `*size` 0.

The socket's `bufsize` caps what waits to be read; datagrams that don't fit are dropped. Ten 1000-byte
datagrams sent in a burst all arrive with a 32 KB buffer, only some with 2 KB, and none with a buffer of 0.

**PPSSPP:** nothing is dropped. The relay delivers in bursts that the radio spreads out, so enforcing the
limit would lose more than a PSP does.

## PTP

```c
typedef struct SceNetAdhocPtpStat {   // 36 bytes
	struct SceNetAdhocPtpStat *next;
	int id;                            // -1 for a connection that hasn't been accepted yet
	SceNetEtherAddr laddr, paddr;
	u16 lport, pport;
	u32 snd_sb_cc;                     // bytes queued but not yet sent
	u32 rcv_sb_cc;                     // bytes waiting to be read
	int state;                         // 0 CLOSED, 1 LISTEN, 2 SYN_SENT, 3 SYN_RCVD, 4 ESTABLISHED
} SceNetAdhocPtpStat;
```

`bufsize` on Open and Listen is each direction's buffer. With 8 KB buffers, a sender whose peer doesn't
read can queue 8-16 KB before sends return `WOULD_BLOCK`, and 16-32 KB in all is in flight.

### sceNetAdhocPtpOpen

```c
int sceNetAdhocPtpOpen(const SceNetEtherAddr *mac, u16 port, const SceNetEtherAddr *dst, u16 dport,
	int bufsize, u32 rexmt_int, int rexmt_cnt, int flag); // NID 0x877F6D66
```

Creates a socket and **starts connecting at once**, in the background. To a listener on the same console,
it's ESTABLISHED by the time Open returns.

| Check | Error |
|---|---|
| `mac` or `dst` NULL, `flag` not 0 | `INVALID_ARG` |
| `mac` not the console's own, `dst` zero | `INVALID_ADDR` |
| `dport` 0 or 65535 | `INVALID_PORT` |

`bufsize`, `rexmt_int` (µs) and `rexmt_cnt` aren't checked: 0 is accepted for each. The broadcast address
is accepted as a destination.

**PPSSPP:** rejects 0 for the sizes and the broadcast destination, accepts a foreign source MAC,
destination ports 0 and 65535, and any flag.

### sceNetAdhocPtpConnect

```c
int sceNetAdhocPtpConnect(int id, u32 timeout, int flag); // NID 0xFC6FC07B
```

Waits for (or, non-blocking, checks) the connection Open started. 0 once established, also when called
again. **A port nobody listens on: `CONNECTION_REFUSED` (0x80410718) at once**, blocking or not, within a
second across the air too.

A socket whose connect was refused stays refused: connecting it again gives `CONNECTION_REFUSED` even once
the peer listens. A connect made a second before the peer starts listening is refused the same way.

**PPSSPP:** connecting to a port nobody listens on can report success (it did between two sockets of one
instance); non-blocking calls otherwise return `WOULD_BLOCK` instead of `CONNECTION_REFUSED`. Over the
relay, a connect waits for a listener to appear, so it succeeds or times out instead of being refused.

### sceNetAdhocPtpListen

```c
int sceNetAdhocPtpListen(const SceNetEtherAddr *mac, u16 port, int bufsize, u32 rexmt_int,
	int rexmt_cnt, int backlog, int flag); // NID 0xE08BDAC1
```

A listening socket. As for Open: the MAC must be the console's own, `flag` 0, and the sizes and
`backlog` aren't checked. Until accepted, an incoming connection shows up in `GetPtpStat` with ID -1.

The backlog is how many connections can wait to be accepted. Their connects complete at once, before any
Accept. With backlog 1 and nobody accepting, a first connect succeeds and further ones get
`CONNECTION_REFUSED` after 1-2 seconds; only the first is accepted afterwards.

**PPSSPP:** over the relay, a connect completes only when the peer accepts it, and the backlog isn't
enforced.

### sceNetAdhocPtpAccept

```c
int sceNetAdhocPtpAccept(int id, SceNetEtherAddr *mac, u16 *port, u32 timeout, int flag); // NID 0x9DF81198
```

Returns a new socket ID for the next incoming connection, with the peer's MAC and port (both pointers may
be NULL). Nothing waiting: `WOULD_BLOCK` or `TIMEOUT`. On a socket that isn't listening: `NOT_LISTENED`
(0x8040070E).

**PPSSPP:** in relay mode, the accepted socket reports the wrong peer port in `GetPtpStat`.

### sceNetAdhocPtpSend / Recv

```c
int sceNetAdhocPtpSend(int id, const void *data, int *len, u32 timeout, int flag); // NID 0x4DA4C788
int sceNetAdhocPtpRecv(int id, void *buf, int *len, u32 timeout, int flag);        // NID 0x8BEA2B3E
```

Both return 0 on success, with `*len` the bytes sent or received. Recv returns whatever is buffered, up to
`*len` (100 bytes sent can be read as 30 then 70).

- Send: NULL `data` or negative `*len` gives `INVALID_DATALEN`, NULL `len` `INVALID_ARG`; `*len` 0 is
  fine.
- On a listener: `NOT_CONNECTED` (0x8041070B).
- After the peer closes, or leaves the group: `DISCONNECTED` (0x8041070C), and the state becomes CLOSED.
  Data the peer sent before closing is still received first. A blocking Recv is woken by the close, with
  `DISCONNECTED`.
- Buffers full: `WOULD_BLOCK`, or `TIMEOUT` for a blocking call.

Flow control: each side holds up to its `bufsize`. With 8 KB buffers on both sides and a receiver that
doesn't read, the sender gets 8-16 KB out before `WOULD_BLOCK`. The receiver's GetPtpStat then shows 8-16 KB
waiting and the sender's shows 8-16 KB not yet sent, and a blocking send times out. Once the receiver
reads, sending works again.

**PPSSPP:** NULL `data`, a length of 0 and a negative length all give `INVALID_ARG`; after the peer
closed, send gives `NOT_CONNECTED`. Buffers are much larger (a full sender queues 64 KB or more), so
`WOULD_BLOCK` comes far later. Over the relay, data still buffered when the peer closes is lost: Recv
gives `DISCONNECTED` at once. Flow control over the relay can't be fixed in the emulator alone: the sender would
have to hear from the receiver how much its game has read, and the relay protocol has no message for that.

### sceNetAdhocPtpFlush

```c
int sceNetAdhocPtpFlush(int id, u32 timeout, int flag); // NID 0x9AC2EEAC
```

Waits until sent data has gone out. Returns 0.

### sceNetAdhocPtpClose

```c
int sceNetAdhocPtpClose(int id, int flag); // NID 0x157E6225
```

Closes. `flag` must be 0. Twice: `INVALID_SOCKET_ID`. The peer's next send or receive gives
`DISCONNECTED`, and a socket closed this way disappears from the peer's `GetPtpStat`.

**PPSSPP:** ignores the flag.

### sceNetAdhocGetPtpStat

```c
int sceNetAdhocGetPtpStat(int *size, SceNetAdhocPtpStat *buf); // NID 0xB9685118
```

All PTP sockets, 36 bytes each, including connections not yet accepted (ID -1). Works like
`GetPdpStat`.

**PPSSPP:** doesn't list unaccepted connections; a connecting socket shows SYN_SENT for a while where the
PSP is already ESTABLISHED; `snd_sb_cc` is always 0.

## Polling and alerts

### sceNetAdhocPollSocket

```c
typedef struct { int id; int events; int revents; } SceNetAdhocPollSd; // 12 bytes

int sceNetAdhocPollSocket(SceNetAdhocPollSd *sds, int count, u32 timeout, int flag); // NID 0x7A662D6B
```

Returns how many sockets have any of their requested events.

| Bit | Event |
|---|---|
| 0x0001 | SEND |
| 0x0002 | RECV |
| 0x0004 | CONNECT |
| 0x0008 | ACCEPT |
| 0x0010 | FLUSH |
| 0x0100 | INVALID (reported only) |
| 0x0200 | DELETE (reported only) |
| 0x0400 | ALERT (reported only) |
| 0x0800 | DISCONNECT (reported only) |

What sockets report, on a PSP:

| Socket | `revents` (of all requested) |
|---|---|
| PDP, idle | SEND |
| PDP, data waiting | SEND, RECV |
| PTP listener, idle | SEND, FLUSH |
| PTP listener, connection waiting | SEND, FLUSH, ACCEPT |
| PTP, connected | SEND, CONNECT, FLUSH, plus RECV with data waiting |
| PTP, peer closed, or connect refused | DISCONNECT only, and the call returns `EXCEPTION_EVENT` (0x80410717), even with no events requested |
| unknown or deleted ID | INVALID, and the call returns `INVALID_SOCKET_ID` |
| poll alert set | ALERT, and the call returns `EXCEPTION_EVENT` |

`revents` holds only requested events, apart from the reported-only ones. `count` 0 or less, NULL `sds`,
or a bad flag: `INVALID_ARG`. Blocking with nothing to report: `TIMEOUT`; timeout 0 waits forever.

**PPSSPP:** never reports FLUSH, CONNECT only while connecting, no DISCONNECT or `EXCEPTION_EVENT` for a
closed peer or a refused connect (it reports SEND, RECV, CONNECT instead), no INVALID for bad IDs, no
ALERT. A blocking poll that times out returns 0, a bad flag is accepted.

### sceNetAdhocSetSocketAlert / GetSocketAlert

```c
int sceNetAdhocSetSocketAlert(int id, int flags);  // NID 0x73BFD52D
int sceNetAdhocGetSocketAlert(int id, int *flags); // NID 0x4D2CE199
```

| Flag | Makes fail with `SOCKET_ALERTED` |
|---|---|
| 0x0010 | send |
| 0x0020 | receive |
| 0x0040 | poll (as `EXCEPTION_EVENT` with ALERT) |
| 0x0080 | connect |
| 0x0100 | accept |
| 0x0200 | flush |

- Only these bits are kept (`flags & 0x3F0`); setting 0 clears them.
- **Alerts persist**: every matching call fails, blocking or not, until cleared.
- Each flag only affects its own call: on a listening socket the receive alert doesn't stop `PtpAccept`,
  and on a connection the flush alert doesn't stop `PtpSend` or `PtpRecv`.
- **Setting an alert wakes a thread already blocked in that call** at once, with `SOCKET_ALERTED`: the
  accept alert wakes a blocked `PtpAccept`, the receive alert a blocked `PtpRecv`. Other flags leave it
  blocked.
- A call that can complete straight away isn't stopped: `PtpConnect` on a connected socket and `PtpFlush`
  with nothing left to send return 0 with their alerts set.
- With the poll alert set, `PollSocket` returns `EXCEPTION_EVENT` (0x80410717) with ALERT (0x0400) in
  `revents`, for PTP sockets as for PDP.

**PPSSPP:** the connect and flush alerts stop calls that would complete at once, and the poll alert sets
IN (0x0001) in `revents` instead of ALERT.
- Unknown ID: `INVALID_SOCKET_ID`. `GetSocketAlert` with NULL: `INVALID_ARG`.

## Game mode data

These only work in game mode (see [sceNetAdhocctlCreateEnterGameMode](/docs/development/adhoc/api/scenetadhocctl#scenetadhocctlcreateentergamemode));
outside it, they fail with `NOT_IN_GAMEMODE` (0x8041071B) and the deletes with `NOT_CREATED`
(0x8041071C).

```c
int sceNetAdhocGameModeCreateMaster(const void *data, int size);                     // NID 0x7F75C338
int sceNetAdhocGameModeCreateReplica(const SceNetEtherAddr *mac, void *data, int size); // NID 0x3278AB0C
int sceNetAdhocGameModeUpdateMaster(void);                                           // NID 0x98C204C8
int sceNetAdhocGameModeUpdateReplica(int id, SceNetAdhocGameModeOptData *info);      // NID 0xFA324B4E
int sceNetAdhocGameModeDeleteMaster(void);                                           // NID 0xA0229362
int sceNetAdhocGameModeDeleteReplica(int id);                                        // NID 0x0B2228E9

typedef struct { u32 size; u32 flag; u64 last_recv; } SceNetAdhocGameModeOptData;    // 16 bytes
```

- **CreateMaster** publishes this console's buffer. Size 0: `INVALID_DATALEN`; a second master:
  `ALREADY_CREATED` (0x8041071A).
- **CreateReplica** returns the ID of a local copy of a member's buffer. **The ID is that member's index in
  the game mode member list** (0 for the master). A MAC that isn't a member: 0x80410D13; twice:
  `ALREADY_CREATED`.
- **UpdateMaster** takes a new snapshot of the master buffer to send.
- **UpdateReplica** copies the newest data received into the replica's buffer. `info` may be NULL; if
  given, `size` comes back as 16, `flag` non-zero if new data arrived since the last call, and `last_recv`
  increases. A bad ID: `NOT_CREATED`.
- Between two consoles, polling every 50 ms, new data arrived on 40 or more polls of 60: the buffers are
  sent at least 20 times a second. Counters written into the master only ever increase on the other side.

**PPSSPP:** a master of size 0 is accepted; with two instances, the test hangs at buffer setup.
