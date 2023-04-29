# Image formats and tricks

The PSP supports a relatively wide range of image formats, and can freely "cast" between them. That means that it can render to a buffer in R5G6B5 format for example, and then texture from it as if it was a CLUT16 texture. This allows for a huge variety of crazy tricks - for the zaniest example I've seen so far, check out [this blog post](/blog/lens-flare-burnout-dominator) about the lens flare in Burnout Dominator.

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

The DXT formats don't perform well, likely due to being expanded to full RGBA in the texture cache, so see little use. One note is that the data order is reversed from the PC one.

## CLUT functionality

CLUT stands for Color Look Up Table, also commonly known as paletted image formats. There are four on the PSP: CLUT4, CLUT8, CLUT16 or CLUT32, where the number specifies the number of bits that represent each pixel.

When using these formats, in reality it only does an 4-bit or 8-bit CLUT lookup because the available palette memory just isn't bigger than 256 entries. However, there's a shift and a mask property, that can be used to pick out any 8-or-smaller set of bits from each pixel of the source texture.

## Depth buffer trickery

It's also possible to texture from depth buffers, but unlike color buffers these are not organized linearly. To resolve this, the PSP has some extra logic in its memory addressing that lets you use special VRAM addresses to "unswizzle" the buffers, so they look like regular CLUT or RGB buffers as needed.
