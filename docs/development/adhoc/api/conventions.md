---
position: 1
---

# Conventions

How to read the API reference pages, and the things all the networking libraries have in common.

## What "confirmed" means

Behaviour described in these pages was observed on real PSPs (two PSP-1000s, firmware 6.60 and 6.61, which
gave identical results) by the pspautotests under `tests/net/adhoc` and `tests/net/inet`, unless it says
otherwise. Where only the function's existence and signature are known, the entry says so. Each entry
ends with how PPSSPP currently differs, where the tests show a difference.

The test names are given as `net/adhoc/pdp`, meaning `pspautotests/tests/net/adhoc/pdp.c`. Tests ending
in `_host`/`_client` under `pair/` run on two consoles at once.

## Return values

Nearly every function returns `int`: 0 (or a positive value such as an ID) on success, and a negative
error code on failure. The error codes have the form `0x8041MMEE`, where `MM` is the library:

| `MM` | Library |
|---|---|
| `01` | sceNet |
| `04` | sceNetResolver |
| `06` | ad hoc authentication (internal, shows up in sceNetAdhocctlInit) |
| `07` | sceNetAdhoc (PDP, PTP, game mode data) |
| `08` | sceNetAdhocMatching |
| `0A` | sceNetApctl |
| `0B` | sceNetAdhocctl |
| `0D` | the Wi-Fi driver (shows up in handler events) |
| `13` | sceNetAdhocDiscover |

A few sceNetAdhoc codes are `0x8040....` instead (NOT_ENOUGH_SPACE, NOT_OPENED, NOT_LISTENED). That's how
the firmware returns them. See [Error codes](/docs/development/adhoc/api/error-codes) for all of them.

sceNetInet is different: it's a BSD socket API, so its functions return -1 on failure and set an errno,
read with `sceNetInetGetErrno`. See [sceNetInet](/docs/development/adhoc/api/scenetinet).

## Argument checking order

When several arguments are wrong, the first failing check decides the error. The PSP's ad hoc functions
check, roughly in this order: NULL pointers and flag bits (`INVALID_ARG`), then whether the library is
initialized (`NOT_INITIALIZED`), then the socket ID (`INVALID_SOCKET_ID`), then addresses
(`INVALID_ADDR`), ports (`INVALID_PORT`) and lengths (`INVALID_DATALEN`). Each function's entry lists its
checks in order where it matters.

**Don't pass NULL where the PSP doesn't expect it.** Several functions don't check their pointers and
hang the console instead (`sceNetInetBind` with a NULL address, `sceNetAdhocDiscoverInitStart(NULL)`).
The entries say where NULL was confirmed safe.

## Blocking calls

The sceNetAdhoc calls that can wait (send, receive, accept, connect, flush, poll) take a `timeout` in
microseconds and a `flag`:

| `flag` | Meaning |
|---|---|
| 0 | Block until done or `timeout` µs have passed, then fail with `TIMEOUT` (0x80410715). **A timeout of 0 means wait forever.** |
| 1 (`ADHOC_F_NONBLOCK`) | Don't wait: fail with `WOULD_BLOCK` (0x80410709) if it can't complete at once. |
| anything else | `INVALID_ARG` (0x80410711), checked before anything else. |

Socket *creation* calls (`PdpCreate`, `PtpOpen`, `PtpListen`) and the delete/close calls take a `flag`
that must be 0; even 1 gives `INVALID_ARG`.

Blocking calls can be interrupted from another thread with
[socket alerts](/docs/development/adhoc/api/scenetadhoc#scenetadhocsetsocketalert-getsocketalert), or by deleting the
socket.

## Addresses

Ad hoc addresses are 6-byte MAC addresses (`SceNetEtherAddr`, no padding) plus a 16-bit port. The only
broadcast address is `FF:FF:FF:FF:FF:FF`. `00:00:00:00:00:00` and other multicast addresses are invalid
destinations. Ports 0 and 65535 aren't valid destinations; port 0 as a local port means "pick one" (the
firmware picks from 0x1000-0x7FFF).

## Asynchronous operations and handlers

sceNetAdhocctl's network operations (scan, create, connect, join, disconnect, game mode) return at once
and report their result to handler functions registered with `sceNetAdhocctlAddHandler`, from
adhocctl's own thread. A failure that happens later (for example the radio being in use) arrives as an
ERROR event, not as the call's return value. sceNetAdhocMatching reports everything through its context's
callback, called from the context's event thread. Handlers and callbacks should return quickly and leave
the work to the game's own threads.

## Memory

`sceNetInit` creates the pool that all of networking allocates from; sceNetAdhocMatching has its own pool
from `sceNetAdhocMatchingInit`. With a 0x20000 pool, a 65523-byte PDP send sometimes fails with 0x80410001
(out of memory).
