# The PSP Media Engine and its implementation

The PSP has a separate CPU called the Media Engine. From a game's perspective, it's only accessible through high-level libraries.

The overall architecture looks something like this:

* Games call into high level libraries that handle media formats and containers. These are [sceMpeg](/docs/development/ppsspp-internals/mpeg), [sceAtrac](/docs/development/ppsspp-internals/atrac), sceMp4, sceMp3 etc. scePsmfPlayer is a special case that itself is a wrapper around sceMpeg (scePsmf is just a parser).
* The high level libraries call into the sceVideocodec and sceAudiocodec libraries.
* sceVideoCodec/sceAudioCodec call into the ME driver libraries.
* Those communicate somehow with the actual ME CPU, and then deliver the decoded results back up the stack.

Some of these are exclusively shipped with games, like scePsmf/scePsmfPlayer. Others like sceAtrac or sceMp3 are also present in the firmware, and games can either ship their own on disc or load the latest version from the firmware directly.

For reverse engineering purposes, psdevwiki has a [list of where modules are](https://www.psdevwiki.com/psp/Modules). To get the prx files, dump them from your own PSP using psardumper.

## Playback

sceAudio is used for playback. We have it mostly implemented with some caveats - the built-in sample rate conversion (sceAudioSRCChReserve) doesn't seem to be implemented correctly.

## Emulation

In PPSSPP traditionally we have emulated the high-level libraries directly.

However, I'm now looking into if we should go one or two levels down the stack.

scePsmf/scePsmfPlayer should really not be emulated, instead we should run the implementations that are present on disc, and make sure that our sceMpeg is accurate enough to work with with them. In 1.19, we do this, eliminating a bunch of bugs.

Additionally, sceAtrac/sceAac/sceMp3 should all be allowed to run when possible, and the underlying sceAudiocodec will take care of the actual decoding. This is partially working but not enabled yet - and many games load these libraries from the firmware, so we need a HLE fallback.