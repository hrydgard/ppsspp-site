# Known differences from the real PSP

PPSSPP tries to replicate the behavior of a real PSP, with regards to running games, as closely as possible, but there are known differences. Some of them are described below.

## Language in sceUtility dialogs

As mentioned [here](https://github.com/hrydgard/ppsspp/issues/16934), we follow the user's set language, while the real PSP follows what the game sets.

## Reverb effect far from accurate

Our audio reverb implementation is basically modeled on old PSX emulators - we know that the PSP's reverb effect is different but it's very hard to know exactly in what way.

## Fonts can be a little off

The PSP firmware includes a set of fonts, that a small percentage of games use for some of their user interface. Unfortunately, these are of course copyrighted, so cannot be supplied with PPSSPP. It's possible to dump fonts on your own and have PPSSPP use them, but PPSSPP also has a set of built-in fonts that are very close in metric and work well for most games, so it's not really something to worry about, though some text may just look a little off compared to the real thing.

## CPU caches are not emulated

The MIPS CPU in the PSP has separate data and instruction caches. The data cache can be easily bypassed by game code by doing `| 0x40000000` to any pointer. This is mainly useful for writing data all the way out to RAM immediately so that the GPU can still read it. There's also a full set of cache invalidation instructions and soon.

PPSSPP just pretends the caches are invisible, which they should be if games use it correct. Which they don't always do - it's believed that a graphical glitch with fires in the Prince of Persia games is caused by some kind of cache misuse.

## VFPU math functions are not fully bit-accurate

The VFPU is a vector co-processor that is controlled with the same instruction stream as the main CPU. It has a lot of very complex instructions like cosines, square roots and so forth, and we haven't figured out the exact formulas behind these so instead we use approximations that are quick to compute. Additionally, dot products use a cool mantissa-aligning optimization, causing quite different results from a naive series of multiplications and adds. This means however there are sometimes some accuracy issues, especially in replays (such as the Ridge Racer built-in replays) that can go out of sync, and various physical and graphical problems such as shaking feet in Tekken 6 (since fixed).

This has been improved greatly recently by fp64's work, but some functions are still a bit off, and replays in Ridge Racer still don't function correctly.

## Media Engine (mpeg, atrac decoder) is rough

The Media Engine APIs are not very well documented and our implementation of them involves a lot of guesswork, and a lot of ugly tricks wrangling FFMPEG into doing our bidding. This does sometimes lead to problems, such as the quick-skipping audio tracks in Flatout, see issue [#6663]. An area that needs more work, but also a very difficult one.

## Color depth is not emulated accurately

The PSP often renders at 16-bit color depth (either RGB555 or RGB565), but PPSSPP always renders with 32-bit color (full RGBA8888).

## And so much more...

The list doesn't end here, of course. Will be expanded as I think of things.
