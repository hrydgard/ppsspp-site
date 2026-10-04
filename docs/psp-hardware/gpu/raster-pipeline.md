---
position: 4
---
# The GE raster pipeline

This page picks up where the [vertex pipeline](/docs/psp-hardware/gpu/ge-vertex-pipeline) leaves off. There, vertices end up with screen X and Y in 12.4 fixed point (4 subpixel bits), a 16-bit Z, an 8-bit fog value, colors and texture coordinates. Here they become pixels.

The rasterizer doesn't work like a PC GPU's, or like the PS2's edge walker. Nearly every value is interpolated with fixed-point planes set up with the GE's own reciprocal table. The arithmetic is described on the [GE arithmetic](/docs/psp-hardware/gpu/arithmetic) page; this page describes what it's applied to. Everything here was measured on hardware with hand-built display lists, and PPSSPP's software renderer reproduces it bit for bit, mostly in [`GPU/Software/Rasterizer.cpp`](https://github.com/hrydgard/ppsspp/blob/master/GPU/Software/Rasterizer.cpp). The pspautotests in [`gpu/exact`](https://github.com/hrydgard/pspautotests/tree/master/tests/gpu/exact) replay a selection of those display lists and check the results.

## Triangle coverage

- Pixels are sampled at their centers, at +8 in 1/16 pixel.
- The fill rule is top-left: a pixel center exactly on an edge belongs to the triangle when that edge is a top or a left edge.
- The edge test is exact. An approximation of the side test (an integer division in an older PPSSPP) went wrong for a center 0.2 subpixels from an edge.
- A zero-area triangle (collinear vertices) lights nothing, even when its edge passes through pixel centers. This holds in through mode and transform mode.
- The winding test (the sign of the doubled area) needs 64 bits for vertices far apart.

### Very tall triangles

There's one coverage quirk. When a triangle's long edge (top vertex to bottom vertex) has `3·dy > 2^17` in subpixels, that is `dy >= 43691` or about 2730 pixels, its rows are drawn in 4-pixel spans. The first pixel of each span along that edge (the last one for a right edge) is lit when any pixel of the span is inside. This doesn't depend on the slope, side or position, and the short edges stay exact. Most likely each span's pixels are derived from one end by subtracting `k·dy`, and the `3·dy` term overflows a 17-bit field. No triangle in over 200 game frame dumps is that tall, but a probe makes one easily.

### Walking order

Pixels are drawn row by row, never in tiles or quads, and every row of a primitive goes the same way. This only shows when a primitive textures from its own render target (see the [texture cache](/docs/psp-hardware/gpu/texture-cache)). Neither vertex order nor winding makes a difference.

- Sprites are drawn top to bottom, each row left to right.
- A triangle's rows start at one end of its long edge (the edge from the top vertex to the bottom one): the end that is farther from the middle vertex in x. When two vertices share the top or bottom row, the left one counts as the top or bottom vertex. So flat-top triangles are drawn top-down, flat-bottom ones bottom-up, and other triangles either way.
- Each row is walked from the long edge's side: right to left when the long edge is the triangle's right side, otherwise left to right.

## Planes

Depth, Gouraud color, fog and texture coordinates all go through the same fixed-point planes:

1. The setup computes the doubled area from the 12.4 positions and takes its reciprocal with the setup reciprocal table: 256 segments, finer than the transform's reciprocal.
2. Each value's gradients in x and y are floored to 14 fraction bits per subpixel.
3. The plane is anchored at the leftmost vertex, or at the rightmost when the long (top to bottom) edge is strictly the triangle's right side, with no ties in y.
4. A pixel's value is the floor of the plane at the pixel center.

So depth isn't `z/w` per pixel. It's an affine function of screen position, rounded the same way for every pixel of a triangle. Depth, Gouraud color and fog are all screen-linear with no perspective correction. For color and fog, that's unlike a PC GPU.

**Depth.** The plane's floored value is clamped to 0 below; it can't overflow at the top. This applies to the depth range test and the write, in through mode and transform mode, with either clamp setting. Frontier Gate Boost's sky ([#6531]) has a seam without it.

**Colors.** The 8-bit channel values go into the planes directly. A Gouraud triangle in transform mode is screen-linear, exactly like through mode.

**Fog.** The 8-bit per-vertex fog value goes through a plane too. Fog has no effect in through mode.

### Texture coordinates

In through mode, the texture coordinates go into the planes as they are. In transform mode, each vertex first gets:

```text
q = R(w)               (the transform's reciprocal of w)
s = float24(u · q)
t = float24(v · q)
```

Then s, t and q of the three vertices become 15-bit integers, scaled to the largest exponent among the three values of each, and go through planes. Per pixel, `u = s · R(q)`, kept to 24 bits. That's the perspective correction, done with the same reciprocal as the perspective divide. See [texture coordinates per pixel](/docs/psp-hardware/gpu/arithmetic#texture-coordinates-per-pixel) for the precision.

With the texture matrix (UV generation mode 1), `q` is the matrix's q times `R(w)`. Projected textures on perspective triangles come out exact that way.

### Consequences

Because each primitive has its own planes, two triangles making up a rectangle don't sample the same texels as one sprite covering it:

- A 480x33 bar textured with 280x20 texels lands on texel row 10 in the triangle anchored at the bottom, and on row 9 in a sprite, which is anchored at the top.
- A 1:1 bilinear sprite only samples texel centers when its area is a power of two. Others (49 or 15 pixels wide, for example) have a gradient that's just below 1 texel per pixel in fixed point, so they sample a little below the centers and blend their neighbors. Games show this: a track map drawn as three 1:1 sprites side by side is blurry in two of them on hardware.

## Sprites (RECTANGLES)

A sprite is two vertices. Its pixels:

- The first column is `(x + 6) >> 4` and the first row `(y + 7) >> 4`, in subpixels. The end is `(v + 7) >> 4`, exclusive.
- A sprite whose vertices go bottom left to top right is drawn *rotated*: its UVs swap axes, its first row rounds like a column, `(y + 6) >> 4`, and its UV plane comes from v0, v1 and the bottom right corner, so it's anchored at the bottom left.

Its UVs come from planes through three of its corners, like a triangle's. The UV corners take s, t and q *component by component* from the two vertices: s from the vertex that supplies that corner's x, t from the one that supplies its y (swapped when rotated), and q from the one that supplies its x. So a sprite whose two vertices have different w is mapped with a bend.

## Lines

Lines use diamond exit rules, like Direct3D's:

- A pixel is lit when the line passes through its diamond `|x - cx| + |y - cy| < 1/2` and doesn't end inside it.
- The major axis is x only when `|dx| > |dy|`, so a diagonal line is y-major.
- An endpoint exactly on a diamond's boundary counts as inside only on the top corner and its two edges, for x-major lines. For y-major lines that's the left corner and its two edges. The major-axis corners count as outside.

The same rule decides a line that only touches a diamond, such as a horizontal line along the top corners of a row.

**Color and depth** along a line come from a gradient along the major axis: the value difference times the setup reciprocal of the major length, floored to 14 fraction bits per subpixel. A pixel's value is the value at v0 plus the gradient times the walk from v0 to the pixel center along the line's direction. Lines shorter than a pixel keep their fog.

**Texture coordinates** along a transform-mode line work the same way, with the triangle's per-vertex values: s, t and q (as 15-bit values at the two vertices' largest exponent) go along the major axis with that gradient, and the pixel's coordinate is `u = s · R(q)`. A pixel just inside a line's end extrapolates from its center, so it samples just short of the end vertex's coordinate, not the coordinate itself: with v = 1.0 at the end, the last pixel reads the last texel row instead of wrapping to the first.

### Antialiased lines

Antialiased lines light the same pixels. Each pixel's alpha replaces the vertex alpha:

```text
g = floor(-16 · dy · q / 2^(e + 2))           (the slope, like a plane gradient)
o = floor((16 · (py - y0) · 2^14 + g · walk) / 2^14)
alpha = 128 - |o|
```

Here `o` is the pixel center's minor offset from the line in 1/16 pixel, `q` and `e` come from the setup reciprocal of the major length, and `walk` is the pixel's major distance from v0. Few games use these lines, echochrome among them.

## Texturing

- A texture coordinate becomes a texel position in 1/16 texel, truncated toward zero.
- **Nearest** filtering takes the position `>> 4`.
- **Bilinear** uses `(k - 8) >> 4` for the texel and `(k - 8) & 15` for the weight, with 4-bit weights. It does two horizontal lerps, each truncated, then the vertical one.
- 16-bit texels and CLUT entries are expanded to 8 bits per channel before filtering.

### Mip level selection

The level of detail D is in 1/16 levels, from the [float-bits logarithm](/docs/psp-hardware/gpu/arithmetic#a-float-bits-logarithm):

- **Auto:** `D = log16(g) - log16(q)`, where g is the largest of the four s and t plane gradients in texels per pixel, and q is the pixel's interpolated q.
- **Slope:** `D = 16 + log16(slope) - log16(q) + bias`.
- **Const:** `D = bias`.

D is clamped to 0..16·(max level). Mipmap linear blends level `D >> 4` with the next one by `D & 15`. Mipmap nearest takes level `(D + 8) >> 4`.

The q isn't taken per pixel, and not per 2x2 quad either like on modern GPUs. Each row is cut into 4-pixel spans (x = 4k..4k+3), and every pixel of a span uses the q at the span's second pixel in walking order: 4k+1 left to right, 4k+2 right to left. When that pixel is outside the triangle, the span uses its first covered pixel instead.

With separate CLUTs per mip level (texture mode bit 8), level n offsets the CLUT index by n shifted above the index bits, wrapped to the CLUT.

### Texture functions

- **Modulate** is `(t · (c + 1)) >> 8` per channel.
- **Color doubling** doubles the secondary (specular) color too: modulate gives `clamp(((2T(P + 1)) >> 8) + 2S)`.
- **Fragment alpha** with texture alpha is `((a_prim + 1) · a_tex) >> 8`.

## Per-pixel operations

These follow PPSSPP's existing model, which the probes confirmed. Write masks, color test, alpha test, every logic op, clear mode with every flag combination, dithering, and 565, 5551 and 4444 conversion were already all exact. A few details are worth spelling out:

- **Stencil** compares `(ref & mask)` against `(stencil & mask)`, with the reference on the left of the comparison.
- **Blending** is `((2c + 1)(2f + 1)) >> 10` for each term, then one clamp. The doubled inverse factors aren't clamped, so for α ≥ 128 their term is subtracted. Min, max and absolute difference use the raw colors. See the [arithmetic page](/docs/psp-hardware/gpu/arithmetic#8-bit-color-products).
- **Fog** blends toward the fog color by the interpolated 8-bit factor.

### REGION1 is a translation

The drawing region registers (REGION1, 0x15, and REGION2) do more than clip. A pixel is drawn when its screen position is inside the scissor and inside [REGION1, REGION2], and it's written at screen position minus REGION1:

- y is shifted exactly by y1;
- x is shifted by x1 rounded up to a multiple of 4, and for odd x1 the pixels of each group of 4 are reversed: `fb_x = (screen_x - shift) ^ 3`;
- an empty region (x2 < x1) draws nothing.

No game in the dump collection sets REGION1 to anything but 0, so PPSSPP ignores it.

### Drawing past the stride

A pixel beyond the framebuffer's stride isn't clipped; it lands at the start of the next row, written in drawing order. Tokimeki Memorial 4's blur ([#6379]) writes x = 128-129 into a 128-wide buffer, which on the PSP ends up at x = 0-1 of the next row.
