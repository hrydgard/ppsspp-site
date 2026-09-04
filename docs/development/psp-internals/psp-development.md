# Tips for developing for the PSP

## Avoid double precision

The PSP has no double precision floating point unit - so doubles (64-bit float) are very, very slow. Avoid at all cost.

Plain 32-bit float, on the other hand, can be used freely. There's even a vector unit, which is nearly but not quite IEEE-compliant.

These compiler flags will catch almost all accidental use of doubles:

```bash
   -Wdouble-promotion
   -Wfloat-conversion
```

## Interpreting PSPLink errors

0x800200D9 - out of memory. Restart PSPLink.

## Debugging and profiling

https://pspdev.github.io/debugging.html

## Cache matters!

When you write things to memory that will later be read by the GPU, you need to make sure that it doesn't just end up in the CPU cache, not written out to memory in time.

If you bitwise-OR a pointer with 0x40000000, you force "uncached" behavior. Do not read from such pointers, but do write to memory through them. This is especially useful for small amounts of dynamic vertex data or display lists.

If you write larger chunks to memory, like after fread-ing a file or wanting to also read from memory when processing, use:

```c
sceKernelDcacheWritebackInvalidateRange((void *)data, (unsigned int)size);
```

## Bandwidth matters!

### Organize your data in a PSP-friendly way

* Use native vertex formats of the PSP GPU - no runtime conversion.
* Use triangle strips exclusively for heavy geometry. If you are writing raw display lists, there's a trick to draw multiple triangle strips very cheaply: Just write PRIM commands, one after another, without re-specifying the vertex buffer pointer - it gets auto-incremented! You can do this with indexed draws too (in which case it's the index pointer that increments appropriately), but be aware that when you use indexing, the PSP GPU has a performance penalty. NOTE: You can precompile this list of PRIM and call it from your main display list.
* Textures can be read directly from main memory with good performance. Do not stream like on the PS2. If you draw a lot with one single texture it may be worth putting it in VRAM (though I am not really sure about that).

### Draw in a PSP-friendly way

* When clearing the screen (depth and/or color), do so in 64-pixel-wide vertical strips. This maximizes the use of the GE memory controller. I'm not quite sure why, that's just how it is.
* Don't bother clearing color unless you have to - if you are drawing a sky on top anyway, don't clear!

## Memory matters!

* Make full use of the paletted texture formats. For really tiny textures probably don't bother, but for larger textures using CLUT4 is a huge win, CLUT8 too, over for example 16-bit RGB555 or 32-bit RGBA8888.
* DXT1,3,5 formats are fine for title screens and basic 2D graphics, but not for pushing the GPU hard as they are rather slow.
