---
position: 2
---

# Writing an ad hoc game

How a PSP game is typically structured to play ad hoc: the setup, the screens the player goes through,
the game loop, and the traps to avoid. Written for homebrew authors, and for emulator authors who want to
know what games expect. The function details are in the [API reference](/docs/development/adhoc/api/conventions).

## Pick the building blocks

| Need | Use |
|---|---|
| Find other players, form a lobby | adhocctl groups, and usually sceNetAdhocMatching on top |
| Fast, frequent game state (positions, inputs) | PDP datagrams |
| Reliable or bulk data (level data, chat, save transfer) | PTP connections, or matching's `SendData` for small messages |
| Lockstep-style shared state for up to 16 players | Game mode |

Most games use one group per game session, matching for the lobby, and PDP during play. PTP is common for
the initial exchange of settings and for turn-based games.

## Setting up

```c
sceUtilityLoadNetModule(PSP_NET_MODULE_COMMON);
sceUtilityLoadNetModule(PSP_NET_MODULE_ADHOC);
sceNetInit(0x20000, 0x20, 0x1000, 0x20, 0x1000);
sceNetAdhocInit();
SceNetAdhocctlAdhocId id = { 0, "ULUS12345" };     // your product code
sceNetAdhocctlInit(0x2000, 0x30, &id);
sceNetAdhocctlAddHandler(adhocctlHandler, NULL);
sceNetAdhocMatchingInit(0x20000);                  // if you use matching
```

- Do this once, when the player enters the multiplayer menu, and tear it down when they leave it. **Never
  call `sceNetAdhocctlInit` twice**: the second call fails and also breaks adhocctl until the modules are
  reloaded.
- Your own MAC address isn't available from `sceNetGetLocalEtherAddr` until a group operation has
  started; use `sceWlanGetEtherAddr`.
- Check `sceWlanGetSwitchState()` first and tell the player to turn on the WLAN switch.
- Infrastructure (an access point) and ad hoc can't be used at once. If the game also has online play,
  terminate one completely before starting the other.

## The screens

A typical flow, with what each screen does underneath:

```
  Multiplayer menu
   ├─ Host a game ─> [Room settings] ─> Create group ─> Lobby (host) ─> Start
   └─ Join a game ─> Room list (scan) ─> Join group ─> Lobby (client) ─> wait for start
```

### Room list (joining)

1. `sceNetAdhocctlScan()`, and show "Searching...". The handler gets SCAN when it's done, within a few
   seconds.
2. `sceNetAdhocctlGetScanInfo()` and list the groups by name. Scan again every few seconds while the
   screen is open; groups appear and disappear.
3. When the player picks one: `sceNetAdhocctlJoin(&scanInfo)`, show "Connecting...", and wait for CONNECT
   (or ERROR) in the handler. Give up after 10-15 seconds.

The group name is up to 8 letters and digits, so games usually encode a room number or the host's name in
it, or use one fixed name for everyone and let matching sort out the rooms.

Alternatively, run the netconf dialog in JOIN mode and let the system draw this screen.

### Hosting

`sceNetAdhocctlCreate(name)` and wait for CONNECT. (Or `Connect(name)`, which joins an existing group of
that name if a scan finds one.) Then, with matching, create a HOST context and `Start` it with the room's
details as the hello data: name, number of players, settings.

### The lobby

- **The player list:** `sceNetAdhocctlGetPeerList()` each frame or so, showing the nicknames. A player who
  leaves drops out of the list within a few seconds, with no event, so poll it.
- **With matching:** clients get HELLO events with the host's hello data, and show the rooms. The client's
  `SelectTarget(host)` asks to join; the host gets REQUEST and accepts with `SelectTarget(client)` or
  rejects with `CancelTarget`; both sides get ESTABLISHED. `GetMembers` lists who's in. A full host
  (`maxnum - 1` clients) stops advertising.
- When the host changes room settings, `SetHelloOpt` updates the advertisement. (On a PSP, the first join
  request after a change can be lost, so clients should retry a join that gets no answer.)
- **Starting:** the host tells everyone (a matching `SendData`, or a PTP message), and everyone moves on.

## The game loop

Never block the main loop on the network. Two common structures:

**Polling, in the main thread**, every frame:

```c
for (;;) {                                         // drain everything that arrived
	int len = sizeof(buf);
	int r = sceNetAdhocPdpRecv(sock, &from, &port, buf, &len, 0, 1);   // non-blocking
	if (r == WOULD_BLOCK) break;
	if (r == NOT_ENOUGH_SPACE) { /* your buffer is too small for this datagram */ }
	handlePacket(&from, buf, len);
}
simulate();
sceNetAdhocPdpSend(sock, &peer, port, &state, sizeof(state), 0, 1);  // non-blocking
```

**A network thread** that blocks in `PdpRecv` with a timeout and hands packets to the game through a queue.
To stop it, set a receive alert (`sceNetAdhocSetSocketAlert(sock, 0x20)`), which wakes the blocked call
with `SOCKET_ALERTED`, or delete the socket, which wakes it with `SOCKET_DELETED`. Alerts stay set until
cleared.

Rules of thumb for PDP:

- **Keep datagrams at or below 1444 bytes.** Larger ones are split into fragments on the air, and losing
  any fragment loses the whole datagram. Above about 16 KB they don't arrive at all, even though the send
  succeeds.
- **Expect loss.** A burst of 50 small datagrams usually loses a few. Number your packets, send state rather
  than deltas where you can, and resend what must arrive (or use PTP for it).
- **Your own broadcasts don't come back to you**, so a host that broadcasts state must apply it locally too.
- Send to each peer individually or broadcast to `FF:FF:FF:FF:FF:FF`; `00:00:00:00:00:00` is invalid.
- Size the receive buffer (`PdpCreate`'s `bufsize`) for a few frames of traffic: datagrams that don't fit
  are dropped.

For PTP, remember that `PtpOpen` starts connecting at once; `PtpConnect` waits for it, and a peer that isn't
listening yet gives `CONNECTION_REFUSED` straight away, so retry for a while when both sides start at the
same moment.

## Noticing that someone left

There are several signals, and a robust game uses more than one:

| Signal | When |
|---|---|
| The peer disappears from `GetPeerList` | within a few seconds of leaving the group |
| PTP send/receive gives `DISCONNECTED` | the peer closed, or left the group (1-2 s) |
| Matching TIMEOUT | about `keepalive_int * init_count` after the peer went silent |
| Matching BYE or LEAVE | the peer stopped matching or cancelled |
| No packets from them | your own heartbeat timeout |

When the host leaves, the group itself survives (the others stay connected to it), so clients must decide
for themselves that the session is over.

## Shutting down

```c
// stop and delete matching contexts, close PTP sockets, delete PDP sockets
sceNetAdhocMatchingTerm();
sceNetAdhocctlDisconnect();     // wait for DISCONNECT
sceNetAdhocctlTerm();
sceNetAdhocTerm();
sceNetTerm();
sceUtilityUnloadNetModule(PSP_NET_MODULE_ADHOC);
sceUtilityUnloadNetModule(PSP_NET_MODULE_COMMON);
```

`sceNetAdhocctlTerm` returns before its thread has fully stopped; wait a little before initializing again.

## Things that bite

- NULL pointers where the API doesn't expect them can hang the console instead of returning an error.
- Socket IDs only increase; don't assume a freed ID comes back.
- 255 sockets at most, PDP and PTP together; 15 matching contexts.
- A matching `SendData` before the previous one to that peer was acknowledged fails with `DATA_BUSY`.
- Game mode with a timeout of 0 waits forever, and can't be exited while it does.
- Game mode groups don't show up in a normal scan.

## Playing well over the internet

On PPSSPP (and with the aemu plugin), ad hoc traffic goes over the internet, with tens to hundreds of
milliseconds of latency instead of one, and through a relay server for many players. Games that work
well there:

- don't use timeouts shorter than a second or two for connects, joins and acknowledgements;
- tolerate a packet arriving several frames late;
- don't depend on exact timing between consoles.
