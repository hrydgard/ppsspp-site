---
title: Quickstart guide
position: 1
---
# Multiplayer on PPSSPP - quickstart

The PSP supports two fundamentally different methods of multiplayer - **infrastructure** and **ad hoc**. *Infrastructure* means that games connect to PSN or other servers, working much like most modern multiplayer games. *Ad hoc* is peer-to-peer, and was meant to be used between player in the same room, much like say the Game Boy link cable, but wireless.

Some games support one or the other, and some games support both.

In both cases, there are now servers run by volunteers that simulates game servers (infrastructure) or just act as a virtual room (ad hoc).

<div class="alert alert-info">NOTE: If you want to enter a server IP the traditional way, just use "Add server...", type in your IP as the hostname, and UNCHECK the "Relay" checkbox.</div>

## First step

Enable networking on the "Networking" tab in PPSSPP's settings (looks like a Wi-Fi symbol in portrait mode). Then, choose your path:

## Getting started with ad hoc on relay servers

From PPSSPP 1.20.1 and upwards, this is very easy, thanks to relay servers, see [How to play](/docs/multiplayer/how-to-play). Here's the minimal setup:

In Settings / Networking, under `Ad Hoc multiplayer`, set up a "Nickname" and choose a server to play on. For ad hoc, this is the same as the system nickname on the emulated PSP (unlike infrastructure).

Do that on all the participating devices, choosing the same server, then you should simply be able to create and/or join an ad-hoc game room as normal in the game you are playing, if the server you chose supports the "Relay" protocol (most listed ones do). There might already be other people playing, too - join them!

The server list is available directly in the app, but you can also browse it here: [Ad hoc server list](/docs/multiplayer/adhoc-servers/)

## Getting started with infrastructure

Not all games that support infrastructure gameplay have "revival" servers yet, but many do. PPSSPP is, from version 1.19.3, set up to autoconfigure everything.

After enabling networking, all you need to do is to set a "Username" under Infrastructure in Networking settings in PPSSPP. Then things will "just work", in supported games.

<div class="alert alert-info">Note: Unlike ad hoc, infrastructure username is separate from the nickname of the emulated PSP.</div>

## Getting started with ad hoc on LAN

It's totally possible to play PSP ad hoc games the way they were meant to, between people on the same LAN or local wireless network. One of the players should check the box for the "Built-in ad hoc server", and the others should add that player's IP address as a custom server in settings.

<div class="alert alert-warning">IMPORTANT: When adding the server, make sure that "Relay server mode" is <i>unchecked</i>.</div>

## Ad hoc in P2P mode, and other setups

P2P ad hoc mode is more efficient than relay servers, but requires a more complicated setup.

See the [How to play multiplayer games in PPSSPP](https://github.com/hrydgard/ppsspp/wiki/How-to-play-multiplayer-games-with-PPSSPP) in the wiki for more information.