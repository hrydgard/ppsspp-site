---
position: 5
---
# GE curves: Bezier and spline patches

The GE can tessellate surfaces itself. A BEZIER command draws one or more bicubic Bezier patches from a grid of control points (4x4 per patch, sharing edges), and a SPLINE command draws a uniform cubic B-spline surface over an n x m grid of control points, with open or closed ends set per direction. The tessellation level per direction comes from a separate command, and the output primitive (triangles, lines or points) from PATCHPRIMITIVE.

Each tessellated vertex then goes through the normal [vertex pipeline](/docs/psp-hardware/gpu/ge-vertex-pipeline): transform, lighting and so on. This page is about how the GE gets those vertices.

The evaluation doesn't use floating point. It uses 8-bit parameters and fixed-point lerps. That's how it was measured: drawing patches with PATCHPRIMITIVE = points makes every tessellated vertex one pixel, with its exact depth, color and texture coordinate. Reading the depth through a narrow depth range window resolves positions to the last bit. PPSSPP's software renderer now evaluates patches bit for bit ([`GPU/Common/SplineCommon.cpp`](https://github.com/hrydgard/ppsspp/blob/master/GPU/Common/SplineCommon.cpp)). Coded Arms ([#21391]), Pursuit Force ([#11216]), Test Drive ([#21763]) and LocoRoco have exact depth with it.

## The parameter

The parameter t of tessellated vertex `i` out of `d` steps is `k/256`, with k rounded toward the middle of the patch:

```text
k = floor(256 · i / d)               when 2i <= d
k = 256 - floor(256 · (d - i) / d)   otherwise
```

So with d = 3, the points are at 0, 85, 171 and 256: symmetric around the middle.

## Bezier: de Casteljau in fixed point

The GE evaluates the patch with de Casteljau's algorithm: the four columns first, at v, and then the row of the four results, at u. Every step is a lerp, and the lerp is the interesting part:

1. Both operands are converted to 16-bit fixed point at the *larger* operand's exponent, truncated toward zero (sign-magnitude).
2. Then `r = A + floor((B - A) · k / 256)`.
3. At k = 0 the lerp passes A through unchanged, and at k = 256 it passes B. Lerping would truncate it to the other operand's precision. This pass-through applies at every level, not just the final result.

The result is exact as a float, with no further rounding. The form is symmetric, so neither end of the patch is "nearer".

- **Positions and texture coordinates** use this lerp.
- **Colors** use it on `(c << 7) | 0x7F` per 8-bit channel, and the result is `>> 7`.
- **Generated texture coordinates** (when the vertices have none) are `k/256` along each direction.
- **A vertex's texture coordinate** is then truncated to 15 significant bits, like one vertex of a triangle (see [planes](/docs/psp-hardware/gpu/arithmetic#planes)).

## Normals

When lighting or shade mapping needs a normal, the GE ignores the vertices' normals. It computes the normal from the surface:

```text
Tu = bc - ab            (the last de Casteljau level of the row, along u)
Tv = row(bc) - row(ab)  (the columns' last two points, each evaluated along u)
N  = Tu × Tv            (each component a row sum)
```

The differences use the [GE adder](/docs/psp-hardware/gpu/arithmetic#the-adder), and the cross product's components are [row sums](/docs/psp-hardware/gpu/arithmetic#row-sums). The patch-facing flag negates N, and lighting normalizes it as usual. Because the lerps pass operands through at k = 0 and 256, the tangents at the patch edges are the control-point differences themselves. Shade mapping uses these normals even with lighting off.

## Splines: de Boor

Splines use de Boor's algorithm on the knot vector `[start] 1 .. m-1 [end]`, where m = n - 3 is the number of segments:

- An open end repeats its knot four times: `0 0 0 0` or `m m m m`.
- A closed end keeps the unit spacing: `-3 -2 -1 0` or `m .. m+3`.

Each segment is tessellated like a Bezier patch, `t = k/256` with the same k. A segment's last point is evaluated as the next segment's first (k = 0).

Each blending factor `(t - K_i) / span`, in 1/256, is computed from the *nearer* knot:

```text
dl = 256 · (t - K_i)
dr = 256 · span - dl
factor = floor(dl / span)          when dl <= dr
factor = 256 - floor(dr / span)    otherwise
```

So 1/3 becomes 85 and 2/3 becomes 171, the same rounding toward the middle as the parameter.

Splines then use the Bezier lerp, color handling, normals and column-then-row order unchanged.

## Drawn as lines

With PATCHPRIMITIVE = lines, each quad of the tessellated grid is drawn as three lines, in a zigzag: its left edge from top to bottom, then its diagonal from the bottom left corner up to the top right, then its right edge from top to bottom. There are no horizontal lines. Since a line lights its start pixel and not its end pixel (see the [raster pipeline](/docs/psp-hardware/gpu/raster-pipeline#lines)), the directions decide which corner pixels are lit, and the order decides which line's color a shared pixel ends up with. Neighboring quads draw their shared edge twice.
