# How to run the XMB (VSH)

The Xross Media Bar, or "Visual Shell (VSH)" is the PSP's main UI.

<video controls style="width: 100%; max-width: 600px; height: auto; border-radius: 8px;">
  <source src="/video/vshdemo.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>

Historically we have not bothered to implement support for it, because it does require quite a lof of funcionality that games do not need, and it's actually almost completely useless. But for some people it might be really nostalgic to just run it and navigate around in the menu a bit! So here we are - from version 1.21, PPSSPP will support running the VSH, in certain configurations.

<div class="alert alert-warning">This functionality will be available in PPSSPP 1.21, which is NOT out yet!</div>

You can also download the latest [development build](/devbuilds), which does include this functionality.

## Getting and installing the required files

First, you need a PSP update. You can download 6.61 directly from Sony [here](http://du01.psp.update.playstation.org/update/psp/image/us/2014_1212_6be8878f475ac5b1a499b95ab2f7d301/EBOOT.PBP).

Next, just open the update in PPSSPP, which will offer to install it for you. Let it.

Then, go to settings/tools/developer tools and change CPU Core to Interpreter (remember to change it back once you want to play games!).

## Running it

1. On Windows, choose "Load the VSH" from the file menu. On other platforms you currently need to navigate to PSP/NAND/flash0/vsh/module/vshmain.prx and run that (this will be made easier).

2. Enjoy playing around! NOTE: None of this will work on PPSSPP versions before 1.21, except late development builds before it.
