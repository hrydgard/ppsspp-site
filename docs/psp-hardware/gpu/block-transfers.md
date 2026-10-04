---
position: 9
---
# Block transfers

The GE can copy a rectangle of pixels from one buffer to another on its own, with a block transfer (TRXKICK, 0xEA): source and destination address and buffer width, source and destination position, size, and 16 or 32 bits per pixel. Games use them to copy framebuffers into textures, to upload textures, and sometimes to shift a buffer onto itself.

A transfer runs alongside drawing, so a display list that textures from a transfer's destination needs a TEXSYNC (0xCC) first. See the [texture cache](/docs/psp-hardware/gpu/texture-cache).

## Overlapping source and destination

When the source and destination overlap, the result is neither a forward nor a backward copy. This was measured on a PSP with many transfers within one buffer of unique values, shifted by -33 to 33 pixels sideways and -2 to 2 rows, at 16 and 32 bits per pixel, from several alignments.

- **Rows are copied one at a time, top to bottom.** When the source and destination rows are different, a row is copied as it is in memory at that point. So a transfer shifted down copies rows it has already rewritten, and one shifted up doesn't.
- **Within a row, the copy goes through a small buffer.** The source is read in whole, aligned 16-byte blocks, four at a time, starting from the row's first block. Each byte is placed at its destination address in a buffer of four 16-byte slots, picked by bits 4-5 of the destination address. A slot holds one destination block at a time. When a byte for another block arrives, the slot writes the bytes it was given for its old block to memory and switches to the new block, keeping its contents. After the row, the buffer is written out.
- **Reads look in the buffer.** When a source block is read while its slot holds that same block, the read returns the slot's 16 bytes as they are: the bytes already written there, and whatever the slot still held from earlier blocks for the rest.

So a row shifted right by less than 64 bytes reads some of what it has just written, and some leftovers from earlier in the row. Shifts by a multiple of 64 bytes, shifts to the left and shifts to other rows behave like a plain copy of the bytes as they are at the time.

The model reproduces 562 of 570 measured transfers exactly. The other eight differ only in the last pixel of a row more than 64 bytes long. PPSSPP copies overlapping rows this way (`GPU/GPUCommon.cpp`).
