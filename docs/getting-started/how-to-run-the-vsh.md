# How to run the XMB (VSH)

The **Xross Media Bar**, or **Visual Shell (VSH)** is the PSP's main UI.

<video class="video" preload="metadata" controls playsinline>
    <source src="/video/vshdemo.mp4" type="video/mp4">
    <p>Your browser does not support HTML video.
        Here is a direct <a href="/video/vshdemo.mp4" download>link to the video</a> instead.
    </p>
</video>

Historically we have not bothered to implement support for it, because it does require quite a lot of functionality that games do not need, and it's actually almost completely useless.
But for some people it might be really nostalgic to just run it and navigate around in the menu a bit!
So here we are &ndash; from version 1.21, PPSSPP will support running the VSH.

<div class="alert alert-warning">This functionality will be available in PPSSPP 1.21, which is NOT out yet!</div>

You can also download the latest [development build](/devbuilds), which does include this functionality.

## Getting the firmware

The XMB is part of the PSP's firmware, so PPSSPP needs a copy of one. You most likely already have it without doing anything:

* **From your games.** Almost every PSP game disc carries a firmware updater. From 1.21, when you start a game, PPSSPP installs the firmware from its disc if you don't have one yet, or if the disc's is newer than the one you have. You can turn this off in Settings / Tools / PSP Firmware ("Install or upgrade firmware from game discs"). A game's info screen also has an "Install PSP firmware update" button, if the disc has one.
* **From an official update.** You can download the last one, 6.61, [directly from Sony](http://du01.psp.update.playstation.org/update/psp/image/us/2014_1212_6be8878f475ac5b1a499b95ab2f7d301/EBOOT.PBP). Open the update in PPSSPP, and it'll offer to install it. You can also use "Install PSP firmware update" in Settings / Tools / PSP Firmware.

In 1.21, every firmware version from 1.50 to 6.61 boots to the XMB. Installing a new one replaces the old one. Settings / Tools / PSP Firmware shows which version you have, and lets you uninstall it.

## Running it

1. Go to Settings / Tools / PSP Firmware and choose **Launch XMB**. On Windows and Mac, you can also choose `File -> Load XMB (VSH)` from the menu bar.

1. Enjoy playing around! You can launch homebrew and PS1 games from the Game menu, and they return to the XMB when you quit them. To put a UMD game or UMD Video disc in the drive, use Insert UMD on the pause screen.

Save states don't work while the XMB is running.

NOTE: None of this will work on PPSSPP versions before 1.21, except late development builds before it.
