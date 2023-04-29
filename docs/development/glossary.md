# Glossary

## Allegrex

The [PSP's main CPU](/docs/psp-hardware/cpu/allegrex-overview). It's a 333MHz MIPS processor with a whole bunch of custom extensions, including a SIMD vector instruction set, and a few very convenient bit manipulation instructions. Most games run it at 222MHz to save power, but some make full use of it.

## Media Engine

The PSP's secondary CPU. This one is never directly accessed by commercial games, and PPSSPP thus does not emulate it directly. It's used in the implementation of platform libraries like sceMpeg, that PPSSPP catch and emulates through HLE instead. It has some unknown extensions to help with decoding h.264 and Atrac+.

A small number of homebrew apps for the PSP make use of this processor to run their own code. This includes later versions of the N64 emulator Daedalus, which thus won't run on the emulator. Additionally, it's believed that PopStation, Sony's PSX emulator, does the same thing, which is why it'll also not run.

## CLUT

Stands for Color Look Up Table. Same thing as a traditional image "palette".

The PSP has some special powers in this area, though. It doesn't only support 4-bit or 8-bit paletted texture formats (where each pixel simply represents a plan index into a palette), but also CLUT16 and CLUT32 formats. Since palette RAM can only fit a maximum of 512 16-bit colors, or more commonly 256 32-bit colors, you'd think that this wouldn't have much use, but you can simply alias a texture in this format on top of a framebuffer that has been rendered to in RGBA8888 or say RGB565 formats - or even a depth buffer. Then, there are parameters to select which range of bits should be used to index into the palette.

Hence, it's possible to do some very tricky stuff that involves looking up rendered color data in a palette in a variety of ways. For example, the [lens flare effect in Burnout Dominator](/blog/lens-flare-burnout-dominator) makes some very creative use of this, and so do effects in a number of other games.

For more information, see [image formats](/docs/psp-hardware/gpu/image-formats) in the hardware section.