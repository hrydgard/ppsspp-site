# Overview

The PSP's main CPU is a 32-bit MIPS architecture derivative, usually running at 222 MHz or 333 MHz. Initially it always ran at 222, but later PSP firmware versions opened up the faster speed, at the cost of higher battery power consumption.

The PSP also has a second, stripped-down CPU called the Media Engine. It's basically the same as an Allegrex but with the VFPU removed (see below), and a different function accelerator added. It performs high-level tasks like decoding video or audio formats, and cannot (officially) be programmed directly.

## ISA features

It's got the standard MIPS integer instruction set, with a few handy additions (additional multiply-add instructions, some bitfield manipulation and byteswapping instructions).

The FPU that supports single-precision only, but that's perfectly fine for games.

It also has the VFPU, a very unusual vector processor. Unlike the VU units on the PS2, it shares a single instruction fetcher with the main CPU, though it can execute in parallel with the normal FPU and integer units. Synchronization is mostly automatic.

## Memory management

It does not have an MMU but still allows for limited memory production, enough to separate the kernel and user memory space from each other. There's also some rather unusually explicit control of the cache, which is useful when interacting with the GPU - the GPU cannot read out of the CPU cache, so you must manually make sure that all data you feed it has been "written back" to RAM/VRAM.

To be continued...