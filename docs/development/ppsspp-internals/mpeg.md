# MPEG / PSMF

The PSP supports playback of video encoded with a single video codec, AVC, or as it's more popularly known, H.264. I believe only "base profile" is supported (H.264 has a LOT of features).

A lot of games use video clips for things like title screens and also cutscenes, some even as background for gameplay. Additionally, the PSP supports UMD video discs, basically tiny Blu-Rays, although PPSSPP does not yet support playback of these (it's complicated).

## sceMpeg

To this end, there's a system library called `sceMpeg`.

For audio, the Sony proprietary Atrac3+ codec is used. Playback is done directly through an owned sceAudiocodec context, rather than going through [sceAtrac](/docs/development/ppsspp-internals/atrac).

Video (h.264 or "AVC") is decoded using the sceVideocodec library. Unlike sceAudiocodec (which I've mostly figured out how it works), the details of sceVideocodec are entirely unknown at this point.

## scePsmf / scePsmfPlayer

[PSMF](https://wiki.multimedia.cx/index.php/PSMF) is a proprietary Sony container for otherwise the same video format as supported by sceMpeg, that is, H.264 video with Atrac3+ audio.

`scePsmf` is a simple format parser, and `scePsmfPlayer` is a full-on player utility library that uses `scePsmf` and also delegates all the actual decoding work to sceMpeg.

Since #20200 was merged, we simply allow games to load `scePsmf` and `scePsmfPlayer` from disc (they are only shipped with games, not present in the firmware) and just run them directly. That way, we don't need to emulate them, and they are completely accurate. They forward the task of actual decoding to sceMpeg, so remaining video playback problems are related to our sceMpeg emulation.

## Example game usage

(more to be added)

scePsmf:

  * Jackass
  * LocoRoco 2
  * Socom Fireteam Bravo 3 (#18094)
  * Parappa The Rapper
  * Machi: Unmei no Kousaten (ULJM05111)

sceMpeg:
  * GTA: Vice City Stories (initial logos)
  * Crisis Core
  * Wipeout Pulse