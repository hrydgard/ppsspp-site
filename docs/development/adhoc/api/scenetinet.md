---
position: 7
---

# sceNetInet, sceNetApctl and sceNetResolver

The infrastructure side: BSD sockets over IP (sceNetInet), connecting to an access point (sceNetApctl), and
DNS (sceNetResolver). Not ad hoc, but documented here because games use both, and because the two can't
be connected at the same time. Tests: `net/inet/errors`, `options`, `addr`, `udp`, `tcp`, `bind`,
`blocking`, `poll`, `net/adhoc/loopback`, `net/adhoc/apctl`, and `net/inet/pair/lan` (two consoles on a
phone hotspot).

## General

- sceNetInet is a BSD socket API. Functions return -1 on failure and set an errno, read with
  `sceNetInetGetErrno`. **The errno numbers are newlib's** (EAGAIN 11, EINVAL 22, ECONNREFUSED 111,
  EADDRINUSE 112, EINPROGRESS 119, EISCONN 127, ENOTCONN 128...); the full list is in
  [Error codes](/docs/development/adhoc/api/error-codes#scenetinet-errno-values). A successful call changes
  errno.
- Load the INET net module, call `sceNetInit`, then `sceNetInetInit`. Loopback (127.0.0.1) works without an
  access point, also while in an ad hoc group.
- Addresses are BSD `sockaddr_in` with a length byte: `{ u8 len; u8 family; u16 port; u32 addr; u8 zero[8]; }`,
  16 bytes, port and address in network byte order. `AF_INET` is 2, `SOCK_STREAM` 1, `SOCK_DGRAM` 2,
  `SOCK_RAW` 3.
- **At most 255 sockets** (then EMFILE). **A closed descriptor number isn't reused**; new sockets get higher
  numbers.
- Don't pass NULL addresses: `bind` with NULL hangs the console.

**PPSSPP:** passes calls through to the host's sockets, so behaviour and errno values are the host's
(macOS numbers, for example ECONNREFUSED is 61); several failures return the errno value itself instead
of -1. In the loopback test, TCP connections to the console itself are accepted on a PSP but not in
PPSSPP.

## Sockets

```c
int sceNetInetInit(void);                                                      // NID 0x17943399
int sceNetInetTerm(void);                                                      // NID 0xA9ED66B9
int sceNetInetSocket(int domain, int type, int protocol);                      // NID 0x8B7B220F
int sceNetInetBind(int s, const SceNetInetSockaddr *addr, int addrlen);        // NID 0x1A33F9AE
int sceNetInetListen(int s, int backlog);                                      // NID 0xD10A1A7A
int sceNetInetAccept(int s, SceNetInetSockaddr *addr, int *addrlen);           // NID 0xDB094E1B
int sceNetInetConnect(int s, const SceNetInetSockaddr *addr, int addrlen);     // NID 0x410B34AA
int sceNetInetClose(int s);                                                    // NID 0x8D7284EA
int sceNetInetCloseWithRST(int s);                                             // NID 0x805502DD
int sceNetInetShutdown(int s, int how);                                        // NID 0x4CFE4E56
int sceNetInetGetsockname(int s, SceNetInetSockaddr *addr, int *addrlen);      // NID 0x162E6FD5
int sceNetInetGetpeername(int s, SceNetInetSockaddr *addr, int *addrlen);      // NID 0xE247B6D6
int sceNetInetSocketAbort(int s);                                              // NID 0x80A21ABD
int sceNetInetGetErrno(void);                                                  // NID 0xFBABE411
int sceNetInetGetPspError(void);                                               // NID 0x8CA3A97E
```

**socket:** TCP and UDP, with protocol 0 or the matching one, and raw sockets are accepted. A mismatched
protocol, a domain other than AF_INET, or an unknown type give EPROTONOSUPPORT.

**bind:** a second socket on the same address gives EADDRINUSE; binding a socket twice, or an `addrlen` too
short, EINVAL; a wrong family or an address the console doesn't have, EADDRNOTAVAIL. Port 0 picks a port
(1024 or above). A just-closed TCP listener's port can't be bound again for a while.

**listen:** on an unbound TCP socket fails with EINVAL (it isn't bound implicitly); twice, also EINVAL; on
UDP, EOPNOTSUPP. The backlog is enforced loosely: with a backlog of 2, five connects all completed and four
could be accepted.

**accept:** on a socket that isn't listening, EINVAL; on UDP, EOPNOTSUPP; non-blocking with nothing
waiting, EAGAIN. `*addrlen` smaller than 16 is accepted and left as it was.

**connect:** to a port nobody listens on, ECONNREFUSED at once (blocking), also between consoles; after
that, connecting the same socket again gives EINVAL. Non-blocking: EINPROGRESS, then `select` reports it
writable and `SO_ERROR` is 0 on success. **On a refused non-blocking connect, `SO_ERROR` holds 0x8001006F**,
a PSP error code (0x80010000 + 111), not 111. Connecting again once connected: EISCONN. A listening socket
can't connect (EINVAL).

**close:** an invalid or closed descriptor gives EBADF. **CloseWithRST** resets the connection: the peer's
next receive gives ECONNRESET, the one after 0.

**shutdown:** on an unconnected socket returns 0; `how` 3 gives EINVAL. After `SHUT_WR`, sends fail with
EPIPE while receiving still works; the peer's receive returns 0 once the data before it is read. After
`SHUT_RD`, receive returns 0.

**getpeername:** unconnected, ENOTCONN.

**SocketAbort** makes a call blocked on the socket in another thread return at once with -1 and EINTR (4).
The socket stays usable: later receives and sends work as before. An invalid descriptor gives EBADF.

**GetPspError** returned 0 after the errors tested.

**PPSSPP:** `listen` on an unbound socket succeeds; binding to an address in use succeeds; `shutdown` on
an unconnected socket fails; a refused non-blocking connect leaves the host's errno (61 on macOS) in
`SO_ERROR`; descriptor numbers are reused; the socket limit is the host's.

## Sending and receiving

```c
int sceNetInetSend(int s, const void *buf, int len, int flags);                                      // NID 0x7AA671BC
int sceNetInetSendto(int s, const void *buf, int len, int flags, const SceNetInetSockaddr *to, int tolen); // NID 0x05038FC7
int sceNetInetRecv(int s, void *buf, int len, int flags);                                            // NID 0xCDA85C99
int sceNetInetRecvfrom(int s, void *buf, int len, int flags, SceNetInetSockaddr *from, int *fromlen); // NID 0xC91142E4
int sceNetInetSendmsg(int s, const void *msg, int flags);                                            // NID 0x774E36F4
int sceNetInetRecvmsg(int s, void *msg, int flags);                                                  // NID 0xEECE61D2
```

| Flag | |
|---|---|
| 0x01 | MSG_OOB |
| 0x02 | MSG_PEEK |
| 0x04 | MSG_DONTROUTE |
| 0x40 | MSG_WAITALL |
| 0x80 | MSG_DONTWAIT |

**UDP:**
- One datagram per call, boundaries kept. A receive buffer smaller than the datagram truncates it and
  discards the rest. `MSG_PEEK` leaves it queued.
- **The largest datagram is `SO_SNDBUF`, 9216 bytes by default**; larger ones give EMSGSIZE. Raising
  `SO_SNDBUF` (and the receiver's `SO_RCVBUF`) allows larger ones (20000 tested).
- Zero-length datagrams work.
- `fromlen` smaller than 16 gets as much of the address as fits, and is left as it was.
- `send` on an unconnected socket: EDESTADDRREQ.
- A connected UDP socket only receives from its peer; `sendto` another address gives EISCONN. If the peer's
  port is closed, the next receive gives ECONNREFUSED.
- Broadcast to 255.255.255.255 needs `SO_BROADCAST` and a network: with only loopback, EHOSTUNREACH. On an
  access point it reaches the other consoles on the network.

**TCP:**
- A stream: two sends can arrive in one receive; a receive returns what's there, up to `len`.
- Non-blocking with nothing there: EAGAIN.
- `MSG_WAITALL` with a receive timeout returns EAGAIN when the timeout passes before `len` bytes arrived, and
  leaves the partial data queued.
- With the default 16 KB buffers, a non-blocking sender to a reader that doesn't read sends 16-64 KB before
  EAGAIN.
- After the peer closes: receive returns 0; the first send succeeds, the next fails with EPIPE.

**sendmsg / recvmsg** take a BSD `msghdr`:

```c
typedef struct {
	void *iov_base;
	u32 iov_len;
} SceNetInetIovec;

typedef struct {
	void *msg_name;            // address, or NULL
	u32 msg_namelen;
	SceNetInetIovec *msg_iov;
	int msg_iovlen;
	void *msg_control;
	u32 msg_controllen;
	int msg_flags;             // set by recvmsg
} SceNetInetMsghdr;
```

- `sendmsg` gathers the parts into one datagram; `recvmsg` scatters it across its parts in order, sets
  `msg_namelen` to 16 and fills in the sender. Both return the total length.
- A datagram bigger than the parts is truncated, the rest discarded, and `msg_flags` gets `MSG_TRUNC`
  (0x10); the return value is the length copied.
- `msg_name` NULL on receive is fine. `sendmsg` without an address on an unconnected socket gives
  EDESTADDRREQ (121); non-blocking with nothing waiting, EAGAIN; an invalid descriptor, EBADF.

**PPSSPP:** `recvmsg` fails with EAGAIN even with a datagram waiting; `SocketAbort` fails and doesn't
wake the blocked call (it times out with EAGAIN); errors return the errno value instead of -1, and errno
values are the host's.

**Between consoles on an access point** (both on a phone hotspot), TCP and UDP work as expected, with
small losses for bursts of UDP. The first unicast datagrams to a new address can be lost while its ARP
entry is resolved, and with a weak signal TCP can stall for several seconds.

**PPSSPP:** `fromlen` is set to 16 even when smaller; broadcast errors differ (EACCES without
`SO_BROADCAST`).

## Socket options

```c
int sceNetInetSetsockopt(int s, int level, int name, const void *val, int len); // NID 0x2FE71FE7
int sceNetInetGetsockopt(int s, int level, int name, void *val, int *len);      // NID 0x4A114C7C
```

| Option | Level/name | Default (TCP / UDP) | Notes |
|---|---|---|---|
| SO_REUSEADDR | 0xFFFF / 0x0004 | 0 | reads back 0x04 when set |
| SO_KEEPALIVE | 0xFFFF / 0x0008 | 0 | reads back 0x08 when set |
| SO_BROADCAST | 0xFFFF / 0x0020 | 0 | |
| SO_LINGER | 0xFFFF / 0x0080 | `{0, 0}` (8 bytes) | `{1, 5}` reads back `{0x80, 5}` |
| SO_REUSEPORT | 0xFFFF / 0x0200 | 0 | |
| SO_SNDBUF | 0xFFFF / 0x1001 | 0x4000 / 0x2400 | over 1 MB: ENOBUFS; 0 or negative: EINVAL |
| SO_RCVBUF | 0xFFFF / 0x1002 | 0x4000 / 0xA280 | as SO_SNDBUF |
| SO_SNDLOWAT | 0xFFFF / 0x1003 | 0x800 | |
| SO_RCVLOWAT | 0xFFFF / 0x1004 | 1 | |
| SO_SNDTIMEO, SO_RCVTIMEO | 0xFFFF / 0x1005, 0x1006 | 0 | **a 4-byte count of microseconds, not a timeval**; reads back in 10 ms ticks (250000 reads 25) |
| SO_ERROR | 0xFFFF / 0x1007 | 0 | read-only (setting it: ENOPROTOOPT) |
| SO_TYPE | 0xFFFF / 0x1008 | 1 / 2 | read-only |
| SO_NBIO | 0xFFFF / 0x1009 | 0 | the PSP's non-blocking switch; reads back 0x80 when set |
| IP_TTL | 0 / 4 | 64 | |
| TCP_NODELAY | 6 / 1 | 0 | reads back 0x04 when set; on UDP, EINVAL |

- Boolean options read back as their flag bit, not 1.
- **A `timeval` passed to SO_RCVTIMEO is read as its first 4 bytes** (the seconds), as microseconds, which
  turns 1.5 s into "no timeout".
- `SO_ACCEPTCONN` isn't supported (ENOPROTOOPT), not even on a listener. Unknown options: ENOPROTOOPT;
  unknown levels: EINVAL.
- `setsockopt` with `len` below 4: EINVAL; larger is fine. `getsockopt` copies at most `*len` bytes and
  sets `*len` to what it wrote: the option's size (4, or 8 for SO_LINGER), or less if `*len` was smaller.

**PPSSPP:** for the boolean options, SO_NBIO and the timeouts, `getsockopt` writes nothing and leaves
`*len` alone; timeouts are host timevals; the buffer defaults and limits are the host's (1 MB is
accepted).

## Polling

```c
int sceNetInetSelect(int nfds, SceNetInetFdSet *r, SceNetInetFdSet *w, SceNetInetFdSet *e, SceNetInetTimeval *t); // NID 0x5BE8D595
int sceNetInetPoll(SceNetInetPollfd *fds, int nfds, int timeoutMs);                                              // NID 0xFAABB1DD
```

The PSP's `fd_set` has 256 bits (32 bytes); newlib's is smaller, so declare your own. `sceNetInetPoll`'s
timeout is in milliseconds. A connected socket can be reported writable even right after a send returned
EAGAIN. See `net/inet/poll` and `net/inet/blocking` for the edge cases tested.

## Addresses

```c
u32 sceNetInetInetAddr(const char *cp);                                          // NID 0xB75D5B0A
int sceNetInetInetAton(const char *cp, u32 *addr);                               // NID 0x1BDF5D13
int sceNetInetInetPton(int af, const char *src, void *dst);                      // NID 0xE30B8C19
const char *sceNetInetInetNtop(int af, const void *src, char *dst, int size);    // NID 0xD0792666
```

- `InetAddr` and `InetAton` accept BSD's short forms: `1.2.3`, `1.2`, a single number, hex (`0x7f.1`) and
  octal (`010.0.0.1` is 8.0.0.1). Out-of-range parts and garbage fail (`InetAddr` returns 0xFFFFFFFF).
- `InetPton` only accepts dotted quads.
- All three accept a trailing space but not a leading one.
- No IPv6: family 23 gives EAFNOSUPPORT.
- `InetNtop` returns `dst`; a buffer too small (15 for 255.255.255.255 is too small) returns NULL with
  ENOSPC.

**PPSSPP:** only dotted quads are accepted, and `010` is read as decimal; `InetNtop` doesn't check the
buffer size or the family.

## Statistics

```c
int sceNetInetGetTcpcbstat(int *size, void *buf); // NID 0xB3888AD4
int sceNetInetGetUdpcbstat(int *size, void *buf); // NID 0x39B0C7D3
```

List the console's TCP and UDP sockets (protocol control blocks), like `netstat`. Both return 0.

- With `buf` NULL, `*size` is set to the size needed: 24 bytes per UDP socket, 28 per TCP socket, 0 with
  none. With a buffer, `*size` is the size of the whole list; a buffer too small for it returns 0 too.
- The entries form a linked list, newest socket first: the first word of each points to the next entry
  in the buffer, and is 0 in the last.
- The local address is at offset 12 and the local port at offset 20, both in network byte order. The
  other fields aren't known.

**PPSSPP:** neither is implemented; both return 0x8002013A.

## sceNetApctl

```c
int sceNetApctlInit(int stackSize, int priority);                    // NID 0xE2F91F9B
int sceNetApctlTerm(void);                                           // NID 0xB3EDD0EC
int sceNetApctlConnect(int profile);                                 // NID 0xCFB957C6
int sceNetApctlDisconnect(void);                                     // NID 0x24FE91A1
int sceNetApctlGetState(int *state);                                 // NID 0x5DEAC81B
int sceNetApctlGetInfo(int code, void *info);                        // NID 0x2BEFDF23
int sceNetApctlAddHandler(void (*h)(int oldState, int newState, int event, int error, void *arg), void *arg); // NID 0x8ABADD51
int sceNetApctlDelHandler(int id);                                   // NID 0x5963991B
int sceNetApctlScanUser(void);                                       // NID 0xE9B2E5E6
int sceNetApctlGetBSSDescIDListUser(int *size, void *buf);           // NID 0x6BDDCB8C
int sceNetApctlGetBSSDescEntryUser(int id, int code, void *info);    // NID 0x04776994
```

`sceNetApctlScan` (0xB3CF6849), `GetBSSDescIDList` (0x0C7FFA5C) and `GetBSSDescEntry` (0x96BEB231) aren't
available to games on firmware 6.60 and 6.61: imports of them stay unlinked and return 0x8002013A. Use the
`User` versions.

States: 0 DISCONNECTED, 1 SCANNING, 2 JOINING, 3 GETTING_IP, 4 GOT_IP, 5 EAP_AUTH, 6 KEY_EXCHANGE.

- `Connect` returns 0 and works in the background: 0 → 2 → 3 → 4, a few seconds in all (getting an IP
  depends on the access point's DHCP). A profile slot with nothing in it gives 0x80110601 at once.
- A failed join goes from JOINING back to DISCONNECTED; the handler's ERROR event says why (0x80410D09 was
  seen).
- **Infrastructure and ad hoc exclude each other.** `Connect` while in an ad hoc group returns 0 and then
  falls back to DISCONNECTED. Creating an ad hoc group while apctl has an IP fails with a 0x80410D01 ERROR
  event. On one of the two consoles tested, apctl couldn't join at all while adhocctl was merely
  initialized. Terminate one before starting the other.
- Before `Init` (and after `Term`), `GetState` gives 0x80410A0D. `Init` twice: `ALREADY_INITIALIZED`;
  `Term` twice returns 0. `GetState(NULL)`: 0x80410A03.
- While disconnected, `GetInfo` gives `NOT_IN_BSS` (0x80410A05) for every code, checked before the code.
- **Only two handlers** can be registered; the third gives 0x80410A0F, also for a NULL handler. Removing
  one frees its ID for reuse. `DelHandler` on a removed ID gives `INVALID_ID` (0x80410A09); on an ID that
  was never valid (100, -1), 0x80410A82.
- `Connect` to a profile slot out of range (10, 11, 100) gives 0x80110601, and to -1, 0x80110602, at once
  and without events. Slot 0 doesn't fail: it starts connecting. `Disconnect` while disconnected returns 0.
  A second `Connect` or `ScanUser` while one is in progress: 0x80410A80.
- **Scanning:** `ScanUser` returns 0, and the handler sees 0 → 1 (SCANNING, event 1) and 1 → 0 (event 2)
  within 1-3 seconds. `GetBSSDescIDListUser(&size, NULL)` then sets `size` (a multiple of 8);
  with a buffer it fills a linked list of 8-byte entries (`next` pointer, then the ID, starting at 1).
  `GetBSSDescEntryUser` takes such an ID and a code; codes 0-5 work and 6 gives `INVALID_CODE`, as does
  100; an unknown ID gives 0x80410A82. NULL `size`: 0x80410A03. What each code returns wasn't checked.
- `GetInfo` codes: 0 profile name, 1 BSSID, 2 SSID, 3 SSID length, 4 security type (0 none), 5 strength
  (1-100), 6 channel, 7 power save, 8 IP, 9 subnet mask, 10 gateway, 11 primary DNS, 12 secondary DNS,
  13-15 proxy, 16 EAP type, 17 start browser, 18 WISP. The addresses are strings. Codes 0, 2-6 and 8-11
  were checked; the rest are from the SDK's headers. Code 19: `INVALID_CODE` (0x80410A02).

**PPSSPP:** the connection is simulated (the host is already online), and doesn't exclude ad hoc.
`GetState` and `GetInfo` work before `Init` and while disconnected; there's no handler limit; errors are
-1 instead of codes; `Scan` and its lists are linked (and fake a scan).

## sceNetResolver

```c
int sceNetResolverInit(void);                                                              // NID 0xF3370E61
int sceNetResolverTerm(void);                                                              // NID 0x6138194A
int sceNetResolverCreate(int *rid, void *buf, int bufLen);                                 // NID 0x244172AF
int sceNetResolverDelete(int rid);                                                         // NID 0x94523E09
int sceNetResolverStartNtoA(int rid, const char *hostname, u32 *addr, int timeout, int retry); // NID 0x224C5F44
int sceNetResolverStartAtoN(int rid, const u32 *addr, char *hostname, int len, int timeout, int retry); // NID 0x629E2FB7
int sceNetResolverStop(int rid);                                                           // NID 0x808F6063
```

Plus `StartNtoAAsync` (0x14C17EF9), `StartAtoNAsync` (0xAAC09184), `WaitAsync` (0x12748EB9) and `PollAsync`
(0x4EE99358).

- `Create` works even before `Init`. NULL `rid` or buffer: `INVALID_PTR` (0x80410403); a buffer of 0
  bytes: `INVALID_BUFLEN` (0x80410404), while 1 byte is accepted. 20 resolvers could exist at once.
  `Init` twice: 0x80000020; `Term` twice returns 0.
- An unknown ID: `BAD_ID` (0x80410408) for every call.
- **With no network** (apctl not connected), every lookup fails at once with `NO_DNS_SERVER`
  (0x80410402), even a dotted IP string like `127.0.0.1`, and reverse lookups too. An empty name gives
  0x80410418, checked first; NULL name or address, `INVALID_PTR`.
- **Async:** `StartNtoAAsync` and `StartAtoNAsync` return 0, and the lookup's result comes back as the
  return value of `PollAsync` or `WaitAsync` (the `status` argument wasn't written in these cases). Once
  collected, both give 0x8041041A (nothing pending). Starting another lookup while one is pending:
  `CTX_BUSY` (0x80410409). `PollAsync(rid, NULL)`: 0x80410419.
- `Stop` with nothing running: `ALREADY_STOPPED` (0x8041040A).

Over an access point with internet access: `StartNtoA` resolves a real name (returns 0 with the address); a
name that doesn't exist gives `INVALID_HOST` (0x80410414); a dotted IP string resolves to itself. (The
test passed a timeout of 5 and 2 retries; their units weren't checked.)
