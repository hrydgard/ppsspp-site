# Introduction

## About PPSSPP

PPSSPP is a PSP emulator written in C++. It translates PSP CPU instructions directly into optimized x86, x64, ARM or ARM64 machine code, using JIT recompilers (dynarecs).

PPSSPP can thus run on quite low-spec hardware, including stronger **ARM-based phones and tablets**, as long as there's OpenGL ES 2.0 support.

## Code and commit access

PPSSPP is an open source project, licensed under the GPL 2.0 (or later). Anyone is welcome to contribute their improvements to the code. The code is hosted at [the Github page for PPSSPP](https://github.com/hrydgard/ppsspp), using the [Git](https://git-scm.com/) source control system.

Following the [Dolphin](https://www.dolphin-emu.org) tradition, the PPSSPP project is very open. Anyone can participate directly in the development by making pull requests, and if you make substantial contributions you may be given rights to commit directly.

## Convenient Links

- [PPSSPP on Github](https://github.com/hrydgard/ppsspp)
- [Commits](https://github.com/hrydgard/ppsspp/commits/master)
- [View open issues](https://github.com/hrydgard/ppsspp/issues)
- [View pull requests](https://github.com/hrydgard/ppsspp/pulls)

## Building

[Build instructions are available here.](https://github.com/hrydgard/ppsspp/wiki/Build-instructions)

## Debugging and logging

You may find yourself relying on the logs a lot to figure out what's really going on. As PPSSPP simulates the PSP OS, we get something similar to a Unix-style strace.

There's a headless (no display/audio) build, which is very useful when running the [pspautotests](https://github.com/hrydgard/pspautotests) through test.py - highly recommended way to find issues.

The Windows port and the Qt build have interactive debuggers (Qt currently broken). This is not available for mobile platforms and "SDL" builds.

### HLE

PPSSPP is a HLE ("high level emulation") emulator, it simulates the PSP OS as seen by the game rather than the full hardware. A program running on the PSP OS can send raw display lists to the graphics chips, but can't access the flash controller or the Media Engine directly, instead it has to go through libraries and the PSP OS kernel. We simply simulate these. This is a lot of work though, the PSP OS is large and has plenty of functionality so achieving 100% compatibility is difficult bordering on the impossible. We can get close though.

### Automated Test Suite

Since this emulator implements most of the PSP OS in order to run games, testing it against the real thing is critical. Some other PSP emulator authors have collaborated around creating a test suite called pspautotest, and we have picked that up and expanded it. There are two scripts: **test.py** and **gentest.py**. gentest.py uses psplink to run a test program on your real PSP and captures the text output and saves it as \[test\].expected. Then you can run the same test in PPSSPP by using test.py, which will automatically compare the output to the expected output.

### Contributing

Pick from the list of tasks below, go look at the [open issues on GitHub](https://github.com/hrydgard/ppsspp/issues), or just implement whatever missing feature you feel like. Send your code as a pull request on Github. If you send a couple of good pull requests, you'll be added as a contributor and get your own commit access.

The PPSSPP coding style is fairly similar to the [Google C++ coding style](https://google.github.io/styleguide/cppguide.html), though there are parts of the code that are a bit off, especially with regards to braces and capitalization.
