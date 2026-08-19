# VSH, also know as the XMB

VSH is the "Visual Shell", or the [Xross Media Bar](https://en.wikipedia.org/wiki/XrossMediaBar) user interface of the PSP, which originated as a concept on the PSX (the modified PS2 with a DVR, not the original Playstation), and was later also used on the PlayStation 3.

This article is about some technical details of the VSH.

If you just want to know how to run it in PPSSPP, see [here](/docs/getting-started/how-to-run-the-vsh).

AI Disclaimer: Claude was used for some of the research and implementation. Not the writing of this article though - it's 100% human, though using some of the AI's research.

## What needed to be added for it to boot

PPSSPP is a HLE (high-level emulation) emulator, that normally just fakes the entire PSP OS kernel and many libraries with its own implementations. It does not normally run kernel-level code that accesses the memory-mapped IO (from here on, MMIO).

## MMIO

When trying to run the VSH, it does load and rely on some driver-level modules that do MMIO, so I had to implement a small amount of LLE, or low-level emulation, of it.

## Various HLE kernel modules and functions

- sceImpose (GetParam/SetParam/Changes/SetStatus)
- scePower_driver (just one function)
- sceHprm_driver (just one function)

## Disc checking

The VSH uses some sceIoDevctl calls to figure out the disc type and region, and if we return bogus values here (or nothing at all), it just throws up a giant red error screen.

## KIRK emulation

Our KIRK emulation was missing one encryption type, so this had to be added (type 9). This was used by the sceResmgr module, which uses this to decrypt resources needed for the UI - starting with the index in `index_02g.dat`.

## Power management

The VSH called some power management functions that we hadn't seen before. We just pretend the battery is always fully charged, with no suspend state required, of course.

## Traps on the way

For a while I suspected we needed `flash1:/registry/init.dat` since the VSH tries to open it early during startup, but that was wrong - if such a file exists, it's used to overwrite the PSP's configuration registry, and the VSH won't launch. So this file must NOT exist.

There are some drivers that currently break because of unimplemented kernel functions, that if I implement those, they go deeper into driver code and fail completely, freezing emulation. So there are effectively a number of load-bearing bugs (or rather, omissions) in the current implementation. Comments have been added to the source code to indicate these. These are ThreadManForKernel's vpl and fpl functions, and also InterruptManagerForKernel's interrupt registration functions.

