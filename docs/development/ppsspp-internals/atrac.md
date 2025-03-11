# Atrac3, Atrac3+ and other codecs

The PSP's (hidden) media engine processor handles decoding of formats like Atrac3+, Aac and so forth. This processor is not directly accessed by games, instead they go through libraries like sceAtrac, sceAac and so on. Atrac3+ is Sony's proprietary format, and the format used for background music in 99% of all games (except those that use MIDI/sequenced music). Mainly "minis" use other formats.

Emulating the sceAtrac library isn't easy, it's a complex high-level library with many different modes and behaviors.

Below is my attempt at documenting how the library is used by games, in the hopes of improving emulation (and who knows, by documenting it, maybe LLMs will one day understand this library, heh).

A lot of the below applies to both Atrac3+ and Atrac3 - the same library is used. Some details differ though like frame sizes,
auxiliary data in the file, etc (Atrac3+ doesn't have any).

## History of sceAtrac emulation

We first started by using an external library, which required buffering up audio in memory in order to decode, rather than feeding a decoder individual packets. This is bad because we had to fake a lot of the behavior.

When ffmpeg finally added support, we never switched over to using the internal buffering.
Mostly because I didn't realize back then that the entire context is conveniently exposed (see below).

But now, finally, we're getting accurate emulation of the buffering, which is critical for some games that use the library in slightly unusual ways, such as Flatout.

## Atrac3+

An Atrac3+ stream is usually stored in a standard WAVE file, which uses the RIFF container format.

It contains looping information.

## Internal states

* `NO_DATA` = 1: Not yet initialized
* `ALL_DATA_LOADED` = 2: The entire size of the audio data, and filled with audio data
* `HALFWAY_BUFFER` = 3: The entire size is allocated in memory, but only partially filled so far
* `STREAMED_WITHOUT_LOOP` = 4: Streaming through a small buffer, without any loop
* `STREAMED_WITH_LOOP_AT_END` = 5: Streaming through a small buffer, sliding with a loop at the end
* `STREAMED_WITH_SECOND_BUF` = 6: Smaller with a second buffer to help with a loop in the middle
* `LOW_LEVEL` = 8: Not managed, decoding using "low level" API (one packet at a time). Manual looping if needed. By far the easiest to emulate.
* `RESERVED` = 16: Not managed, reserved externally - possibly by sceSas

## Main function documentation

### `sceAtracSetHalfwayBuffer(id, bufferPtr, validDataSize, bufferSize)`

(Similar: sceAtracSetHalfwayBufferAndGetID).

Tells the library that you'll supply the whole buffer in memory, but only partially filled.

Use the combination `sceAtracGetRemainFrame/(last output of sceAtracDecodeData)`/`sceAtracGetStreamDataInfo`/`sceAtracAddData` to progressively fill it up while it's being played.

This is most useful when you have spare memory, and want to loop the audio without extra I/O later (or have irregular access to I/O, like GTA which also does a lot of streaming).

Sets internal state to `HALFWAY_BUFFER` (see above).

### `sceAtracSetDataAndGetID(data, size)`

Like `sceAtracSetHalfwayBuffer`, but either sets the whole buffer or sets up a shorter buffer that can be
used for streaming data (by looping), which is a lot more memory efficient.

Size can be any value (down to certain limitations), even byte-aligned! 0x4301 works and doesn't crash.
Games generally uses sizes like 0x18000, 0x10000, etc.

Depending on the file contents and the buffer size, the internal state can end up as any of:

* `ALL_DATA_LOADED`
* `STREAMED_WITHOUT_LOOP`
* `STREAMED_WITH_LOOP_AT_END`
* `STREAMED_WITH_SECOND_BUF`

Calling any of these functions will end up running a Decode on the first one or two blocks (or just skipping it).

### `sceAtracGetNextSample(id, *outPtr)`

Returns how many audio samples will be written by the next call to `sceAtracDecodeData`.

### `sceAtracGetMaxSample(id, *outPtr)`

Returns the maximum possible value that `sceAtracGetNextSample` can return (use it to size your buffer, maybe?).

### `sceAtracGetNextDecodePosition(id, *outPtr)`

Returns the sample position (NOT frame position) in the decoded audio track of the next frame to be decoded. Usually not very useful, although can be used to synchronize events to the audio playback.

### `sceAtracDecodeData(id, *outPtr, *outNumSamples, *outFinishFlag, *outRemainFrames)`

Decodes some data to memory, reading from the previously provided data (if outAddr is null, the decoded data is thrown away, processing happens as normal).

*outNumSamples is set to the number of (mono or stereo) samples written. The first frame is usually short, while all other frames are 0x800 samples (2048),
at least for Atrac3+. If you're playing back by directly pushing decoded buffers to `sceAudioOutputBlocking`, you're gonna need to move the frame forward
within the buffer size, to avoid a glitch. Example:

```c
// (dec_frame == outPtr, maxSamples == 2048, and a pointer to `samples` is passed in to the `outNumSamples` argument)
if (firstFrame && samples < maxSamples) {
    printf("Deglitching first frame\n");
    memmove(dec_frame + (maxSamples - samples) * 4, dec_frame, samples * 4);
    memset(dec_frame, 0, (maxSamples - samples) * 4);
}
```
Note that we need to check for firstFrame, otherwise we might do this to the last frame, which can also be short!

If the end is reached and looping is off, *outFinishFlag is set to 1, otherwise 0 is written.

This call is blocking, so while decoding is being processed (by the ME), the main CPU will switch over to other available threads to perform work.

The number of frames remaining (left to decode) in the buffer is returned through *outRemainFrames (same as calling `sceAtracGetRemainFrame`).

### `sceAtracGetRemainFrame(id, *outPtr)`

Returns the number of frames that can be decoded from the currently buffered data, or -1 or -2 if you're not
gonna need to load more before the loop. If streaming, usually when the returned value goes below a game-specific
threshold, the game will load more data into memory so that it doesn't run out. I haven't figured out the correct
way to compute this threshold but it has to depend on the buffer size you passed into sceAtracSetData.

Note, this same value is also returned through the last parameter to `sceAtracDecodeData`, so it's usually not necessary
to call this one on its own.

### `sceAtracSetSecondBuffer(id, buffer, bufSize)`

This informs sceAtrac about where we want a second buffer to be place. This is read from in a few circumstances like when
we are looping with a tail.

## General calling pattern

During init, you should call sceAtracGetMaxSample to figure out your audio channel buffer size. You need to round-robin these, not sure if 2 or 3 are needed,
I've seen games use both I think. This gets passed into the second argument of `sceAudioChReserve`.

`sceAtracGetNextSample` returns how many audio samples will be written by the next call to `sceAtracDecodeData`. If it's less than `sceAtracGetMaxSample()` you probably want to offset the output a bit to line up with the next frame to avoid a small glitch.

The game calls `sceAtracDecodeData` with a pointer to a destination buffer to get the next frame(s).

To know when to top up the buffer (if streaming), a game calls `sceAtracGetRemainFrame(id, *u32)` or just checks the last argument to `sceAtracDecodeData`.

When that value goes below a specific threshold, call `sceAtracGetStreamDataInfo` to figure out what data you need to read,
and where in memory to read it to. It will be somewhere inside the specified buffer (in SetData).

Exactly how the space inside that buffer is managed, I haven't yet figured out, although dumping the context
will probably make it possible to figure it out.

// This buffer is generally filled by sceAtracAddStreamData, and where to fill it is given by
// either sceAtracGetStreamDataInfo when continuing to move forwards in the stream of audio data,
// or sceAtracGetBufferInfoForResetting when seeking to a specific location in the audio stream.

## Streaming details

It just can't work! Well, that was my first reaction. But there's a nasty hack going on.

The first time around when we set up a buffer for streaming using sceAtracSetDataAndGetID(), sceAtrac isn't able to
give us any advice on how much data to write, in order to avoid splitting packets. So, what it does is, it'll
accept say 0x8000 bytes. As it turns out, at that size, offset 0x8000 is right in the middle of a packet, so it simply
copies the first half of the packet right on top of the headers it just read, at the beginning of the buffer.

Subsequently, the next time we call sceAtracGetStreamInfo, it'll apply the appropriate offset,
and we'll end up completing the packet! So when decoding comes along, it won't have to read a split packet.

In future laps around the streaming buffer, sceAtracGetStreamInfo is in control and it'll ask us to not completely fill
the buffer in order to avoid the problem the second time around, no more split packets happen after that.

## Usage patterns by games

### Halfway buffer

- GTA - Vice City Stories

### All-data-loaded

- MotoGP (menu music with specified loop). Simple repeated calls to sceAtracDecodeData
- Archer MacLean's Mercury (in-game, not menu)
- Crisis Core

### Streaming
- Good ones (early)
  - Everybody's Golf 2 (0x2000 buffer size, loop from end)
  - Burnout Legends (no loop, 0x1800 buffer size)
  - Suicide Barbie
- Others
  - Bleach
  - God of War: Chains of Olympus
  - Ape Academy 2 (bufsize 8192)
  - Half Minute Hero (bufsize 65536)
  - Flatout (tricky! needs investigation)

### Not using Atrac3+, but instead sequenced MIDI music

* Bust-a-move Deluxe
* Breath of Fire III
* Every Extend Extra

## Detailed game cases

NOTE: Logs below are from PPSSPP (and the old sceAtrac implementation), not hardware.

### Suicide Barbie (homebrew)

`STREAMED_WITHOUT_LOOP`

```
0=sceAtracSetDataAndGetID(09ef9f00, 00040000)
0=sceAtracGetRemainFrame(0, 09ffee24[0000015f])
0=sceAtracGetMaxSample(0, 09ffee3c[00000800])
loop:
0=sceAtracDecodeData(0, 09ef1f00, 09ffee28[00000690], 09ffee38[00000000], 09ffee24[350])
Until remainFrame (last param) <= 175. Then:
0=sceAtracGetStreamDataInfo(0, 09ffee2c[09ef9fa0], 09ffee30[00020228], 09ffee34[00040000])
0=sceAtracAddStreamData(0, 00020000)
goto loop
More examples (always followed by AddStreamData(20000) or a little less):
0=sceAtracGetStreamDataInfo(0, 09ffee2c[09f19fa0], 09ffee30[0001fe60], 09ffee34[00060000])
0=sceAtracGetStreamDataInfo(0, 09ffee2c[09ef9f00], 09ffee30[00020268], 09ffee34[0007fe60])
0=sceAtracGetStreamDataInfo(0, 09ffee2c[09f19f00], 09ffee30[0001ff00], 09ffee34[0009fe60])
```

### Archer Maclean's Mercury

`ALL_DATA_LOADED`

```
0=sceAtracSetData(0, 0952e700, 00177dac)
0=sceAtracSetLoopNum(0, -1)
0=sceAtracDecodeData(0, 09fe1680, 09fdb664[00000180], 09fdb660[00000000], 09fdb668[-1])
loop:
0=sceAtracDecodeData(0, 09fe1680, 09fdb664[00000800], 09fdb660[00000000], 09fdb668[-1])
goto loop
```

### Daxter (title screen)

Starts as `STREAMED_LOOP_FROM_END` then switches to `ALL_DATA_LOADED` once it's full (!).

~~Don't quite understand how this can happen, the initial buffer size (0x28000) is smaller than
the full track size (0x475dc)~~.

Wait, this is probably due to our wrong internal buffering! It should never reach ALL_DATA_LOADED,
the buffer is too small.

Curiously, during the transition (when it loops the first time), remainFrames as returned by Decode
goes from 313 to -1. 313 is a strange number here..

```
0=sceAtracSetDataAndGetID(08bf2b00, 00028000)
SCE_KERNEL_ERROR_SCE_ERROR_ATRAC_SECOND_BUFFER_NOT_NEEDED=sceAtracGetSecondBufferInfo(0, 09fbca74[00000000], 09fbca78[00000000])
0=sceAtracGetSoundSample(0, 08e265c0[00183773], 09fbca7c[00000000], 09fbca80[00183773])
0=sceAtracSetLoopNum(0, -1)
0=sceAtracGetRemainFrame(0, 09fbca54[000001b2])
0=sceAtracDecodeData(0, 08c1ab00, 09fbca58[00000774], 09fbca50[00000000], 09fbca54[433])
0=sceAtracGetNextSample(0, 09fbca70[00000800]): 2048 samples to be written
0=sceAtracDecodeData(0, 08c1c8d0, 09fbca58[00000800], 09fbca50[00000000], 09fbca54[432])
0=sceAtracGetNextSample(0, 09fbca70[00000800]): 2048 samples to be written
...
0=sceAtracDecodeData(0, 08c1ebd0, 09fbca58[00000800], 09fbca50[00000000], 09fbca54[218])
0=sceAtracGetNextSample(0, 09fbca70[00000800]): 2048 samples to be written
0=sceAtracDecodeData(0, 08c1abd0, 09fbca58[00000800], 09fbca50[00000000], 09fbca54[217])
0=sceAtracGetStreamDataInfo(0, 09fbca5c[08bf2b74], 09fbca60[00014060], 09fbca64)
0=sceAtracGetNextSample(0, 09fbca70[00000800]): 2048 samples to be written
0=sceAtracAddStreamData(0, 00014060)
0=sceAtracGetRemainFrame(0, 09fbca54[000001b3])
0=sceAtracDecodeData(0, 08c1cbd0, 09fbca58[00000800], 09fbca50[00000000], 09fbca54[434])
0=sceAtracGetNextSample(0, 09fbca70[00000800]): 2048 samples to be written
```

### Tekken 6

Uses an unusually small buffer. We match this in our stream.prx test.

```
0=sceAtracSetDataAndGetID(092bd940, 00010000)
0=sceAtracSetLoopNum(0, -1)
0=sceAtracGetRemainFrame(0, 09fdea78[00000056])
0=sceAtracIsSecondBufferNeeded(0)
0=sceAtracDecodeData(0, 092cfa00, 09fdea70[000003b4], 09fdea74[00000000], 09fdea78[85])
0=sceAtracDecodeData(0, 092d08d0, 09fdea70[00000800], 09fdea74[00000000], 09fdea78[84])
0=sceAtracDecodeData(0, 092d28d0, 09fdea70[00000800], 09fdea74[00000000], 09fdea78[83])
0=sceAtracGetStreamDataInfo(0, 093241e0[092bdbc4], 093241e4[000080e8], 093241e8[00010000])
0=sceAtracDecodeData(0, 092d1ad0, 09fdea70[00000800], 09fdea74[00000000], 09fdea78[41])
0=sceAtracAddStreamData(0, 000080e8)
0=sceAtracDecodeData(0, 092d3ad0, 09fdea70[00000800], 09fdea74[00000000], 09fdea78[85])
```

### Flatout (problematic!)

Uses the FMOD library.

NOTE: Does two setdata! First one just with the header I guess. Then the proper one.

Later, it seems to run out of data just before refilling, even though there should be
~88 buffered frames left. This makes little sense.

```
0=sceAtracSetDataAndGetID(08b89d40, 00001000)
0=sceAtracGetChannel(0, 09fdee70[00000002])
0=sceAtracGetBitrate(0, 09fdee74[00000060])
0=sceAtracGetSoundSample(0, 09fdee78[007946db], 09fdee7c[ffffffff], 09fdee80[ffffffff])
0=sceAtracGetBufferInfoForResetting(0, 0, 09fdee40)  << maybe important that this is called before SetData?
0=sceAtracSetData(0, 08fe93c0, 00018000)
SCE_KERNEL_ERROR_SCE_ERROR_ATRAC_NO_LOOP_INFORMATION=sceAtracSetLoopNum(0, 0): no loop information to write to!
0=sceAtracGetRemainFrame(0, 09fdedb0[000000ae])
0=sceAtracDecodeData(0, 08fd5bd0, 09fdedcc[00000690], 09fdedd0[00000000], 09fdedb0[173])
0=sceAtracGetRemainFrame(0, 09fdedb0[000000ad])
0=sceAtracDecodeData(0, 08fd7610, 09fdedcc[00000800], 09fdedd0[00000000], 09fdedb0[172])
0=sceAtracGetRemainFrame(0, 09fdedb0[000000ac])
0=sceAtracDecodeData(0, 090014c0, 09fdedcc[00000800], 09fdedd0[00000000], 09fdedb0[171])
0=sceAtracGetRemainFrame(0, 09fe1980[000000ab])
0=sceAtracDecodeData(0, 08fdb610, 09fe199c[00000800], 09fe19a0[00000000], 09fe1980[170])
...
0=sceAtracDecodeData(0, 090014c0, 09fe199c[00000800], 09fe19a0[00000000], 09fe1980[87])
0=sceAtracGetRemainFrame(0, 09fe1980[00000057])
0=sceAtracGetStreamDataInfo(0, 09fe1990[08fe9490], 09fe1994[0000c010], 09fe1998[00018000])
0=sceAtracAddStreamData(0, 0000c010)
0=sceAtracDecodeData(0, 08fdb610, 09fe199c[00000800], 09fe19a0[00000000], 09fe1980[174])
0=sceAtracGetRemainFrame(0, 09fe1980[000000ae])
0=sceAtracDecodeData(0, 090014c0, 09fe199c[00000800], 09fe19a0[00000000], 09fe1980[173])
0=sceAtracGetRemainFrame(0, 09fe1980[000000ad])
0=sceAtracDecodeData(0, 08fd7610, 09fe199c[00000800], 09fe19a0[00000000], 09fe1980[172])
0=sceAtracGetRemainFrame(0, 09fe1980[000000ac])
0=sceAtracDecodeData(0, 090014c0, 09fe199c[00000800], 09fe19a0[00000000], 09fe1980[171])
0=sceAtracGetRemainFrame(0, 09fe1980[000000ab])
...
0=sceAtracGetRemainFrame(0, 09fe1980[00000059])
0=sceAtracDecodeData(0, 08fd7610, 09fe199c[00000800], 09fe19a0[00000000], 09fe1980[88])
0=sceAtracGetRemainFrame(0, 09fe1980[00000058])
at3_standalone\compat.cpp:23 Atrac3/3+: Frame data doesn't match channel configuration! ch_block 1 >= num_channel_blocks 1
0=sceAtracDecodeData(0, 090014c0, 09fe199c[00000800], 09fe19a0[00000000], 09fe1980[87])
0=sceAtracGetRemainFrame(0, 09fe1980[00000057])
0=sceAtracGetStreamDataInfo(0, 09fe1990[08ff54a0], 09fe1994[0000bdf0], 09fe1998[00024010])
0=sceAtracAddStreamData(0, 0000bdf0)
```

## Misc function documentation

### `sceAtracGetChannel(id, *outPtr)`

Returns the number of channels the audio track has.

### `sceAtracGetSoundSample(id, *outEndSample, *outLoopStartSample, *outLoopEndSample)`

Gets the length of the track and the extents of the loop (if any, otherwise -1), counted in output audio sample positions.

### `sceAtracReleaseAtracID(id)`

Obvious.

### `_sceAtracGetContextAddress(id)`

Returns a pointer to the internal context data.

The previous Atrac3+ implementation only copied some variables back to this. The new one uses it to hold most state.

### `sceAtracGetNextDecodePosition`

This seems to be busted (at least it doesn't do what it says on the tin).
