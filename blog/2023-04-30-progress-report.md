---
slug: progress-report-spring-2023
title: Progress Report - spring 2023
authors: hrydgard
tags: [ppsspp]
---

## PPSSPP Progress Report - Spring 2023

Welcome to the first official Progress Report! I've long enjoyed reading Dolphin's progress reports, but it turns out to take quite the effort to write one. So, it's not likely that these will be monthly going forward, but either way, here's the first one. More likely these will accompany each official new official release. Essentially, these will be longer form versions of the [release notes](/news/).

PPSSPP has been an ongoing project for over 11 years now, but work never ends! The new version, 1.15, is another pretty good upgrade. The main themes are stutter fixes and quality-of-life improvements, and obviously some compatibility fixes and performance improvements as usual. Additionally, macOS is now an officially supported platform, and support has been improved substantially.

So go [Download PPSSPP 1.15](/download/)! New: There's now an official Mac build!

Please note that the Android version on Google Play will as usual roll out slowly over multiple days in order to catch any bad crashes early, so you may have to wait a few days before you receive the update.

## 1.15 - top improvements

### Reduction of shader compilation stutter

PPSSPP has long had problems with stutter caused by shader compilation, and it had reached a point where it was stuttering enough to nearly be unplayable initially on some devices with very slow shader compilers, such as devices with the PowerVR GE8320.

Two changes have been made to improve on this: First, the sheer amount of different shader variants was a bit excessive. We'd compile separate shaders for every little shading state bit, such as the double-color-value or fog flags. Now, a bunch of these are controlled by uniform values instead, so the shader count has gone down by as much as half or more in many games. We still have more variants than we really need, but the problem is much smaller now. This change affects all rendering backends.

Additionally, in Vulkan, we now do shader compilation (as in pipeline creation) in parallel on multiple threads, which is a huge improvement on some drivers like the ones for PowerVR GPUs (which are increasingly common in lower-end phones like Galaxy A21), a minor improvement on Adreno and generally neutral on Mali. The devices seeing the biggest improvements were also the slowest before, so definitely worthwhile.

### Texture replacement rework

In PPSSPP, you can install "[texture replacement packs](/docs/reference/use-texture-replacement)", containing higher resolution textures for a game. PSP games were made for a low resolution screen.

The texture replacement code has evolved over the years but had some lingering problems, like too much I/O on the main thread. It's now been substantially reorganized and partly rewritten, moving more work to background threads - which reduces stutters caused by loading new replacement textures by quite a bit. Additionally, support for efficient compressed texture formats has been added (KTX2 with Basis and UASTC, DDS with BC1-7), along with built-in mipmap support ([#17134], [#17092], [#12059]). Using these can further speed up loading and improve quality while also reducing the memory use.

In connection with this, video memory use when loading large textures to VRAM has been optimized. This mainly helps when using texture replacement packs with very high-resolution textures, but will reduce the risk of running out of memory in general, which is one of the major remaining causes of reported crashes on Android.

### Display and post-processing features

Integer scaling can now be enabled. This ensures that the emulated PSP framebuffer is scaled to an even multiple of the rendering resolution. If you then enable "Nearest" filtering, you get nice square pixels, if you're after the retro look.

Additionally, a bug has been fixed where temporary buffers during applications of post filters were the wrong size if we detected oversized framebuffers in games, causing little glitches.

### Rendering fixes and features

* We can now texture-sample from render targets that have been cast to CLUT formats, using a palette that comes from yet another render target. Fixed some special effects: [#8390](https://github.com/hrydgard/ppsspp/issues/8390). See the [GPU image formats page](/docs/psp-hardware/gpu/image-formats) for more information about the PSP's CLUT functionality and what can be done with it. Additionally, we now support a few additional CLUT texturing modes, which fixes night vision and other effects in the SOCOM games.

* Xiaomi Poco C40 phones, which are quite widespread in some countries, only expose a software Vulkan device, which is missing some mandatory features. This caused PPSSPP's UI to render incorrectly and slowly, and even worse in-game. I have corrected the Vulkan device detection code to reject this software device - which simply means these phones will fallback to using OpenGL instead, which is reasonably functional on them.

* Fixed a slowdown in Naruto Heroes: [#16733](https://github.com/hrydgard/ppsspp/issues/16733). This is a very typical case of unnecessary readbacks. The PSP has a "block copy" command to copy around arbitrary GPU memory between RAM and VRAM. PPSSPP designates areas in VRAM as framebuffers if they have been previously rendered to, and copies between two framebuffers can be simply converted to a GPU copy on the host. However, if data is copied to "new" VRAM or to RAM, by default we assume that the game will try to read the copied data with the CPU. This means we have to do a very costly readback. Often it actually doesn't need the data on the CPU, though, so we added a per-game compatibility option where we automatically create framebuffers covering the destination area of memory. If the game later simply textures from it, this becomes much, more more efficient than a readback from the GPU. We enabled this for Naruto Heroes.

### The curious case of skewed input

It was noticed that Daxter and the two God of War games (all three from the studio Ready At Dawn) both rotate the analog stick input by 15 degrees before passing it to the game's internal control system. This makes the analog stick feel better aligned on the real PSP and likely good for ergonomics, but is not appropriate for an emulator that takes many kinds of different inputs, so automatic pre-rotation in the other direction was added to compensate. Daxter now walks correctly along the cardinal directions as expected, and Kratos does too in the God of War games. See [#17020](https://github.com/hrydgard/ppsspp/issues/17020).

### Tilt control has been improved

The Android-only tilt (accelerometer) input feature has not been maintained nor tested for a long time, so it was simply time to go through it and fix it up, which is now done. The new behavior works much better and is more intuitive to configure, thanks to an updated configuration dialog.

## Regressions fixed

* Thrillville rendering corrected ([#17169])
* A major performance regression in Dante's Inferno has been fixed ([#17032])
* Mipmaps are now actually used with texture replacement, if provided ([#17144])

## Other changes

* Depth readback added, fixing lens flares in Syphon Filter (at a perf cost.. turn off GPU readbacks for the old behavior) ([#16907], [#16905])
* Enabling rewind savestates (automatic savestates every N seconds) no longer slow down gameplay noticeably.
* Software renderer is even more playable, after another round of optimizations and fixes by \[Unknown\]
* fp64 has worked on [VFPU](/docs/psp-hardware/cpu/allegrex-overview) accuracy. It's not yet all enabled, though.
* Multiple fixes for various depth clipping issues ([#17055](https://github.com/hrydgard/ppsspp/issues/17020), more)
* VR has gained some new features (top-down perspective), new control options, and various fixes, thanks Lubos!
* Several large code cleanups and refactors have been performed across the code base, to make future changes easier.
* The RISC-V JIT compiler has been improved by \[Unknown\]. Future-proofs the emulator a bit!
* New app icon ([#11996]), assorted bugfixes ([#16988], [#17017], more)
* And [much more](/news/2023-04-30-ppsspp-1.15)!

## Go get it!

Go [download](/download/) 1.15 now to benefit from all this!