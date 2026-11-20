---
position: 2
---
# CPU timing

How long things take on the PSP's main CPU, measured on a real PSP. Emulators tend to run PSP code far faster than the hardware does, even at the same nominal clock, because the real CPU spends much of its time waiting: on memory, on taken branches, on loads and on its floating point units. These pages collect what those waits are, in enough detail to build a timing model from.

All figures are CPU cycles at the default 222 MHz CPU / 111 MHz bus clock unless stated otherwise. At 333 MHz the memory costs stay about the same in CPU cycles (see [Memory](/docs/psp-hardware/cpu/memory#clock-speed)).

| page | what's on it |
|---|---|
| [Pipeline and instructions](/docs/psp-hardware/cpu/pipeline) | issue, branches, every latency between units, multiply and divide, the FPU and the VFPU |
| [Caches](/docs/psp-hardware/cpu/caches) | geometry and policies, coherency with the GE and for new code, the `cache` instruction, line locking, the kernel's maintenance calls |
| [Memory](/docs/psp-hardware/cpu/memory) | RAM, VRAM and the scratchpad: miss and uncached costs, DRAM rows, the write buffer, copying, clock speeds |
| [DMA, the GE and the kernel](/docs/psp-hardware/cpu/bus) | DMA rates, how the GE and DMA slow the CPU down and the other way around, interrupts, syscalls and thread switches |
| [How it was measured](/docs/psp-hardware/cpu/measuring) | the CPU's hardware profiler, the test setup, and what to avoid |

## The short version

The Allegrex is a simple in-order CPU that issues at most one instruction per cycle. When nothing gets in the way, that's what it does: integer ALU operations take one cycle each, including the Allegrex's own additions like `ext`, `ins`, `min`, `max` and `clz`. Things get in the way often:

| cost | cycles |
|---|---|
| taken branch | +2 (jumps +1) |
| using a loaded value in the next instruction | +2 |
| a store right after a store, or a load right after a store | +about 2 |
| dependent FPU or VFPU operation | 5, or 7 for `vdot` and the special functions |
| moving a value between the GPRs, FPU and VFPU | 3 to 8 each way |
| integer divide | 19 to 35, in the background |
| `div.s` / `sqrt.s` | ~30, blocking the FPU |
| D-cache or I-cache miss | ~55 to 70 from RAM, ~40 from VRAM |
| uncached load | ~40 from RAM, ~34 from VRAM, ~13 from the scratchpad |
| syscall | ~176 |
| interrupt, from the interrupt to a handler and back | ~1,500 (an alarm) to ~13,000 (VBlank) |
| switching threads | ~2,900 |

There's no branch prediction, no out-of-order execution, and a cache miss stalls everything until the line has arrived. Memory costs go up, sometimes by a factor of two or three, while DMA or the GE is busy with the same memory.

## A timing model

The figures predict compiled code well. Counting instructions and adding the costs above:

| function (gcc -O2) | measured | predicted |
|---|---|---|
| bitwise CRC32 over 4 KB | 315,370 | 315,392 |
| float dot product over 4,096 elements | 94,864 | ~94,600 |
| walking a linked list of 4,096 scattered 64-byte nodes | 328,806 | ~355,000 |

The CRC32 is 77 cycles per byte: the inner loop is 8 iterations of 7 instructions and a taken branch, the outer loop adds 5 instructions and another taken branch, and the byte load sits in that branch's delay slot so its latency is hidden. The list walk is predicted about 8% too slow because the model charges two loads to the same missing line more than the hardware does.

A model that gets most of the way there:

1. Every instruction costs 1 cycle.
2. Taken branches cost 2 more, jumps 1 more.
3. Track when each register's value is ready, using the latencies on the [pipeline page](/docs/psp-hardware/cpu/pipeline#latencies), and stall an instruction until its sources are ready. A taken branch's two lost cycles count towards that wait.
4. A store occupies the memory unit for 3 cycles. A load or store that wants it sooner waits.
5. Model the two caches (16 KB, 2-way, 64-byte lines, LRU). A miss stalls for the [region's miss cost](/docs/psp-hardware/cpu/memory#costs-per-region), plus ~48 for writing back a dirty line it evicts. An uncached access stalls for the region's uncached cost.
6. Integer divides run alongside other work for 19-35 cycles. `div.s`, `sqrt.s` and `vdiv.s` block their own unit.

Steps 1 to 3 account for most of the cycles in code that stays in the cache, and step 5 dominates everything else. Even a rough version of step 5 makes a large difference for code that walks through large data.

## What isn't covered yet

- How much the Media Engine slows the main CPU down while it decodes video or audio. It uses main RAM, so it's likely to behave like DMA, but it hasn't been measured.
- The CPU's own exception entry, apart from the kernel's handling around it.
