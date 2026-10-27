---
position: 4
---

# PDP and PTP sockets

sceNetAdhoc's two socket types carry almost all game traffic:

- **PDP** (Peer Datagram Protocol): unreliable, unordered datagrams, like UDP. Can broadcast.
- **PTP** (Peer Transport Protocol): reliable, ordered byte streams between two consoles, like TCP.

Both are addressed by MAC address and a 16-bit port. Socket IDs are small positive integers shared between
the two types.

## Common rules

### Blocking, timeouts and flags

Every call that can wait takes a `timeout` in microseconds and a `flag`:

- `flag & 1` (`ADHOC_F_NONBLOCK`): don't wait. If the call can't complete at once it returns
  `WOULD_BLOCK` (0x80410709).
- Otherwise the call blocks for up to `timeout` µs, then fails with `TIMEOUT` (0x80410715). A timeout of 0
  means "wait forever".
- Any other bit set in `flag` gives `INVALID_ARG`.

### Alerts

`sceNetAdhocSetSocketAlert(id, flags)` arms a socket so that the next blocking operation of a given kind
fails with `SOCKET_ALERTED` (0x80410708) instead of waiting. This is how one thread interrupts another
thread blocked in a receive. `sceNetAdhocGetSocketAlert` reads the flags back.

| Flag | Alerts |
|---|---|
| 0x0010 | send |
| 0x0020 | receive |
| 0x0040 | poll |
| 0x0080 | connect |
| 0x0100 | accept |
| 0x0200 | flush |

### Polling

`sceNetAdhocPollSocket(sds, count, timeout, flag)` takes an array of
`SceNetAdhocPollSd { int id; int events; int revents; }` (12 bytes each) and returns how many sockets have
events, like `poll`:

| Bit | Event |
|---|---|
| 0x0001 | SEND: the socket can send |
| 0x0002 | RECV: data is waiting |
| 0x0004 | CONNECT: a PTP connect finished |
| 0x0008 | ACCEPT: a PTP listener has an incoming connection |
| 0x0010 | FLUSH: a PTP flush finished |
| 0x0100 | INVALID: no such socket (reported, not requested) |
| 0x0200 | DELETE: the socket was deleted (reported, not requested) |
| 0x0400 | ALERT: an alert fired |
| 0x0800 | DISCONNECT: the PTP connection was closed (reported, not requested) |

### Error codes

| Code | Name | Code | Name |
|---|---|---|---|
| 0x80410701 | INVALID_SOCKET_ID | 0x8041070F | SOCKET_ID_NOT_AVAIL |
| 0x80410702 | INVALID_ADDR | 0x80410710 | PORT_NOT_AVAIL |
| 0x80410703 | INVALID_PORT | 0x80410711 | INVALID_ARG |
| 0x80410704 | INVALID_BUFLEN | 0x80410712 | NOT_INITIALIZED |
| 0x80410705 | INVALID_DATALEN | 0x80410713 | ALREADY_INITIALIZED |
| **0x80400706** | NOT_ENOUGH_SPACE | 0x80410714 | BUSY |
| 0x80410707 | SOCKET_DELETED | 0x80410715 | TIMEOUT |
| 0x80410708 | SOCKET_ALERTED | 0x80410716 | NO_ENTRY |
| 0x80410709 | WOULD_BLOCK | 0x80410717 | EXCEPTION_EVENT |
| 0x8041070A | PORT_IN_USE | 0x80410718 | CONNECTION_REFUSED |
| 0x8041070B | NOT_CONNECTED | 0x80410719 | THREAD_ABORTED |
| 0x8041070C | DISCONNECTED | 0x8041071A | ALREADY_CREATED |
| **0x8040070D** | NOT_OPENED | 0x8041071B | NOT_IN_GAMEMODE |
| **0x8040070E** | NOT_LISTENED | 0x8041071C | NOT_CREATED |

The three codes in bold really are `0x8040...`, not `0x8041...`.

## PDP

### Functions

| Function | Does |
|---|---|
| `PdpCreate(mac, port, bufsize, flag)` | Creates a socket bound to the local MAC and `port` (0 picks a free port). `bufsize` is the receive buffer. Returns the socket ID. |
| `PdpSend(id, dstmac, dstport, data, len, timeout, flag)` | Sends one datagram. `FF:FF:FF:FF:FF:FF` broadcasts to the group. |
| `PdpRecv(id, &srcmac, &srcport, buf, &len, timeout, flag)` | Receives one datagram. `len` is the buffer size going in and the datagram size coming out. |
| `PdpDelete(id, flag)` | Deletes the socket. |
| `GetPdpStat(&size, buf)` | Lists all PDP sockets. NULL buffer returns the size needed. |

`SceNetAdhocPdpStat` (20 bytes): `next` (4), `id` (4), local MAC (6), local port (2), bytes waiting in the
receive buffer (4).

### Rules on the PSP

Confirmed with the `net/adhoc/pdp` test in pspautotests:

- **Create**:
  - Only works once the console is in a group; before that it gives `INVALID_ADDR` even for the
    console's own MAC.
  - The MAC must be the console's own (`INVALID_ADDR` otherwise, `INVALID_ARG` for NULL).
  - Port 0 picks a free port; 65535 gives `INVALID_PORT`; a port in use gives `PORT_IN_USE`.
  - Any `bufsize` is accepted, even 0 or negative.
  - The flag must be 0 (`INVALID_ARG` for anything else, including 1).
- **Send**:
  - Returns **0** on success, not the length.
  - A NULL destination or a flag other than 0 or 1 gives `INVALID_ARG`, before the socket ID is checked.
  - `00:00:00:00:00:00` and multicast addresses other than `FF:FF:FF:FF:FF:FF` give `INVALID_ADDR`.
  - Ports 0 and 65535 give `INVALID_PORT`.
  - Lengths of 0 to 65523 bytes are accepted (0 even with NULL data); longer, negative, or non-zero with
    NULL data gives `INVALID_DATALEN`. Anything over 1444 bytes is fragmented on the air (see
    [On the PSP](/docs/development/adhoc/on-the-psp#sizes)).
  - Sending to a MAC nobody has succeeds.
  - Sending to the console's own MAC is delivered locally; its own broadcasts are not.
- **Send queue full**: non-blocking sends return `WOULD_BLOCK`; blocking ones wait until there's room or
  the timeout.
- **Recv**:
  - Returns **0** on success, with `len` set to the datagram's size and the sender's MAC and port filled
    in.
  - NULL MAC and port pointers are allowed; a NULL buffer or length pointer, or a flag other than 0 or 1,
    gives `INVALID_ARG`.
  - Nothing waiting: `WOULD_BLOCK` (non-blocking) or `TIMEOUT` (blocking), with `len` left alone.
  - Buffer too small: `NOT_ENOUGH_SPACE` (0x80400706), with `len` set to the datagram's size and the
    sender filled in. Nothing is copied, and the datagram stays queued for the next call.
- **Delete**: the flag must be 0; deleting twice gives `INVALID_SOCKET_ID`.
- **Socket IDs** keep increasing rather than reusing freed ones, and creates that fail on the address,
  port or port-in-use checks use up an ID too.
- **`sceNetAdhocTerm`** returns 0 even when called twice.
- Delivery is not guaranteed and datagrams can arrive out of order.

## PTP

### Functions

| Function | Does |
|---|---|
| `PtpOpen(mac, port, dstmac, dstport, bufsize, rexmt_int, rexmt_cnt, flag)` | Creates a socket for an outgoing connection and starts connecting. `rexmt_int` (µs) and `rexmt_cnt` set the retransmit interval and count. Returns the socket ID. |
| `PtpConnect(id, timeout, flag)` | Waits for the connection started by Open to be established (or, non-blocking, checks on it). |
| `PtpListen(mac, port, bufsize, rexmt_int, rexmt_cnt, backlog, flag)` | Creates a listening socket. |
| `PtpAccept(id, &peermac, &peerport, timeout, flag)` | Waits for an incoming connection and returns a new socket ID for it. The MAC and port pointers may be NULL. |
| `PtpSend(id, data, &len, timeout, flag)` | Sends; `len` returns how much was queued. |
| `PtpRecv(id, buf, &len, timeout, flag)` | Receives whatever is available, up to `len`. |
| `PtpFlush(id, timeout, flag)` | Waits until everything sent has been acknowledged. |
| `PtpClose(id, flag)` | Closes. |
| `GetPtpStat(&size, buf)` | Lists all PTP sockets. |

`SceNetAdhocPtpStat` (36 bytes): `next`, `id`, local MAC, peer MAC, local port, peer port, bytes in the send
buffer, bytes in the receive buffer, and the state:

| State | |
|---|---|
| 0 | CLOSED |
| 1 | LISTEN |
| 2 | SYN_SENT |
| 3 | SYN_RCVD |
| 4 | ESTABLISHED |

### Rules on the PSP

Confirmed with the `net/adhoc/ptp` test in pspautotests, with the console connecting to itself:

- **Open and Listen**:
  - The local MAC must be the console's own (`INVALID_ADDR`; `INVALID_ARG` for NULL).
  - `bufsize`, `rexmt_int`, `rexmt_cnt` and `backlog` aren't checked: 0 is accepted for all of them.
  - Open rejects a NULL destination (`INVALID_ARG`), a zero destination (`INVALID_ADDR`) and destination
    ports 0 and 65535 (`INVALID_PORT`), but accepts the broadcast address.
  - The flag must be 0 (`INVALID_ARG`).
- **Connecting**:
  - The connection is made in the background by Open. A connection to a listener on the same console is
    ESTABLISHED by the time Open returns, and `PtpConnect` then returns 0, also when called again.
  - Until it's accepted, the incoming connection shows up in `GetPtpStat` with ID -1.
  - A connect to a port nobody is listening on fails at once with `CONNECTION_REFUSED`, blocking or not.
- **Accept**: nothing waiting gives `WOULD_BLOCK` or `TIMEOUT`; on a socket that isn't listening,
  `NOT_LISTENED` (0x8040070E). The accepted socket reports the connecting socket's MAC and port.
- **Send and Recv**:
  - Both return 0 on success. Recv returns whatever is buffered, up to `len` (100 bytes sent, read as 30
    then 70).
  - On a listener they give `NOT_CONNECTED`.
  - Send with NULL data or a negative length gives `INVALID_DATALEN`, a NULL length pointer `INVALID_ARG`;
    a length of 0 is accepted.
- **When the peer closes**, Send and Recv give `DISCONNECTED`, the socket's state becomes CLOSED, and the
  closed peer's socket disappears from `GetPtpStat`.
- **Close**: twice gives `INVALID_SOCKET_ID`; a non-zero flag gives `INVALID_ARG`.
- `PtpFlush` returns 0 once the data is sent.
- Segments are at most 1444 bytes on the air.

## How PPSSPP implements them (P2P mode)

In P2P mode every PDP socket is a host UDP socket and every PTP socket a host TCP socket. In relay mode
they're connections to the relay instead; see [The relay](/docs/development/adhoc/relay). Both modes share
the argument checking and the blocking emulation.

### Ports

The host port is the PSP port plus the **port offset** setting (default 10000). Games often use low ports,
which ordinary programs can't bind on Linux and Android, and the offset lifts them out of that range. All
players must use the same offset, because the destination port of every send is offset the same way.
A port that wraps around to 0 is mapped to 65535.

Port 0 ("any port") is passed to the OS as port 0, and the port the OS picked is mapped back by subtracting
the offset.

### Addresses

There's no ARP and no IP-level discovery. The only link between MAC and IP addresses is the peer list from
the adhoc server:

- **Sending**: the destination MAC is looked up in the peer list to get the peer's IP address. Sending to a
  MAC that isn't in the list returns 0 (success) and sends nothing.
- **Receiving**: the sender's IP address is looked up to get its MAC. Datagrams from an IP address that
  isn't a known peer are dropped. The source port is the host port minus the port offset.
- **Accepting**: as receiving. A connection from an unknown IP address is closed.
- **Two players behind the same public IP address** can't be told apart: the first one in the list wins.

The local MAC is always written into the game's source MAC buffer by `PdpCreate`, `PtpOpen` and
`PtpListen`, because some games pass a stale address from their save data.

### PDP

| Call | PPSSPP |
|---|---|
| `PdpCreate` | A UDP socket, non-blocking, with address and port reuse, bound to port + offset (to 127.0.0.*N* when using the built-in server on localhost, so instances on one machine don't collide). Send and receive buffers are 5 and 10 times `bufsize`. Requests a UPnP mapping if UPnP is on. A bind failure shows "Failed to Bind Port" on screen and returns 0x80410001. **Only allowed once the game has scanned for, created or joined a group**; before that it gives `INVALID_ADDR`. |
| `PdpSend` unicast | One UDP datagram to the peer's IP address and port + offset. **Returns 0 on success, not the length.** |
| `PdpSend` broadcast | One UDP datagram to each live peer in the list, then returns 0. The sender doesn't get a copy. There's no IP broadcast. |
| `PdpRecv` | Peeks at the next datagram. If it's from an unknown IP address it's dropped. If it doesn't fit, `NOT_ENOUGH_SPACE` with the needed `len`, and the datagram stays queued. Otherwise it's read, and the sender's MAC and port are filled in. **Returns 0 on success, not the length**; some games (Kingdom Hearts Birth by Sleep) break otherwise. |
| `GetPdpStat` | The receive count is the number of bytes the OS has waiting, which can include more than one datagram. |

### PTP

PPSSPP sends nothing over the TCP connection except the game's data. The peer's MAC and port are worked
out from the TCP connection's IP address and port.

| Call | PPSSPP |
|---|---|
| `PtpOpen` | A TCP socket, non-blocking, with Nagle off, bound to port + offset, with keep-alive. **Starts a non-blocking `connect` immediately**, because some games (The Warriors) send without ever calling `PtpConnect`. |
| `PtpConnect` | Checks the connect. If it succeeded: ESTABLISHED. If it's in progress: SYN_SENT, and a blocking call waits. If the other side refused (it isn't listening yet), the socket is recreated on the same port and the connect retried, since a TCP socket can't connect again after a refusal on most systems. Non-blocking calls return `WOULD_BLOCK` both while connecting and after a refusal, so P2P mode never reports `CONNECTION_REFUSED`. |
| `PtpListen` | A TCP socket bound to port + offset, `listen` with the game's backlog. Requests a UPnP mapping. |
| `PtpAccept` | `accept`, then maps the IP address to a MAC. The new socket inherits the listener's settings. |
| `PtpSend` | `send`. Allowed while still connecting, too. Partial sends report the length sent. An error other than "would block" closes the socket and returns `DISCONNECTED`. |
| `PtpRecv` | `recv`. A return of 0 bytes (the peer closed) or an error closes the socket and returns `DISCONNECTED`. This, send errors, and TCP keep-alive are how a vanished peer is detected. |
| `PtpFlush` | Always returns 0. Nagle is off, so data is never held back. |
| `PtpClose` | Closes the host socket. |
| `GetPtpStat` | Also notices completed connects, a while after the attempt (about two frames), because some games poll this instead of `PtpConnect`. |

The retransmit arguments set the TCP keep-alive interval and count; TCP's own retransmission does the
rest.

### Polling

`PollSocket` runs one `select` over the host sockets: readable gives RECV (ACCEPT for a listener), writable
gives SEND. For a PTP socket that's still connecting, CONNECT is reported once the connect has been under
way for more than a millisecond, and DISCONNECT once it has closed after a connection attempt. Several games
(Fate Unlimited Codes, Carnage Heart EXA, Bleach Heat the Soul 7) retry their connects based on these
events.

### The "Forced first connect" setting

With this on, the first PTP connect of a socket blocks until it succeeds or times out, and `PtpOpen` takes
`rexmt_int` µs longer to return. This helps games that give up on a connection too quickly over a
high-latency link (GvG Next Plus, Dissidia 012, Fate Unlimited Codes).

See [Emulation details](/docs/development/adhoc/emulation) for how blocking calls are implemented and for
timing, and [Known issues](/docs/development/adhoc/improving) for where this differs from the PSP.
