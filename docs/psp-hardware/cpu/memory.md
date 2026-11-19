---
position: 5
---
# Memory

The CPU reaches three kinds of memory over the bus: main RAM (32 MB on the PSP-1000, 64 MB on later models), 2 MB of VRAM, and the 16 KB scratchpad. All three can be read through the [caches](/docs/psp-hardware/cpu/caches) or uncached.

## Costs per region

Cycles at 222 MHz, with nothing else using the bus:

| | RAM | VRAM | scratchpad |
|---|---|---|---|
| D-cache miss, extra cycles | ~55 consecutive lines, ~70 scattered | ~40 | ~33 |
| uncached load | ~40 | ~34 | ~13 |
| uncached store, sustained | ~6.4 | ~8.4 | ~8.3 |
| running uncached code, per instruction | ~40 | ~32 | ~14.5 |
| dirty line written back on eviction, extra | ~48 | | |

A cache hit costs nothing beyond the normal [load latency](/docs/psp-hardware/cpu/pipeline#latencies), wherever the line came from.

- **Every miss and every uncached load stalls the CPU until it's done**, whether the value is needed yet or not. Independent instructions after a load don't hide it, and a cache hit right after a miss waits for the miss. Two misses in a row take twice as long as one: there's only ever one access in flight.
- **There's no early restart.** A miss costs the same whichever word of the line was wanted, and another access to the same line right after the miss waits about 9 more cycles for the rest of the line to arrive.
- **Uncached accesses go one word at a time.** `lb`, `lh` and `lw` all cost the same, four uncached loads from one line cost four times as much as one, an uncached `lv.q` costs four uncached loads (about 155 cycles) and an uncached `sv.q` four stores.
- **Uncached code runs one instruction per memory access.** There's no prefetch.
- **Streaming through cached RAM** costs about 9.4 cycles per word with one load per word: the line's miss spread over 16 words, plus the loop. Cached stores stream the same way, since every store miss reads the line first. Storing to every 64th byte costs 107 cycles per store: a miss plus the write-back of the dirty line it evicts.

### DRAM rows

**Main RAM has rows of about 1 KB.** Uncached loads that alternate between two addresses less than 1 KB apart cost about 41 cycles each; 1 KB or more apart, about 57. Misses behave the same way, ~58 → ~72. This is why a D-cache miss is cheaper when consecutive lines miss than when they're scattered.

VRAM has no such effect: uncached loads cost 31-36 cycles at every stride.

### The write buffer

Uncached stores go through a write buffer:

- It takes about four stores at one cycle each. After that, each store waits for the buffer to drain.
- It drains at about 6 cycles per word to RAM and 8 to VRAM or the scratchpad.
- It doesn't combine stores. Eight stores to the same word, to consecutive words or to scattered words all cost the same.
- **A load waits for the buffer to drain first.** Each pending store adds about 6 cycles to the uncached load that follows it.

So a short burst of uncached stores followed by computation is nearly free, a long one costs about 6 cycles per word, and alternating uncached stores and loads is the slowest of all.

## The scratchpad

The scratchpad is a small separate block of fast memory on the bus, at `0x00010000`. It isn't part of a cache, like the PS1's scratchpad is. It works just like any other memory:

- `0x00010000` reaches it through the D-cache. Cached scratchpad accesses miss, write back and get evicted exactly like cached RAM, only with a cheaper miss.
- `0x40010000` is the uncached view. As with RAM, the two views aren't coherent: a store through one isn't seen through the other until the cache line is written back or invalidated.
- Code runs from it at full speed through the I-cache, and at about 14.5 cycles per instruction uncached.
- Its uncached accesses go over the bus like RAM's (they show up in the profiler's bus counter), and it shares the main RAM's path: when the GE textures from RAM, uncached scratchpad loads go from 13 to 54 cycles. See [the GE and the bus](/docs/psp-hardware/cpu/bus#the-ge-and-the-bus).

Uncached, it's about three times as fast as RAM, which makes it a good place for small tables that would otherwise be read uncached. Through the cache the difference shrinks to a cheaper miss.

## Copying memory

| method | rate |
|---|---|
| `memcpy` through the cache, RAM to RAM, including the write-back | ~66 MB/s (0.30 bytes per cycle) |
| the same to VRAM | ~85 MB/s |
| uncached `memcpy` | ~0.085 bytes per cycle |
| `sceDmacMemcpy`, RAM to RAM | ~126 MB/s, plus ~36 µs per call |

With the data already in the cache, copying 64 bytes takes 62 cycles with `lw`/`sw` and 30 with `lv.q`/`sv.q` (38 if the loads and stores alternate). DMA is twice as fast as the CPU for large copies, but its call overhead makes the CPU faster below about 16 KB. See [DMA](/docs/psp-hardware/cpu/bus#dma).

## Clock speed

The PSP's CPU normally runs at 222 or 333 MHz, with the bus at half that. **From 222 to 333 MHz, memory costs stay the same in CPU cycles**, within about 3%:

| | 222 / 111 MHz | 333 / 166 MHz |
|---|---|---|
| RAM uncached load | 42 | 41 |
| RAM miss | 59 | 57 |
| VRAM uncached load | 36 | 35.5 |

So the memory gets faster along with the CPU, and code that waits on memory doesn't get any faster in cycles: at 333 MHz everything simply takes 2/3 of the time. Running the CPU at 222 MHz from a 333 MHz PLL, with the bus at 166, costs exactly the same cycles as 222/111.

At the low settings (166 MHz and 100 MHz, with the PLL around 190), VRAM and scratchpad accesses cost more CPU cycles (a VRAM uncached load takes 41 at 166 MHz and 70 at 100), while RAM costs stay about the same. The PLL only runs at some frequencies: asking for 300/300/150 got a PLL of about 333.
