# Help - my saves won't load!

## Where are saves kept?

Saves can be a few different places, depending on your platform and how PPSSPP is installed.  They're always in the "memory stick" which is just a folder.  Inside this folder, saves are usually in PSP -> SAVEDATA.

### Windows

If you extracted the zip somewhere in your Downloads or Desktop, the memory stick will be in "memstick" next to the PPSSPP program.

When PPSSPP can't create files next to the program (usually because it's in Program Files or something), it puts them inside My Documents.  Look for a PPSSPP folder.

### Android

The memstick is just the same as your storage - it's meant to be just like the PSP.

### macOS and Linux

The memstick is found in your "home" or "user" folder, inside `~/.config/ppsspp` (you can select Go To and paste that path into the Finder or other file manager.)

## Do you have the right game?

If you long press on a game, you should see the "game id" at the end of the title.  This might be something like ULUS12345.

Different regions have different codes - and often create different save files.  If you're trying to load a save from a European game with a US game, it probably won't work.

Games almost always have a separate folder inside SAVEDATA for each save, and it starts with their game id.  Often, it might be ULUS1234501 when it's the "01" save for "ULUS12345".  This is the same structure you'll see on a real PSP or in downloaded save data.

## Is the save data corrupted?

Corrupted save data can cause crashes or glitches, and may not load.  Some ways you can get corrupted data:

* Attempting to hack the savedata (or downloading hacked savedata)
* Cheats
* Crash or power loss while saving
* Bugs in PPSSPP (rare) or even games

Note that sometimes games have bugs - some even delete your save data depending on how much free space you have.

## Have you created a space-time paradox?

See [Save state time warps](/docs/troubleshooting/save-state-time-warps).

## Maybe it's a bug in PPSSPP?

Saves work pretty well these days, but sometimes there are bugs.  Check the [PPSSPP forum](https://forums.ppsspp.org/) to see if other people have the same problem with that same game.