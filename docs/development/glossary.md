# Glossary

## HLE

High Level Emulation. Instead of emulating the entire hardware and running the PSP operating system on it, we

## Allegrex

The PSP's main CPU. It's a 333MHz MIPS processor with a whole bunch of custom extensions, including a SIMD vector instruction set, and a few very convenient bit manipulation instructions. Most games run it at 222MHz to save power, but some make full use of it.

## Media Engine

The PSP's secondary CPU. This one is never directly accessed by commercial games, and PPSSPP thus does not emulate it directly. It's used in the implementation of platform libraries like sceMpeg, that PPSSPP catch and emulates through HLE instead. It has some unknown extensions to help with decoding h.264 and Atrac+.

A small number of homebrew apps for the PSP make use of this processor to run their own code. This includes later versions of the N64 emulator Daedalus, which thus won't run on the emulator. Additionally, it's believed that PopStation, Sony's PSX emulator, does the same thing, which is why it'll also not run.

## CLUT

Stands for Color Look Up Table. Same thing as a traditional image "palette".
