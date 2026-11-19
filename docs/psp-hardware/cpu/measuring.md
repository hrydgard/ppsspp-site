---
position: 7
---
# How it was measured

The figures on these pages come from small test programs run on a PSP with firmware 6.61, over PSPLink. They're kernel modules, because they read the CPU's hardware profiler, which user mode can't reach.

## The hardware profiler

The Allegrex has a set of performance counters at `0xBC400000`: twenty 32-bit registers, laid out as PSPSDK's `PspDebugProfilerRegs`. Write 0 to the first register and zero the others to reset them, and write 1 to the first to start counting.

| offset | name | what it counts |
|---|---|---|
| `0x00` | enable | write 1 to count, 0 to stop |
| `0x04` | `systemck` | clock cycles of the PLL. The same as `cpuck` at 222 and 333 MHz, 1.5 times it when the CPU runs at 222 MHz from a 333 MHz PLL |
| `0x08` | `cpuck` | CPU cycles **while the CPU is running**. It stops while the CPU sleeps waiting for an event, for example while a thread waits for a DMA copy |
| `0x0C` | `internal` | cycles stalled inside the pipeline: load-use stalls, back-to-back stores |
| `0x10` | `memory` | cycles stalled waiting for memory: misses and uncached accesses |
| `0x14` | `copz` | cycles stalled on the FPU (for example waiting for an `add.s` result) |
| `0x18` | `vfpu` | cycles stalled on the VFPU \* |
| `0x1C` | `sleep` | cycles asleep \* |
| `0x20` | `bus_access` | roughly the bus cycles spent on uncached accesses and misses, the scratchpad's included |
| `0x24` | `uncached_load` | uncached loads |
| `0x28` | `uncached_store` | uncached stores |
| `0x2C` | `cached_load` | cached loads, the scratchpad's included |
| `0x30` | `cached_store` | cached stores, the scratchpad's included |
| `0x34` | `i_miss` | I-cache line misses |
| `0x38` | `d_miss` | D-cache line misses |
| `0x3C` | `d_writeback` | dirty D-cache lines written back |
| `0x40` | `cop0_inst` | COP0 instructions \* |
| `0x44` | `fpu_inst` | FPU instructions \* |
| `0x48` | `vfpu_inst` | VFPU instructions \* |
| `0x4C` | `local_bus` | always 0 in everything tried |

The counters marked \* go by their names and weren't checked against a known count. The miss and writeback counts are exact, which makes the profiler good for checking a cache model line by line, not just for timing.

## The tests

Each measurement turns interrupts off, resets and starts the profiler, runs a loop, and stops the profiler. The counts of an empty loop are subtracted. Most tests run each loop twice, once with warm caches and once after `sceKernelDcacheWritebackInvalidateAll`.

The instruction and pipeline tests are generated: each test is a loop whose body repeats one pattern several times, such as eight dependent `add.s`, eight independent ones, or a producer, N `nop`s and a consumer for N from 0 upwards. The point where adding a `nop` starts costing a cycle is the latency.

A kernel module and the file it was loaded from both have to fit in about 236 KB of free kernel memory, which the generated tests don't. They're linked as flat binaries at a fixed address in user memory instead, read from the host by a small kernel module, and called after a D-cache writeback and an I-cache invalidate. Kernel-mode code runs fine from user memory.

The DMA tests measure wall time with `sceKernelGetSystemTimeWide`, because the thread sleeps while the copy runs and `cpuck` stops with it. The GE tests use hand-built display lists and wait for them with `sceGeListSync` in polling mode, with interrupts on and a time limit.

## Things to avoid

These wedged the PSP during testing, needing a manual restart:

- **Running VFPU code in a kernel module's main thread.** It hangs on the first VFPU instruction. Create a thread with `PSP_THREAD_ATTR_VFPU` (`0x4000`) for it.
- **Waiting for the GE with interrupts off.** The kernel finishes a display list in the GE interrupt, so with interrupts off the queue stands still at the list's `FINISH`, and a wait never returns.
- **`sceGeBreak` with a list queued.** It leaves the GE driver paused, later lists never run, and PSPLink's `reset` hangs.
- **Loading from a pointer that a test might overwrite.** An address error in a kernel module takes down the whole PSP, not just the test.
