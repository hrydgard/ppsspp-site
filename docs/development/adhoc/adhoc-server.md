---
position: 7
---

# The adhoc server

The adhoc server replaces what a PSP would learn from the radio: which groups exist, who is in them, and
how to reach them. PPSSPP, and PSPs and Vitas running the aemu plugin, all speak the same protocol to it,
which goes back to Coldbird's PRO Online server. It's sometimes called the "adhocctl server" or "PRO
server".

## Protocol (version 1)

The client opens a TCP connection to port **27312** and sends LOGIN, then any of the other requests. The
stream has no framing: each packet starts with an opcode byte, which determines its size. All structs
are packed. IP addresses are IPv4, in network byte order.

| Opcode | Name | Direction | Size | Contents |
|---|---|---|---|---|
| 0 | PING | client → server | 1 | |
| 1 | LOGIN | client → server | 144 | MAC (6), nickname (128), product code (9) |
| 2 | CONNECT | client → server | 9 | group name (8): join this group, creating it if it doesn't exist |
| 2 | CONNECT | server → client | 139 | nickname (128), MAC (6), IP (4): this player is in your group |
| 3 | DISCONNECT | client → server | 1 | leave the group |
| 3 | DISCONNECT | server → client | 5 | IP (4): this player left your group |
| 4 | SCAN | client → server | 1 | list the groups of my product code |
| 4 | SCAN | server → client | 15 | group name (8), host MAC (6): one group |
| 5 | SCAN_COMPLETE | server → client | 1 | end of the list |
| 6 | CONNECT_BSSID | server → client | 7 | MAC (6) of the group's creator: you are now in the group |
| 7 | CHAT | client → server | 65 | message (64) |
| 7 | CHAT | server → client | 193 | message (64), sender's nickname (128) |

### Rules

- **Login**: the product code must be 9 characters of `A-Z0-9`, the MAC must not be all zeros or all
  `FF`, and the nickname must not be empty.
- **Keep-alive**: the client sends PING every 2 seconds. The server drops clients it hasn't heard from in a
  while (15 seconds in PPSSPP's server, 30 in aemu_postoffice's).
- **Joining**: when a player joins a group, the server sends them a CONNECT for every member already in it,
  sends each member a CONNECT for the new player, and finally sends the new player CONNECT_BSSID.
- **Leaving**: the remaining members get DISCONNECT with the IP address of the player who left. Since
  players are identified only by IP address here, two players behind one IP address confuse it.
- **Scanning** returns the groups of the same product code (after crosslinking, see below).
- **Chat** goes to everyone else in the group. Messages are cut to 63 characters.
- **Crosslinks**: a server can map product codes to each other, so that different regional releases of a
  game end up in the same "game" and can see each other's groups.

### Using UDP (ENet)

aemu_postoffice's server also accepts the same protocol over [ENet](http://enet.bespin.org/) (reliable
UDP) on UDP port 27312, with one channel; the byte stream is the same. PPSSPP uses this when "Connect to
adhoc server using UDP (ENet) instead of TCP" is on. The setting applies to the relay connections too.

## PPSSPP's built-in server

PPSSPP contains a port of the original PRO server (`Core/HLE/proAdhocServer.cpp`). With "Enable built-in
ad hoc server" on, PPSSPP starts it when emulation starts, on TCP port 27312, unless something is already
answering on that port. It also requests a UPnP mapping for the port.

Its main use is local play: several PPSSPP instances on one computer, or a few on a LAN, with one of them
running the server.

- **Several instances on one machine**: with the server address set to `localhost` (or any `127.*`
  address), each instance binds its sockets to its own loopback address, 127.0.0.*N* for instance *N*, and
  gets its own MAC address. The server sees them as different players.
- **One player per IP address**: the server refuses a second connection from an IP address that's already
  connected.
- **No relay, no UDP**: in relay mode or with UDP on, PPSSPP shows a warning on screen.
- **Strict states**: a client that sends a request in the wrong state (joining while in a group, scanning
  while in a group, chatting while not in one) is logged out.
- **Status file**: on every change it writes `www/status.xml` relative to the working directory, if a `www`
  folder exists there, in the same format as the public PRO servers' status pages (below).
- It handles one packet per client every 10 ms, single-threaded.

## aemu_postoffice's server

The server in `ext/aemu_postoffice/server_cpp` (from Kethen's [aemu_postoffice](https://github.com/Kethen/aemu_postoffice))
is the one most public servers run. It combines the adhoc server with the
[relay](/docs/development/adhoc/relay) and a status page, and differs from PPSSPP's built-in one:

- TCP and ENet, multi-threaded.
- Up to 16 clients per IP address (configurable), and a logging-in client with an existing MAC address
  replaces the old session.
- Clients that have logged in but not joined a group are kept in a hidden "groupless" group.
- **`adhocctl_relay_only`** (on by default): instead of real IP addresses, the CONNECT and DISCONNECT
  packets carry unique IDs, which hides players' addresses and fixes the "two players behind one IP" problem.
  It also means P2P mode can't work against such a server; clients must use relay mode.
- A game database (`game_db.json`) with display names and crosslinks, and a few quirk fixes (for example,
  all non-lobby groups of Call of Duty: Roads to Victory are merged into one).

Its full configuration is documented in
[hosting.md](https://github.com/Kethen/aemu_postoffice/blob/main/hosting.md).

## The server list

PPSSPP's list of public servers is downloaded from `http://metadata.ppsspp.org/adhoc-servers.json` (cached
for 24 hours; the URL is configurable and can be a local file), falling back to the copy shipped in
`assets/adhoc-servers.json`. While a game is running, only the cached copy is used.

```json
{
  "servers": [
    {
      "name": "Example",
      "host": "adhoc.example.com",
      "location": "Europe",
      "description": "...",
      "web": "https://example.com",
      "discord": "https://discord.gg/...",
      "data_mode": "AemuPostoffice",
      "status_data_json": "http://adhoc.example.com:8888/data.json",
      "status_xml": "https://example.com/status.xml",
      "hidden": false
    }
  ]
}
```

| Key | Meaning |
|---|---|
| `host` | Hostname or IP address; required |
| `name`, `location`, `description`, `web`, `discord` | Shown in the server browser |
| `data_mode` | `"AemuPostoffice"` if the server has a relay; otherwise P2P |
| `status_data_json` | URL of a `data.json` live status page |
| `status_data_json_2` | The same, for pages whose numbers are strings; older PPSSPP versions crash on those, so it has its own key |
| `status_xml` | URL of a PRO-style `status.xml` |
| `status_web` | A status web page |
| `hidden` | Only shown while selected |

Users can also add their own servers, either as P2P or as relay servers.

## Status pages

Two formats are shown in PPSSPP's server browser.

`data.json`, served by aemu_postoffice on port 8888:

```json
{ "games": [ {
    "game_ids": ["ULUS10336"],
    "name": "Game name",
    "usercount": 3,
    "groups": [ {
        "name": "MyGroup", "usercount": 2,
        "users": [ { "name": "nickname", "pdp_ports": [10123], "ptp_ports": [] } ]
    } ]
} ] }
```

`status.xml`, from PRO-style servers and PPSSPP's built-in one:

```xml
<prometheus usercount="3">
	<game name="Game name" usercount="3">
		<group name="MyGroup" usercount="2">
			<user>nickname</user>
		</group>
		<group name="Groupless" usercount="1" />
	</game>
</prometheus>
```

See also [Ad hoc live server status](/docs/multiplayer/adhoc-server-status) and
[Setting up relay servers](/docs/multiplayer/setting-up-relay-servers).
