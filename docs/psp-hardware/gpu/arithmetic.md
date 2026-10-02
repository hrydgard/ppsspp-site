---
position: 7
---
# GE arithmetic

The GE doesn't compute with IEEE floats, or with any single format. Each stage of the pipeline has its own small set of operations, and each of them rounds in its own way, nearly always by truncating toward zero. This page collects those operations. The pages on the [vertex pipeline](/docs/psp-hardware/gpu/ge-vertex-pipeline), [lighting](/docs/psp-hardware/gpu/lighting), [curves](/docs/psp-hardware/gpu/curves) and the [raster pipeline](/docs/psp-hardware/gpu/raster-pipeline) describe where they're used.

Everything here was measured on a PSP with small hand-built display lists, reading results back from the framebuffer and the depth buffer, and is reproduced bit for bit by PPSSPP's software renderer. The reference implementation is `GPU/Software/GEMath.cpp` and `GEMath.h`, with unit tests in `unittest/TestGEMath.cpp`.

## float24

Display list commands have 24 bits of payload, so floating point parameters (matrix entries, light positions, viewport scale and offset, fog parameters, texture scale and offset) are 32-bit floats with the low 8 mantissa bits cut off: 1 sign bit, 8 exponent bits and 15 fraction bits, a 16-bit significand. This page calls that format float24.

Most of the pipeline also *computes* at this precision. Vertex positions are truncated to float24 on input, and most intermediate results are truncated back to float24.

- Every rounding truncates toward zero, unless noted otherwise.
- Denormals behave as zero.
- Sign and magnitude are kept separately, so truncating a negative value moves it toward zero, not toward minus infinity.

## The adder

The GE's adder has no guard bits. Both operands are first truncated to the precision of the larger one: with `e` the larger of the two exponents, each operand loses everything below `2^(e-15)`. The truncated values are then added exactly. The sum may need a 17th significant bit (when it carries), and some uses truncate it back to float24 while others pass it on as is.

```text
add(a, b):
    e = max(exponent(a), exponent(b))
    a' = truncate a to a multiple of 2^(e - 15)
    b' = truncate b to a multiple of 2^(e - 15)
    return a' + b'
```

So adding a small value to a large one loses the small value's low bits, *both* of them truncated, not the result rounded. This shows up wherever a translation, offset or constant is added: the viewport center, the texture offset, the fog end, the `+ 1` of shade mapping, and the near-plane clipper's `z + w`.

## Products

A product of two float24s is formed exactly, then truncated at a fixed bit weight: `2^(ea + eb - 15)`, where `ea` and `eb` are the operands' exponents. When the two significands multiply to 2 or more, the product keeps a 17th significant bit; below 2 it keeps 16. The product isn't renormalized to float24 on its own: its lowest bit stays at that weight until it's summed.

## Row sums

A matrix row `x·m0 + y·m1 + z·m2 + t` isn't a chain of additions. It's one multi-operand sum:

1. Each product is formed as above. The translation `t` counts as a term with its lsb at `2^(ex(t) - 15)`.
2. Every term is truncated to a multiple of the largest lsb among the terms, that is, to the precision of the largest term.
3. The truncated terms are added exactly.
4. The sum is truncated to float24.

There is no evaluation order, and pairwise additions in any order don't reproduce the hardware.

The VFPU's dot product (`vdot`) works on the same idea: its terms are aligned to the largest one, truncated, and summed exactly as integers, with no order between them. The VFPU's version is more careful, though. Its products keep two extra bits and a sticky bit (rounding to odd), so what the truncation drops still nudges the result, and the final sum is rounded to nearest rather than truncated. The GE does without all of that. See fp64's [`vfpu_dot_reference`](https://github.com/hrydgard/ppsspp/blob/a2f4ce214f224c6c26b3319409ec50205ff8253d/Core/MIPS/MIPSVFPUUtils.cpp#L763-L849) in PPSSPP, and [#21070].

The same sum is used for:

- the vertex transform (all four rows),
- combining the world, view and projection matrices (each entry of `(W·V)·P` is a row sum),
- dot products (a row sum without the translation): `N·L`, `N·H`, squared lengths,
- the texture matrix,
- the vector from a vertex to a light (see [lighting](/docs/psp-hardware/gpu/lighting)),
- the cross product of Bezier tangents.

## The reciprocal

The perspective divide doesn't divide. `z/w` is `z · R(w)`, truncated, where R is the GE's reciprocal of the significand. R is linearly interpolated over 128 segments of the 15-bit mantissa `i`:

```text
s = i >> 8          (segment, 0..127)
x = i & 255         (position within the segment)
q = (64 · B[s] + 63 + M[s] · x) >> 7      (1 / 1.i in units of 2^-16)
```

B runs from 131072 (1.0) down to 65793 and M from -254 to -64. M is the chord slope rounded in 124 of the 128 segments, and B is within a couple of half-ulps of the exact value at the segment start. There's no simple generating rule, so it's a stored table (`geRecipSegments` in GEMath.cpp). It's also not the VFPU's `vrcp`, which interpolates quadratically (see the [VFPU blog post](/blog/vfpu-math-re)).

The same R serves the perspective divide of X, Y and Z, perspective-correct texture coordinates, the near-plane clipper's interpolation factor, and point-light attenuation.

## The reciprocal square root

Normalizing a vector uses a reciprocal square root with the same layout: two tables of 128 segments, one for even and one for odd exponents. For `d = 1.i · 2^E`:

```text
s = i >> 8,  x = i & 255
table = (E even) ? 1/sqrt(1.i) : 1/sqrt(2 · 1.i)
q = (64 · B[s] + 63 + M[s] · x) >> 7      (units of 2^-16)
rsqrt(d) = q · 2^-16 · 2^-floor(E/2)
```

To normalize `v`, the GE computes `d2 = v·v` as a row sum, then each component is `float24(c · rsqrt(d2))`. A vector's length, where needed, is `d2 · rsqrt(d2)`. A zero vector stays zero: there's no fallback direction.

## The setup reciprocal

Triangle setup divides by the doubled triangle area (and line setup by the line's major length). It uses a *third* table, separate from R and finer: 256 segments, indexed by the top 16 bits of the integer's significand.

```text
e = floor(log2(absDet))
index = (absDet << 16 >> e) - 65536       (16 bits)
s = index >> 8,  x = index & 255
q = (2 · K[s] + M[s] · x) >> 8           (about 2^(e + 16) / absDet)
```

Nearly every K is `floor(2^39 / (65536 + 256 s) / 16) · 16`, the exact segment start rounded down to a multiple of 16. The only exception is segment 0, at 8388735, which is 127 above 2^23. M runs from -257 to -64; it's mostly the chord slope, but segments 105 and 106 both have -128. The table was fit to probe data first. Where the probes left a segment's K loose, some games turned out to need the formula value: Blade Dancer's depth needed it for segment 39, and a single Gouraud-shaded probe triangle for segment 161.

## Planes

The rasterizer interpolates depth, color, fog and texture coordinates with fixed-point planes, not with per-pixel floating point:

- A vertex value `v` and the vertices' screen positions (12.4 fixed point) give the plane's numerators `n` through the usual cross products.
- The gradients are `floor(n · q / 2^(e + 2))`, with `q` and `e` from the setup reciprocal of the doubled area. That leaves 14 fraction bits per subpixel.
- The plane is anchored at one vertex (see the [raster pipeline](/docs/psp-hardware/gpu/raster-pipeline) for which), and a pixel's value is the floor of the plane at the pixel center.

Lines interpolate color and depth the same way along their major axis, with the setup reciprocal of the major length.

Texture coordinates go into their planes as 15-bit integers. The three vertices' values `s`, `t` and `q` are each scaled to the largest exponent among the three and truncated to 15 bits. A lone value keeps 15 significant bits, and a small UV range on a large base value collapses as those 15 bits run out.

## Texture coordinates per pixel

In transform mode, a pixel's texture coordinate is `u = s_pixel · R(q_pixel)`, kept to 24 significant bits and truncated: a float32 product without rounding, not a float24. In through mode there's no divide, and the coordinate is the 15-bit plane value directly.

A coordinate becomes a texel position in 1/16 texel by truncating toward zero, so a negative coordinate comes out 1/16 higher than flooring would give. Nearest filtering takes that position `>> 4`. Bilinear filtering uses `(k - 8) >> 4` for the texel and `(k - 8) & 15` for the weight.

## A float-bits logarithm

Mip level selection takes logarithms by reading a float's bits: the exponent and the top 4 mantissa bits, a piecewise linear `log2` floored to 1/16.

```text
log16(x) = ((float bits of x) >> 19 & 0xFFF) - 127 · 16
```

## The lighting pow

Specular, powered diffuse and spot exponents all use the same pow, Mitchell's approximation on float bits:

```text
log2(x) = k + (x / 2^k - 1)        for x in [2^k, 2^(k+1))
exp2(y) = 2^floor(y) · (1 + frac(y))
pow(x, e) = exp2(e · log2(x))
```

Both are straight lines between powers of two, exact at the powers of two. In bits, `log2(x)` is just `bits(x) - bits(1.0)`. Details that matter:

- The exponent keeps only the top 4 bits of its mantissa, so 5.1 acts as 5.0, 12.7 as 12.5, 1.1 as 1.0625 and 0.3 as 0.296875.
- The product `e · (bits(x) - bits(1.0))` is exact, then truncated toward zero to units of 16 in the bits (19 fraction bits of the log).
- An exponent of 0 or less gives 1.

Compared to a true pow, the result is never brighter and up to 10-30 steps darker through the falloff at exponents 4-64, so highlights are tighter.

## 8-bit color products

Color arithmetic multiplies 8-bit values as if each were a fraction of 256, with an extra half step on both sides:

```text
mul(a, b) = ((2a + 1) · (2b + 1)) >> 10
```

This one formula is used for:

- **Lighting**: each light's color times the material color, and then each term scaled by its factor (`N·L`, the specular power, attenuation, the spot term). A factor `f` in 0..1 first becomes an 8-bit `s = floor(256 · f)` (256 for exactly 1), so scaling by 1 leaves the color unchanged.
- **Blending**: each of the two terms is `mul(color, factor)`, and the result is clamped once at the end. The doubled inverse factors (`1 - 2·α`) are `255 - 2α` without a clamp: for α ≥ 128 the factor is negative, and its term is subtracted instead.

## Texture function arithmetic

- **Modulate** is `(t · (c + 1)) >> 8` per channel. With color doubling it's `(2t · (c + 1)) >> 8`, and the secondary (specular) color is doubled too.
- **Bilinear filtering** uses 4-bit weights. The two horizontal lerps `(a · (16 - f) + b · f) >> 4` come first and are truncated to 8 bits, then the vertical one, also `>> 4`.
- **Fragment alpha** with texture alpha is `((a_prim + 1) · a_tex) >> 8`.
- **16-bit texels** are expanded to 8 bits per channel first, then filtered exactly like 8888.

## Fog

The fog factor is computed per vertex, converted to 8 bits with `min(floor(256 · f), 255)`, and interpolated screen-linearly (see the [vertex pipeline](/docs/psp-hardware/gpu/ge-vertex-pipeline)). Fog is never perspective corrected.

## Curve lerps

Bezier and spline tessellation uses yet another format: 16-bit fixed point at the larger operand's exponent, with 8-bit parameters. See [curves](/docs/psp-hardware/gpu/curves).

## Clipped vertex colors

When the near-plane clipper makes a new vertex, its colors and fog (8 bits per channel) are `(a · (256 - t8) + b · t8) >> 8`, with `t8 = round(256 · t)`. Its position and texture coordinates use float24 products and the adder (see the [vertex pipeline](/docs/psp-hardware/gpu/ge-vertex-pipeline)).
