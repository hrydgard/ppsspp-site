---
position: 9
---

# The relay (aemu_postoffice)

In P2P mode, players send PDP and PTP traffic straight to each other, so each of them has to be reachable
from the internet: port forwarding, UPnP, or a virtual LAN. Many can't manage that (carrier-grade NAT,
mobile networks, locked-down routers). Relay mode removes the requirement: every socket connects *out* to a
relay server, which forwards the traffic between players by MAC address and port. It was designed by
Kethen as **aemu_postoffice**, and is used by PPSSPP (since 1.20) and by the aemu plugin on PSPs and
Vitas, so all of them can play together through it.

```
  P2P mode                               Relay mode

  A ---- UDP/TCP, port+offset ---- B     A --- TCP 27313 ---+                +--- TCP 27313 --- B
                                                            |  postoffice    |
  (B must accept incoming traffic)                          +--- forwards ---+
                                                            by (MAC, port)
```

The adhoc server is still used for groups and peer lists ([The adhoc server](/docs/development/adhoc/adhoc-server));
only the data path changes. The relay normally runs on the same host as the adhoc server, on port
**27313**.

## Protocol

Every PDP socket, every PTP listening socket and every PTP connection is its own connection to the relay:
TCP, or an ENet (reliable UDP) connection on UDP port 27313. All structs are packed and **little-endian**.
MAC address fields are 8 bytes, of which the first 6 are used.

### Opening

The first 24 bytes on every connection say what it's for:

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | type: 0 = PDP, 1 = PTP_LISTEN, 2 = PTP_CONNECT, 3 = PTP_ACCEPT, 4 = PDP_UNRELIABLE |
| 4 | 8 | source MAC (own) |
| 12 | 2 | source port |
| 14 | 8 | destination MAC (PTP_CONNECT and PTP_ACCEPT only) |
| 22 | 2 | destination port |

The relay names each connection by these addresses (`PDP <mac> <port>`, `PTP_LISTEN <mac> <port>`,
`PTP_CONNECT <src> <sport> <dst> <dport>`, ...). A new connection with the same name replaces the old one.
There's no version number or capability exchange: an old relay just closes a connection with a type it
doesn't know.

### PDP

```
  A: open  {PDP, A, 1234}
  B: open  {PDP, B, 1234}
  A: send  [B, 1234, size] data        --->  relay  --->  B receives [A, 1234, size] data
```

Each datagram, in either direction, is a 14-byte header followed by the data:

| Offset | Size | Field |
|---|---|---|
| 0 | 8 | MAC: the destination when sending, the source when receiving |
| 8 | 2 | port: likewise |
| 10 | 4 | size of the data |

The relay rewrites the header with the sender's address and passes the datagram to the connection named
`PDP <destination MAC> <destination port>`. If there's no such connection, the datagram is dropped. The
relay has no notion of broadcast: the client sends a broadcast as one datagram per peer.

The client library refuses to send datagrams larger than 10 KiB.

Over TCP, PDP becomes reliable and ordered: datagrams are never lost or reordered, which is not what the
game would see on a PSP, but almost no game minds. Over ENet, PDP is reliable by default too; with
type 4 (PDP_UNRELIABLE), both directions use unsequenced ENet packets, one datagram per packet, so loss is
passed through to the game.

### PTP

```
  A (listener)                        relay                         B (connector)
  open {PTP_LISTEN, A, 80}  ------->
                                             <------- open {PTP_CONNECT, B, 2000, A, 80}
  on listen connection:     <-------  [B, 2000]       (connect request)
  new connection:
  open {PTP_ACCEPT, A, 80, B, 2000} --->
  on accept connection:     <-------  [B, 2000]   [A, 80]  ------->  on connect connection
                                       (both ends are now bonded)
  [size] data ...           <------------------------------------->  [size] data ...
```

1. The listener opens a PTP_LISTEN connection.
2. The connector opens a PTP_CONNECT connection naming the listener. If the listener doesn't exist yet,
   the relay waits up to 5 seconds for it to appear, then closes the connection (the connector sees a
   refused connection).
3. The relay sends the listener a 10-byte connect request (MAC 8, port 2) on its listen connection.
4. To accept, the listener opens a new PTP_ACCEPT connection naming both ends.
5. The relay sends each end the other's address (10 bytes) and from then on passes data between the two
   connections. The connector waits up to 5 seconds for the accept.
6. Data is sent as chunks of `[u32 size]` + data, up to 50 KiB each; the receiver strips the headers and
   treats the result as a byte stream.
7. When either connection closes, the relay closes the other.

### Server limits

aemu_postoffice's server (defaults, configurable):

| Setting | Default |
|---|---|
| Time to send the opening bytes | 5 s |
| PTP connector waits for the accept | 5 s |
| Outgoing queue per connection | 500 KB; a client that stops reading is disconnected |
| Connections | 4000 in total, 20 per MAC address |
| Tick | 8 ms |
| Strict mode | on |

In **strict mode**, the relay only accepts a connection once its MAC address is logged in to the server's
own adhoc server, from the same IP address, and has joined a group; PTP connections must also be to a
member of the same group, and data is only forwarded between members of the same group. That keeps
players from injecting traffic into games they aren't in.

## How PPSSPP uses it

### Choosing the mode

The "Try to use server-provided packet relay" setting:

- **Auto** (default): use the relay if the selected server is marked `"data_mode": "AemuPostoffice"` in the
  [server list](/docs/development/adhoc/adhoc-server#the-server-list), or is a custom server added as a relay
  server. PPSSPP never asks the server itself.
- **Yes**: always use the relay.
- **No**: always P2P.

The mode is decided in `sceNetAdhocInit` and stays fixed while the game runs, so these settings are locked
in-game. The relay host is the adhoc server's host. An on-screen message says which mode is in use.

### The calls

Every PDP/PTP function branches on the mode. In relay mode:

| Call | Relay |
|---|---|
| `PdpCreate` | Opens a PDP connection (type 0, or type 4 over ENet with "Reliable datagram packet delivery on packet relay in UDP mode" off). |
| `PdpSend` | Sends a datagram; a broadcast is sent to each live peer in turn. |
| `PdpRecv` | Takes the next complete datagram from the client's buffer. If it doesn't fit, `NOT_ENOUGH_SPACE` with the size needed. |
| `PtpListen` | Opens a PTP_LISTEN connection. |
| `PtpAccept` | Takes a connect request and opens the PTP_ACCEPT connection. |
| `PtpOpen` | Starts connecting immediately, on a separate thread (the library's connect blocks). Until it's done, send and receive return `WOULD_BLOCK`. Port 0 gets a random free port, since the relay needs a real one. |
| `PtpConnect` | Checks on that thread: ESTABLISHED once connected, `CONNECTION_REFUSED` if the relay closed the connection. |
| `PtpSend` / `PtpRecv` | Send and receive chunks; sends above 50 KiB are cut short and the game sees a shorter length. |
| `PtpFlush` | Returns 0; the relay library never holds data back. |
| `PollSocket` | Checks the client library's buffers (no `select`). |

Notes:

- **Ports are offset** in relay mode as well, so everyone in a game must use the same port offset, or their
  traffic goes to connection names that don't exist and is silently dropped.
- **MAC addresses**: before sending, a destination MAC that matches a known peer is replaced with that
  peer's exact MAC, because some games (Gran Turismo) change the first byte, and the relay matches names
  exactly.
- **Lost connections** are reopened on the next operation, so a PDP socket created before joining a group
  (which a strict-mode relay closes after 5 seconds) recovers once the player is in one.
- **Giving up**: if the relay can't be reached for 15 seconds in a row, relay mode is switched off for the
  rest of the session, with a message on screen; sockets then fail to open.
- With UDP (ENet) on, a host thread pumps all ENet connections about 120 times a second.
- Blocking calls work as in P2P mode: the operation is retried every 500 µs of emulated time.

### Trade-offs

- **Latency**: every packet goes through the server, so two players near each other but far from the
  relay see more latency than in P2P. Latency-tolerant games (Monster Hunter, Ridge Racer, Call of Duty:
  Roads to Victory) are fine; fast action games suffer.
- **Reliability**: no port forwarding, and TCP (or reliable ENet) underneath means no packet loss. In
  exchange, a single lost packet over TCP delays everything behind it.
- **Privacy**: with a relay-only adhoc server, players never learn each other's IP addresses.
