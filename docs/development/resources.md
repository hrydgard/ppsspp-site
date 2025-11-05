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

### Rasterization

- [Optimizing software occlusion culling / rasterization](https://fgiesen.wordpress.com/2013/02/17/optimizing-sw-occlusion-culling-index/) Blog series by Fabian.
- [Rasterizer Notes by nlguiellemot](https://nlguillemot.wordpress.com/2016/07/10/rasterizer-notes/)

### Meshes

- [Half-edge based mesh representations](https://fgiesen.wordpress.com/2012/02/21/half-edge-based-mesh-representations-theory/)

### SIMD resources

#### SSE gap filling

- [SSE - mind the gap!](https://fgiesen.wordpress.com/2016/04/03/sse-mind-the-gap/)
- [SSE sign extension tricks](https://mastodon.gamedev.place/@rygorous/109799623402856305)

#### AVX-512

- [Why is AVX-512 useful for RPCS3?](https://whatcookie.github.io/posts/why-is-avx-512-useful-for-rpcs3/) (by whatcookie)
- [No, AVX-512 is power-efficient in RPCS3](https://www.youtube.com/watch?v=N6ElaygqY74) (also by whatcookie)
- [Zen5 AVX-512 benchmarking](https://www.numberworld.org/blogs/2024_8_7_zen5_avx512_teardown/)
- [MNIST in AVX-512 assembly](https://github.com/mohammad-ghaderi/mnist-asm-nn)
- [AVX-512 use cases](https://albertvilella.substack.com/p/intels-avx-512-use-cases-part1)
- [The least interesting thing is the 512bit width](https://mastodon.gamedev.place/@rygorous/110572829749524388)
- [Searching with AVX-512](https://gab-menezes.github.io/2025/01/13/using-the-most-unhinged-avx-512-instruction-to-make-the-fastest-phrase-search-algo.html)

#### NEON tricks

- [Porting SSE movemask optimizations to ARM64 NEON](https://developer.arm.com/community/arm-community-blogs/b/servers-and-cloud-computing-blog/posts/porting-x86-vector-bitmask-optimizations-to-arm-neon) (magic line: `uint64_t bitfields = vget_lane_u64(vreinterpret_u64_u8(vshrn_n_u16(mask, 4)), 0);`)
