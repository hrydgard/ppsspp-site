# How to run the XMB (VSH)

The **Xross Media Bar**, or **Visual Shell (VSH)** is the PSP's main UI.

<video class="video" preload="metadata" controls playsinline>
    <source src="/video/vshdemo.mp4" type="video/mp4">
    <p>Your browser does not support HTML video.
        Here is a direct <a href="/video/vshdemo.mp4" download>link to the video</a> instead.
    </p>
</video>

<div class="alert alert-warning">This functionality will be available in PPSSPP 1.21, which is NOT out yet!</div>

From version 1.21, PPSSPP will support running the VSH.

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
