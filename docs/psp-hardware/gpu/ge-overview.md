# Overview

The PSP GPU, known as the GE (graphics engine), while at first glance looking like a conventional small fixed-function GPU not unlike something like the original Geforce (but without multitexturing support), is actually a quite strange and interesting little beast.

Unlike the PS2's GPU which is more of a pure rasterizer, it has a T&L (transform and lighting) vertex pipeline, so you give it world, view and projection matrixes with the vertex data, and it'll automatically get transformed and lit, if enabled.

It also has (mostly) conventional blend modes with a few special additions, and it supports all the regular depth and stencil operations.

However, it does differ from modern GPUs on modern OSs in a number of ways, some of which make it a bit tricky to emulate:

## Differences from PC GPUs

* Single texturing only, and no shaders, only a few simple combiner modes (add to vertex color, multiply by vertex color, alpha blend with vertex color, etc). Vertex transform is fixed-function, 4 lights supported (but more than 1 is rarely used).

* Color masking is not just per channel, you can set a full bitmask and control each individual bit being written or not.

* The extra blend modes are somewhat tricky to emulate - 2xSRC, 2xDST, and the special blend op ABSDIFF.

* A game has full control over video memory, and can alias/cast the various image formats on top of each other - so you can render to a R5G6B5 target for example, and texture from is as if it was a paletted texture, even though it isn't. As we will see, this allows for some really cool tricks, like [Burnout's lens flare](/blog/lens-flare-burnout-dominator). More information in [the page about image formats](/docs/psp-hardware/image-formats).

* It's got a bezier/spline curve drawing unit! It can draw rectangular bezier patches and b-spline patches with knots, with some limitations. It's underused by games, mainly used by games with large landscape things like some snowboarding games, Test Drive, Pursuit Force and a few others. There are also some early games that use it for drawing simple 2D rectangles, for no good reason, like Puzzle Bobble.

* Stencil buffers are not stored separately, and are not interleaved with the depth buffer. Instead the color alpha channel servers double duty as both stencil and alpha data. This means that if you specify that your framebuffer has the format R5G5B5A1, there really is only 1 bit of stencil, while with R4G4B4A4 you effectively have a 4-bit stencil buffer and with R8G8B8A8 your stencil buffer is the full 8 bits wide. And if you render using R5G6B5 format, there simply is no stencil buffer available.

* Display list commands are all 32 bits in size, of which 8 bits is a command number, and the remaining 24 bits represent data. This means that commands that load floating point values such as matrix coefficients really can only specify 24 bits and not a full 32-bit floats for example. This is solved by simply chopping the lower 8 bits off the mantissa of regular floats - usually, there's still enough precision. All transform matrices and light parameters are in this 24-bit float format.

* The clipper is single-plane (near-Z) and rather strange.

* Since the clipper only has one plane, it relies on guardbands for clipping on the sides. Unfortunately this guardband is not infinite - triangles that reach outside a virtual 4096x4096 viewport are discarded. And some games rely on that, so it must be emulated.

* You can turn off the T&L pipeline and put it in "through mode". In this mode, vertex data just goes straight through from the vertex buffer to the rasterizer, without any clipping or transform.

* There's support for a RECTANGLE primitive type, which like lines have two points, but result in a filled screen-space rectangle, with UVs rotated according to the relative positions of the two points.

* There's support for special "antialiasing lines", though not heavily used, so can be largely ignored during emulation.

* You can draw to same image you're texturing from, with limitations.

* Textures, vertex data and commands can be read directly from RAM, there's no need to copy them to VRAM (although doing so can improve performance somewhat).

* Performance has a few quirks. For example, it's profitable to draw 2D images in 64- or 32- pixel wide vertical strips, depending on color depth and texture format.
