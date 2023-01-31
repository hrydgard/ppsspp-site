# Disc streaming

With the disc streaming feature, you can stream games from one device to another on your local network. This can be useful for testing a large variety of games on a mobile device, for example, without having to copy them over one by one.

This feature is not yet fully polished, but quite workable.

## Quick setup guide

On your PC (or, the device that has your ISO files and will be streamed from):

1. Run the games you want to share, one after the other, so they all end up in Recent.

1. Then, go into Settings/Tools/Remote Disc Streaming.

1. Click "Share games (server)"

Then, on the device you want to play the games on:

1. Make sure it's on the same network (LAN or WLAN) as the PC from above.

1. Go into Settings/Tools/Remote Disc Streaming.

1. Click "Browse games"

You will hopefully be presented with a list of the games from Recent on the PC, and they should just work to click on and play.

## Use a regular web server instead

You can also just run a webserver (nginx, apache, etc) and serve the files from there, or a little docker / nodejs thing setup that will serve a directory tree.

If you do either of those, you have to go into Settings from the Remote Disc Streaming screen, and tell it the IP and port, it won't find it magically. Generally, all PPSSPP actually requires is a webserver that supports HTTP/1.1 range requests (most do).