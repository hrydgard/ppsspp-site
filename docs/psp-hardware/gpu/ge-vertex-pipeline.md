---
position: 2
---
# The GE vertex transform pipeline

The GE implements something pretty similar to a standard T&L fixed function pipeline from early PC GPUs, but with some differences and additions.

I'm going to gloss over the actual transform and lighting details for now and focus on the surrounding parts, which matter for my work-in-progress re-implementation of this functionality in PPSSPP.

Vertex formats have a bit, "through mode", which if set, skips the entire T&L pipeline, sending coordinates directly to the rasterizer.

The clipper is positioned before the viewport transform. You'd think it would be after given how the viewport transform is defined, but not so.

The transform pipeline uses 24-bit floats (32-bit floats with 8 bits cut off, so 1 sign bit, 8 exponent bits and 15 mantissa bits) for its inputs and nearly all of its internal calculations, with its own adder, multi-operand sums, reciprocal and reciprocal square root. Those are described on the [GE arithmetic](/docs/psp-hardware/gpu/arithmetic) page; this page refers to them. Lighting has a page of its own, [GE lighting](/docs/psp-hardware/gpu/lighting), and so do [curves](/docs/psp-hardware/gpu/curves). What happens after the viewport is on the [raster pipeline](/docs/psp-hardware/gpu/raster-pipeline) page.

References: [LocoRoco2 Tropuca investigation by [unknown]](https://github.com/hrydgard/ppsspp/issues/12058#issuecomment-913225641)

## Through mode

   * X and Y of float position vertex coordinates multiplied by 16, then converted back to integer. If Z is floating point, it's simply cast to integer.
   * X and Y of integer coordinates are simply multiplied by 16.

We then proceed to the rasterizer directly.

## Transform mode

Conceptually, XYZ from the vertex is transformed by first the world matrix, then the view matrix, and then the projection matrix, landing us in clip space. Lighting happens in world space along the way and produces the two output colors (see [GE lighting](/docs/psp-hardware/gpu/lighting)). The clip space is OpenGL style, -1 to 1 on all four dimensions.

In practice the GE doesn't transform the vertex three times:

   * Vertex positions and matrix entries are float24s. 8- and 16-bit positions are divided by 128 and 32768, exactly.
   * The three matrices are combined first, as `(W·V)·P`, with every entry of each product a [row sum](/docs/psp-hardware/gpu/arithmetic#row-sums).
   * The combined matrix is applied to the raw position, each row a row sum: the products exact, all of them truncated to the precision of the largest term, summed exactly, and the sum truncated to float24.

So a translation in one matrix and its negation in the next cancel exactly, where staged float math would lose the vertex's low bits.

### Morphing

With morph weights, everything is morphed before anything else: position, normal, UV, color and even the skinning weights. The morphed position is the sum of `float24(w_k · p_k)` over the targets, accumulated left to right with the [GE adder](/docs/psp-hardware/gpu/arithmetic#the-adder), each sum truncated to float24. UVs morph the same way. Colors morph per channel after expanding to 8 bits, and the sum is floored.

### Skinning

Each bone matrix entry is first scaled by the bone's weight, `float24(w · m)`. Then one accumulator runs through the bones in order, adding each bone's translation, then x, y and z times that bone's columns. Every step is a GEAdd truncated to float24, so unlike the matrix rows this is a sequential sum. Normals are skinned the same way without the translation. Weights aren't normalized: u8 weights are divided by 128 and u16 by 32768.

The skinned position then goes through the combined world, view and projection matrix like any other position.

### Texture coordinates

* **UV mode 0** (scale and offset) computes `u' = GEAdd(float24(u · scale), offset)` per vertex. 8- and 16-bit texture coordinates are unsigned, divided by 128 and 32768.
* **UV mode 1** (the texture matrix) computes `(s, t, q)` as the source times the 4x3 texture matrix, each component a row sum. The source is the position, the UV, the normal or the normalized normal. The normalized normal uses the GE's reciprocal square root.
* **UV mode 2** (shade mapping) computes the coordinates from two lights; see [GE lighting](/docs/psp-hardware/gpu/lighting#shade-mapping).

In transform mode the coordinates are then divided by w per pixel, through planes. That's described on the [raster pipeline](/docs/psp-hardware/gpu/raster-pipeline#texture-coordinates) page.

### Vertex formats without UVs or a normal

A draw whose vertex format has no texture coordinates uses the last ones read before it, from the last vertex drawn (in index order for an indexed draw) by any earlier draw. That holds across display lists, between through mode and transform mode, and whether or not the earlier draw was textured. The coordinates are kept as they were read, before scale and offset: the draw that uses them applies its own UV scale and offset.

Normals are carried the same way. A draw without normals doesn't change the carried one.

The pspautotest `gpu/vertices/carry` shows each of these.

## Fog

The fog factor is computed per vertex from the view space Z: `f = (viewZ + fogEnd) * fogSlope`. Each vertex then converts it to 8 bits, `min(floor(256 * f), 255)`, with negative values going to 0 and values of 1 and up to 255 (infinities and NaNs follow their sign), and those 8-bit values are interpolated linearly in screen space, *not* perspective corrected (only texture coordinates are). So a triangle whose fog range crosses 0 or 1 gets a gradient between the clamped vertex values, not a plateau.

The result blends between the fog color (at 0) and the fragment color (at 255), within a level of `(color * f + fogColor * (255 - f)) / 255`. There is no fog in through mode.

## Cull

A vertex is "outside" a plane when |X| > W, |Y| > W or |Z| > W. The GE compares the 24-bit clip space values directly, with no division, so this is the same as X/W, Y/W or Z/W being at least one 24-bit step beyond 1.0. A vertex exactly on a plane (X = W) is inside.

Primitives where all corners are outside the same plane are culled here. This applies to every primitive type (points, lines, rectangles, triangles), with clipping and Z clamp enabled or not. Corners outside different planes, or on opposite sides of one (some beyond the far plane, the rest beyond the near one), don't cull. A primitive with only some corners outside is drawn (see below for what happens to the part beyond the plane).

There are no X and Y clip planes, only this cull: a triangle that reaches beyond X = W is drawn in full, up to the guard band, the scissor and the drawing region. The X/Y cull only becomes visible when the viewport maps clip space beyond ±1 onto the screen: with a viewport scale of 10, a triangle at clip X = 1.25..2.5 lands on screen but isn't drawn. Normal viewports put everything that's culled off screen anyway. PPSSPP's software renderer does the cull in [`GPU/Software/Clipper.cpp`](https://github.com/hrydgard/ppsspp/blob/master/GPU/Software/Clipper.cpp), and the pspautotests test [`gpu/clipping/xycull`](https://github.com/hrydgard/pspautotests/tree/master/tests/gpu/clipping) checks it.

## Z Clipper

If clipping is enabled, triangles that intersect the Z=-1 plane are clipped here. Note that this is *BEFORE* the viewport scale is applied! This was found in #12058. First I thought it was very strange that the clipper is here, but it's the same on PC, except that on PC the viewport can't then push the vertices outside the clip space again, which it can on the PSP! So the PSP has some extra behaviors here.

Definitions:
- "Z outside": |Z| > W, as in Z Cull above. Any Z/W with greater magnitude than 0x3F8000FF (1.0000304) (i.e. where its 24-bit truncation would be greater than 1.0) is outside. Same for negative.

In every case:

- Discard primitives where *all* vertices are outside the same plane, X, Y or Z (see Cull above)
- Discard triangles where XY of any vertex in screen space is outside 0..4096

Nothing is clipped at the far plane: the part of a triangle beyond Z/W = 1 is rasterized with its interpolated depth (beyond what Z/W = 1 maps to), and only the per-pixel MinZ/MaxZ test removes pixels. With the depth range set to exactly the viewport's -1..1, that test is what cuts the triangle along the far plane. The near plane is only clipped with clipping enabled, see below.

### Clipping disabled

- Discard primitives where screen space Z (after the viewport, before it's cast to integer) of any vertex is outside 0..65536 (!): below 0, or 65536 and up. So 65535.5 is kept, -0.01 isn't. This holds even when every vertex has -W < Z < W, for every primitive type, and for both corners of a RECT (even though its depth comes from the second one).
- No clipping at the near plane either: the part beyond it is drawn with extrapolated depth (and culled by the rule above if a vertex lands below 0).
- Discard triangles and lines with any vertex behind the camera (W <= 0).

### Clipping enabled

- Clip and divide triangles (and lines) hitting the near Z surface, including ones with vertices behind the camera. Clipping happens before the viewport, so a vertex that gets clipped away doesn't discard the primitive by landing outside the screen range (even at W = 0).
- A vertex exactly on the near plane (Z = -W) isn't clipped, so its screen XY still has to be inside 0..4096, or the primitive is discarded.
- Clamp out-of-bounds screen space Z to 0..65535, per vertex, before interpolation. A pixel next to a clamped vertex gets a depth in between, not 65535. The clamp is always to 0..65535, whatever MinZ and MaxZ are: those then reject pixels as usual.
- RECT primitives are clamped

The clipped vertices are interpolated from the inside vertex, with the [GE's arithmetic](/docs/psp-hardware/gpu/arithmetic):

```
dIn  = GEAdd(in.z, in.w)
dOut = GEAdd(out.z, out.w)
t    = float24(float24(dIn) * R(float24(GEAdd(dIn, -dOut))))
c    = float24(GEAdd(float24(t * float24(GEAdd(out.c, -in.c))), in.c))     for x, y, z, w and the UVs
```

The `float24` around `out - in` matters: the adder can carry into a 17th significand bit, which the hardware drops before the multiply. Colors and fog (8 bits per vertex) are `(a * (256 - t8) + b * t8) >> 8`, with `t8 = round(256 * t)`.

With one vertex outside, the triangle becomes two: (p, a, b) and (p, b, n), p being the vertex before the outside one in the GE's order, with the winding kept. With two outside, it becomes one triangle. The GE's order reverses odd triangles of a strip, and keeps fans in order.

RECTs are never clipped: one with a vertex behind the camera (W < 0) is discarded, with clipping enabled or not.

## Viewport

The viewport transform is applied:

   * X, Y and Z are divided by W.
   * X, Y and Z are multiplied by viewportScaleXYZ, and viewportOffsetXYZ is added in.
   * X and Y coordinates are multiplied by 16 and converted to integer.
   * Z is cast to integer.

### Precision

Measured bit exact for X, Y and Z alike. Every value is a 24-bit float, and every rounding truncates toward zero:

   * Clip space X, Y, Z and W come from the combined matrix's rows, each a [row sum](/docs/psp-hardware/gpu/arithmetic#row-sums).
   * NDC is `float24(clip * R(W))`, where R is the GE's own [reciprocal](/docs/psp-hardware/gpu/arithmetic#the-reciprocal), not a divide and not the VFPU's `vrcp`.
   * Screen is `GEAdd(float24(ndc * scale), offset)`, with the [guard-bit-less adder](/docs/psp-hardware/gpu/arithmetic#the-adder).
   * X and Y become 12.4 fixed point as `floor(X * 16)`. Z is floored, then goes through the cull and clamp rules above.

PPSSPP's software renderer implements all of this ([`GPU/Software/TransformUnit.cpp`](https://github.com/hrydgard/ppsspp/blob/master/GPU/Software/TransformUnit.cpp) and `GEMath.cpp`), and the pspautotests test [`gpu/depth/transformprecision`](https://github.com/hrydgard/pspautotests/tree/master/tests/gpu/depth) checks the depth part.

We are now in screen space. X and Y are now 12.4 fixed point coordinates with four fractional bits, while Z is a 0.16 fixed point value.

Note that the scene now is usually centered around 2048, 2048.

From here, we only have 16 bits to represent X and Y, with 4 bits of fraction. If any coordinate represents a value outside 0..4096, the primitive is discarded.

Next, we move into drawing space. This is local framebuffer coordinates.
The offset is subtracted from X and Y, and both are divided back down by 16 (note: the fractions, if any, are used for subpixel precision. Hence, we only rasterize with 4-bit subpixel).

X = (X - OFFSET_X) / 16
Y = (Y - OFFSET_Y) / 16

Now that we are in drawing coordinates, we proceed to the rasterizer.

However, we now need to get into screen coordinates, relative to the framebuffer origin pointer.

Now, any pixels in this space outside the scissor rectangle is discarded, which is how framebuffer size is implemented.

W is preserved, and in the vertex shader we end by multiplying it back into X and Y, to get a homogenous coordinate as that's mandated by the API (and required for perspective correction to work).

## MinZ and MaxZ

MinZ and MaxZ are two GPU registers that configure a range of valid Z values. Pixels that produce a Z value outside this range are discarded. If MinZ > MaxZ, no pixels are produced.

The per-pixel depth that's tested comes from the triangle's depth plane (see the [raster pipeline](/docs/psp-hardware/gpu/raster-pipeline#planes)). A plane value below 0 is set to 0 for both the test and the write, in through and transform mode and whether clamping is on or not.

## Z Clamp

If clamp is enabled, a vertex's screen Z below 0 is set to 0, and above 65535 to 65535, before interpolation. This happens whatever MinZ and MaxZ are: measured with MinZ = 5000 and with MaxZ = 60000, where the drawn area only fits a vertex clamped to 0 or 65535 (and then the per-pixel MinZ/MaxZ test).

An earlier version of this page said the clamp only applies when MinZ == 0 (or MaxZ == 65535), so that setting MinZ to 1 and MaxZ to 65534 disables it. The measurements above contradict that, at least for MinZ/MaxZ that far from the ends.

