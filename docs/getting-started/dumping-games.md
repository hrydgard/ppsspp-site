---
position: 4
---
# Dumping UMD games

To read a game disc into an ISO is called "dumping" for historical reasons - it's old computer lingo for copying every single bit of a medium to a file, for archival storage or other use.

## Prerequisites

- A PSP with custom firmware.
- A PC or other device with a USB port that can copy files through it
- A USB cable with a USB-Mini connector (to fit the PSP).

## Steps

1. Insert the UMD into your PSP.
1. Connect your PSP to your PC with a USB cable.
1. At the PSP main menu, press Select. In the menu that pops up (only on Custom Firmware), choose to UMD as USB Device.
1. Still in the PSP menu, choose USB Connection in the Settings menu (to the left).
1. On your PC, a folder will open automatically, containing a virtual ISO file representing the disc contents. To copy the game to your PC, simply drag this file to somewhere on your hard drive and the copy will start. When the copy is done, rename the file to the name of the game, and it's ready for use.

## Compressed ISO formats

PPSSPP can play files directly from these formats without decompressing them first, which makes them much more practical than keeping your ISOs files in .ZIP or .RAR archives.

### CSO

CSO is the oldest and most well-tested compressed ISO format for PSP, and it's compatible with various custom firmwares for the real PSP, so the files are playable from a memory stick on hardware. It has a good compression ratio, performs very well in PPSSPP, practically as fast or faster than ISO, depending on how slow the drive is you're reading from.

The best tool to compress tools to CSO is [maxcso](https://github.com/unknownbrackets/maxcso/releases) by \[Unknown\].

### CHD

CHD is supported from PPSSPP 1.17, and stands for Compressed Hard Drive. It's MAME's format, originally made for archiving arcade game hard drives, but later repurposed for disc-based systems. It has a slightly better compression ratio than CSO, but to perform properly, PSP ISOs need to be compressed to CHD in the DVD mode (`createdvd`).

The tool to create CHD files is called `chdman` and is distributed with [MAME](https://www.mamedev.org/release.php). You want to use a recent version to compress PSP ISOs.

Here's the command line for compressing a PSP ISO to CHD with chdman, once you have acquired the tool:

```sh
chdman createdvd -i game.iso -o game.chd
```

Just replace "game" with the name of your iso file.

PPSSPP 1.17.1 also supports an additional mode, using zstd for compression:

```sh
chdman createdvd -i game.iso -o game.chd -c zstd
```

IMPORTANT! DO NOT use the `createcd` or `createraw` commands. `createdvd` is what you want to use.

## Digital downloads

If you have digital downloads on your real PSP, they can be used directly on PPSSPP. Just copy the `EBOOT.PBP` over. Note that this has not been tested as much as ISO loading so there may still be issues with some games.
