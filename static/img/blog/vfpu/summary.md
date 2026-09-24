# VFPU transcendentals without tables: summary

**Result.** All eight table-driven VFPU functions are now computed from about 10 KB of coefficients. The functions are vrcp, vrsq, vsqrt, vexp2, vlog2, vsin, vcos and vasin. vnrcp, vrexp2, vnsin and vrot come along with them. Each is bit-exact with fp64's table-based code over all 2^32 float inputs. `assets/vfpu` (4.9 MB), the loader, `InitVFPU` and the fallback code are gone.

**Branch.** `vfpu-interpolator`, based on master, in the worktree `../ppsspp-vfpu`. It has two commits and is not pushed. The changed files syntax-check cleanly. I didn't do a full build or run the unit tests.

## What the hardware does

It uses one quadratic interpolator with a 128-entry coefficient table per function:

- The top 7 bits of a 23-bit index pick the segment. The low 16 bits, x2, go into the linear term in full.
- The squared term only sees the top 10 bits of x2, as a distance t from the middle of the segment. The squarer rounds t² up to a multiple of 256.
- The result is `v = c0 + floor(m·x2 / 2^17) + floor(n·ceil(t²/256) / 512)`, in ulps of a per-segment exponent, then truncated to 22 bits.
- The squared term is floored separately. That produces a one-ulp sawtooth, which is the fingerprint that gave the structure away.
- c0 is a whole number of ulps, m has about 18 bits, and n has 7 to 8 bits.

Per-function twists:

- **sin/cos.** Indexed from the top of the quarter wave. Each segment has its own exponent, and results are truncated to 4 of its ulps even when they fall into a lower binade.
- **asin.** Same rules. Its first segment is linear.
- **log2.** The result is `exponent + log2(1.m)`, truncated toward zero to 22 significant bits. Where that step is coarser than 2^-24, the coefficients lose low bits to match the step, and c0 absorbs the dropped part of the squared term. This also covers the region just below 1.0 that the table code special-cased, including the −0.

## Numbers

| | Before | After |
|---|---|---|
| Data | 4.9 MB of `.dat` files, loaded at runtime | 10.5 KB of `static const` tables |
| rcp / rsqrt / exp2 | 4.2 / 5.4 / 6.3 ns | 3.6 / 3.6 / 3.9 ns |
| log2 / sin / asin | 7.1 / 14.6 / 6.0 ns | 5.1 / 5.9 / 3.6 ns |
| vrot (sincos) | sin + cos: 7.8 ns | shared reduction: 7.0 ns |

These are single-threaded timings on an M-series Mac, random inputs, measured with the scratch harness rather than in the emulator.

## Follow-ups

- Run the unit tests and pspautotests, which I didn't run.
- The JITs call these functions for vsin, vcos, vexp2 and so on. The interpreter still uses plain `sqrtf` for vsqrt and vrsq, because `USE_VFPU_SQRT` is false. Switching it on would make cpu/vfpu/exact pass on the interpreter.
- The new hardware test cpu/vfpu/exact (on the `cpu-test-holes` work) is recorded but not committed in pspautotests.
