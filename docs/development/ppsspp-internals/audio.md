# sceAudio - raw audio output

To actually output audio you don't just need to decode it with sceAtrac or mix it with sceSas, but you need to actually play it back. The output from sceSas or sceAtrac is normally passed directly to channels of the sceAudio interface.

It has 8 "normal" channels, 0-7, that are simply 44.1kHz PCM channels with individual stereo volume controls that are mixed together somehow and then just played back. Usually, one is used for playing back audio mixed with sceSas, and another one is used for sceAtrac playback, but it's common to see other setups like multiple atrac channels being played together.

In addition there's an 9th channel available (number 8, but this isn't visible in the API) with some special features in later firmware versions. On it, you can active sample rate conversion. There's also a simpler Audio2 interface for this, for some reason.
