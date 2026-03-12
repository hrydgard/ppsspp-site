---
position: 4
---

# Networking / Multiplayer

## Quick start guide

To get started with multiplayer in PPSSPP: [Quick-start guide](/docs/multiplayer/quickstart).

Below is the detailed documentation for all the settings.

## Networking

### Enable networking/WLAN

This corresponds to the physical WLAN switch on the real PSP. Just turn it on if you want to try multiplayer. However, if you're only going to play single player, it's recommended to turn this off - a few games like Obscure will enable networking if this is on, which disables save states.

### Send Discord "Rich Presence" information

This setting is currently only available on Windows. It will send information to a Discord client running on the same Windows session, showing it as "currently playing" if you are currently sharing your status.

### Change MAC address

You can change the MAC address of the emulated PSP here. However, it's not recommended to change it once you have set it - since some games will lock savegames to the specific address, invalidating them if you change it!

### WLAN channel

This setting doesn't really do anything meaningful, considering removing it.

## Ad hoc multiplayer

### Change PRO adhoc server IP address

This lets you select what server to use.

### Try to use server-provided packet relay

Relay mode is the new way of playing adhoc which is very easy to set up. See [How to play](/docs/multiplayer/how-to-play) for more information.

### Change nickname

Lets you set the Nickname setting of the emulated PSP. This is what shows up as your username when playing in ad hoc mode.

### Enable built-in PRO adhoc server

This is used for local play on LAN or wifi. See [How to play](/docs/multiplayer/how-to-play) for more information.

## Infrastructure

This is the setup to use when playing on infrastructure revival servers.

### Username

This is your player name when playing on infrastructure servers.

Try to make it as unique as possible, as there's no global registry of these and no registration mechanism.

### Autoconfigure

See [Infrastructure servers](/docs/multiplayer/infrastructure-servers) for more information.

## UPnP (port forwarding)

This is used when playing ad hoc in the old P2P (non-relay) mode.

## Chat / Quick chat

Configure multiplayer chat here.

## Miscellaneous

### Port offset

If you want to try connecting to a real PSP, use 0 here. Otherwise, unless you have a very specific an unusual use case, always leave this at the default of 10000. Players who want to play together need to have the same port offset for connections to succeed.

### Allow speed control while connected

This lets you use fast-forward while connected. This frequencly leads to desyncs and disconnections so it's not recommended for use. There are a few games that are resilient to this though, like Phantasy Star Online.