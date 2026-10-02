---
position: 6
---
# The texture cache and self-texturing

The GE reads textures through a small texture cache, 8 KB in size. For nearly all rendering, the cache is invisible: textures don't change while they're drawn with. It does become visible when a game draws to a buffer while also texturing from it, or when the CPU changes a texture without telling the GE.

## TEXFLUSH

The cache isn't coherent with the GE's own framebuffer writes or with CPU writes. The TEXFLUSH command (0xCB) empties it, and the SDK emits one whenever a texture is set, which is why most games never notice the cache.

A texture that fits in the cache, 8 KB or less, stays there until TEXFLUSH, across primitives and across draw calls. Re-drawing with the same texture address and no flush in between reads the *cached* contents, not what's in memory now.

This was shown with a 16x16 4444 buffer (512 bytes) blurred onto itself three times. The result differs in 177 pixels with and without a TEXFLUSH between the passes, and both versions are reproduced exactly when the cache keeps its contents until the flush. Final Fantasy Type-0's blur ([#20104]) depends on this.

## Self-texturing

Some post-processing effects texture from the buffer they're drawing to: a blur that draws a buffer onto itself shifted by a pixel, a bloom or a glare pass. What such a draw does depends entirely on when each texel was fetched relative to when the pixels around it were written.

PPSSPP's hardware backends copy the target before such draws, so they read it as it was before the draw. The software renderer used to read live memory, in its own order. The PSP does neither, and its behavior is only partly understood.

Mostly, the GE reads the pre-draw contents:

- A primitive's texels come from data fetched before its own writes. So for most purposes, "the texture as it was before this primitive" is a good model. With that model, The 3rd Birthday's blur is exact, Burnout Dominator's glare is 98%, and Ridge Racer's blur is 94%.
- What's left looks like lazy loading in 8-row blocks. The texture is fetched in lines 8 rows tall and 64-128+ texels wide, loaded the first time a primitive needs them, and never refreshed during that primitive. Rows that were written by earlier parts of the same primitive before their block was loaded *do* show the new data. A sprite drawn onto itself shifted down by 8 or 20 rows reads the rewritten rows, while one shifted by 1 row only does at block boundaries. In Ridge Racer, rows 9 and 17 of the blur show exactly this.
- Pixel order matters for the same reason, and is described on the [raster pipeline](/docs/psp-hardware/gpu/raster-pipeline#walking-order) page: flat-top triangles go top-down, flat-bottom ones bottom-up.

PPSSPP's software renderer now gives each primitive the texture as it was before it, and keeps a cache-sized texture until TEXFLUSH ([`GPU/Software/BinManager.cpp`](https://github.com/hrydgard/ppsspp/blob/master/GPU/Software/BinManager.cpp)). The 8-row block loading isn't modeled yet.

## Measuring it

These effects were measured by making the framebuffer also the texture, prefilled by the CPU, and drawing it at a one-pixel offset with a fixed 1 + 1 additive blend. Each pixel then shows whether its neighbor had already been drawn when it was read.
