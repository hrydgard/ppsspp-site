---
position: 2
---
# CPU timing: pipeline, caches and memory

How long things take on the PSP's main CPU, measured on a real PSP. Emulators tend to run PSP code far faster than the hardware does, even at the same nominal clock, because the real CPU spends much of its time waiting: on memory, on taken branches, on loads and on its floating point units. This page collects what those waits are.

All the figures are CPU cycles at the default 222 MHz CPU / 111 MHz bus clock unless stated otherwise. They were measured with small test programs running in kernel mode, which read the CPU's built-in hardware profiler (cycle, cache miss, writeback and stall counters) around each test with interrupts off.

## The pipeline

The Allegrex issues one instruction per cycle, in order.

- **Integer ALU operations** take one cycle, and their results can be used by the next instruction. That includes the Allegrex's own additions: `ext`, `ins`, `min`, `max`, `seb`, `seh`, `wsbh`, `wsbw`, `bitrev`, `clz`, `clo`, `rotr`, `movz` and `movn`.
- **Taken branches cost 2 extra cycles**, forward or backward, branch-likely or not. Not-taken branches are free (an annulled branch-likely delay slot still costs its cycle). **There's no dynamic branch prediction**: a branch following a random direction pattern costs exactly the same as one following a regular pattern with the same number of taken branches.
- **Jumps cost less than branches:** `j`, `jal`, `jr` and `jalr` cost 1 extra cycle. A call to a function that immediately returns takes 6 cycles, delay slots included. Where a branch lands within a cache line makes no difference.
- **A taken branch's two lost cycles hide latency.** A load in the delay slot, used at the branch target, doesn't stall at all.
- **Loads** have a latency of 3: an instruction that uses a loaded value right away stalls for 2 cycles, and two instructions in between hide it.
- **Stores keep the memory unit busy for about 3 cycles.** Back-to-back stores go out about every 2.9 cycles, and a load right after a store waits about 2 cycles, whatever address it reads. Two other instructions between stores hide it.
- **Multiply:** `mult` can start every cycle; `mflo`/`mfhi` can read the product 3 cycles later. Chained `madd`/`msub` about 4 each.
- **Divide:** `div`/`divu` take about 35 cycles with a large dividend and about 19 with a small one, so the time depends on the data. They run in the background: other integer work before the `mflo` is free. A new `div` or `mult` cancels a divide that's still running.

### FPU

`add.s`, `sub.s`, `mul.s`, `abs.s`, `neg.s`, `mov.s` and the conversions have a latency of 5 cycles and can start every cycle. `div.s` (about 31 cycles) and `sqrt.s` (about 29) aren't pipelined. Integer work overlaps them, but every further FPU instruction waits until they're done, whether it needs the result or not. A compare followed by `bc1t`/`bc1f` costs nothing extra. A value loaded with `lwc1` or moved in with `mtc1` can be used 4 cycles later, and an FPU result can be moved out with `mfc1` 6 cycles after the operation.

### VFPU

The vector width (`.s`, `.p`, `.t`, `.q`) doesn't change the timing.

| operations | latency | a new one every |
|---|---|---|
| `vadd`, `vsub`, `vmul`, `vmin`, `vmax`, `vscl`, `vsgn`, `vi2f`, `vf2in`, `vf2iz` | 5 | 1 |
| `vmov` | 3 | 1 |
| `vdot`, `vhdp`, `vavg` | 7 | 1 |
| `vrcp`, `vrsq`, `vsqrt`, `vsin`, `vcos`, `vexp2`, `vlog2`, `vasin` | 7 | 1 |
| `vdiv.s` | ~17 | ~13 |
| `vcrsp.t` | ~9 | ~3 |
| `vqmul.q`, `vtfm4.q` | ~10 | ~4 |
| `vmmul.q` | ~16 | ~16 |
| `vmmov.q`, `vmscl.q` | ~4 | ~4 |
| `lv.q` | ~5 to use | ~3.5 |
| `sv.q` | | ~7 |

The special functions (`vsin`, `vrsq` and friends) are fully pipelined, just like additions. A prefix instruction (`vpfxs` and friends) costs one cycle of issue and adds no latency. Values loaded with `lv.s`/`lv.q` can be used 5 cycles later. `vdiv.s` holds up the VFPU while it runs, but not the FPU, and an FPU divide doesn't hold up the VFPU.

### Moving values between units

Getting a value from one unit to another is slow. Round trips, in cycles:

| path | cycles |
|---|---|
| FPU → GPR → FPU (`mfc1`, `addu`, `mtc1`) | ~7 |
| VFPU → GPR → VFPU (`mfv`, `addu`, `mtv`) | ~12 |
| `add.s` → `c.lt.s` → `bc1t` | 8 |
| `vadd.s` → `vcmp` → `bvt` | 15 |
| FPU result through memory (`swc1`, `lw`, `mtc1`) | ~15.5 |

## Caches

The I-cache and the D-cache are both **16 KB, 2-way set associative (two ways of 8 KB), with 64-byte lines and LRU replacement**. Three lines 8 KB apart that are used in turn evict each other every time.

The D-cache is **write-back and write-allocate**:

- A cached store isn't visible through the uncached view of the same memory until the line is written back.
- A store miss reads the line in first, so it costs about as much as a load miss.
- An uncached store to a line that's in the cache leaves the cached copy stale.
- Invalidating a dirty line throws the store away.

**Instruction fetch reads memory, not the D-cache.** Code written through a cached pointer needs both a D-cache writeback and an I-cache invalidate before it can run (either one alone runs the old code). Code written through an uncached pointer needs only the I-cache invalidate.

### Misses and memory latency

**Every miss stalls the CPU until it's done**, whether or not the value is needed yet. Independent instructions after a load don't hide its miss, a cache hit right after a miss waits for it too, and two misses in a row take twice as long as one.

| | RAM | VRAM | scratchpad |
|---|---|---|---|
| D-cache miss, extra cycles | ~55 consecutive lines, ~70 scattered | ~40 | ~33 |
| uncached load | ~40 | ~34 | ~13 |
| uncached store | ~6.4 | ~8.4 | ~8.3 |
| running uncached code, per instruction | ~40 | ~32 | ~14.5 |

- **Main RAM has rows of about 1 KB.** Accesses that stay within 1 KB of the previous one are about 16 cycles cheaper than ones that jump further. VRAM has no such effect.
- **There's no early restart.** A miss costs the same whichever word of the line is wanted, and another access to the same line right after the miss waits about 9 more cycles for the rest of it.
- **Uncached accesses are one word at a time.** `lb`, `lh` and `lw` all cost the same, and an uncached `lv.q` costs four uncached loads.
- **A dirty line that gets evicted** adds about 48 cycles.
- **An I-cache miss** costs about 55 cycles, so straight-line code that misses on every line runs at about 4.5 cycles per instruction.
- **Uncached stores go through a write buffer** that absorbs about four of them at full speed and drains at roughly 6 cycles per word to RAM. It doesn't combine stores. A load waits for the buffer to drain first, so each pending store adds about 6 cycles to a following uncached load.
- **Copying data that's in the cache** with `lv.q`/`sv.q` is twice as fast as with `lw`/`sw`: 64 bytes in about 30 cycles instead of 62.

### The scratchpad

The 16 KB scratchpad is simply a small block of faster memory on the bus, not a repurposed cache. `0x00010000` reaches it through the D-cache, like any cached memory, and `0x40010000` is the uncached view. The two behave exactly like cached and uncached RAM, and code runs from it at full speed through the I-cache. It's faster than RAM and VRAM, especially uncached, but it shares the bus with main RAM.

### `cache` instructions

| op | what it does | cycles |
|---|---|---|
| `0x04`, `0x08` | I-cache index / hit invalidate | 11-13 |
| `0x0A` | I-cache fill | ~64 |
| `0x14` | D-cache index writeback-invalidate | 6, ~21 if dirty |
| `0x18` | D-cache create dirty exclusive: allocates the line without reading memory. It keeps whatever that cache entry held before, which is stale data, not zeros | ~6 |
| `0x19` | D-cache hit invalidate (discards dirty data) | ~6 |
| `0x1A` | D-cache hit writeback (the line stays) | 6, ~19 if dirty |
| `0x1B` | D-cache hit writeback-invalidate | 6, ~19 if dirty |
| `0x1E` | D-cache fill (prefetch). Stalls like a miss | ~60, ~8 if present |
| `0x0B` / `0x1F` | I-cache / D-cache fill and lock: the line stays when the rest of its set is replaced | |
| `0x1C` | D-cache create dirty exclusive and lock | |
| `0x06` / `0x16` | I-cache / D-cache index unlock | |

The index operation `0x14` cleans the least recently used line of the set, and a second one at the same index cleans the other. A locked D-cache line is still removed by `0x14`, `0x19` and `sceKernelDcacheWritebackInvalidateAll`.

The kernel's maintenance functions cost:

| function | cycles |
|---|---|
| `sceKernelDcacheWritebackAll` | about 3,700 even with nothing dirty, about 8,000 with the whole cache dirty |
| `sceKernelDcacheWritebackInvalidateAll` | about 1,000, or 10,300 with the whole cache dirty |
| range functions | about 200, plus 6 per line and 14 per dirty line. From 64 KB on they do the whole cache |
| `sceKernelIcacheInvalidateAll` | about 4,800 |

From user mode each of these is also a syscall.

## Clock speed

From 222 to 333 MHz, memory costs stay the same in CPU cycles (within about 3%), so a cache miss takes the same number of cycles at either speed and code that waits on memory doesn't run any faster in cycles. At the low settings (166 and 100 MHz) VRAM and scratchpad accesses cost more CPU cycles.

## DMA

`sceDmacMemcpy` puts the calling thread to sleep until the copy is done. It bypasses the CPU caches.

| route | rate |
|---|---|
| RAM → RAM | ~126 MB/s |
| RAM → VRAM | ~222 MB/s |
| VRAM → RAM | ~308 MB/s |
| VRAM → VRAM | ~240 MB/s |

Each call has an overhead of about 36 µs, so a CPU copy is faster below about 16 KB. Copying through the cache, writeback included, manages about 66 MB/s from RAM to RAM.

DMA and the CPU slow each other down a lot. While a DMA runs, a CPU uncached load from RAM takes about 140 cycles instead of 40. A thread doing uncached loads meanwhile makes a RAM-to-RAM DMA take 2.6 times as long, and a RAM-to-VRAM DMA 4 times as long.

## The GE and the bus

Drawing a full-screen 32-bit sprite takes about 0.6 ms filled (about one pixel per CPU cycle, and alpha blending is free). Texturing it from a stretched 256×256 32-bit texture in VRAM takes 10 times as long, and from main RAM 67 times as long. A stretched texture keeps missing in the GE's small [texture cache](/docs/psp-hardware/gpu/texture-cache), and from RAM each miss goes over the main bus.

While the GE draws, the CPU's accesses slow down (cycles, idle → busy):

| CPU access | GE filling | GE texturing from RAM |
|---|---|---|
| RAM uncached load | 42 → 43 | 42 → 105 |
| RAM cache miss | 74 → 76 | 74 → 123 |
| VRAM uncached load | 36 → 68 | 36 → 83 |
| scratchpad uncached load | 13.5 → 13.5 | 13.5 → 54 |
| VRAM uncached store | 8 → 31 | 8 → 51 |

The other direction holds too: CPU traffic to VRAM makes fills take 2 to 2.7 times as long, and CPU traffic to RAM makes texturing from RAM take up to 2.2 times as long. RAM and the scratchpad are on one shared path and VRAM on another, and anything that reads main RAM heavily (the CPU, DMA, the GE fetching textures, the Media Engine decoding video) slows everything else that uses it.

## Syscalls and threads

From user mode:

| | cycles |
|---|---|
| a simple syscall, like `sceKernelGetSystemTimeLow` | ~176 |
| `sceKernelCpuSuspendIntr` + `sceKernelCpuResumeIntr` | ~34 |
| `sceKernelDelayThread(0)` | ~22,000 (~100 µs) |
| switching to another thread and back through semaphores | ~6,300 |

## What it adds up to

These figures predict compiled code well. A bitwise CRC32 over 4 KB, for example, comes to 77 cycles per byte by counting the instructions, taken branches and stalls above: 315,392 cycles. The PSP took 315,370.

A tight loop that stays in the cache and rarely branches runs at close to one instruction per cycle. Typical game code doesn't. The cycles mostly go to:

- cache misses and uncached accesses, 40 to 75 cycles each, and more while the GE or DMA is busy with RAM;
- taken branches, 2 each;
- loads used right away, 2 each;
- runs of stores;
- floating point and vector latencies, 5 to 7 per dependent operation;
- divides;
- moving values between the integer, FPU and VFPU registers.
