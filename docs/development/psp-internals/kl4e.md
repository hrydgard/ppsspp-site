# The KL4E / KL3E compression format

Documentation entirely by Claude.

KL4E is Sony's own compressed-executable format for the PSP, used alongside gzip as the second
compression scheme a `~PSP` module can be packed with. KL3E is the same bitstream with one
constant changed. This document describes the format;
everything below has been checked against a working decoder run over a real 6.61 firmware dump.

It is an LZ77 scheme - a stream of "output this literal byte" and "repeat N bytes from M bytes
back" tokens - where every individual bit is coded with an adaptive binary arithmetic coder
instead of being written out directly. That makes it structurally close to LZMA, and closer
still to Sony's other in-house scheme `2RLZ` (a.k.a. LZR), which shares the same idea with
different table layouts and a different token grammar.

## Where it turns up

- **Kernel modules.** `memlmd_01g.prx` and `loadexec_01g.prx` are KL4E-compressed inside their
  own PRX. Others in the `kd/` tree follow.
- **VSH modules.** `paf.prx` and `pafmini.prx`, which matters because they are on the critical
  path for booting the real XMB.
- **PSAR archives.** `Core/Util/PSARUnpack.cpp` sniffs for it, though across the 43 firmware
  versions surveyed in [PsarFileFormat.md](PsarFileFormat.md) every archive entry turned out to
  be zlib. The KL4E in a firmware updater is a layer deeper, inside the PRXes it carries.
- **As a service.** `sysmem.prx` exports the decompressor as `UtilsForKernel_6C6887EE`, so
  anything running on the PSP can call it:

  ```c
  int UtilsForKernel_6C6887EE(void *outBuf, int outSize, const void *inBuf, const void **end);
  ```

  (argument order per JPCSP, which can invoke the real MIPS routine through its interpreter
  instead of using its own Java port). KL3E's decompressor is a separate, unexported routine in
  `loadexec`.

Checked against a decrypted 6.61 `flash0` dump: the NID `6C6887EE` is present in `sysmem.prx`'s
NID tables, and the four-byte string `KL4E` appears as a constant in `loadcore.prx`,
`me_wrapper.prx` and every `reboot*.bin` - the modules that have to recognise a compressed
payload before handing it off. `KL3E` and `2RLZ` do not appear as constants there; those
comparisons are presumably built as immediates.

## Container framing

A KL4E payload is always preceded by a four-byte ASCII magic:

| Magic | Format |
| --- | --- |
| `KL4E` | KL4E |
| `KL3E` | KL3E |
| `2RLZ` | LZR, a different bitstream - see `libLZR.c` |
| `1F 8B` | gzip |

The five-byte stream header described below starts immediately after that magic. Note that the
decompressor entry point itself is handed a pointer *past* the magic - both `pspdecrypt` and
JPCSP call it with `inBuf + 4`.

Inside a `~PSP` module header (`Core/ELF/PrxDecrypter.h`), the relevant fields are:

| Offset | Field | Meaning |
| --- | --- | --- |
| 0x06 | `comp_attribute` (u16) | bit 0 set = payload is compressed |
| | | bits 8-11 all zero = gzip, non-zero = KL4E |
| 0x28 | `elf_size` (u32) | decompressed size - the output buffer to allocate |
| 0x2C | `psp_size` (u32) | compressed size |

JPCSP names bit 9 `SCE_EXEC_FILE_KL4E_COMPRESSED = 0x200` but tests the whole `0xF00` nibble,
which is the safer thing to copy. PPSSPP reads only bit 0 and then picks the scheme from the
payload's magic (gzip, KL4E or KL3E) in `sceKernelModule.cpp`; the decoder is `Core/Util/KL4E.cpp`.

## The five-byte stream header

| Bytes | Contents |
| --- | --- |
| 0, bit 7 | Set: the payload is *stored*, not compressed. See below. |
| 0, bits 6-5 | Unused/unknown |
| 0, bits 4-3 | Selects the initial probability: `0x80 - (v << 4)`, i.e. 0x80, 0x70, 0x60 or 0x50 |
| 0, bits 2-0 | `shift`, the literal context selector (0-7) |
| 1-4 | **Big-endian** u32 |

The big-endianness is worth flagging - almost nothing else on the PSP is stored that way.

That u32 does double duty. In stored mode it is the payload length. In compressed mode it is the
initial code value of the arithmetic decoder, i.e. the first four bytes of the coded stream, and
actual coded bytes resume at offset 5.

Unlike 2RLZ, every probability starts from the header-supplied value rather than a flat 0x80.
That is the only tuning knob the encoder has.

### Stored mode

If bit 7 of byte 0 is set, the header word is a byte count, the payload follows at offset 5, and
it is copied out verbatim. The length must be **strictly** less than the output buffer size -
an exactly-full buffer is rejected with `SCE_KERNEL_ERROR_INVALID_SIZE` (0x80000104). On success
the `end` out-parameter points at the *last byte consumed*, not one past it; the compressed path
sets it one past. Anything reading `end` has to know which mode it was in.

## The arithmetic decoder

State is three values: a 32-bit `range` starting at 0xFFFFFFFF, a 32-bit `code` starting at the
header word, and the input pointer at offset 5.

A probability is one byte, read as `p(bit == 1) = prob / 256`. Reading a bit:

```
if ((range >> 24) == 0) {          // renormalize: pull in one more byte
    code = (code << 8) + *in++;
    bound = range * prob;          // note: uses the pre-shift range
    range <<= 8;
} else {
    bound = (range >> 8) * prob;
}
prob -= prob >> decay;             // decay happens either way
if (code >= bound) {               // a 0
    code -= bound;  range -= bound;  store prob;
} else {                           // a 1
    range = bound;  store prob + decay_bonus;
}
```

Two adaptation profiles are used, and the choice is fixed per call site, not per table:

| `decay`, `bonus` | Effect | Used by |
| --- | --- | --- |
| 3, 31 | `p *= 7/8` then `+31` on a 1; saturates near 248 | everything except the flag chain |
| 4, 15 | `p *= 15/16` then `+15` on a 1; saturates near 240 | the literal/match flag and the unary length-bit-count |

Two probability-free variants exist for bits the encoder considers incompressible - the low bits
of long length and distance codes:

- **uniform**: same renormalization test, but `range <<= 7` when refilling (equivalently `<< 8`
  then `>> 1`) and `range >>= 1` otherwise; then the usual compare-and-subtract against `range`.
- **uniform, non-normalizing**: `range >>= 1` and compare, with no refill at all. Only valid
  immediately after a read that did renormalize, which is exactly how it is used - one uniform
  read, then a run of non-normalizing ones.

## The context tables

Five separate tables, all filled with the header's seed value at the start. Their sizes are not
round numbers, and the fact that every one of them is *exactly* large enough for the maximum
reachable index is good evidence that the reverse-engineered sizes are the real ones:

| Table | Size | Max index reachable | Indexed by |
| --- | --- | --- | --- |
| `litProbs` | 2040 | 2039 | 8 literal contexts x 255 |
| `copyCountBitsProbs` | 64 | 63 | 8 states x 8 unary steps |
| `copyCountProbs` | 256 | 247 | length-bit-count, output alignment, state |
| `copyDistBitsProbs` | 304 | 303 | length class, then the doubling walk |
| `copyDistProbs` | 144 | see below | distance-bit-count |

`copyDistProbs` is the exception, and the discrepancy is discussed under "Bounds" below.

## The token grammar

The very first literal is decoded with no preceding flag bit - a small saving, and a difference
from 2RLZ, which flags every token. After that the loop is:

1. Advance the output position by one.
2. Read one bit at `copyCountBitsProbs[state]` with profile (4, 15).
   - **0** - a literal. `state = max(state - 1, 0)`, then decode a literal byte. If the output
     position has reached the end of the buffer, fail with `INVALID_SIZE`.
   - **1** - a match; continue below.
3. Read the *length bit-count* as a unary run, using the same table. **The index is stepped by 8
   before each read, including the read that ends the run**, so a run that stops immediately has
   still moved the index once. Count the 1-bits into `copyCountBits`, which starts at -1 and
   stops at 6.
4. Decode the length. One of its bits also selects the distance code's parameters.
5. Decode the distance bit-count, then the distance value.
6. Copy, then set `state = 6 + (outputPosition & 1)`.

`state` is an index into `copyCountBitsProbs` that persists between tokens, and step 3 leaves it
wherever the unary walk stopped rather than putting it back. That is why the length code's
context uses `state & 7` - the low three bits survive the walk, and the mask is doing real work.

### Literals

The context is picked from a mix of the previous output byte and the output position's alignment:

```
ctx    = ((outputPosition & 7) << 8) | (previousByte & 0xFF);
bucket = (ctx >> shift) & 7;           // shift comes from the header
base   = bucket * 255 - 1;
```

`shift` slides the window over that composite value: at 0 it selects the low three bits of the
previous byte, at 5 the top three bits (the same choice LZMA's `lc` makes), and at 7 mostly the
output alignment. The byte is then read MSB-first, eight bits, in the standard "implicit leading
1" way - the accumulator starts at 1, each bit shifts it left, the probability used for each bit
is `litProbs[base + accumulator]`, and the loop ends when the accumulator reaches 0x100, leaving
the byte in the low 8 bits.

`base` is deliberately biased by -1 so that an accumulator of 1..255 addresses 0..254 within the
bucket; the maximum index is `7 * 255 + 254 = 2039`, exactly filling the table.

### Match lengths

`copyCountBits` from step 3 runs from -1 to 6. When it is -1 the length code is empty and
`copyCount` stays at its initial value of 1. Otherwise the code is assembled from
`copyCountProbs` at

```
offset = (copyCountBits << 5)
       | (((outputPosition & 3) << (copyCountBits + 3)) & 0x18)
       | (state & 7);
```

The alignment term vanishes for `copyCountBits >= 2` - output alignment only refines the short
length codes.

The value is **not** a plain MSB-first read. It is built in three parts, and the order matters
because the middle part has a side effect:

```
copyCount = 1;
if (copyCountBits >= 3) {                       // high bits
    copyCount = 2 + readBit(probs + 24);
    if (copyCountBits > 3) {
        copyCount = (copyCount << 1) | readBit(probs + 24);
        if (copyCountBits > 4) {
            copyCount = (copyCount << 1) | readBitUniform();
        }
        for (i = 5; i < copyCountBits; i++) {
            copyCount = (copyCount << 1) | readBitUniformNoNorm();
        }
    }
}

copyCount <<= 1;                                // the selector bit
if (readBit(probs + 0)) {
    copyCount |= 1;
    if (copyCountBits <= 0) { powLimit = long; distBase = 56 + copyCountBits; }
} else {
    if (copyCountBits <= 0) { powLimit = 64;   distBase = copyCountBits; }
}

if (copyCountBits > 0) {                        // low bits
    copyCount = (copyCount << 1) | readBit(probs + 8);
    if (copyCountBits != 1) {
        copyCount <<= 1;
        if (readBit(probs + 16)) {
            copyCount++;
            if (copyCount == 0xFF) return;      // end of stream
        }
    }
    powLimit = long;  distBase = 56 + copyCountBits;
}
```

Note that `probs + 24` is read **twice** for long codes, that the bit at `probs + 0` is the one
that picks the distance parameters, and that `copyCountBits == 1` and `copyCountBits == 2` take
visibly different paths through the low bits.

The result is a length *minus one*: the copy loop runs `copyCount + 1` times, so lengths run from
2 bytes up to 255.

**The value 0xFF is the end-of-stream marker**, not a length. There is no output length in the
header and no terminator byte; the stream simply stops. It is only reachable on the
`copyCountBits > 1` path, which is the only place it is tested.

### Match distances

Two parameters are chosen while decoding the length, per the code above. `powLimit` is 64 for a
short match whose selector bit was 0, and otherwise 256 for KL4E or **128 for KL3E** - the one
constant that separates the two formats. When `copyCountBits` is -1 the length code never runs,
and the short parameters apply with `distBase = -1`.

The bit-count comes from a doubling walk. The bit is read *before* the doubling is committed, and
the extra 8 is only picked up when the walk continues:

```
curPow = 8;
for (;;) {
    prob   = copyDistBitsProbs[distBase + curPow - 7];
    curPow <<= 1;
    copyDistBits = curPow - powLimit;
    if (readBit(prob) == 0) {
        if (copyDistBits >= 0) {
            if (copyDistBits != 0) { copyDistBits -= 8; break; }
            distance = 0; break;          // run-length: match the byte just emitted
        }
    } else {
        curPow += 8;
        if (copyDistBits >= 0) break;
    }
}
```

The `copyDistBits -= 8` on the zero-bit exit is easy to miss and shifts every subsequent table
index. A `copyDistBits` of exactly 0 reached on a 0 bit is the "distance 0" case and skips the
value decode entirely.

The value is then read from `copyDistProbs[copyDistBits ..]`, in the same three-part shape as the
length, with `readBits = copyDistBits / 8`:

```
if (readBits < 3) {
    copyDist = 1;
} else {
    copyDist = 2 + readBit(probs + 3);
    if (readBits > 3) {
        copyDist = (copyDist << 1) | readBit(probs + 3);
        if (readBits > 4) {
            copyDist = (copyDist << 1) | readBitUniform();
            readBits--;                       // mutated - later tests see the new value
        }
        while (readBits > 4) {
            copyDist = (copyDist << 1) + readBitUniformNoNorm();
            readBits--;
        }
    }
}

copyDist <<= 1;
if (readBit(probs + 0)) { if (readBits > 0)  copyDist++; }
else                    { if (readBits <= 0) copyDist--; }

if (readBits > 0) {
    copyDist <<= 1;
    if (readBit(probs + 1)) { if (readBits != 1) copyDist++; }
    else                    { if (readBits == 1) copyDist--; }
    if (readBits != 1) {
        copyDist <<= 1;
        if (!readBit(probs + 2)) copyDist--;
    }
}
```

The `+1`/`-1` adjustments are what make the code ranges for successive bit counts contiguous, and
they depend on `readBits` *after* the decrement above, not before.

Like the length, the result is a distance minus one: the source is `outputPosition - copyDist - 1`.
It is validated against how much output exists so far, failing with
`SCE_KERNEL_ERROR_INVALID_FORMAT` (0x80000108) otherwise.

### The copy

A plain forward byte-at-a-time copy, so overlapping source and destination is legal and is how
runs are encoded. Afterwards the "previous byte" becomes the last byte copied, and the output
position advances by `copyCount` (not `copyCount + 1` - the loop head adds the last one).

## Bounds, and two real holes

Everything above describes a well-formed stream. A malformed one is a different matter. Both of
these are real in the reference and on hardware, and PPSSPP's implementation deliberately checks
rather than reproduces them, since it runs this over a file the user was handed:

**The match copy is not bounds-checked against the output buffer.** Only the literal path tests
for it. A crafted stream can emit a match at the very end of the output and write up to 255 bytes
past it. In the reference, that's a heap overflow (PPSSPP's `Core/Util/KL4E.cpp` checks it).

**`copyDistProbs` can be indexed out of range.** The distance bit-count walk can leave
`copyDistBits` far larger than the 144-byte table allows, and the value decode reads it at
`copyDistBits + 3`. Legitimate streams don't get anywhere near: a `readBits` of 17 already
encodes distances around 2^19, far more than any PSP module needs, and that only needs index 140.
But nothing in the format stops an encoder - or an attacker - from emitting a longer code, and
there is no check.

Neither is the input pointer bounded; the decoder pulls bytes as the range needs refilling and
trusts the stream to terminate.

## Implementing it

A note for anyone reimplementing from this page: the arithmetic decoder, the five-byte header and
the literal coding are straightforward and can be got right from the description alone - a decoder
with those correct but the token grammar guessed will decode the first ~20 bytes of a real module
and then diverge, which makes for a confusing debugging session. The ELF header of a decompressed
PRX is a good oracle while working: `7f 45 4c 46 01 01 01 00`, eight zero bytes, then `a0 ff`
(`ET_PSP_PRX`) and `08 00` (`EM_MIPS`), and the decompressed size must equal the `elf_size` field
at offset 0x28 of the `~PSP` header.

One subtlety in any port: The original code indexes several contexts by the low bits of *pointers* -
`curOut & 7`, `curOut & 3`, `curOut & 1`, and `curCopyCountBitsProbs & 7`. Both the C reference
and JPCSP substitute buffer-relative offsets, which is only equivalent if the output buffer and
the probability table are 8-byte aligned. They will have been in the kernel, and the encoder must
have made the same assumption for the format to be self-consistent, so offsets are the right
reading - but a from-scratch implementation should use offsets deliberately rather than
reproducing the pointer arithmetic and hoping the allocator cooperates.

## Sources

The token grammar above was corrected against `kl4e.c` (below) after an attempt to implement the
format from the earlier revision of this page failed; everything on this page has since been
checked against a decoder that decompresses all 129 `kd/` modules of a 6.61 dump.

- `kl4e.c` in [John-K/pspdecrypt](https://github.com/John-K/pspdecrypt) - the primary source.
  Reverse engineered from 6.60 firmware by artart78, from `UtilsForKernel_6C6887EE` in
  `sysmem.prx`; KL3E was covered by finding the one constant that differs rather than by
  reversing `loadexec`'s copy, so KL3E corner cases are less certain. Not derived from Sony code.
- `src/jpcsp/format/KL4E.java` and `src/jpcsp/crypto/PRX.java` in
  [jpcsp](https://github.com/jpcsp/jpcsp) - a Java port of the above, plus the `comp_attribute`
  dispatch and the option to run the real MIPS routine under the interpreter.
- `libLZR.c` by BenHur, for 2RLZ - the closest relative, and what artart78 credits for working
  out the shape of the arithmetic coder in the first place.
