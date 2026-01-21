---
position: 3
---

# Ad-Hoc Server List

For more information about Ad-Hoc play, see [How to play Ad-Hoc Multiplayer](/docs/multiplayer/how-to-play).

Here's a list of known working servers:

[socom.cc](http://socom.cc)

Some older Ad-Hoc servers that no longer work, listed here in order to appear in search results:

* coldbird
* myneighborsushicat

# Ad-Hoc tips

## Port offsets

Some games use network ports that are not allowed for general use on general Linux systems (like Android), where ports 1-1024 are considered `privileged`.

To get around this, PPSSPP supports adding an offset to all network port numbers. Note, this of course means that for two
players to be able to play each other, they must all agree on and use the same port offset. Setting this up can be
tricky to coordinate if you don't have a shared chat, or something.

## Server-provided packet relay

This is a new feature in PPSSPP 1.20, implemented by Kethen in #21116.

It uses the `aemu_postoffice` [protocol](https://github.com/Kethen/aemu_postoffice/blob/main/design.md) to route packets in a more reliable way than the usual direct connections.

It's for users in these situations:

### Users currently have access to reliable adhoc in P2P mode

For those who have no issue using P2P mode with their current friend group, either with working opened internet ports, or using virtual LAN networks like hamachi/netmaker, this update is somewhat irrelevant to them, since they have things working just fine on P2P already.

Going for a relay server on the internet might even cause extra latency and timing issues that was not experienced before, since all communication go through the server first, especially when they are physically far away from the server, but are close to each other.

### Users struggling to get adhoc working

Quite a number of users complain about not being able to setup port forwarding on their network, due to ISP router issues and restrictions, carrier grade NAT, user unfriendly router configuration UI, mobile internet, other people not wanting to setup virtual LAN with them etc.

For those users, it makes it much more simple to get started. With stable connection to the relay server, one can expect latency forgiving titles to just work, like the monster hunter series, ridge racer series, call of duty road to victory, maybe a bit of GTA as well. It also provides a much more simple way to cross-play with PSP/PSVitas, since relay server support is also available on those, one does not need to setup port forwarding/virtual LAN for consoles with this configuration.

Since the relay server code is also available to everyone, one with access to working port forwarding and the know-how can also setup their own relay for their friends to keep latency low. Those who are joining will only be required to enable relay mode, and change the ip address to the server, instead of going through setting up port forwarding / virtual LAN.
