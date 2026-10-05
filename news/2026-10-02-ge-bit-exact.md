---
slug: ge-bit-exact
title: Cracking the PSP GPU's transform and raster
authors: hrydgard
tags: [blog]
---

## Introduction

After the [VFPU math functions](/news/vfpu-math-re) were cracked, I figured, let's try the same with the GE, the PSP's GPU. PPSSPP has a software renderer, mostly used as a reference, and for debugging. It's long been pretty close to accurate, but that isn't good enough for a reference, and some games really depend on the details: depth fighting in the distance, banding in fog, a seam in a sky box, a lens flare that reads back the depth buffer.

So I pointed Claude at it again, with a real PSP hooked up over USB like the last time, and let it write its own test programs for the hardware. It ran a bit over 170 experiments on the PSP. When it was done testing and applying fixes, the software renderer reproduces 125 of a set of 132 frame dumps from various games, bit-exact pixel by pixel against the same dumps played back on a real PSP.

Claude has extended my GE documentation with the findings from this project [GPU section of the docs](/docs/psp-hardware/gpu/ge-overview), starting with the new page on [GE arithmetic](/docs/psp-hardware/gpu/arithmetic).

The whole thing was done over about 2 days, with long breaks.

This is the [pull request](https://github.com/hrydgard/ppsspp/pull/22414) implementing all this.

Below is Claude's own writeup.

## Claude says (AI-written below)

The GE is a fixed-function GPU: transform and lighting, a clipper, a rasterizer, texturing and blending, all configured with a display list. That makes it a black box with a lot of inputs and only a few visible outputs, the color and depth buffers. The job was to explain every bit of those outputs, for any input.

The VFPU work had an oracle: fp64's tables gave the exact answer to every input, on a laptop, in a second. Here there was no oracle except the PSP itself. Every question had to be turned into a display list, run on the hardware, and read back.

### The setup

The tool for that is `geprobe`. It builds GE display lists on the host from a Python description, sends them to a small PRX on the PSP over PSPLink, and reads the framebuffer and the depth buffer back. A probe is a job: some vertices, some state, a readback.

The main difficulty is that the GE's outputs are narrow. A color channel has 8 bits and depth has 16, while the values inside the pipeline are floats with much more precision. So most of the work was designing readouts that carry the bits you want to see:

- **Points as samples.** A point primitive lights one pixel with its exact depth and color. Drawing a Bezier patch as points gives every tessellated vertex its own pixel.
- **Depth windows.** Setting the viewport so that a tiny range of z covers all 65536 depth values reads a transformed z down to its last bit.
- **Texture ramps.** A texture whose texel value is its own position, sampled with bilinear filtering, shows where a pixel's texture coordinate landed in 1/16 of a texel.
- **Using one unit to read another.** The texture matrix can take the normalized normal as its input. With the matrix scaling the interesting part up, the texture coordinate carries every bit of the GE's normalization. Shade mapping turns a light vector into a texture coordinate the same way.

The other half was a replayer. PPSSPP's frame dumps record everything a game sent to the GE for one frame. The pspautotests repository has a [tool](https://github.com/hrydgard/pspautotests/tree/master/utils/ppdmp-playback) that plays a dump on a real PSP and captures the result, so every dump became a test case: the software renderer's output against the PSP's.

### Floats, but not IEEE

*Details: [GE arithmetic](/docs/psp-hardware/gpu/arithmetic), [the vertex pipeline](/docs/psp-hardware/gpu/ge-vertex-pipeline).*

The first thing to fall was the number format. Display list parameters are 32-bit floats with the low 8 bits cut off, and it turned out the GE also *computes* in that format: a 16-bit significand, and truncation toward zero everywhere.

That alone wasn't enough, because how the operations are built matters as much as the precision:

- **The adder has no guard bits.** Both operands are truncated to the precision of the larger one before adding. So adding a small offset to a large value loses the offset's low bits, before the sum is ever rounded.
- **A matrix row is one sum.** No order of pairwise additions fits the data. The hardware forms the four products exactly, truncates each of them to the precision of the largest one, adds them exactly, and truncates once. The same unit combines the world, view and projection matrices before any vertex is transformed, and computes dot products, the texture matrix and the vector to a light. If that sounds familiar from the [VFPU post](/news/vfpu-math-re), it should: the VFPU's `vdot`, as fp64 [worked out](https://github.com/hrydgard/ppsspp/blob/a2f4ce214f224c6c26b3319409ec50205ff8253d/Core/MIPS/MIPSVFPUUtils.cpp#L763-L849), aligns and sums its terms the same way, with no order between them. It's the more careful design of the two, keeping guard and sticky bits on its products and rounding the final sum to nearest, where the GE just truncates.
- **There's no divide.** The perspective divide multiplies by a reciprocal from a table of 128 linear segments. Normalization uses a reciprocal square root with the same layout. Neither is the VFPU's interpolator, which is quadratic.

### A second reciprocal

*Details: [the raster pipeline](/docs/psp-hardware/gpu/raster-pipeline).*

The rasterizer was the biggest surprise. Depth isn't `z/w` per pixel, and it isn't walked along edges like on the PS2 either. Each triangle gets fixed-point planes for its depth, color, fog and texture coordinates. The setup divides by the triangle's area with a *third* table, finer than the first two: 256 segments of 16-bit indices.

Fitting that table took a while, and finishing it took games. The probes pinned most segments, but left a few with a window of possible values. Blade Dancer's depth needed one of them at the top of its window, and a single Gouraud-shaded test triangle, with 37 pixels off by one, needed another. Both landed on the same formula as nearly all the others: the exact reciprocal at the segment start, rounded down to a multiple of 16. The table's starting values are now that formula, except for the first segment.

With the planes, depth, colors and texture coordinates all fell into place, including details nobody would guess:

- the plane is anchored at the leftmost vertex, unless the long edge is the triangle's right side;
- the mip level uses one q per 4-pixel span, taken at the span's second pixel in the direction the row is walked;
- a 1:1 bilinear sprite only lands exactly on texel centers when its area is a power of two, since the gradient is a fixed-point reciprocal of the area.

### Lighting

*Details: [GE lighting](/docs/psp-hardware/gpu/lighting).*

Lighting had a known bug report behind it: the hair shine in iDOLM@STER SP looked wrong ([#12376]). The specular half vector uses a viewer direction taken from the view matrix's third column, not (0, 0, 1). The specular power is Mitchell's approximation, straight lines between powers of two, with the exponent cut to 4 mantissa bits. And light colors are scaled by 8-bit factors with `((2x + 1)(2s + 1)) >> 10`, which turns out to be the same product the blender uses.

The last lighting bug came from a single vertex in Syphon Filter ([#13568]), whose spot factor was 33 on the PSP and 32 in the emulator. The GE never forms a world-space position for lighting. The vector to the light is one row sum, of the light position minus the world translation and the model position times the world matrix.

### Curves

*Details: [curves](/docs/psp-hardware/gpu/curves).*

Bezier and spline patches turned out to be evaluated without floating point arithmetic, even though every control point is a float.

De Casteljau's algorithm evaluates a Bezier curve with nothing but linear interpolation. Take the four control points, lerp each neighboring pair at the parameter t, and you have three points; lerp those, and you have two; lerp once more, and you're on the curve. A patch does that for each column of control points, then once more along the row of results. So the whole evaluation is a few dozen lerps, `a + (b - a) · t`.

The GE does each lerp as integer arithmetic. It looks at the two operands' exponents and takes the larger one, then writes both operands as 16-bit integers in units of that exponent's lowest bit, truncating whatever doesn't fit. The parameter t is an 8-bit fraction, k/256. The lerp is then `A + floor((B - A) · k / 256)` on those integers, and the result is a float again, at that same exponent. Lerping 1.5 and 0.001, for example: at 1.5's exponent, the lowest bit is 2^-15, so 0.001 becomes 32 units (0.0009765625) before anything else happens, and the result can't carry more precision than that. The fixed point is chosen anew for every lerp, which makes it a kind of block floating point: an alignment shift, an integer multiply by k, and a truncating shift, with no normalization in between.

At k = 0 and k = 256 the lerp passes an operand through untouched instead of truncating it, and that detail decides the normals at patch edges, which come from the tangents. With all of that, Coded Arms ([#21391]), Pursuit Force ([#11216]), Test Drive ([#21763]) and LocoRoco have exact depth.

### Oddities

*Details: [the raster pipeline](/docs/psp-hardware/gpu/raster-pipeline).*

A few things are hard to call anything but quirks:

- A triangle taller than about 2730 pixels lights extra pixels along its long edge, one per 4-pixel span, as if a 17-bit field overflowed.
- The REGION1 register, which looks like a clip rectangle, translates the drawing. For odd x offsets it also reverses each group of 4 pixels.
- A pixel drawn past the framebuffer's stride lands at the start of the next row.

### The texture cache

*Details: [the texture cache and self-texturing](/docs/psp-hardware/gpu/texture-cache).*

The GE reads textures through an 8 KB cache that the GE's own drawing doesn't update, which is normal for a GPU of this generation. It only shows when a game textures from the buffer it's drawing to, as bloom and blur passes do. A texture that fits in the cache stays there until the next TEXFLUSH, across draws, so a game that blurs a small buffer onto itself several times reads the first pass's input in all of them (Final Fantasy Type-0, [#20104]). Larger self-textures are mostly read as they were before the draw, in 8-row blocks loaded the first time a primitive needs them.

### Bugs that weren't the GE

Many differences between the emulator and the PSP turned out not to be the GE at all:

- **The replayer had bugs.** Its list buffer could wrap and let the GE run a lap of stale commands, which drew a screen-filling skinned triangle that wasn't in the game. It also didn't wait for the GE to finish before CPU and DMA writes to VRAM. Fixing those made several "unexplained" dumps exact without touching the emulator.
- **The software renderer had a race.** I spent a probe on a theory that the GE caches recently written pixels, to explain a blur in Tokimeki Memorial 4 ([#6379]). The probe showed the PSP simply draws in order. The rows that differed changed from run to run in the emulator, because two of its rendering threads raced over pixels past the buffer's stride.
- **Merging primitives changes the result.** The software renderer merged adjacent sprites, and drew rectangles made of two triangles as one sprite, for speed. Since every primitive has its own planes, both changed which texels were sampled.

### What came out

| | |
|---|---|
| Game frame dumps exact against the PSP | 125 of 132 |
| Framedump tests (`frametests.py`) exact | 29 of 30 |
| Hardware probes | about 174 |

The rest are known cases:

- **Swizzled depth reads.** These dumps read the depth buffer through its swizzled mirrors, which we deferred. Fixing those is next.
- **Lazy texture loading.** Two self-texturing effects depend on the GE loading textures in 8-row blocks, which isn't modeled yet.

Three dumps used to hang the PSP's replayer, which looked like a GE problem and wasn't: old dumps packed their data unaligned, which the PSP's CPU faults on, and one restored a CLUT load from an address that was only valid in the game. With the replayer fixed, all three are exact.

All of the arithmetic lives in one file, `GPU/Software/GEMath.cpp`, with unit tests. A selection of the experiments became pspautotests, [`gpu/exact`](https://github.com/hrydgard/pspautotests/tree/master/tests/gpu/exact), recorded on a PSP, so a change that moves the renderer away from the hardware shows up in the tests.

### Lessons

- **Design the readout first.** Each probe was only as good as the number of bits it could get out of the GE. Most of the breakthroughs came from a new way to read something out, not from a new hypothesis.
- **Fit to the probes, then to the games.** A table fitted to synthetic tests is right where the tests looked and loose everywhere else. Game dumps found the loose spots.
- **Suspect the pipeline around the hardware.** The replayer and the emulator's own threading each produced differences that looked like GE behavior.
- **The hardware is cheap, in the hardware sense.** Whenever a hypothesis needed per-pixel floating point math, it was wrong. The right answers were always the cheap ones: tables, truncation, narrow fields, shared units.
- **Be nice to the PSP.** A readback that's too big or two jobs at once wedges it until someone resets it by hand.
