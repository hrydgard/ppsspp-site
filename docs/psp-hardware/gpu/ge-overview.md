# The GE (Graphics Engine)

## Overview

At a first glance, the PSP GPU, known as the GE (graphics engine),  looks like a conventional small fixed-function GPU not unlike something like the original Geforce 256 (but without multitexturing support). In reality though, it's actually a quite strange and interesting little beast.

Unlike the PS2's GS which is a pure rasterizer, it has a T&L (transform and lighting) vertex pipeline, so you give it world, view and projection matrixes with the vertex data, and it'll automatically get transformed and lit, if enabled.

It also has (mostly) conventional blend modes with a few special additions, and it supports all the regular depth and stencil operations.

However, it does differ from modern GPUs on modern OSs in a number of ways, some of which make it a bit tricky to emulate:

## Evolution from the PS2's GS

The PSP has a rasterizer that's pretty similar to the GS in some ways, but the frontend is completely different. The PSP put a traditional T&L engine, command list queue and vertex fetch in front, removing all the PS2's highly complex VIF/GIF/VU stuff and making it feel very different.

In comparison to the famously tricky GS, the PSP's rasterizer has far less crazy tiling/swizzling formats, and explicit support for "channel shuffle" - it adds 32-bit and 16-bit CLUT formats where you get to specify shift and mask before the actual 8-bit lookup, so you don't have to actually exploit wacky tiling formats and draw thin rectangles, you can just specify your shuffle and lookup directly and go.

So overally it's easier to emulate (although still not super easy, and the T&L pipe is non-trivial). The blender is also much more standard (although still has a few hard-to-emulate modes).

## Feature set

### Pixel pipeline

Single texturing only, and no shaders, only a few simple combiner modes (add to vertex color, multiply by vertex color, alpha blend with vertex color, etc). There's a flag that changes how the alpha channel is processed, too.

Color masking is not just per channel like on PC, you can set a full bitmask and control whether each individual bit in each pixel will be written to the framebuffer.

A scissor rectangle is supported, and is the only thing that limits the bounds of rendering (the viewport parameters do not specify a rectangle).

It has a normal set of blend factors and blend modes, with some additions, that are somewhat tricky to emulate - 2xSRC, 2xDST, their negations, and the special blend op ABSDIFF which does what you'd guess. This one is pretty rare but is used for example for the cartoon rendering effect in Dragon Ball Z - Tenkaichi Tag Team.

Stencil buffers are not stored separately, and are not interleaved with the depth buffer like on PC. Instead, the color alpha channel serves double duty as both stencil and alpha data. This means that if you specify that your framebuffer has the format R5G5B5A1, there really is only 1 bit of stencil, while with R4G4B4A4 you effectively have a 4-bit stencil buffer and with R8G8B8A8 your stencil buffer is the full 8 bits wide. And if you render using R5G6B5 format, there simply is no stencil buffer available.

There's support for special "antialiasing lines", though not heavily used, so can be largely ignored during emulation.

### Vertex transform & lighting

Unlike the PS2 it has a hardwired vertex transform and lighting unit. It supports world/view/proj matrices. 4 lights supported (but more than 1 is rarely used).

The viewport is specified by a scale/offset instead of a rectangle. This also implies that it doesn't affect the scissor.

Parameters like matrices are specified by 24-bit floating point values, though it does appear that it processes 32-bit floats internally (this has not been proven though).

You can turn off the T&L pipeline and put it in "through mode". In this mode, vertex data just goes straight through from the vertex buffer to the rasterizer, without any clipping or transform.

Vertex formats have a fixed component order. The following component formats are supported:

- Color:
  - 16-bit: RGB565, RGBA5551, RGBA4444
  - 32-bit: RGBA8888 (32-bit)
- Normals: 8-bit signed, 16-bit signed, 32-bit float
- Positions: 8-bit signed, 16-bit signed, 32-bit float
- Bone weights: 8-bit unsigned, 16-bit unsigned, 32-bit float
- Texture coordinates: 8-bit unsigned, 16-bit unsigned, 32-bit float

Due to the limited range of 8-bit and 16-bit texture coordinate formats, you can specify an additional offset and scale as floating point numbers to stretch them to the range you need.

Note that the unsigned fixed point formats have the same scale factor as the signed ones. This means that an 8-bit texture coordinate is divided by 128.0, not 256.0, before being fed into the scale/offset, and thus can represent numbers from 0.0 to 2.0.

In through mode, positions behave a bit differently (and normals and bone weights are of course not used). They then specify integer pixel screen coordinates.

### Framebuffers and memory

The PSP GPU can draw to four different color framebuffer formats: RGBA5551, RGBA565, RGBA4444, RGBA8888. It only supports a single depth buffer format, 16-bit, although it's laid out differently in memory depending on whether the color format is 16-bit or 32-bit. This matters when games try to read from it. More information in [the page about image formats](/docs/psp-hardware/image-formats).

A game has full control over video memory, and can alias/cast the various image formats on top of each other - so you can render to a 16-bit "RGB565" target for example, and texture from it as if it were a paletted texture, even though it isn't. As we will see, this allows for some really cool tricks, like [Burnout's lens flare](/blog/lens-flare-burnout-dominator).

The GPU can only draw into VRAM, but it can read textures from regular RAM in addition to VRAM, at a slightly reduced rate. This means that games don't really need to stream textures to VRAM like on the PS2.

Memory performance has a few quirks. For example, it's profitable to draw 2D images in 64- or 32- pixel wide vertical strips, depending on color depth and texture format. This is why screen clears are usually performed as a series of vertical strips.

### Primitives

The PSP supports the usual set of primitives: LINE, LINE_STRIP, TRIANGLE_LIST, TRIANGLE_STRIP, TRIANGLE_FAN. In addition, a  screen-space RECTANGLE primitive type is suppported, which like lines are specified by just two points. The result is a filled screen-space rectangle, with UVs rotated according to the relative positions of the two points.

### Curve rendering

It's got a bezier/spline curve drawing unit! It can draw rectangular bezier patches and b-spline patches with knots, with some limitations. It's underused by games, mainly used by games with large landscape things like some snowboarding games, Test Drive, Pursuit Force and a few others. There are also some early games that use it for drawing simple 2D rectangles, for no good reason, like Puzzle Bobble. Additionally, Loco Roco uses it.

### Clipping

The PSP does not have a full clipper unit. It does have a near plane clipper (that behaves a bit strangely) but the other sides of the view frustum need to be handled by software.

Since the clipper only has one plane, it relies on guardbands for clipping on the sides. Unfortunately this guardband is not infinite - triangles that reach outside a virtual 4096x4096 viewport are discarded. And some games rely on that, so it must be emulated (notoriously, the TOCA racing games have horrible artifacts if you don't).

### Raw display list access

Games write display lists directly into RAM, generally using a SDK-provided library but many games bypass it entirely and even load raw display lists directly from disk.

Display list commands are all 32 bits in size, out of which the upper 8 bits is a command number, and the remaining 24 bits represent data. This means that commands that load floating point values such as matrix coefficients really can only specify 24 bits of payload and not full 32-bit floats for example. This is solved by simply chopping the lower 8 bits off of the mantissa of regular floats - usually, there's still enough precision. All transform matrices and most light parameters (except the ones that are 8-bit) are in this 24-bit float format.
