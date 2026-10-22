# UMD Video

UMD Video is a format very similar to DVD, but it has a different menu system, which as far as I know has not been reverse engineered.

On the real hardware, the player app for these is built into the PSP firmware, and since PPSSPP is a HLE emulator, we don't run the firmware so we would have to write our own player in order to run these. This would involve reverse engineering an unknown scripting language.

This is a lot of work for something very niche - UMD Video is pretty low resolution and not very interesting over other sources of the same videos, and there are few if any exclusives.

If you really want to play UMD video, use a real PSP.

However, from PPSSPP 1.21 (and current [development builds](/devbuilds)), PPSSPP can run the PSP's own firmware, including its UMD Video player. [Run the XMB](/docs/getting-started/how-to-run-the-vsh), use Insert UMD on the pause screen to put the UMD Video image in the drive, and play it from the Video column.

For more information about the UMD format variants, see [Universal Media Disc](https://en.wikipedia.org/wiki/Universal_Media_Disc) on Wikipedia.