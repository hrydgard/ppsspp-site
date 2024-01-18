---
position: 1
---
# Introduction to PPSSPP

PPSSPP is a PSP emulator, which means that it can run games and other software that was originally made for the Sony PSP.

The PSP had multiple types of software. The two most common are native PSP games on UMD discs and downloadable games (that were stored in the directory PSP/GAME on the "memory stick"). But there were also UMD Video discs, and PS1 games that could run in a proprietary emulator. PPSSPP does not run those.

## Installing PPSSPP

On Android, you can install PPSSPP like any other app from the Google Play Store or from the Huawei store, if that's what your device has. If your device doesn't have access to these stores, you can install the app by downloading the APK and then "side-loading" it onto your device.

On PC, you have the option of a "portable" install (just a zip file to unzip where you want it) or you can use a traditional installer.

On Mac, it's a standard .dmg install (open the dmg, drag the app to Applications).

On Linux, we have a flatpak. Manual install from source is also possible.

## Getting games

See [How to get games](/docs/getting-started/how-to-get-games).

## Setting up PPSSPP

Generally, the defaults should be fine, unless you want to set up controllers and so on. However, PPSSPP has a lot of functionality which means a lot of different settings. See the [Settings section](/docs/settings).

## Updating PPSSPP

On Android, if you installed PPSSPP from the Play or Huawei store, the app will auto update from time to time. New releases are made 2-3 times per year. If you installed via APK, you'll have to manually update by simply installing the new version on top of the old one.

On PC, if you're using an installer, just download the new installer and run it, or alternatively if you do manual ZIP installs, just unzip the new version on top of the old one, overwriting files as you go. This will not touch your save data.

Similarly on Mac, just download the new version and install it on top of the old one. Your data will not be lost.

## Save data

The original PSP saved data in a directory structure on the memory stick you had plugged into the device. PPSSPP mimics this by designating a folder as the memory stick and handling it exactly the same as the original hardware.

To find this folder on Windows or Mac, simply choose File->Open Memory Stick... in the menu.

Alternatively, and on other platforms, go to Settings/System/Show Memory Stick Folder...

On Android, on Android 10 and older, the memory stick will simply be /PSP in your USB-accessible storage partition. However, on Android 11 or newer, you have to create and choose a folder yourself. I recommend creating a PSP folder in the root of your storage, but it's up to you. See [Save data and storage](/docs/getting-started/save-data-and-storage) for more information.
