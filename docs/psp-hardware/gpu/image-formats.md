---
position: 8
---
# Image formats and tricks

The PSP supports a relatively wide range of image formats, and can freely "cast" between them. That means that it can render to a buffer in R5G6B5 format for example, and then texture from it as if it was a CLUT16 texture. This allows for a huge variety of crazy tricks - for the zaniest example I've seen so far, check out [this blog post](/news/lens-flare-burnout-dominator) about the lens flare in Burnout Dominator.

Here's a list of the renderable formats, along with their bitwise representations:

```text
RGB565   - BBBBBGGG GGGRRRRR
RGBA4444 - AAAABBBB GGGGRRRR
RGBA1555 - ABBBBBGG GGGRRRRR
RGBA8888 - AAAAAAAA BBBBBBBB GGGGGGGG RRRRRRRR
```

Textures can have these formats, in addition to the above:

* CLUT4 - 4-bit palette lookup
* CLUT8 - 8-bit palette lookup
* CLUT16 - 8-bits-out-of-16 palette lookup
* CLUT32 - 8-bits-out-of-32 palette lookup
* DXT1 - same as modern BC1 compression
* DXT3 - same as modern BC2 compression
* DXT5 - same as modern BC3 compression

The DXT formats don't perform well, and see little use. The [texture cache](/docs/psp-hardware/gpu/texture-cache) holds them decoded to 8888, so a DXT1 texture takes 8 times its size in the cache, and DXT3/5 4 times. One note is that the data order is reversed from the PC one.

## CLUT functionality

CLUT stands for Color Look Up Table, also commonly known as paletted image formats. There are four on the PSP: CLUT4, CLUT8, CLUT16 or CLUT32, where the number specifies the number of bits that represent each pixel.

When using these formats, in reality it only does an 4-bit or 8-bit CLUT lookup because the available palette memory just isn't bigger than 256 entries. However, there's a shift and a mask property, that can be used to pick out any 8-or-smaller set of bits from each pixel of the source texture.

## Depth buffer trickery

It's also possible to texture from depth buffers, but unlike color buffers these are not organized linearly. To resolve this, the PSP has some extra logic in its memory addressing that lets you use special VRAM addresses to "unswizzle" the buffers, so they look like regular CLUT or RGB buffers as needed.

### The depth buffer layout

A depth buffer is 16 bits per pixel, and has a base address and a stride like a color buffer. If it were linear, pixel (x, y) would live at

```text
linear = zbuf + (y * zstride + x) * 2
```

counted in bytes from the start of VRAM (0x04000000). It isn't stored there, though: the GE scrambles that address, and how depends on two things:

- whether the **color buffer** it's drawing to at the time is 16-bit (565, 5551, 4444, which all behave the same) or 32-bit (8888), and
- the EDRAM address translation value, set with `sceGeEdramSetAddrTranslation()`.

The scrambling works on the **absolute** VRAM address, not the offset within the depth buffer, and doesn't depend on the stride. With a translation value T (a power of two from 0x200 up), it goes like this:

- With a 32-bit color buffer, the address bits from bit 5 up to bit log2(T) - 1 are rotated left by one: bits 5 through log2(T) - 2 each move up a position, and the top one of them wraps around to bit 5. With T = 0x400 that's bits 5-9, so it shuffles 32-byte groups within each 1 KB. With a 16-bit color buffer, there's no rotation.
- Then, in both cases, the address is XORed with `(T << 3) | 0x40`. So with T = 0x400, it's XORed with 0x2040, which swaps pairs of 8 KB blocks and pairs of 64-byte groups.

T = 0 is special: the XOR is 0x600, and with a 32-bit color buffer, bits 9 and 10 are swapped instead of rotated.

Here's the same thing as code:

```cpp
// Where depth pixel (x, y) is actually stored, in bytes from the start of VRAM.
uint32_t DepthAddress(uint32_t zbuf, int zstride, int x, int y, bool color32, uint32_t translation) {
	uint32_t addr = zbuf + (y * zstride + x) * 2;
	if (translation == 0) {
		if (color32) {
			// Swap bits 9 and 10.
			uint32_t b9 = (addr >> 9) & 1, b10 = (addr >> 10) & 1;
			addr = (addr & ~0x600) | (b9 << 10) | (b10 << 9);
		}
		return addr ^ 0x600;
	}
	if (color32) {
		// Rotate bits 5 to log2(translation) - 1 left by one.
		int bits = 0;
		while ((1u << (bits + 5)) < translation)
			bits++;
		uint32_t mask = ((1u << bits) - 1) << 5;
		uint32_t field = (addr & mask) >> 5;
		field = ((field << 1) | (field >> (bits - 1))) & ((1u << bits) - 1);
		addr = (addr & ~mask) | (field << 5);
	}
	return addr ^ ((translation << 3) | 0x40);
}
```

### The unswizzling mirrors

VRAM is visible four times, at 0x04000000, 0x04200000, 0x04400000 and 0x04600000. The first and third show memory as it is. The other two undo the depth scrambling:

- 0x04200000 shows depth buffers that were drawn with a 16-bit color buffer linearly,
- 0x04600000 does the same for depth drawn with a 32-bit color buffer.

These follow the translation value too, so a depth buffer read through the right mirror always comes out linear, whatever the translation. That's what games that read depth do, often by pointing a texture at the mirror address.

All of this was measured on hardware by drawing every pixel of 512x272-pixel depth buffers with its own coordinates as the depth value, for all four color formats, several buffer addresses and strides, and translation values 0, 0x200, 0x400, 0x800 and 0x1000. It matches every pixel, and also the block-level results of the `gpu/ge/edramswizzle` test in pspautotests.

PPSSPP stores depth linearly and doesn't emulate the scrambling or the mirrors in general, only in a few special cases for games that depend on them.
