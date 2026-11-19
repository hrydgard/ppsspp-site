---
position: 6
---
# DMA, the GE and the kernel

The CPU shares main RAM and VRAM with the GE, the DMA controller and the Media Engine. When they're busy, the CPU's memory accesses get slower, and the CPU slows them down in return. This page covers how much, along with the cost of interrupts and of calling into the kernel.

## DMA

`sceDmacMemcpy` copies with the DMA controller and puts the calling thread to sleep until the copy is done. It bypasses the CPU caches, so the source has to be written back first (see [Caches](/docs/psp-hardware/cpu/caches#coherency-with-the-ge-dma-and-new-code)).

| route | rate for large copies |
|---|---|
| RAM → RAM | ~126 MB/s (0.57 bytes per CPU cycle) |
| RAM → VRAM | ~222 MB/s |
| VRAM → RAM | ~308 MB/s |
| VRAM → VRAM | ~240 MB/s |

Each call costs about 8,000 cycles (36 µs) on top. Time per copy, in CPU cycles of wall time:

| size | DMA RAM → RAM | CPU `memcpy` through the cache, with write-back | CPU uncached |
|---|---|---|---|
| 256 bytes | 8,213 | 463 | 3,071 |
| 4 KB | 15,900 | 5,619 | 48,215 |
| 16 KB | 36,463 | 57,525 | 192,807 |
| 64 KB | 119,214 | 220,779 | 770,673 |
| 1 MB | 1,844,043 | 3,503,826 | |

Copies of up to a few KB whose source is already in the cache are fastest on the CPU. DMA wins from about 16 KB, and does twice as well for large copies. An uncached CPU copy is never worth it.

### DMA and the CPU slow each other down

While a DMA copy runs, CPU accesses to memory wait for it:

| CPU uncached load | idle | during RAM → RAM DMA | during RAM → VRAM DMA |
|---|---|---|---|
| from RAM | ~42 | ~142 | ~127 |
| from VRAM | ~36 | ~91 | ~81 |

And the other way around: a thread doing uncached loads from RAM while the DMA runs makes 8 MB of RAM → RAM DMA take 2.6 times as long (67 ms → 174 ms), and 4 MB of RAM → VRAM 4 times as long (19 ms → 76 ms).

## The GE and the bus

How long the GE takes for a full-screen (480×272) 32-bit sprite:

| | time per sprite | |
|---|---|---|
| filled | ~0.61 ms (136,000 CPU cycles) | about one pixel per CPU cycle |
| filled with alpha blending | ~0.62 ms | blending is free |
| textured from a 256×256 32-bit texture in VRAM, stretched, bilinear | ~6 ms | 10 times as long |
| the same texture in main RAM | ~41 ms | 67 times as long |

A stretched 32-bit texture keeps missing in the GE's small [texture cache](/docs/psp-hardware/gpu/texture-cache), and from RAM each refill goes over the main bus. Smaller textures and 16-bit or CLUT formats miss less.

While the GE draws, the CPU's accesses slow down (cycles, idle → GE busy):

| CPU access | GE filling | GE texturing from VRAM | GE texturing from RAM |
|---|---|---|---|
| RAM uncached load | 42 → 43 | 43 | 42 → **105** |
| RAM cache miss (scattered) | 74 → 76 | 74 | 74 → **123** |
| VRAM uncached load | 36 → **68** | **55** | **83** |
| scratchpad uncached load | 13.5 → 13.5 | 13.5 | 13.5 → **54** |
| RAM uncached store | 6.4 → 6.5 | 6.4 | 6.4 → **59** |
| VRAM uncached store | 8.2 → **31** | **31** | **51** |

The GE is slowed down in turn: CPU traffic to VRAM makes fills take 2.1 to 2.7 times as long (and texturing from VRAM 2 to 2.4 times), and CPU traffic to RAM makes texturing from RAM take 1.4 to 2.2 times as long. CPU traffic to RAM doesn't slow down fills at all.

So **RAM and the scratchpad are on one path and VRAM is on another**. Anything that reads main RAM heavily (the CPU missing in its cache, DMA, the GE fetching textures from RAM, the Media Engine decoding video) slows everything else that uses RAM. This is the likely cause of games whose video playback drops to 30 fps when the GE is busy: the decoder, the CPU and the GE all compete for RAM.

## Interrupts

Measured with a loop that reads the CPU's cycle counter every 24 cycles and records every gap, while handlers stamp the counter on entry and exit. So each figure here is accurate to within about 24 cycles. A PSP running PSPLink, at 222 MHz:

| | cycles |
|---|---|
| interrupts on an idle system, nothing of our own registered | two per frame, ~10,000 each: 0.55% of the CPU's time |
| a whole VBlank interrupt, with a minimal VBlank sub-interrupt handler of our own | ~12,500 (56 µs) |
| from the VBlank interrupt to our sub-interrupt handler | ~4,600 (4,456 at least) |
| from our VBlank handler's return to the interrupted code | ~7,600 |
| a whole alarm (`sceKernelSetAlarm`) interrupt, minimal handler | ~1,500 (6.7 µs), ~3,400 the first time |
| from the system timer interrupt to the alarm handler | ~770-930 |
| from the alarm handler's return to the interrupted code | ~620 |

- **The VBlank interrupt is expensive** because the kernel runs every registered sub-interrupt handler (the display and controller drivers among them), before and after ours. Anything that registers more, like a plugin, makes it longer.
- **Interrupts leave the caches colder.** Each of the idle system's interrupts cost about 88 I-cache misses and 10 D-cache misses, about a third of its time. An alarm handler that runs often stays mostly in the cache: about 7 more I-cache misses each.
- These are kernel paths, so they depend on the firmware version and what's loaded.

## Syscalls and threads

From user mode:

| | cycles |
|---|---|
| a simple syscall, like `sceKernelGetSystemTimeLow` or `sceKernelGetThreadId` | ~176 |
| `sceKernelCpuSuspendIntr` + `sceKernelCpuResumeIntr` | ~34 |
| `sceKernelRotateThreadReadyQueue` with no other thread ready | ~268 |
| `sceKernelDcacheWritebackRange` on one line | ~157 |
| `sceKernelDelayThread(0)` | ~22,000 (~100 µs) |
| switching to another thread and back through semaphores | ~6,300 (~3,100 per switch) |

`sceKernelCpuSuspendIntr` and `sceKernelCpuResumeIntr` cost far less than one syscall, so they can't be going through one. `sceKernelDelayThread(0)` doesn't return right away: it still takes about 100 µs.
