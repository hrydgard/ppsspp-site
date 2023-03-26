---
slug: new-web-site
title: Spring 2023 PPSSPP Progress Report
authors: hrydgard
tags: [ppsspp]
---

## PPSSPP Progress Report - Spring 2023

Welcome to the very first official Progress Report! I'm not going to pretend that these will be monthly or anything like that going forward, but here it is! More likely, like this one, these will come out along with new official releases. Essentially, these will be longer form versions of the [release notes](/news/).

PPSSPP has been an ongoing project for over 11 years now, but work never ends. The next version, 1.15, is another pretty big upgrade. The main themes are stutter fixes and quality-of-life improvements, and obviously some regular performance improvements and compatibility fixes as always.

Additionally, macOS support has been improved substantially.

## Improvements that are coming in 1.15

* Texture replacement has been rewritten, moving more work to background threads - there's now no I/O on the main thread, which reduces stutters caused by loading new replacement textures by quite a bit. Additionally, support for efficient compressed texture formats has been added (KTX2 with Basis and UASTC, DDS with BC1-7), along with built-in mipmap support (#17134, #17092, #12059).

* Video memory use when loading large textures to VRAM has been optimized. This mainly helps when using texture replacement packs with very high-resolution textures, but will reduce the risk of running out of memory in general, which is one of the major remaining causes of reported crashes on Android.

* Integer scaling can now be enabled. This ensures that the emulated PSP framebuffer is scaled to an even multiple of the rendering resolution. If you then enable "Nearest" filtering, you get nice square pixels, if you're after the retro look.

* In Vulkan, we now do shader compilation (as in pipeline creation) in parallel on multiple threads, which has massive benefits on some drivers like the ones for PowerVR GPUs (which are increasingly common in lower-end phones like Galaxy A21), some benefit on Adreno and nearly no benefit on Mali. Either way, overall it's a major improvement on a lot of devices, which makes it a win even if not all are helped.

* We can now texture-sample from render targets that have been cast to CLUT formats, using a palette that comes from yet another render target. Fixed some special effects: [#8390](https://github.com/hrydgard/ppsspp/issues/8390). See the [glossary](/docs/development/glossary] for more information about the CLUT functionality and what can be done with it. Additionally, we now support a few additional CLUT texturing modes, which fixes night vision and other effects in the SOCOM games.

* Daxter and God of War (both from the studio Ready At Dawn) both rotate the analog stick input by 15 degrees before passing it to the game's internal control system. This makes the analog stick feel better aligned on the real PSP, but is not appropriate, so I added automatic pre-rotation in the other direction to compensate. Daxter now walks correctly along the cardinal directions as expected, and Kratos does too in the God of War games. See [#17020](https://github.com/hrydgard/ppsspp/issues/17020).

* Xiaomi Poco C40 phones, which are quite widespread in some countries, only expose a software Vulkan device, which is missing some mandatory features. This caused PPSSPP's UI to render incorrectly, and also very, very slowly. I have corrected the Vulkan device detection code to reject this device - which simply means they'll use OpenGL instead, which is functional (although not performing that great either).

* Fixed a slowdown in Naruto Heroes: #16733. This is a very typical case of unnecessary readbacks. The PSP has a "block copy" command to copy around arbitrary GPU memory between RAM and VRAM. PPSSPP designates areas in VRAM as framebuffers if they have been previously rendered to, and copies between two framebuffers can be simply converted to a GPU copy on the host. However, if data is copied to "new" VRAM or to RAM, by default we assume that the game will try to read the copied data with the CPU. This means we have to do a very costly readback. Often it actually doesn't need the data on the CPU, though, so we added a per-game compatibility option where we automatically create framebuffers covering the destination area of memory. If the game later simply textures from it, this becomes much, more more efficient than a readback from the GPU. We enabled this for Naruto Heroes.

## Other changes

* Multiple fixes for various depth clipping issues ([#17055](https://github.com/hrydgard/ppsspp/issues/17020), more)
* VR has gained some new features (top-down perspective) and various fixes, thanks Lubos!
* Several large code cleanups and refactors have been performed across the code base, to make future changes easier.
* The RISC-V JIT compiler has been improved, thanks Unknown. Future proofs the emulator a bit.
* New app icon (#11996), assorted bugfixes (#16988, #17017)

## Regressions fixed:

* Thrillville rendering corrected (#17169)
* Performance regression in Dante's Inferno has been fixed. (#17032)

## How to try

To test all these new features, either upgrade your existing install to the newest
