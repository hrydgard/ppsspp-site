---
position: 4
---
# Caches

The Allegrex has an instruction cache and a data cache, both **16 KB, 2-way set associative (two ways of 8 KB), with 64-byte lines and LRU replacement**. Each has 128 sets, picked by address bits 6 to 12, so addresses 8 KB apart share a set.

Each memory region has a cached and an uncached view. Setting bit 30 (`0x40000000`) of an address bypasses the caches: `0x08800000` is cached main RAM and `0x48800000` is the same RAM uncached. The same goes for VRAM (`0x04000000` / `0x44000000`) and the scratchpad (`0x00010000` / `0x40010000`).

## How they behave

- **Exactly LRU.** Reading a working set of 16 KB in a loop never misses. At 17 KB, 0.17 loads in each 64-byte line miss, at 18 KB 0.33 and at 20 KB 0.56, which is what LRU gives. Three lines 8 KB apart, used in turn, evict each other every time. The I-cache behaves the same way: three blocks of code 8 KB apart miss on every call.
- **The D-cache is write-back.** A cached store stays in the cache until the line is evicted or written back with a `cache` instruction. Through the uncached view, the memory still holds the old value until then.
- **The D-cache is write-allocate.** A store that misses reads the whole line in first and then changes it in the cache, so it costs about as much as a load miss.
- **The two views aren't kept coherent.** An uncached store to a line that's in the D-cache leaves the cached copy stale, and a later cached read returns the old value. Invalidating a dirty line without writing it back throws its stores away.
- **Instruction fetch reads memory, not the D-cache.** Code that was just written through a cached pointer is still in the D-cache, and the I-cache may still hold the old code at that address.

### Coherency with the GE, DMA and new code

The GE, the DMA controller and the Media Engine read and write memory directly, never the CPU's caches. So:

- **Before the GE or DMA reads something the CPU wrote through the cache**, write the lines back (`sceKernelDcacheWritebackRange` or `...All`), or the hardware reads the old contents.
- **Before the CPU reads something the GE or DMA wrote**, invalidate the lines, or reads can return stale cached data. A dirty line in that range would also overwrite the new data whenever it's evicted, so write back and invalidate before the transfer starts.
- **Before running code written through a cached pointer**, it needs both a D-cache writeback and an I-cache invalidate. Either one alone runs the old code. Code written through an uncached pointer needs only the I-cache invalidate.

## The `cache` instruction

| op | what it does | cycles |
|---|---|---|
| `0x04` | I-cache index invalidate. Doesn't touch the D-cache | 13 |
| `0x06` | I-cache index unlock | |
| `0x08` | I-cache hit invalidate. Doesn't touch the D-cache | 11 |
| `0x0A` | I-cache fill: loads the line into the I-cache | ~64 |
| `0x0B` | I-cache fill and lock | |
| `0x14` | D-cache index writeback-invalidate | 6, ~21 if dirty |
| `0x16` | D-cache index unlock | |
| `0x18` | D-cache create dirty exclusive | ~6 |
| `0x19` | D-cache hit invalidate | ~6 |
| `0x1A` | D-cache hit writeback | 6, ~19 if dirty |
| `0x1B` | D-cache hit writeback-invalidate | 6, ~19 if dirty |
| `0x1C` | D-cache create dirty exclusive and lock | |
| `0x1E` | D-cache fill (prefetch) | ~60, ~8 if the line is present |
| `0x1F` | D-cache fill and lock | |

- **`0x14`, the index operation, works on a set, not a line.** It writes back and invalidates the set's least recently used line, and a second one at the same index takes the other. Doing it twice for every set clears the whole cache, which is what the kernel's `...All` functions do.
- **`0x18` doesn't zero the line.** It claims the line for an address without reading memory, but the line keeps whatever that cache entry held before. Writing the whole line afterwards is the only safe use, and it saves the read that a store miss would do.
- **`0x19` discards dirty data.** It's how stores get lost when invalidating a range that wasn't written back first.
- **`0x1E` stalls like a miss.** It isn't a background prefetch: the CPU waits for the line, so it's only useful for filling the cache ahead of time.

### Locking

A locked line stays in the cache when the rest of its set is replaced. `0x0B`, `0x1F` and `0x1C` fill (or create) and lock a line, and `0x06` and `0x16` unlock one by index. A locked I-cache line survived a loop that thrashed its set with three other blocks, and a locked D-cache line likewise.

Locks are easy to lose. A locked D-cache line is still removed by `0x14`, `0x19` and `sceKernelDcacheWritebackInvalidateAll`, so any kernel call or library that cleans the whole cache unlocks everything.

## The kernel's maintenance calls

| function | cycles |
|---|---|
| `sceKernelDcacheWritebackAll` | ~3,670 with nothing dirty, ~8,050 with the whole cache dirty |
| `sceKernelDcacheWritebackInvalidateAll` | ~990 with nothing dirty, ~10,340 with the whole cache dirty |
| `sceKernelDcache*Range` | ~170-250, plus ~6 per line and ~14 more per dirty line. From 64 KB on the kernel does the whole cache instead (~3,600) |
| `sceKernelIcacheInvalidateAll` | ~4,800 |
| `sceKernelIcacheInvalidateRange` | 248 for 64 bytes, 616 for 1 KB, 936 for 4 KB, ~4,200 from 16 KB on |

`sceKernelDcacheWritebackAll` is slower than `WritebackInvalidateAll` when little is dirty. After `sceKernelIcacheInvalidateAll`, all the code that runs next misses: refilling 8 KB of code costs 54 cycles per line on top.

From user mode each of these calls is also a syscall, at ~176 cycles: `sceKernelDcacheWritebackRange` on one line costs about 157 from user mode. Calling it per small object adds up fast; one call over the whole range, or `...All` for large amounts, is cheaper.

## Miss costs

How long a miss takes depends on the memory behind it, and on what else is using that memory. See [Memory](/docs/psp-hardware/cpu/memory#costs-per-region) for the figures. In short: a D-cache miss costs about 55 cycles from main RAM when consecutive lines miss and about 70 when they're scattered, 40 from VRAM and 33 from the scratchpad. A miss that evicts a dirty line costs about 48 more. An I-cache miss costs about 55, so straight-line code that misses on every line runs at about 4.5 cycles per instruction, and a 64-byte loop body that's already in the cache runs at full speed.
