# sceAtrac: Atrac3 and Atrac3+

The PSP's (hidden) media engine processor handles decoding of formats like Atrac3+, Atrac3, AAC, h.264, MP3 and so forth. This processor is not directly accessed by games, instead they go through libraries like sceAtrac, sceAac etc. Atrac3+ is Sony's proprietary format, and the format used for background music in 99% of all games (except those that use MIDI/sequenced music). Mainly "minis" use other formats like MP3 and AAC.

Emulating the sceAtrac library isn't easy, it's a complex high-level library with many different modes and behaviors.

Below is my attempt at documenting how the library is used by games, in the hopes of improving emulation (and who knows, by documenting it, maybe LLMs will one day understand this library, heh).

A lot of the below applies to both Atrac3+ and Atrac3 - the same library is used. Some details differ though like frame sizes, auxiliary data in the file, etc (Atrac3+ doesn't have any aux data).

EDIT: Actually, this work ended up resulting in a full solution, that seems to work 100%. But I'm leaving this up anyway.

## History of sceAtrac emulation

We first started by using an external library, which required buffering up audio in memory in order to decode, rather than feeding a decoder individual packets. This is bad because we had to fake a lot of the behavior.

When ffmpeg finally added support, we never switched over to using the internal buffering.
Mostly because I didn't realize back then that the entire context is conveniently exposed (see below).

But now, finally, we're getting accurate emulation of the buffering, which is critical for some games that use the library in slightly unusual ways, such as Flatout.

## Loading the sceAtrac library

Some games use sceUtilityLoadModule, and others supply their own libatrac3plus.prx library on disk.

In the latter case, in theory, we could just run the native library (since it talks to sceAudioCodec to do its decoding, and we mostly know how to emulate that) instead of HLE-emulating the library.

However, it wouldn't work with the sceSas integration, and also would be limited to games that supply their own prx. So it's still necessary to have a good HLE implementation.

### Games that use sceUtilityLoadModule:

* Wipeout Pulse
* Many more....

### Games that use sceUtilityLoadAvModule

This was previously not hooked up properly. Worked anyway because our sceAtrac impl doesn't require initialization.

* MotoGP
* Many more....

### Games that supply their own:

* Burnout Legends (1.1)
* Gitaroo Man (1.2)
* Daxter (1.3)
* Flatout (1.5)
* Many more....

### Different versions of the library

Early sceAtrac versions seems to have supported only 4 simultaneous codec contexts, while the current versions support 6.

Or possibly, earlier versions used smaller structs... Needs investigation.

The decoder contexts are stored in an array of 256-byte structs (where the first 128 bytes is the raw codec, the second is the playback state) inside the "BSS" section of the `libatrac3plus.prx` library.

Notably, this requires 0x600 bytes (6 * 256), but an early version of `libatrac3plus.prx` only has 0x520 bytes of BSS...

- 1.1: BSS=0x520 bytes (4 contexts, different data layout)
- 1.2: BSS=0x67c bytes
- 1.3: BSS=0x67c bytes
- 1.5: BSS=0x644 bytes (6 contexts, firmware, gets loaded by `sceUtility*LoadModule`)

## Atrac3/Atrac3+

An Atrac3/Atrac3+ stream is usually stored in a standard WAVE file, which uses the RIFF container format. There's also the possibility of storing it in an AA3 file, a proprietary and simpler format.

WAVE files contain standard looping information. Only the first loop is used.

Most games only use Atrac3+ audio, but a few games (especially early games and PSX ports do use the older, less efficient Atrac3 codec).

### Games using Atrac3 (not plus)

* Patapon
* Mega Man Powered Up
* A lot more...

## Coordinate systems for offsets

There are multiple "coordinate systems" for offsets to be aware of.

* File offsets - Just plain offsets into the .at3 file we're reading or streaming from.
* Frame byte offsets - Like file offsets, but with an additional offset added to skip the header and reach the actual frame data.
* Sample offsets - Offsets measured in *decoded* audio samples, from the first byte of decoded data. Loop points are specified in this system. You can convert from encoded byte offsets to these by dividing by the encoded frame size and multiplying by the decoded frame size.
* Actually played sample offsets - The first X samples are actually skipped and will not be played back. This value is computed in a pretty awkward way, to be documented
* Byte offsets into the memory buffer(s). For the `ALL_DATA_LOADED` and `HALFWAY_BUFFER` modes, these correspond exactly to file offsets. However, when streaming, there's no simple mapping, it's treated like a circular buffer of "frames", and there's a second buffer for the "tail" when looping an internal segment of a file.

The internal state variables uses all these coordinate systems for various things.

There's also a "frame skipping" concept, where (presumably) to warm up the decoder, after initializing and seeking, the decoder will skip a frame or two (depending on the seek position within a frame - and that also depends on the codec!) before actually returning data. This causes a lot of complications in the buffering code.

## Internal state

The status field can have the following values:

* `NO_DATA` = 1: Not yet initialized
* `ALL_DATA_LOADED` = 2: The entire size of the audio data, and filled with audio data
* `HALFWAY_BUFFER` = 3: The entire size is allocated in memory, but only partially filled so far
* `STREAMED_WITHOUT_LOOP` = 4: Streaming through a small circular buffer, without any loop
* `STREAMED_WITH_LOOP_AT_END` = 5: Streaming through a small circular buffer, with the "loop end" at the end of the file
* `STREAMED_WITH_SECOND_BUF` = 6: Streaming through a small circular buffer, with a tail after the loop end (which is contained in the second buffer)
* `LOW_LEVEL` = 8: Not managed, decoding using "low level" API (one packet at a time). Manual looping if needed. By far the easiest to emulate.
* `RESERVED` = 16: Not managed, reserved externally - possibly by sceSas

## Main function documentation

### `sceAtracSetHalfwayBufferAndGetID(bufferPtr, validDataSize, bufferSize)`

Tells the library that you'll supply a partially filled buffer (validDataSize out of bufferSize).

Use the combination `sceAtracGetRemainFrame/(last output of sceAtracDecodeData)`/`sceAtracGetStreamDataInfo`/`sceAtracAddStreamData` to progressively fill it up while it's being played.

This is most useful when you have spare memory, and want to loop the audio without extra I/O later.

If the buffer can fit the whole file, the internal state is set to `HALFWAY_BUFFER` (see above).

You can also use it to only partially initialize a buffer for streaming, GTA does this. In this case, the state will end up in one of the streaming modes, depending on the loop parameters stored in the WAVE header.

Possible resulting states:

* `ALL_DATA_LOADED`
* `HALFWAY_BUFFER`
* `STREAMED_WITHOUT_LOOP`
* `STREAMED_WITH_LOOP_AT_END`
* `STREAMED_WITH_SECOND_BUF`

### `sceAtracSetDataAndGetID(data, size)`

Like `sceAtracSetHalfwayBuffer`, but fills the entire specified buffer.

Size can be any value (down to certain limitations), even byte-aligned! 0x4301 works and doesn't crash.
Games generally uses sizes like 0x18000, 0x10000, etc.

Depending on the file contents and the buffer size, the internal state can end up as any of:

* `ALL_DATA_LOADED`
* `STREAMED_WITHOUT_LOOP`
* `STREAMED_WITH_LOOP_AT_END`
* `STREAMED_WITH_SECOND_BUF`

### `sceAtracGetNextSample(id, *outPtr)`

Returns how many audio samples will be written by the next call to `sceAtracDecodeData`.

### `sceAtracGetMaxSample(id, *outPtr)`

Returns the maximum possible value that `sceAtracGetNextSample` can return (use it to size your buffer, maybe?).

### `sceAtracGetNextDecodePosition(id, *outPtr)`

Returns the sample position (NOT frame position) in the decoded audio track of the next frame to be decoded. Usually not very useful, although is quite suitable for synchronizing events to the audio playback.

Note that at the start of the song, this will have some value like 0x970 because the decoder always skips a certain number of samples at the start before writing them out.

### `sceAtracDecodeData(id, *outPtr, *outNumSamples, *outFinishFlag, *outRemainFrames)`

Decodes some audio data to memory, reading from the previously provided data (if outPtr is null, the decoded data is simply discarded, processing happens as normal).

*outNumSamples is set to the number of (mono or stereo) samples written. The first and last frames, and also possibly frames when looping, are usually short (unless the size and/or loop points are carefully aligned), while all other frames are 0x800 samples (2048), at least for Atrac3+. If you're playing back the result by directly pushing decoded buffers to `sceAudioOutputBlocking`, you're gonna need to move the frame forward within the buffer size, to avoid a glitch. Example:

```c
// (dec_frame == outPtr, maxSamples == 2048, and a pointer to `samples` is passed in to the `outNumSamples` argument)
if (firstFrame && samples < maxSamples) {
    printf("Deglitching first frame\n");
    memmove(dec_frame + (maxSamples - samples) * 4, dec_frame, samples * 4);
    memset(dec_frame, 0, (maxSamples - samples) * 4);
}
```

Note that we need to check for firstFrame, otherwise we might do this to the last frame, which can also be short! In the latter case, you might want to zero the rest of the buffer, or maintain your own ring buffers and feed that to `sceAudioOutputBlocking` (this is necessary for smooth looping, too).

Although of course you don't actually need a bool to determine the first frame, you can also just call `sceAtracGetNextDecodePosition`, though the return value isn't super easy to interpret.

If the end is reached and looping is off, *outFinishFlag is set to 1, otherwise 0 is written (including when looping).

This call is blocking, so while decoding is being processed in the background (by the ME), the main CPU will switch over to other available threads to perform work.

The number of frames remaining (left to decode) in the buffer is returned through *outRemainFrames (exactly the same as calling `sceAtracGetRemainFrame`, so that function is redundant, really).

### `sceAtracGetRemainFrame(id, *outPtr)`

Returns the number of frames that can be decoded from the currently buffered data, or -1 or -2 if you're not
gonna need to load more before the loop (depending on if looping is on). If streaming or half-way filled, when the returned
value goes below a game-specific threshold, the game will load more data into memory so that it doesn't run out.

Note, this same value is also returned through the last parameter to `sceAtracDecodeData`, so it's usually not necessary
to call this one on its own.

### `sceAtracGetBufferInfoForResetting`

This one is a lot less innocent than it looks! It returns the data range it wants you to read in order to call sceAtracResetPlayPosition, but also, if there are any frames to skip, it'll go ahead and skip it, for no apparent reason. It makes no sense that this happens, given that the next thing that gets called actually resets everything again.

### `sceAtracResetPlayPosition(id, buffer, bytesWrittenFirstBuffer, bytesWrittenSecondBuffer)`

This is called after `sceAtracGetBufferInfoForResetting` and then filling the buffers accordingly.

### `sceAtracGetStreamDataInfo`

Call this to figure out what data to read from the file and where in the main buffer to put it.
Then call `sceAtracAddStreamData` to notify sceAtrac about how much data you actually added.

### `sceAtracAddStreamData`

Tells the engine that you loaded the data it wanted.

Note: It's OK to "add" a smaller amount of data than `sceAtracGetStreamDataInfo` asked you to.

### `sceAtracSetSecondBuffer(id, buffer, bufSize)`

This informs sceAtrac about where we want a second buffer to be placed. This is read from in a few circumstances like when looping with a tail (that gets played after the last loop).

### `sceAtracGetChannel(id, *outPtr)`

Returns the number of channels the audio track has.

### `sceAtracGetSoundSample(id, *outEndSample, *outLoopStartSample, *outLoopEndSample)`

Gets the length of the track and the extents of the loop (if any, otherwise -1), counted in output audio sample positions.

### `sceAtracReleaseAtracID(id)`

Releases the Atrac context, free-ing up a bit of memory.

### `_sceAtracGetContextAddress(id)`

Returns a pointer to the internal context data.

The previous Atrac3+ implementation only copied some variables back to this. The new one uses it directly to hold most state.

### `sceAtracReinit`

Don't really understand the purpose of this.

## General calling pattern

During init, you should call sceAtracGetMaxSample to figure out your audio channel buffer size. For use with `sceAudioOutputBlocking` you need to round-robin these, not sure if 2 or 3 are needed, I've seen games use both I think. This gets passed into the second argument of `sceAudioChReserve`.

`sceAtracGetNextSample` returns how many audio samples will be written by the next call to `sceAtracDecodeData`. If it's less than `sceAtracGetMaxSample()` you probably want to offset the output a bit to line up with the next frame to avoid a small glitch.

The game calls `sceAtracDecodeData` with a pointer to a destination buffer to get the next frame(s).

To know when to top up the buffer (if streaming), a game calls `sceAtracGetRemainFrame(id, *u32)` or just checks the last argument to `sceAtracDecodeData`.

When that value goes below a specific threshold, call `sceAtracGetStreamDataInfo` to figure out what data you need to read, and where in memory to read it to. It will be somewhere inside the specified buffer (in SetData). After adding the data, you call `sceAtracAddStreamData` to notify the library of exactly how much data you added.

To seek when streaming, you call `sceAtracGetBufferInfoForResetting` to figure out what to write, then you call
`sceAtracResetPlayPosition`.

## Streaming buffer management details

It just can't work! Well, that was my first reaction. But there's a nasty hack going on.

The first time around when we set up a buffer for streaming using sceAtracSetDataAndGetID(), sceAtrac isn't able to give us any advice on how much data to write, in order to avoid splitting packets. So, what it does is, it'll accept say 0x8000 bytes. As it turns out, at that size, offset 0x8000 is right in the middle of a packet, so it simply copies the first half of the packet right on top of the headers it just read, at the beginning of the buffer.

Subsequently, the next time we call sceAtracGetStreamInfo, it'll apply the appropriate offset, and we'll end up completing the packet! So when decoding comes along, it won't have to read a split packet.

However, if you supply less data than asked, you can still create split packets temporarily, though you should complete them before decoding gets there.

## Usage patterns by games

### All-data-loaded (looping or not)

- MotoGP (menu music with specified loop). Simple repeated calls to sceAtracDecodeData
- Archer MacLean's Mercury (in-game, not menu)
- Crisis Core
- 3rd Birthday (also uses AT3 in addition to AT3+)

### Streaming

- Starts with a halfway initialized buffer:
  - Grand Theft Auto: Vice City Stories

- Good ones for testing (early)
  - Burnout Legends (no loop, 0x1800 buffer size, uses irregular AddData sizes)
  - Everybody's Golf 2 (0x2000 buffer size, loop from end)
  - Suicide Barbie (simple stream, no loop)

- Others
  - Bleach
  - God of War: Chains of Olympus
  - Ape Academy 2 (bufsize 8192)
  - Half Minute Hero (bufsize 65536)
  - Flatout (tricky! needs investigation)
  - Outrun 2006
  - Many, many more...

## Streaming with looping

* Daxter main menu music
* Patapon main menu music

### Low-level decoding

In this case, the game just submits packets to decode to the decoder, sceAtrac doesn't really do anything. This is the easiest mode to emulate and has worked reliably for a long time.

* Digimon Adventures
* Hunter X Hunter

### AA3 file format

As mentioned before, Atrac3+ streams are usually stored in RIFF WAVE containers, but there's also AA3, which is one possible extension for "Sony OpenMG Music". This is used by very, very few games though:

* Mod Nation Racers
* Beats
* A few more, can't find the list anymore

### sceSas integration

There's a way to use Atrac3+ audio as a sound channel in the sceSas software mixer, by using the `__sceSasSetVoiceATRAC3` function.

Some also use __sceSasConcatenateATRAC3(08876d00, 31, 09e77978, 32768) to perform streaming progressive loading. Similarly to normal Atrac, it gets into this mode when you use SetData to set a smaller buffer than can fit the whole file, and then do __sceSasSetVoiceATRAC3 on it. It checks the state for != 2 (so, not fully loaded).

When you call this function, secondBuffer is set to the pointer you passed in. Not sure exactly when, but later when it starts running out of data, secondBuffer is changed to 0xFFFFFFFF (after a call to sasCore). This is your clue to call __sceSasConcatenateATRAC3 again, providing it with more data.

Note: It seems the buffer you feed it from using Concatenate.. has to be double-buffered, you cannot re-use the same one over and over.

This is not that common, known games that are using it:

- Sol Trigger
- Mad Blocker Alpha (mini)
- L.A Gridlock (mini)
- Actually, a bunch more according to reports:
  - https://report.ppsspp.org/logs/kind/193  SetVoice
  - https://report.ppsspp.org/logs/kind/197  Concatenate

### Not using Atrac3+, but instead sequenced MIDI music

- Bust-a-Move Deluxe
- Breath of Fire III
- Every Extend Extra
- ... quite a few more, especially PSX ports.

## Detailed game cases

NOTE: Logs below are from PPSSPP's old sceAtrac implementation, not hardware.

### Suicide Barbie (homebrew)

`STREAMED_WITHOUT_LOOP`

```text
0=sceAtracSetDataAndGetID(09ef9f00, 00040000)
0=sceAtracGetRemainFrame(0, 09ffee24[0000015f])
0=sceAtracGetMaxSample(0, 09ffee3c[00000800])
loop:
0=sceAtracDecodeData(0, 09ef1f00, 09ffee28[00000690], 09ffee38[00000000], 09ffee24[350])
Until remainFrame (last param) <= 175. Then:
0=sceAtracGetStreamDataInfo(0, 09ffee2c[09ef9fa0], 09ffee30[00020228], 09ffee34[00040000])
0=sceAtracAddStreamData(0, 00020000)
goto loop
```

### Archer Maclean's Mercury

`ALL_DATA_LOADED`

```text
0=sceAtracSetData(0, 0952e700, 00177dac)
0=sceAtracSetLoopNum(0, -1)
0=sceAtracDecodeData(0, 09fe1680, 09fdb664[00000180], 09fdb660[00000000], 09fdb668[-1])
loop:
0=sceAtracDecodeData(0, 09fe1680, 09fdb664[00000800], 09fdb660[00000000], 09fdb668[-1])
goto loop
```

### Tekken 6

Uses an unusually small buffer. We match this in our stream.prx test.

```text
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

### Flatout

Uses the FMOD library.

NOTE: Does two setdata! First one just with the header I guess. Then the proper one.

Later, it seems to run out of data just before refilling, even though there should be
~88 buffered frames left. This makes little sense.

This never worked before (see below), but the new implementation fixes this just fine!

```text
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
...
0=sceAtracDecodeData(0, 090014c0, 09fe199c[00000800], 09fe19a0[00000000], 09fe1980[87])
0=sceAtracGetRemainFrame(0, 09fe1980[00000057])
0=sceAtracGetStreamDataInfo(0, 09fe1990[08fe9490], 09fe1994[0000c010], 09fe1998[00018000])
0=sceAtracAddStreamData(0, 0000c010)
0=sceAtracDecodeData(0, 08fdb610, 09fe199c[00000800], 09fe19a0[00000000], 09fe1980[174])
0=sceAtracGetRemainFrame(0, 09fe1980[000000ae])
```

### Grand Theft Auto: Vice City Stories

Uses sceAtracSetHalfwayBufferAndGetID with a very small readSize, and a buffer much smaller
than the size of the file. This results in a streaming state, not a "halfway" state, but differently
initialized. This is now implemented and working in the new sceAtrac implementation.
