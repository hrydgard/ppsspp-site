---
position: 3
---
# GE lighting

The GE lights each vertex with up to four lights: directional, point or spot. Each light has ambient, diffuse and specular colors, and its "computation" is diffuse only, diffuse plus specular, or powered diffuse. The vertex gets two colors, primary and secondary (specular), which go on to the rasterizer.

The model looks like the standard fixed-function one, but several details differ from OpenGL's, and from what PPSSPP did until 2026. Everything below was measured on hardware and is bit exact in PPSSPP's software renderer. The operations (row sums, the reciprocal square root, the pow, the 8-bit products) are described on the [GE arithmetic](/docs/psp-hardware/gpu/arithmetic) page.

## Space

Lighting happens in **world space**:

- The normal goes through the world matrix itself, not its inverse transpose.
- The normal is never normalized on its own. Instead, `N·L` and `N·H` are each divided by the normal's length: `float24(dot(v, N) · rsqrt(N·N))`. With a scaled or sheared world matrix, that's what comes out exact.

## The light vector

For a directional light, L is the light's position vector. For point and spot lights, the GE never forms a world-space vertex position. The vector from the vertex to the light is one [row sum](/docs/psp-hardware/gpu/arithmetic#row-sums):

```text
L_i = rowsum(lpos_i - T_i, -x · W_xi, -y · W_yi, -z · W_zi)
```

The first term is the light position minus the world matrix translation, from the GE adder. The other terms are the model-space position times the world matrix. Forming the world position first and then subtracting, at any precision, doesn't match. (This was found through a single vertex in Syphon Filter ([#13568]), whose spot factor was 33 on the PSP and 32 in the emulator.)

L is then normalized with the GE's reciprocal square root. A zero-length L stays zero:

- It gives no diffuse.
- Its specular comes from the eye vector alone, since H = normalize(0 + V).

## Diffuse and specular

```text
diffuse factor  = max(N·L, 0)
powered diffuse = pow(N·L, exponent)
specular factor = N·L >= 0 ? pow(N·H, exponent) : 0
H = normalize(L + V)
V = normalize(third column of the view matrix)
```

V is the viewer at infinity along view-space +z, expressed in world space. PPSSPP used to take V = (0, 0, 1) in world space, which is only right with an unrotated view matrix. A local viewer and lighting in view space were both ruled out.

A zero normal gives no diffuse and no specular.

### The pow

`pow` is the GE's [Mitchell approximation](/docs/psp-hardware/gpu/arithmetic#the-lighting-pow). It's linear between powers of two, so highlights are tighter than a true pow would give: up to 10-30 steps darker through the falloff, with the same peak. Its exponent keeps only the top 4 bits of its mantissa, so an exponent of 5.1 acts as 5.0.

## Spot lights and attenuation

- The spot direction isn't normalized either: `cos = float24(dot(dir, L) · rsqrt(dir·dir))`, then `pow(cos, spotExponent)` with the same pow.
- A spot direction component with exponent 255 (infinity or NaN) acts as the largest value of its sign, so after the scaling the finite components vanish next to it: a direction of NaNs or infinities points along (1, 1, 1), and one of -NaNs or -infinities along (-1, -1, -1).
- A negative spot factor (a light pointing away, with cutoff -1) scales the light by 0. It doesn't subtract from the other terms.
- Point and spot attenuation is `1 / (a0 + a1·d + a2·(L·L))`, with `d = (L·L) · rsqrt(L·L)` and the GE reciprocal. The quadratic term uses the squared length that the normalization already computed, not `d·d`.

## Output

Each term starts from an 8-bit product of a light color and a material color, per channel:

```text
x = ((2l + 1) · (2m + 1)) >> 10
```

The products are light ambient × material ambient, light diffuse × material diffuse, light specular × material specular, and global ambient × material ambient.

A factor f in 0..1 then scales x as if it were a color. f is first quantized to `s = floor(256 · f)`, with s = 256 for exactly 1:

```text
scale(x, f) = ((2x + 1) · (2s + 1)) >> 10
```

Attenuation and the spot term are separate factors, applied one after the other, not multiplied together first:

```text
ambient  = scale(scale(x_a, att), spot)
diffuse  = scale(scale(scale(x_d, N·L), att), spot)
specular = scale(scale(scale(x_s, pow), att), spot)
```

The terms of all lights, the emissive color and the global ambient are summed and clamped to 255. With separate specular, the specular sum goes to the secondary color.

## Shade mapping

Shade mapping (texture coordinate generation mode 2, environment mapping) takes the texture coordinates from two lights, LS0 and LS1:

```text
S = (N·L0' + 1) / 2
T = (N·L1' + 1) / 2
L' = normalize(directional ? lpos : L)       (zero staying zero)
L' = normalize(L' + V)                      if that light is diffuse + specular
```

The `+ 1` uses the GE adder, and `N·L'` is divided by the normal's length as in lighting. This doesn't depend on lighting or the light being enabled, the exponent, the spot cone, or the texture scale and offset. PPSSPP used the light position as a direction for every light type, never the half vector, and (0, 0, 1) for a zero vector, so the hair shine in iDOLM@STER SP ([#12376]) came out wrong.

## Normals from elsewhere

- **Skinned normals** use the bone matrices like positions, without the translation (see the [vertex pipeline](/docs/psp-hardware/gpu/ge-vertex-pipeline#skinning)).
- **Bezier and spline patches** ignore the vertex normals and use the cross product of the surface tangents (see [curves](/docs/psp-hardware/gpu/curves)).

PPSSPP's software renderer does all of this in [`GPU/Software/Lighting.cpp`](https://github.com/hrydgard/ppsspp/blob/master/GPU/Software/Lighting.cpp). These tests in pspautotests cover this page: [`gpu/lighting/specular` and `gpu/lighting/shademap`](https://github.com/hrydgard/pspautotests/tree/master/tests/gpu/lighting).
