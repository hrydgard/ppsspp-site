---
position: 5
---

# sceNetAdhocMatching

The matchmaking library: hosts advertise, clients join, and the library keeps the member list alive and
carries small messages. For the concepts and wire protocols, see
[sceNetAdhocMatching](/docs/development/adhoc/matching). Tests: `net/adhoc/matching`, `self`, `limits`,
`pair/matching`, `pair/matching2`, `pair/vanish`.

## Types

```c
typedef void (*SceNetAdhocMatchingHandler)(int id, int event, SceNetEtherAddr *peer, int optlen, void *opt);

typedef struct SceNetAdhocMatchingMember {   // 12 bytes
	struct SceNetAdhocMatchingMember *next;
	SceNetEtherAddr mac;
	u8 pad[2];
} SceNetAdhocMatchingMember;

typedef struct { u32 poolSize; u32 maxSize; u32 freeSize; } SceNetMallocStat;
```

| Mode | |
|---|---|
| 1 | HOST (parent): advertises, accepts up to `maxnum - 1` clients |
| 2 | CLIENT (child): finds hosts, joins one |
| 3 | P2P: both advertise and look, pairs with one partner |

| Event | When | `opt` |
|---|---|---|
| 1 HELLO | an advertisement arrived (host's or P2P peer's), and again whenever its data changes | hello data |
| 2 REQUEST | a join request arrived | the joiner's `SelectTarget` opt |
| 3 LEAVE | a member left or was kicked | |
| 4 DENY | our request was rejected | the host's `CancelTargetWithOpt` opt |
| 5 CANCEL | a pending request was withdrawn | |
| 6 ACCEPT | our request was accepted | the host's `SelectTarget` opt |
| 7 ESTABLISHED | matching with a peer is complete (both sides) | |
| 8 TIMEOUT | a peer stopped responding | |
| 9 ERROR | | |
| 10 BYE | a peer stopped matching | |
| 11 DATA | `SendData` data arrived | the data |
| 12 DATA_ACK | the peer acknowledged our `SendData` | |
| 13 DATA_TIMEOUT | our `SendData` was never acknowledged | |

The callback runs on the context's event thread. Repeated HELLOs from the same host arrive many times
(every `hello_int`).

## sceNetAdhocMatchingInit / Term

```c
int sceNetAdhocMatchingInit(int poolSize); // NID 0x2A2A1E07
int sceNetAdhocMatchingTerm(void);          // NID 0x7945ECDA
```

`Init` creates the library's own memory pool. Twice: `ALREADY_INITIALIZED` (0x80410812). `Term` returns
0, also when not initialized. Before Init, `Create` gives `NOT_INITIALIZED` (0x80410813).

## sceNetAdhocMatchingCreate

```c
int sceNetAdhocMatchingCreate(int mode, int maxnum, u16 port, int rxbuflen, u32 hello_int,
	u32 keepalive_int, int init_count, u32 rexmt_int, SceNetAdhocMatchingHandler handler); // NID 0xCA5EDA6F
```

Creates a matching context on a PDP port. Returns its ID. Intervals are in microseconds.

| Check | Error |
|---|---|
| `mode` not 1-3 | `INVALID_MODE` (0x80410801) |
| `maxnum` not 2-16 | `INVALID_MAXNUM` (0x80410803) |
| `rxbuflen` below `6 * maxnum + 4` (28 for 4, 16 for 2) | `RXBUF_TOO_SHORT` (0x80410804) |
| HOST or P2P with `hello_int` or `keepalive_int` 0 | `INVALID_ARG` (0x80410806) |
| `init_count` negative, `rexmt_int` 0 | `INVALID_ARG` |
| another context on the same port | `PORT_IN_USE` (0x80410814) |
| 15 contexts already | `ID_NOT_AVAIL` (0x80410808) |

A client may pass 0 for `hello_int` and `keepalive_int`. Port 0, `init_count` 0 and a NULL handler are
accepted. No defaults are substituted.

Timing, from the two-console tests: a peer that goes silent times out after about
`keepalive_int * init_count` (2.5 s for 500 ms and 5: TIMEOUT arrived after 2-4 s).

**PPSSPP:** checks only `maxnum`, `rxbuflen >= 1`, the mode (as `INVALID_ARG`) and the port; a
`keepalive_int` of 0 becomes 2 seconds; the timeout is `(keepalive_int + rexmt_int) * init_count + 0.5 s`
(4-8 s in the same test). Context IDs are the lowest free one, with no limit of 15.

## sceNetAdhocMatchingStart / Start2

```c
int sceNetAdhocMatchingStart(int id, int evthPrio, int evthStack, int inthPrio, int inthStack,
	int optlen, const void *opt); // NID 0x93EF3843
int sceNetAdhocMatchingStart2(int id, int evthPrio, int evthPart, int evthStack, int inthPrio,
	int inthPart, int inthStack, int optlen, const void *opt); // NID 0xE8454C65
```

Starts the context's event and input threads; `opt` is the hello data a host or P2P context advertises.
Returns 0. (Only `Start` was tested; `Start2` adds memory partitions for the two threads.) Already running: `IS_RUNNING` (0x8041080A). All 15 possible contexts can run at once.

**PPSSPP:** returns a thread ID instead of 0; starting again gives `PORT_IN_USE`.

## sceNetAdhocMatchingStop / Delete

```c
int sceNetAdhocMatchingStop(int id);   // NID 0x32B156B3
int sceNetAdhocMatchingDelete(int id); // NID 0xF16EAF4F
```

`Stop` returns 0, also when not running; peers get BYE. `Delete` on a running context gives `IS_RUNNING`;
stop it first. Unknown ID: `INVALID_ID` (0x80410807).

**PPSSPP:** `Delete` stops a running context and deletes it; unknown IDs return 0.

## sceNetAdhocMatchingSelectTarget

```c
int sceNetAdhocMatchingSelectTarget(int id, const SceNetEtherAddr *peer, int optlen, const void *opt); // NID 0x5E3D4B79
```

A client: ask to join this host (the host gets REQUEST with `opt`). A host: accept this client's request
(the client gets ACCEPT with `opt`, then both get ESTABLISHED).

- Not running: `NOT_RUNNING`. Not a peer: `UNKNOWN_TARGET` (0x8041080C).
- A second call for the same peer: `REQUEST_IN_PROGRESS` (0x8041080F), on both sides.
- A host accepting a request the client already cancelled: `TARGET_NOT_READY` (0x8041080D).
- A full host (`maxnum - 1` clients) stops advertising.
- **The first join request after a host changes its hello data (`SetHelloOpt`) never reaches the host**,
  even if the client has seen the new hello. Later requests work.

**PPSSPP:** the second call gives `ALREADY_ESTABLISHED`; the first request after `SetHelloOpt` works.

## sceNetAdhocMatchingCancelTarget / CancelTargetWithOpt

```c
int sceNetAdhocMatchingCancelTarget(int id, const SceNetEtherAddr *peer);                               // NID 0xEA3C6108
int sceNetAdhocMatchingCancelTargetWithOpt(int id, const SceNetEtherAddr *peer, int optlen, const void *opt); // NID 0x8F58BEDF
```

A host rejecting a request (the client gets DENY, with `opt`), a client withdrawing its request (the host
gets CANCEL), or either side ending an established match (the other gets LEAVE). Not running:
`NOT_RUNNING`; not a peer: `UNKNOWN_TARGET`.

**PPSSPP:** an unknown peer returns 0.

## sceNetAdhocMatchingSendData / AbortSendData

```c
int sceNetAdhocMatchingSendData(int id, const SceNetEtherAddr *peer, int len, const void *data); // NID 0xF79472D7
int sceNetAdhocMatchingAbortSendData(int id, const SceNetEtherAddr *peer);                      // NID 0xEC19337D
```

Sends data to an established peer, which gets DATA. When the peer acknowledges it, the sender gets
DATA_ACK. **A second `SendData` to the same peer before the first is acknowledged gives `DATA_BUSY`
(0x80410818).** Not running: `NOT_RUNNING`; not a peer: `UNKNOWN_TARGET`.

Data to a peer that has left the network without ending the matching session (it's already gone from
adhocctl's peer list) is accepted (0), and DATA_TIMEOUT arrives 1-2 seconds later.

`AbortSendData` stops waiting for the acknowledgement (no DATA_ACK comes), so the next `SendData` is
accepted at once. The aborted data still arrives. Returns 0, also with nothing pending; an unknown peer
gives `UNKNOWN_TARGET`.

**PPSSPP:** DATA_ACK is generated locally right away, so `DATA_BUSY` never happens; data events can arrive
in reverse order.

## sceNetAdhocMatchingSetHelloOpt / GetHelloOpt

```c
int sceNetAdhocMatchingSetHelloOpt(int id, int optlen, const void *opt); // NID 0xB58E61B7
int sceNetAdhocMatchingGetHelloOpt(int id, int *optlen, void *opt);      // NID 0xB5D96C2A
```

`SetHelloOpt` changes the advertised data; clients get a new HELLO with it. Not running: `NOT_RUNNING`.

`GetHelloOpt` copies the hello data into `opt`, **at most `*optlen` bytes, and leaves `*optlen` as it
was**. Not running: `NOT_RUNNING`.

**PPSSPP:** `GetHelloOpt` writes the full length and copies all of it, regardless of the buffer; it returns
0 before Start.

## sceNetAdhocMatchingGetMembers

```c
int sceNetAdhocMatchingGetMembers(int id, int *size, SceNetAdhocMatchingMember *buf); // NID 0xC58BCD9E
```

The members, 12 bytes each: **the console itself first**, then the others. A lone host reports one member,
itself. After a member leaves, it's gone from the list. Not running: `NOT_RUNNING`.

**PPSSPP:** a client lists the host first; a member that left can stay listed.

## sceNetAdhocMatchingGetPoolStat / GetPoolMaxAlloc

```c
int sceNetAdhocMatchingGetPoolStat(SceNetMallocStat *stat); // NID 0x9C5CFB7D
int sceNetAdhocMatchingGetPoolMaxAlloc(void);              // NID 0x40F8F435
```

The pool's statistics. For a pool of 0x20000, `poolSize` reads 0x1FFE0. NULL: `INVALID_ARG`.
`GetPoolMaxAlloc` returned 0 or less right after Init.

**PPSSPP:** made-up values (the full size, half of it as the maximum).
