# The GE vertex transform pipeline

The GE implements something pretty similar to a standard T&L fixed function pipeline from early PC GPUs, but with some differences and additions.

I'm going to gloss over the actual transform and lighting details for now and focus on the surrounding parts, which matter for my work-in-progress re-implementation of this functionality in PPSSPP.

Vertex formats have a bit, "through mode", which if set, skips the entire T&L pipeline, sending coordinates directly to the rasterizer.

The clipper is positioned before the viewport transform. You'd think it would be after given how the viewport transform is defined, but not so.

The transform pipeline uses 24-bit floats (32-bit floats with 8 bits cut off, so 1 sign bit, 8 exponent bits and 15 mantissa bits) for inputs and likely many internal calculations, if not all. As a result, post-transform-matrix Z gets quantized to 15 bits.

References: [LocoRoco2 Tropuca investigation by [unknown]](https://github.com/hrydgard/ppsspp/issues/12058#issuecomment-913225641)

## Through mode

   * X and Y of float position vertex coordinates multiplied by 16, then converted back to integer. If Z is floating point, it's simply cast to integer.
   * X and Y of integer coordinates are simply multiplied by 16.

We then proceed to the rasterizer directly.

## Transform mode

XYZ from the vertex is transformed by first the world matrix, then the view matrix. Lighting is applied, affecting the two output colors. We'll gloss over that here.

Then, the vertex is transformed by the projection matrix, landing us in clip space.
The clip space is OpenGL style, -1 to 1 on all four dimensions.

## Z Cull

Triangles where all three corners are outside -W < Z < W here are culled.

## Z Clipper

If clipping is enabled, triangles that intersect the Z=-1 plane are clipped here. Note that this is *BEFORE* the viewport scale is applied! This was found in #12058. First I thought it was very strange that the clipper is here, but it's the same on PC, except that on PC the viewport can't then push the vertices outside the clip space again, which it can on the PSP! So the PSP has some extra behaviors here.

Definitions:
- "Z outside": Any Z value with greater magnitude than 0x3F8000FF (1.0000304) (i.e. where its 24-bit truncation would be greater than 1.0) is considered outside. Same for negative.

In every case:

- Discard triangles where *all* vertices are outside the same side of the viewing volume (Z only?)
- Discard triangles where XY of any vertex in screen space is outside 0..4096

### Clipping disabled

- Discard triangles where Z of any vertex is outside 0..65536 (!)

### Clipping enabled

- Clip and divide triangles hitting the near Z surface (and far? not sure)
- Clamp out-of-bounds Z (of triangles that are partially outside the Z volume)
- RECT primitives are clamped

## Viewport

The viewport transform is applied:

   * X, Y and Z are divided by W.
   * X, Y and Z are multiplied by viewportScaleXYZ, and viewportOffsetXYZ is added in.
   * X and Y coordinates are multiplied by 16 and converted to integer.
   * Z is cast to integer.

We are now in screen space. X and Y are now 12.4 fixed point coordinates with four fractional bits, while Z is a 0.16 fixed point value.

Note that the scene now is usually centered around 2048, 2048.

From here, we only have 16 bits to represent X and Y. If any coordinate represents a value outside 0..4096, the primitive is discarded.

Next, we move into drawing space. This is local framebuffer coordinates.
The offset is subtracted from X and Y, and both are divided by 16 (note: the fractions, if any, are used for subpixel precision. Hence, we only rasterize with 4-bit subpixel).

X = (X - OFFSET_X) / 16
Y = (Y - OFFSET_Y) / 16

Now that we are in drawing coordinates, we proceed to the rasterizer.

However, we now need to get into screen coordinates, relative to the framebuffer origin pointer.

Now, any pixels in this space outside the scissor rectangle is discarded, which is how framebuffer size is implemented.

W is preserved, and in the vertex shader we end by multiplying it back into X and Y, to get a homogenous coordinate as that's mandated by the API (and required for perspective correction to work).

## MinZ and MaxZ

MinZ and MaxZ are two GPU registers that configure a range of valid Z values. Pixels that produce a Z value outside this range are discarded. If MinZ > MaxZ, no pixels are produced.

## Z Clamp

If clamp is enabled, and MinZ == 0, any Z < 0 is set to 0.

## Rasterizer

