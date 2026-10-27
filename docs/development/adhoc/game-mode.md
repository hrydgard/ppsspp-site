---
position: 6
---

# Game mode

Game mode is a special kind of ad hoc network for fast-paced games. The members don't exchange packets
directly. Instead each one publishes a buffer, and the system keeps an up-to-date copy of every other
member's buffer on each console, so the game just reads "the latest version" whenever it likes. Few games
use it; PPSSPP's code mentions Pocket Pool, Shaun White Snowboarding, Fading Shadows, BattleZone, Bomberman
and Star Wars: The Force Unleashed.

## API

### Entering and leaving (sceNetAdhocctl)

| Function | Does |
|---|---|
| `CreateEnterGameMode(group, type, num, macs, timeout, flag)` | Called by the master. `macs` lists all `num` members, master first; `timeout` (µs) is how long to wait for all of them. |
| `CreateEnterGameModeMin(group, type, min, num, macs, timeout, flag)` | The same, but game mode starts once `min` members are present. |
| `JoinEnterGameMode(group, mastermac, timeout, flag)` | Called by everyone else. |
| `ExitGameMode()` | Leaves. |
| `GetGameModeInfo(info)` | Fills `{ int num; u8 macs[16][6]; }` with the members. |

`type` is 1 (type 1A, up to 4 members), 2 (1B) or 3 (2A). `num` is 2 to 16. The group's SSID uses `_G_`
instead of `_L_`. When all members are present, adhocctl raises the GAME event and the state becomes
GAMEMODE (3).

### Sharing data (sceNetAdhoc)

| Function | Does |
|---|---|
| `CreateMaster(data, size)` | Publishes this console's buffer. |
| `CreateReplica(mac, data, size)` | Creates a local copy of another member's buffer. Returns its ID. |
| `UpdateMaster()` | Takes a new snapshot of the master buffer to send. |
| `UpdateReplica(id, &info)` | Copies the newest received data into the replica buffer. |
| `DeleteMaster()` / `DeleteReplica(id)` | |

The `info` struct of `UpdateReplica` is `{ u32 length; u32 updated; u64 timestamp; }`: `updated` is
non-zero if new data arrived since the last call, and `timestamp` says when.

Outside game mode, these fail with `NOT_IN_GAMEMODE` (0x8041071B), and the delete calls with
`NOT_CREATED` (0x8041071C).

### On the PSP

Confirmed with the `net/adhoc/gamemode` test, on one console:

- `CreateEnterGameMode` gives `INVALID_ARG` for a NULL or invalid group name, NULL members, a type other
  than 1-3, fewer than 2 members, more than 4 members for type 1, or more than 16.
- A member list whose first entry isn't the console's own MAC is accepted, then fails with an ERROR event
  carrying `INVALID_ADDR`.
- If the members don't all turn up within the timeout, the handler gets ERROR with the adhocctl `TIMEOUT`
  (0x80410B05), and the state stays DISCONNECTED. The same goes for `JoinEnterGameMode` when the master
  isn't there.
- **A timeout of 0 waits forever**, and `ExitGameMode` returns `BUSY` while it does.
- Entering game mode while in a normal group raises ERROR with `ALREADY_CONNECTED`; `ExitGameMode` there
  gives `INVALID_IBSS` (0x80410B0B). Outside game mode it returns 0.
- `GetGameModeInfo` outside game mode gives `NOT_ENTER_GAMEMODE` (0x80410B0C).

## How PPSSPP implements it

Games named in this section are the ones PPSSPP's code gives as the reason for a behaviour. That those games need it hasn't been rechecked.

PPSSPP follows the aemu plugin's design: the adhoc server sets up the group, and the buffers travel as PDP
datagrams on port 31000 (plus the port offset).

### Entering

- **Master**: `CreateEnterGameMode` checks the arguments, records the list of required members, and
  creates the group through the adhoc server like a normal Create. Each CONNECT message from the server
  adds a member. When everyone on the required list has arrived, the GAME event fires.
- **Others**: `JoinEnterGameMode` records the master's MAC and joins the group. The GAME event fires as soon
  as the server confirms the join.
- If not everyone arrives within the game's timeout, the game gets an ERROR event with `TIMEOUT`.
- `CreateEnterGameModeMin` ignores its minimum and waits for all members.

### Exchanging data

A timer runs every 500 µs of emulated time:

1. Once the master and all replicas exist, it creates a PDP socket on port 31000, sized for the largest
   buffer (members can use different sizes).
2. If the master data changed since it was last sent (`UpdateMaster`), it sends it to every member.
3. It receives one datagram and copies it into the replica for the member that sent it.

During start-up, `CreateMaster`/`CreateReplica` block until every member's data has arrived once (or has
left), resending the master data while waiting, with a 250 ms timeout before everything is resent. Some
games (Pocket Pool) need all replicas to be filled before they start.

`UpdateReplica` copies the latest data and reports a timestamp at most 10 ms old.

### Differences from the PSP

- No time slots or lockstep. Each replica holds whichever datagram arrived last; on a slow or lossy
  connection, members see stale or skipped updates.
- No sequence numbers and no retransmission after start-up.
- Buffers larger than a datagram aren't fragmented by PPSSPP (the host's UDP does that).
- The PSP firmware also synchronizes timing between members over PTP; that isn't emulated.
- The joiner's GAME event doesn't wait until all members are present.
- The update interval of 500 µs was chosen to keep BattleZone at 60 FPS; it's much more frequent than any
  network can deliver, so in practice the rate is limited by the connection.
