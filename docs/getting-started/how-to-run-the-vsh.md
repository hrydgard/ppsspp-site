# How to run the XMB (VSH)

The Xross Media Bar, or "Visual Shell (VSH)" is the PSP's main UI.

<video controls style="width: 100%; max-width: 600px; height: auto; border-radius: 8px;">
  <source src="/video/vshdemo.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>

Historically we have not bothered to implement support for it, because it does require quite a lof of funcionality that games do not need, and it's actually almost completely useless. But for some people it might be really nostalgic to just run it and navigate around in the menu a bit! So here we are - from version 1.21, PPSSPP will support running the VSH, in certain configurations.

<div class="alert alert-warning">This functionality will be available in PPSSPP 1.21, which is NOT out yet!</div>

## Getting and installing the required files

To get the required files the most reliable way, you'll need a physical PSP with custom firmware installed.

1. Get a PSP firmware update somewhere (it's available inside many game ISO files). Rename it to EBOOT.PBP and put it in the root of your PSP's memory stick.

2. Get the app "psardumper" and run it on your PSP the usual way (put it in PSP/GAME on the memory stick).

3. When the app is running, press the square button. It will decrypt the firmware and put it in a folder called F0.

4. Put the contents of the folder either directly in assets/flash0 if that's possible on your platform (it will ask you to overwrite some font files - let it), or if on something like Android or iOS, put the files in in /PSP/SYSTEM/flash0. You'll recognize the files by there being a "reboot.bin" and a bunch of variants of it, and a few folders: `codepage`, `data`, `dic`, `font`, `kd`, and `vsh`. There might also be a PSARDUMPER folder inside, which will contain a bunch of file that we do not yet need.

## Running it

1. Start up PPSSPP, and on Windows, choose "Load the VSH" from the file menu. On other platforms you currently need to navigate to flash0/vsh/module/vshmain.prx and run that (this will be made easier).

2. Enjoy playing around! NOTE: None of this will work on PPSSPP versions before 1.21, except late development builds before it (not yet released).

## Future

There might be more ways to install this later - ideally we'll be able to just unpack a firmware update file directly. In that case all you'll need is a PSP game with an update on board.