---
position: 6
---
# The texture cache and self-texturing

The GE reads textures through a small texture cache, 8 KB in size. For nearly all rendering, the cache is invisible: textures don't change while they're drawn with. It does become visible when a game draws to a buffer while also texturing from it, when it switches textures without flushing, or when the CPU or a block transfer changes a texture without telling the GE.

Everything on this page was measured on a PSP with hand-built display lists. Each texture is filled with its own marker value, then overwritten in memory after the scenario, and read back one texel at a time, so a readout tells whether a texel came from the cache or from memory, and from which texture.

## Lines, sets and tags

- The cache holds **64 lines of 16 bytes x 8 rows**. A line covers 4 x 8 texels of a 32-bit format, 8 x 8 of a 16-bit one, 16 x 8 of CLUT8 and 32 x 8 of CLUT4, aligned to its size in texture coordinates.
- The lines are organized as **16 sets of 4 ways**, replaced least recently used first. With `col` the line's 16-byte column and `band` its 8-row band, the set index takes four bits, split by the texture format so that a set's lines repeat over a roughly square area of texels:

  | bits per texel | set index | a set's lines repeat every |
  |---|---|---|
  | 32 | `(col & 7) \| (band & 1) << 3` | 32 x 16 texels |
  | 16 | `(col & 3) \| (band & 3) << 2` | 32 x 32 |
  | 8 | `(col & 3) \| (band & 3) << 2` | 64 x 32 |
  | 4 | `(col & 1) \| (band & 7) << 1` | 64 x 64 |

- **A line is tagged by its position in the texture and its mip level only**: column, band and level. Not by the texture's address, buffer width, format or swizzling. So a texture drawn right after another one, with no flush in between, gets the first texture's cached bytes wherever their lines coincide, read in the new format.
- **Lines hold raw texture bytes**, loaded from memory when a sample first needs them.
- **Swizzling doesn't change the line shape or the sets.** A swizzled texture's 16-byte x 8-row blocks are exactly the cache lines, so loading a line is one contiguous 128-byte read instead of eight separate 16-byte ones. That is why swizzling matters for speed.
- **DXT1, DXT3 and DXT5 are cached decoded**, as 8888 texels. A DXT line covers 4 x 8 texels, sits in the 32-bit sets and shares tags with 8888 lines (an 8888 texture read after a DXT one gets the decoded colors). In the cache, a DXT1 texture takes 8 times its size in memory and DXT3/5 4 times, which goes a long way to explaining why DXT textures are slow on the PSP.
- The CLUT is separate: loading one doesn't touch the texture cache.

## TEXFLUSH and TEXSYNC

The cache isn't coherent with anything. **TEXFLUSH** (0xCB) drops every line, and nothing else does: not TEXSYNC, a display list ending or starting, CPU writes, block transfers, rendering into the texture, or changing any texture register. The SDK emits a TEXFLUSH whenever a texture is set, which is why most games never notice the cache.

So whatever was loaded stays until the next flush, across primitives and draw calls. Final Fantasy Type-0's blur ([#20104]) draws a 16x16 4444 buffer onto itself three times. The result differs in 177 pixels with and without a TEXFLUSH between the passes, and both versions are reproduced exactly with the cache keeping its lines until the flush.

**TEXSYNC** (0xCC) is about [block transfers](/docs/psp-hardware/gpu/block-transfers): a transfer runs alongside drawing, so texturing from its destination needs a TEXSYNC first, or lines can load before the transfer reaches them.

## Self-texturing

Some post-processing effects texture from the buffer they're drawing to: a blur that draws a buffer onto itself shifted by a pixel, a bloom or a glare pass. What such a draw does depends on when each texture line was loaded relative to when the pixels in it were written. Within one primitive:

- **Lines load lazily**, the first time a pixel's sample needs them, and are never refreshed during the primitive. A line loaded after some of its pixels were written shows the new values, and one loaded before shows the old ones.
- **Sprites are drawn row by row, each row left to right, top to bottom**, whatever their size, position, vertex format, blending or texture direction.
- **Triangles** are drawn row by row too. Their order depends on their shape, and is described on the [raster pipeline](/docs/psp-hardware/gpu/raster-pipeline#walking-order) page.
- **Pixels reach memory 16 bytes at a time** (4 pixels of a 32-bit framebuffer, 8 of a 16-bit one), about one pixel after the next 16-byte block starts, or four with bilinear filtering.
- **The texture fetches can run ahead of the pixel writes.** While a primitive's samples hit the cache, fetching is faster than writing, and the gap grows for about two rows before it stops growing. When a sample then misses, as at the first row of a new 8-row band, the line it loads can lack most of the row above. In a 64-pixel-wide sprite, the last 10 to 14 blocks of the row above are still unwritten at the next row's first pixel, depending on where the row starts within a 64-byte group. In narrower sprites, often the whole row is. Four pixels later, all of it has arrived. While samples keep missing, the writes keep up and only the usual last block is missing. Exactly how far ahead the fetches get depends on the row's width and alignment in a way that hasn't been worked out.

These were measured with sprites and triangles texturing from the framebuffer with their texture coordinates scaled, so that every sample is the first read of its own line. A pixel's result then tells whether its source pixel had already reached memory when it was read.

### In PPSSPP

PPSSPP's hardware backends copy the render target before such draws, so they read it as it was before the draw.

The software renderer simulates the cache across primitives: which lines are loaded, from where, and when their memory changes (`GPU/Software/TexCache.cpp`). A primitive that reads cached bytes that differ from memory is drawn from a copy of its texture with those lines laid over it.

A sprite that textures from what it draws is drawn pixel by pixel in the GE's order, with its lines loaded through the cache as its samples first need them, and its writes delayed as above. The run-ahead of the fetches is only approximated, for the narrow rows where it matters most. With that, Split Second's blur passes come out as on the PSP. Triangles that texture from themselves still read the texture as it was before the primitive.
