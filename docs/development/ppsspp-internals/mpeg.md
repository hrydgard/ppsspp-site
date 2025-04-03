# MPEG / PSMF

The PSP supports playback of video encoded with a single video codec, AVC, or as it's more popularly known, H.264. I believe only "base profile" is supported (H.264 has a LOT of features).

A lot of games use video clips for things like title screens and also cutscenes, some even as background for gameplay. Additionally, the PSP supports UMD video discs, basically tiny Blu-Rays, although PPSSPP does not yet support playback of these (it's complicated).

## sceMpeg

To this end, there's a system library called `sceMpeg`.

For audio, the Sony proprietary Atrac3+ codec is used. Playback is done directly through an owned sceAudioCodec context, rather than going through [sceAtrac](/docs/development/ppsspp-internals/atrac).

## scePsmf / scePsmfPlayer

[PSMF](https://wiki.multimedia.cx/index.php/PSMF) is a proprietary Sony container for otherwise the same video format as supported by sceMpeg, that is, H.264 video with Atrac3+ audio.

`scePsmf` is a simple format parser, and `scePsmfPlayer` is a full-on player utility library that uses `scePsmf` and also delegates all the actual decoding work to sceMpeg.

I think it would actually be possible to simply allow games to load `scePsmf` and `scePsmfPlayer` from disc (they are only shipped with games, not present in the firmware) and just run them, if we can get all the details correct. Experimenting with this: #20200



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