# How to run the XMB (VSH)

The **Xross Media Bar**, or **Visual Shell (VSH)** is the PSP's main UI.

<video class="video" preload="metadata" controls playsinline>
    <source src="/video/vshdemo.mp4" type="video/mp4">
    <p>Your browser does not support HTML video.
        Here is a direct <a href="/video/vshdemo.mp4" download>link to the video</a> instead.
    </p>
</video>

Historically we have not bothered to implement support for it, because it does require quite a lof of funcionality that games do not need, and it's actually almost completely useless.
But for some people it might be really nostalgic to just run it and navigate around in the menu a bit!
So here we are &ndash; from version 1.21, PPSSPP will support running the VSH, in certain configurations.

<div class="alert alert-warning">This functionality will be available in PPSSPP 1.21, which is NOT out yet!</div>

You can also download the latest [development build](/devbuilds), which does include this functionality.

## Getting and installing the required files

First, you need a PSP update.
You can download 6.61 [directly from Sony](http://du01.psp.update.playstation.org/update/psp/image/us/2014_1212_6be8878f475ac5b1a499b95ab2f7d301/EBOOT.PBP).

Next, just open the update in PPSSPP, which will offer to install it for you. Let it.

You no longer have to switch to the Interpreter if you are using a build newer than `v1.20.4-1365`.
However, the IR Interpreter will not work and the IR JIT is untested.
Only Interpreter and JIT work.

## Running it

1. On Windows, choose `File -> Load the VSH` from the menubar.
   On other platforms you currently need to navigate to `PSP/NAND/flash0/vsh/module/vshmain.prx` and run that (this will be made easier).

1. Enjoy playing around!
   NOTE: None of this will work on PPSSPP versions before 1.21, except late development builds before it.
