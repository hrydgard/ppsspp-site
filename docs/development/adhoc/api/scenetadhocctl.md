---
position: 3
---

# sceNetAdhocctl

Ad hoc control: joining and leaving groups (ad hoc networks), scanning for them, the peer list, and game
mode setup. For the concepts, see [sceNetAdhocctl: groups and peers](/docs/development/adhoc/adhocctl).
Tests: `net/adhoc/adhocctl`, `net/adhoc/gamemode`, `net/adhoc/pair/basic`, `pair/lifecycle`,
`pair/vanish`, `pair/gamemode`.

## Types

```c
typedef struct { int type; char product[9]; u8 pad[3]; } SceNetAdhocctlAdhocId;       // 16 bytes
typedef struct { u8 data[6]; } SceNetEtherAddr;                                        // 6 bytes

typedef struct SceNetAdhocctlPeerInfo {                                                // 152 bytes
	struct SceNetAdhocctlPeerInfo *next;
	char nickname[128];
	SceNetEtherAddr mac;
	u8 pad[6];
	u64 last_recv;      // microseconds; when the peer was last heard from
} SceNetAdhocctlPeerInfo;

typedef struct SceNetAdhocctlScanInfo {                                                // 28 bytes
	struct SceNetAdhocctlScanInfo *next;
	int channel;
	char group[8];
	SceNetEtherAddr bssid;
	u8 pad[2];
	int mode;           // 0 normal
} SceNetAdhocctlScanInfo;

typedef struct {                                                                       // 148 bytes
	int channel;
	char group[8];
	char nickname[128];
	SceNetEtherAddr bssid;
	u8 pad[2];
} SceNetAdhocctlParameter;

typedef struct { int num; SceNetEtherAddr macs[16]; } SceNetAdhocctlGameModeInfo;

typedef void (*SceNetAdhocctlHandler)(int event, int error, void *arg);
```

List functions (`GetPeerList`, `GetScanInfo`, `GetAddrByName`) all work the same way: pass `*size` and a
buffer; with a NULL buffer, `*size` is set to the bytes needed. The entries are linked through `next`,
the last one NULL.

### States

| State | |
|---|---|
| 0 | DISCONNECTED |
| 1 | CONNECTED (in a normal group) |
| 2 | SCANNING |
| 3 | GAMEMODE |
| 4 | DISCOVER (sceNetAdhocDiscover running) |
| 5 | WOL |

### Events

| Event | When |
|---|---|
| 0 ERROR | an operation failed; `error` says why |
| 1 CONNECT | now in a group (after Create, Connect or Join) |
| 2 DISCONNECT | left the group (after Disconnect, `ExitGameMode`, or discover stopping) |
| 3 SCAN | a scan finished |
| 4 GAME | game mode entered: all members are present |
| 5 DISCOVER | |
| 6 WOL | |
| 7 WOL_INTERRUPTED | |

Each registered handler gets each event once.

## sceNetAdhocctlInit

```c
int sceNetAdhocctlInit(int stackSize, int priority, const SceNetAdhocctlAdhocId *adhocId); // NID 0xE26F226E
```

Starts adhocctl, with its thread at the given stack size and priority, for the given product code.
Consoles only see groups of the same type and product code.

| Check | Error |
|---|---|
| `stackSize` below 0xC00 | `STACKSIZE_TOO_SHORT` (0x80410B13) |
| priority outside the thread priority range (0 or 0x80 fail) | 0x80020193 (illegal priority) |
| `adhocId` NULL, or `type` above 2 | `INVALID_ARG` (0x80410B04) |
| already initialized | `ALREADY_INITIALIZED` (0x80410B07), see below |

The product code itself isn't checked (lowercase is accepted).

**Calling Init a second time breaks adhocctl.** It returns `ALREADY_INITIALIZED` *and* leaves adhocctl
uninitialized (`GetState` gives `NOT_INITIALIZED`), and after that Init fails with 0x80410601 until the
modules are reloaded.

**Term followed straight by Init** can be undone by Term finishing in the background; wait a little
(100 ms is enough) between them.

**PPSSPP:** checks none of these. A second Init leaves adhocctl initialized. Init also logs in to the ad
hoc server and blocks for up to 5 seconds while it does, returning `BUSY` if it can't.

## sceNetAdhocctlTerm

```c
int sceNetAdhocctlTerm(void); // NID 0x9D689E13
```

Leaves any group, stops adhocctl. Returns 0, also when not initialized. Returns before adhocctl's thread
has stopped (see Init).

## sceNetAdhocctlAddHandler / DelHandler

```c
int sceNetAdhocctlAddHandler(SceNetAdhocctlHandler handler, void *arg); // NID 0x20B317A0
int sceNetAdhocctlDelHandler(int id);                                  // NID 0x6402490B
```

`AddHandler` returns the handler's ID (0, 1, 2, 3). **At most 4 handlers**; a fifth gives
`TOO_MANY_HANDLERS` (0x80410B12). The same function can be added more than once, under a new ID, and is
then called once per registration. NULL gives `INVALID_ARG`. Before Init: `NOT_INITIALIZED`.

`DelHandler` with an unknown ID (or -1) gives `ID_NOT_FOUND` (0x80410B06).

**PPSSPP:** up to 32 handlers; `AddHandler` works before Init; adding the same function again returns 0
without a new registration; `DelHandler` with an unknown ID returns 0.

## sceNetAdhocctlCreate

```c
int sceNetAdhocctlCreate(const char *groupName); // NID 0xEC0635C1
```

Creates a new group (network) and joins it. Returns 0 at once; the handler gets CONNECT when the group
is up (well under a second on a PSP, with nobody else around). The group name is up to 8 characters of
letters and digits; a dash, space or underscore gives `INVALID_ARG` (0x80410B04). **A NULL name is
accepted.**

- The BSSID is a random locally administered address, not the console's MAC.
- Two consoles each calling Create with the same name get two separate networks that don't see each
  other.
- While already in a group: returns 0, then the handler gets ERROR with `ALREADY_CONNECTED`
  (0x80410B02).
- While apctl is connected to an access point: returns 0, then ERROR with 0x80410D01.

**PPSSPP:** an invalid name gives the sceNetAdhoc `INVALID_ARG` (0x80410711). Groups are made by the ad
hoc server; two Creates with the same name end up in the same group. The CONNECT event comes after a
built-in delay of up to 2 seconds.

## sceNetAdhocctlConnect

```c
int sceNetAdhocctlConnect(const char *groupName); // NID 0x0AD043ED
```

Joins the group with that name if a scan finds one, otherwise creates it. Results as for Create. A
console that left a group can rejoin the same network (same BSSID) with Connect, as long as someone is
still in it.

## sceNetAdhocctlJoin

```c
int sceNetAdhocctlJoin(const SceNetAdhocctlScanInfo *scanInfo); // NID 0x5E7F79C9
```

Joins the network described by a scan result. CONNECT follows; the BSSID is the scanned one.

**PPSSPP:** only the group name in the scan result is used.

## sceNetAdhocctlDisconnect

```c
int sceNetAdhocctlDisconnect(void); // NID 0x34401D65
```

Leaves the group. Returns 0 at once; DISCONNECT follows. Returns 0 when not in a group too.

- The group lives on without its creator: the others stay CONNECTED with the same BSSID and get no
  event; they just see the peer leave their peer lists within a few seconds.
- Sockets aren't closed. PTP connections to the remaining peers fail with `DISCONNECTED` after 1-2
  seconds, on the side that stayed; on the side that left, they report no error.

## sceNetAdhocctlScan

```c
int sceNetAdhocctlScan(void); // NID 0x08FFF7A0
```

Scans for groups of the same product code. Returns 0 at once; SCAN follows when it's done, and the
state goes back to DISCONNECTED. While in a group: 0, then ERROR with `ALREADY_CONNECTED`.

**Game mode groups don't show up in a normal scan.**

## sceNetAdhocctlGetScanInfo

```c
int sceNetAdhocctlGetScanInfo(int *size, SceNetAdhocctlScanInfo *buf); // NID 0x81AEE1BE
```

The groups found by the last scan, 28 bytes each, with their channel (1, 6 or 11), BSSID and mode.
Returns 0, with `*size` 0 when nothing was found (including while connected).

## sceNetAdhocctlGetState

```c
int sceNetAdhocctlGetState(int *state); // NID 0x75ECD386
```

The current state. `NOT_INITIALIZED` before Init.

## sceNetAdhocctlGetParameter

```c
int sceNetAdhocctlGetParameter(SceNetAdhocctlParameter *param); // NID 0xDED9D28E
```

The channel, group name, own nickname and BSSID. While not in a group: `DISCONNECTED` (0x80410B09).

**PPSSPP:** returns 0 with an empty group name while not in a group. The BSSID is the group creator's MAC.

## sceNetAdhocctlGetPeerList

```c
int sceNetAdhocctlGetPeerList(int *size, SceNetAdhocctlPeerInfo *buf); // NID 0xE162CB14
```

The other consoles in the group, 152 bytes each, with nickname, MAC and `last_recv`. The console itself
isn't listed. While not in a group: 0x80410682, with `*size` 0.

A peer that leaves disappears within a few seconds (under 5 in the tests); there's no event for it.

**PPSSPP:** returns 0 while not in a group. `last_recv` is always reported as at most 10 ms old.

## sceNetAdhocctlGetPeerInfo

```c
int sceNetAdhocctlGetPeerInfo(const SceNetEtherAddr *mac, int size, SceNetAdhocctlPeerInfo *info); // NID 0x8DB83FDC
```

One peer's entry. A MAC that isn't a peer, **including the console's own**, gives `NO_ENTRY`
(0x80410716). A `size` smaller than the struct gives `INVALID_BUFLEN` (0x80410704).

**PPSSPP:** answers for the console's own MAC; a small size gives the adhocctl `INVALID_ARG`.

## sceNetAdhocctlGetNameByAddr

```c
int sceNetAdhocctlGetNameByAddr(const SceNetEtherAddr *mac, char *nickname); // NID 0x8916C003
```

A peer's nickname (128 bytes). The console's own MAC, or any other non-peer, gives `NO_ENTRY`.

**PPSSPP:** answers for the console's own MAC.

## sceNetAdhocctlGetAddrByName

```c
int sceNetAdhocctlGetAddrByName(const char *nickname, int *size, SceNetAdhocctlPeerInfo *buf); // NID 0x99560ABE
```

The peers with that nickname, as peer list entries.

**PPSSPP:** also includes the console itself if its nickname matches.

## sceNetAdhocctlGetAdhocId

```c
int sceNetAdhocctlGetAdhocId(SceNetAdhocctlAdhocId *id); // NID 0x362CBE8F
```

The ad hoc ID passed to Init.

## sceNetAdhocctlCreateEnterGameMode

```c
int sceNetAdhocctlCreateEnterGameMode(const char *groupName, int type, int num,
	const SceNetEtherAddr *members, u32 timeout, int flag); // NID 0xA5C055CE
```

Creates a game mode group as its master. `members` lists all `num` members, the master (this console)
first; `timeout` is how long to wait for them, in microseconds. Returns 0 at once; the handler gets GAME
when every member has joined, or ERROR.

| Check | Error |
|---|---|
| NULL or invalid group name, NULL members | `INVALID_ARG` |
| `type` not 1-3 | `INVALID_ARG` |
| `num` below 2, above 16, or above 4 for type 1 | `INVALID_ARG` |

- Members not there in time: ERROR with the adhocctl `TIMEOUT` (0x80410B05), state DISCONNECTED.
- The first member isn't this console: accepted, then ERROR with `INVALID_ADDR` (0x80410702).
- **A timeout of 0 waits forever**, and `ExitGameMode` returns `BUSY` (0x80410B10) meanwhile.
- While in a normal group: 0, then ERROR with `ALREADY_CONNECTED`.
- Now and then the first attempt never delivers its timeout event, and adhocctl stays BUSY until reset.

**PPSSPP:** a NULL group name is accepted and a bad one gives 0x80410711; the timeout ERROR carries the
sceNetAdhoc `TIMEOUT` (0x80410715) instead.

## sceNetAdhocctlCreateEnterGameModeMin

```c
int sceNetAdhocctlCreateEnterGameModeMin(const char *groupName, int type, int min, int num,
	const SceNetEtherAddr *members, u32 timeout, int flag); // NID 0xB0B80E80
```

The same, but game mode starts once `min` members are present. `min` must be at least 2 and at most
`num`, else `INVALID_ARG`.

**PPSSPP:** ignores `min` and its checks.

## sceNetAdhocctlJoinEnterGameMode

```c
int sceNetAdhocctlJoinEnterGameMode(const char *groupName, const SceNetEtherAddr *master,
	u32 timeout, int flag); // NID 0x1FF89745
```

Joins the master's game mode group. NULL group or master: `INVALID_ARG`. GAME follows once in; if the
master isn't there in time, ERROR with `TIMEOUT`.

## sceNetAdhocctlExitGameMode

```c
int sceNetAdhocctlExitGameMode(void); // NID 0xCF8E084D
```

Leaves game mode; DISCONNECT follows. Returns 0 outside game mode, `INVALID_IBSS` (0x80410B0B) in a normal
group, `BUSY` while game mode waits for members with no timeout.

## sceNetAdhocctlGetGameModeInfo

```c
int sceNetAdhocctlGetGameModeInfo(SceNetAdhocctlGameModeInfo *info); // NID 0x5A014CE0
```

The members, master first, on every console. Outside game mode: `NOT_ENTER_GAMEMODE` (0x80410B0C).

**PPSSPP:** returns 0 with 0 members outside game mode.
