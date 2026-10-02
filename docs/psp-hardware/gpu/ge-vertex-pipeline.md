# The GE vertex transform pipeline

The GE implements something pretty similar to a standard T&L fixed function pipeline from early PC GPUs, but with some differences and additions.

I'm going to gloss over the actual transform and lighting details for now and focus on the surrounding parts, which matter for my work-in-progress re-implementation of this functionality in PPSSPP.

Vertex formats have a bit, "through mode", which if set, skips the entire T&L pipeline, sending coordinates directly to the rasterizer.

The clipper is positioned before the viewport transform. You'd think it would be after given how the viewport transform is defined, but not so.

The transform pipeline uses 24-bit floats (32-bit floats with 8 bits cut off, so 1 sign bit, 8 exponent bits and 15 mantissa bits) for inputs and likely many internal calculations, if not all. As a result, post-transform-matrix Z gets quantized to 15 bits. The exact depth path is in [Depth precision](#depth-precision) below.

References: [LocoRoco2 Tropuca investigation by [unknown]](https://github.com/hrydgard/ppsspp/issues/12058#issuecomment-913225641)

## Through mode

   * X and Y of float position vertex coordinates multiplied by 16, then converted back to integer. If Z is floating point, it's simply cast to integer.
   * X and Y of integer coordinates are simply multiplied by 16.

We then proceed to the rasterizer directly.

## Transform mode

XYZ from the vertex is transformed by first the world matrix, then the view matrix. Lighting is applied, affecting the two output colors. We'll gloss over that here.

Then, the vertex is transformed by the projection matrix, landing us in clip space.
The clip space is OpenGL style, -1 to 1 on all four dimensions.

## Fog

The fog factor is computed per vertex from the view space Z: `f = (viewZ + fogEnd) * fogSlope`. Each vertex then converts it to 8 bits, `min(floor(256 * f), 255)`, with negative values going to 0 and values of 1 and up to 255 (infinities and NaNs follow their sign), and those 8-bit values are interpolated linearly in screen space - *not* perspective corrected, unlike everything else. So a triangle whose fog range crosses 0 or 1 gets a gradient between the clamped vertex values, not a plateau.

The result blends between the fog color (at 0) and the fragment color (at 255), within a level of `(color * f + fogColor * (255 - f)) / 255`. There is no fog in through mode.

## Z Cull

A vertex is "outside" when |Z| > W. The GE compares the 24-bit clip space values directly, with no division, so this is the same as Z/W being at least one 24-bit step beyond 1.0.

Primitives where all corners are outside on the same side are culled here. This applies to every primitive type (points, lines, rectangles, triangles), and with Z clamp enabled or not. Corners outside on opposite sides (some beyond the far plane, the rest beyond the near one) don't cull either. A primitive with only some corners outside is drawn (see below for what happens to the part beyond the plane).

## Z Clipper

If clipping is enabled, triangles that intersect the Z=-1 plane are clipped here. Note that this is *BEFORE* the viewport scale is applied! This was found in #12058. First I thought it was very strange that the clipper is here, but it's the same on PC, except that on PC the viewport can't then push the vertices outside the clip space again, which it can on the PSP! So the PSP has some extra behaviors here.

Definitions:
- "Z outside": |Z| > W, as in Z Cull above. Any Z/W with greater magnitude than 0x3F8000FF (1.0000304) (i.e. where its 24-bit truncation would be greater than 1.0) is outside. Same for negative.

In every case:

- Discard triangles where *all* vertices are outside the same side of the viewing volume (Z only?)
- Discard triangles where XY of any vertex in screen space is outside 0..4096

Nothing is clipped at the far plane: the part of a triangle beyond Z/W = 1 is rasterized with its interpolated depth (beyond what Z/W = 1 maps to), and only the per-pixel MinZ/MaxZ test removes pixels. With the depth range set to exactly the viewport's -1..1, that test is what cuts the triangle along the far plane. The near plane is only clipped with clipping enabled, see below.

### Clipping disabled

- Discard primitives where screen space Z (after the viewport, before it's cast to integer) of any vertex is outside 0..65536 (!): below 0, or 65536 and up. So 65535.5 is kept, -0.01 isn't. This holds even when every vertex has -W < Z < W, for every primitive type, and for both corners of a RECT (even though its depth comes from the second one).
- No clipping at the near plane either: the part beyond it is drawn with extrapolated depth (and culled by the rule above if a vertex lands below 0).
- Discard triangles and lines with any vertex behind the camera (W <= 0).

### Clipping enabled

- Clip and divide triangles (and lines) hitting the near Z surface, including ones with vertices behind the camera. Clipping happens before the viewport, so a vertex that gets clipped away doesn't discard the primitive by landing outside the screen range (even at W = 0).
- Clamp out-of-bounds screen space Z to 0..65535, per vertex, before interpolation. A pixel next to a clamped vertex gets a depth in between, not 65535. The clamp is always to 0..65535, whatever MinZ and MaxZ are: those then reject pixels as usual.
- RECT primitives are clamped

RECTs are never clipped: one with a vertex behind the camera (W < 0) is discarded, with clipping enabled or not.

## Viewport

The viewport transform is applied:

   * X, Y and Z are divided by W.
   * X, Y and Z are multiplied by viewportScaleXYZ, and viewportOffsetXYZ is added in.
   * X and Y coordinates are multiplied by 16 and converted to integer.
   * Z is cast to integer.

### Depth precision

Measured bit exact for Z (X and Y are probably the same, but not measured yet). Every value is a 24-bit float, and every rounding truncates toward zero:

   * Clip space Z and W: the matrix product is truncated to 24 bits, then the translation row is added with an adder that has no guard bits: the smaller term is first truncated to the ulp of the larger one.
   * Z/W is `Z * R(W)`, truncated, where R is the GE's own reciprocal, not a divide and not the VFPU's `vrcp`. It's piecewise linear over 128 segments of the mantissa: with `i` the 15 mantissa bits, `s = i >> 8` and `x = i & 255`, `R = (64 * B[s] + 63 + M[s] * x) >> 7` in units of 2^-16 (scaled by the exponent). B runs from 131072 down to 65793 and M from -254 to -64.
   * Screen Z is `floor(truncate(ndcZ * scaleZ) + offsetZ)`, with the same guard-bit-less adder.

The matrices are combined before the vertex is transformed (a translation in one and its negation in the next cancel exactly), and texture coordinates keep a 15-bit significand too.

PPSSPP's software renderer implements all of this (`GPU/Software/TransformUnit.cpp`, which has the B/M table), and the pspautotests test `gpu/depth/transformprecision` checks it.

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

Note that this can be used o

## Z Clamp

If clamp is enabled, a vertex's screen Z below 0 is set to 0, and above 65535 to 65535, before interpolation. This happens whatever MinZ and MaxZ are: measured with MinZ = 5000 and with MaxZ = 60000, where the drawn area only fits a vertex clamped to 0 or 65535 (and then the per-pixel MinZ/MaxZ test).

An earlier version of this page said the clamp only applies when MinZ == 0 (or MaxZ == 65535), so that setting MinZ to 1 and MaxZ to 65534 disables it. The measurements above contradict that, at least for MinZ/MaxZ that far from the ends.

