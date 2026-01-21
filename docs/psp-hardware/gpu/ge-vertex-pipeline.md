# The GE vertex transform pipeline

The GE implements something pretty similar to a standard T&L fixed function pipeline from early PC GPUs, but with some differences and additions.

I'm going to gloss over the actual transform and lighting details for now and focus on the surrounding parts, which matter for my upcoming re-implementation of this functionality in PPSSPP.

Vertex formats have a bit, "through mode", which if set, skips the entire T&L pipeline, sending coordinates directly to the rasterizer.

If through mode:

   * X and Y of float position vertex coordinates multiplied by 16, then converted back to integer. Floating point Z is cast to integer.
   * X and Y of integer coordinates are simply multiplied by 16.

If not through mode:

   * X, Y and Z are divided by W (after the projection matrix transform).
   * X, Y and Z are multiplied by viewportScaleXYZ, and viewportOffsetXYZ is added in.
   * X and Y coordinates are multiplied by 16 and converted to integer.
   * Z is cast to integer.

X and Y are now 12.4 fixed point coordinates with four fractional bits, while Z is a 0.16 fixed point value.

Note that the scene now is usually centered around 2048, 2048 (due to the viewport offset).
From here, we only have 16 bits to represent X and Y, so if any coordinate represents a value outside 0..4096, the primitive is discarded.

However, we now need to get into screen coordinates, relative to the framebuffer origin pointer.

X -= OFFSET_X
Y -= OFFSET_Y

To find the actual pixel coordinates:

X /= 16
Y /= 16

Now, any pixels in this space outside the scissor rectangle is discarded, which is how framebuffer size is implemented.

## MinZ and MaxZ

MinZ and MaxZ are two GPU registers that configure a range of valid Z values. Pixels that produce a Z value outside this range are discarded.
If MinZ > MaxZ, no pixels are produced.

## Clipper

TODO: Add information about triangle clipping against the Z=0 plane.