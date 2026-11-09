# How to run the XMB

The **Xross Media Bar**, or **Visual Shell (VSH)** is the PSP's main UI.

<video class="video" preload="metadata" controls playsinline>
    <source src="/video/vshdemo.mp4" type="video/mp4">
    <p>Your browser does not support HTML video.
        Here is a direct <a href="/video/vshdemo.mp4" download>link to the video</a> instead.
    </p>
</video>

<div class="alert alert-warning">This functionality will be available in PPSSPP 1.21, which is NOT out yet!</div>

From version 1.21, PPSSPP will support running the XMB, or VSH (Visual Shell) as it's known internally.

You can also download the latest [development build](/devbuilds), which does include this functionality.

## Getting the firmware

The XMB is part of the PSP's firmware. If you're running 1.21 and have tried some games in it, you probably already have it installed. It can be installed the following ways:

* **Automatically from your games.** Almost every PSP game disc carries a firmware updater. From 1.21, when you start a game, PPSSPP installs the firmware from its disc if you don't have one yet, or if the disc's is newer than the one you have. You can turn this off in Settings/Tools/PSP Firmware. A game's info screen also has an "Install PSP firmware update" choice.
* **From an official update.** You can download the latest one, 6.61, [directly from Sony](http://du01.psp.update.playstation.org/update/psp/image/us/2014_1212_6be8878f475ac5b1a499b95ab2f7d301/EBOOT.PBP). Open the update in PPSSPP, and it'll offer to install it. You can also use "Install PSP firmware update" in Settings/Tools/PSP Firmware.

## Running it

1. Go to Settings / Tools / PSP Firmware and choose **Launch XMB**. On Windows and Mac, you can also choose `File -> Load XMB (VSH)` from the menu bar.

1. Enjoy playing around! You can launch homebrew and PS1 games from the Game menu, and they return to the XMB when you quit them. To put a UMD game or UMD Video disc in the drive, use Insert UMD on the pause screen.

Save states don't work while the XMB is running.

NOTE: None of this will work on PPSSPP versions before 1.21, except late development builds before it.

## File locations on the memory stick

If you want to use the XMB as a media player, you can, although it doesn't make too much sense to do so on modern hardware, unless you really like the PSP's music visualizer, for example.

NOTE: Folders directly under the root is only possible on Android if your memstick directory is NOT named PSP.
Otherwise the PSP directory is the PSP directory inside the memstick, so you can't place stuff outside (it's a bit complicated for legacy reasons).

## Downloadable apps

The app download mechanism isn't currently working, but you can for example install SensMe (obtained separately) to the memory stick the normal way (put its two files, EBOOT.PBP and SENSME.DAT in `PSP/APP/NPIA00013`), and it will launch, both from the XMB and from the Homebrew and Demos tab in PPSSPP.

### Music

Put your MP3 and other music files in any of these memstick locations:

* `/PSP/MUSIC`
* `/MUSIC`

### Pictures

* `/PICTURE`
* `/PSP/PHOTO`

### Video

* `/VIDEO`

Possibly more directories work. Files must have dimensions lower or equal to 720x480. Must be an:

* MP4 H.264 Baseline/Main with AAC audio
* AVI MJPEG with μ-Law(mulaw) or LPCM audio
* MP4 MPEG-4 Part 2 Simple Profile (Not tested!), AAC audio

So not that many video will actually work.

Thumbnails are JPEGs with filenames corresponding to video and with .THM extension instead of .JPG (e.g. output.mp4 - output.thm), dimensions 160x120.

#### Encoding video
If you want to encode a video to AVI format with PPSSPP playback, use this ffmpeg command line:

Encoding example by mavemvl with thumbnail:

```ffmpeg -i input.mp4 -c:v mjpeg -c:a pcm_s16le -s 480x272 output.avi -vf "thumbnail,scale=160:120" -c:v mjpeg -frames:v 1 -f image2 output.thm```

Without:

```ffmpeg -i input.mp4 -c:v mjpeg -c:a pcm_s16le -s 480x272 output.avi```



If you want to encode video for playback in PPSSPP and on the PSP, use these ffmpeg command lines.


Encoding example by Ahayri:

```ffmpeg -i "input.mp4" -c:v libx264 -profile:v main -level 3.0 -pix_fmt yuv420p -x264-params "ref=3:bframes=1:b-pyramid=none" -b:v 700k -vf "scale=480:272" -c:a aac -b:a 128k -movflags +faststart -f psp output.mp4```

With thumb:

```ffmpeg -i "input.mp4" -c:v libx264 -profile:v main -level 3.0 -pix_fmt yuv420p -x264-params "ref=3:bframes=1:b-pyramid=none" -vf "scale=480:272" -movflags +faststart -f psp output.mp4 -ss 00:01:00 -vframes 1 -s 160x120 -f image2 -c:v mjpeg output.thm```

Tweaked by mavemvl, also generates a thumbnail:

```ffmpeg -i input.mp4 -c:v libx264 -profile:v main -level 3.0 -pix_fmt yuv420p -x264-params "ref=3:bframes=1:b-pyramid=none" -vf "scale=480:272" -movflags +faststart -f psp output.mp4 -vf "thumbnail,scale=160:120" -frames:v 1 -f image2 -c:v mjpeg output.thm```
## Themes
You can customize your VSH with custom themes made by community.
Simply put them in `/PSP/THEME/` and select them in the **Theme Settings** in the VSH.
<div class="alert alert-warning>If you are running devbuild earlier than 4555, you <b>NEED</b> to create (yes, just to create) a folder <code>flash1/vsh/theme</code> in your firmware folder.</div>
The only supported format is PTF because CTF requires custom firmware to work and PPSSPP doesnt support that yet.