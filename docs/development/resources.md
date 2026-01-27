# Resources

## Links about developing for the PSP

- [PPSSPP on GitHub](https://github.com/hrydgard/ppsspp)
- [PPSSPP Development Wiki](https://github.com/hrydgard/ppsspp/wiki)
- [PSP Dev Wiki](https://www.psdevwiki.com/psp/)
- [forums.ps2dev.org](https://forums.ps2dev.org/) Also has PSP information.
- [PSP Homebrew Community Discord](https://discord.gg/bePrj9W) (invite link)
- [Minimalist PSP SDK for Windows](https://github.com/pmlopes/minpsp)
- [PSP Archive](https://psp-archive.github.io/) - a website about emulation on the PSP

## Useful reporting links

These are links to report.ppsspp.org, where we collect automatic bug reports.

- [Shader compilation errors](https://report.ppsspp.org/logs/kind/970)

## General programming resources

This is just a collection of interesting things.

### Floating point tricks

- [UNORM and SNORM to float, hardware edition](https://fgiesen.wordpress.com/2024/12/24/unorm-and-snorm-to-float-hardware-edition/)
- [Exact UNORM8 to float](https://fgiesen.wordpress.com/2024/11/06/exact-unorm8-to-float/)

### Software optimization

- [Agner's Software Optimization Resources](https://www.agner.org/optimize/)
- [The most efficient way to do nothing (optimize sleeps in RPCS3)](https://www.youtube.com/watch?v=3dkN-6TJNHs)

### Rasterization

- [Optimizing software occlusion culling / rasterization](https://fgiesen.wordpress.com/2013/02/17/optimizing-sw-occlusion-culling-index/) Blog series by Fabian.
- [Rasterizer Notes by nlguiellemot](https://nlguillemot.wordpress.com/2016/07/10/rasterizer-notes/)

### Meshes

- [Half-edge based mesh representations](https://fgiesen.wordpress.com/2012/02/21/half-edge-based-mesh-representations-theory/)

### SIMD resources

#### SSE gap filling

- [SSE - mind the gap!](https://fgiesen.wordpress.com/2016/04/03/sse-mind-the-gap/)
- [SSE sign extension tricks](https://mastodon.gamedev.place/@rygorous/109799623402856305)

#### AVX2 and AVX-512

- [Why is AVX-512 useful for RPCS3?](https://whatcookie.github.io/posts/why-is-avx-512-useful-for-rpcs3/) (by whatcookie)
- [No, AVX-512 is power-efficient in RPCS3](https://www.youtube.com/watch?v=N6ElaygqY74) (also by whatcookie)
- [More wacky whatcookie stuff](https://www.youtube.com/watch?v=40tyEVx_umY&t=360s)
- [Zen5 AVX-512 benchmarking](https://www.numberworld.org/blogs/2024_8_7_zen5_avx512_teardown/)
- [MNIST in AVX-512 assembly](https://github.com/mohammad-ghaderi/mnist-asm-nn)
- [AVX-512 use cases](https://albertvilella.substack.com/p/intels-avx-512-use-cases-part1)
- [The least interesting thing is the 512bit width](https://mastodon.gamedev.place/@rygorous/110572829749524388)
- [Searching with AVX-512](https://gab-menezes.github.io/2025/01/13/using-the-most-unhinged-avx-512-instruction-to-make-the-fastest-phrase-search-algo.html)
- [Old MS AVX tutorial](https://devblogs.microsoft.com/cppblog/accelerating-compute-intensive-workloads-with-intel-avx-512/)
- [Vector filtering, AVX2 vs AVX-512](https://quickwit.io/blog/simd-range)
- [Mask register details on Intel](https://travisdowns.github.io/blog/2019/12/05/kreg-facts.html)
- [Unexpected uses for Galois field instructions](https://gist.github.com/animetosho/d3ca95da2131b5813e16b5bb1b137ca0)
- [GF2P8AFFINEQB: int8 shifting](https://wunkolo.github.io/post/2020/11/gf2p8affineqb-int8-shifting/)

#### NEON tricks

- [Porting SSE movemask optimizations to ARM64 NEON](https://developer.arm.com/community/arm-community-blogs/b/servers-and-cloud-computing-blog/posts/porting-x86-vector-bitmask-optimizations-to-arm-neon) (magic line: `uint64_t bitfields = vget_lane_u64(vreinterpret_u64_u8(vshrn_n_u16(mask, 4)), 0);`)
- [Coding for NEON - tutorial series from 2013](https://developer.arm.com/community/arm-community-blogs/b/architectures-and-processors-blog/posts/coding-for-neon---part-1-load-and-stores)
- [NEON vs SSE](https://blog.yiningkarlli.com/2021/09/neon-vs-sse.html)

### When can you start require feature X?

#### x86-64 CPUs that **do not** support SSSE3

SSSE3 has the nice instruction pshufb, which is useful for various format conversion operations and a lot more. There are very few x86-64 CPUs left that do not support SSSE3.

| Vendor | Models Lacking SSSE3 | Notes |
|-------|----------------------|------|
| **AMD** | *K8 family only* (first-gen AMD64):<br>• Athlon 64<br>• Opteron 1xx/2xx/8xx<br>• Turion 64<br>• Sempron (AMD64 variants) | Produced ~2003–2007. All **Phenom (K10)** and later support SSSE3. |
| **Intel** | *Pre-Penryn Core era*:<br>• Early Core Solo/Duo (Yonah – mostly 32-bit)<br>• Some first-gen Core 2 (early Merom/Conroe steppings) | SSSE3 widely deployed starting with **Core 2 refresh / Penryn (~2006–2007)**. |
| **Via** | Early VIA Nano | Very rare in the wild. |

SSE4.2 is also useful, but there are still some CPUs around that don't support it, so we probably don't want to require it.

#### x86-64 CPUs that **do not** support SSE 4.2
*(latest non-supporting generations)*

| Vendor | Last Generation Without SSE4.2 | Models / Examples | Notes |
|-------|-------------------------------|------------------|------|
| **Intel** | **Penryn (Core 2 Refresh, ~2007)** | Core 2 Duo E8xxx / T9xxx, Core 2 Quad Q9xxx, Xeon 51xx / 53xx | Penryn **has SSE4.1 only**, no SSE4.2. SSE4.2 starts with **Nehalem (Core i7-900 series, 2008)**. |
| **AMD** | **Phenom II / K10 (~2008–2010)** | Phenom II X4/X6, Athlon II, Opteron 23xx/83xx | K10 supports **SSE4a only** (not SSE4.1/4.2). Proper SSE4.2 starts with **Bulldozer (FX-8000 / Opteron 6200, 2011)**. |
| **VIA** | **Early VIA Nano** | VIA Nano 2000-series | Later Nano C-series implements SSE4.2-like crypto/string ops; early ones do not. |
