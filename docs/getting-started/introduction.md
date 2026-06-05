---
position: 1
---
# Introduction to PPSSPP

PPSSPP is a PSP emulator, which means that it can run games and other software that was originally made for the Sony PSP.

The PSP had multiple types of software.
The two most common are native PSP games on UMD discs and downloadable games, that were stored in the directory `PSP/GAME` on the "Memory Stick" (aka "memstick" or "ms").

But there were also [UMD Video](/docs/reference/umd-video) discs, and PS1 games that could run in a proprietary emulator.
PPSSPP does not run those.

## Installing PPSSPP

[See the download options here.](/download)

On Android<img src="/static/img/icons/android.svg" aria-hidden="true" class="icon-24 icon-right">,
you can install PPSSPP like any other app from Google Play or from the HUAWEI AppGallery, if that's what your device has.
If your device doesn't have access to these stores, you can install the app by downloading the APK and then "sideloading" it onto your device.

<!--
For Android TV<img src="/static/img/icons/android.svg" aria-hidden="true" class="icon-24 icon-right">
devices, you may have to sideload the [Legacy Edition](/docs/reference/legacy-edition).
-->

On [iOS](/docs/reference/ios-support)<img src="/static/img/icons/ios.svg" aria-hidden="true" class="icon-24 icon-right">,
you can find PPSSPP on the Apple App Store.

On Windows<img src="/static/img/icons/windows.svg" aria-hidden="true" class="icon-24 icon-right">,
you can use a traditional installer, or you have the option of a "portable" install (just a ZIP file to unzip where you want it).
Make sure you run the 64-bit executable &ndash; `PPSSPPWindows64.exe` &ndash;  unless you're on a 32-bit computer!

[Windows on ARM](/docs/faq/#arm64win)<img src="/static/img/icons/windows.svg" aria-hidden="true" class="icon-24 icon-right">
users should download the native ARM64 build.

A [Universal Windows Platform](/docs/getting-started/how-to-run-on-xbox)<img src="/static/img/icons/uwp.svg" aria-hidden="true" class="icon-24 icon-right">
build, that can be installed even on Xbox consoles, is also available.

On [macOS](/docs/reference/mac)<img src="/static/img/icons/macos.svg" aria-hidden="true" class="icon-24 icon-right">, it's a standard `.dmg` install.
Open the DMG and drag the app to Applications.

On Linux<img src="/static/img/icons/linux.svg" aria-hidden="true" class="icon-24 icon-right">,
we have a Flatpak package, and an AppImage that's suitable for most distributions.
Manual build from source is also possible.

PPSSPP is also available to sideload onto [Nintendo Switch](/legacybuilds)<img src="/static/img/icons/switch.svg" aria-hidden="true" class="icon-24 icon-right">
and [compatible Android VR devices](/docs/reference/vr-apk)<img src="/static/img/icons/vr.svg" aria-hidden="true" class="icon-24 icon-right">.

## Getting games

See [How to get games](/docs/getting-started/how-to-get-games).

## Setting up PPSSPP

Generally, the defaults should be fine, unless you want to set up controllers and so on.
However, PPSSPP has a lot of functionality which means a lot of different settings.
See the [Settings section](/docs/settings).

## Updating PPSSPP

New releases are made 2-3 times per year.

On Android, if you installed PPSSPP from the Play or HUAWEI AppGallery, the app will auto update from time to time.
If you installed via APK, you'll have to manually update by simply installing the new version on top of the old one.

On PC, if you're using an installer, just download the new installer and run it,
or alternatively if you do manual ZIP installs, just unzip the new version on top of the old one, overwriting files as you go.
This will not touch your save data.

Similarly on Mac, just download the new version and install it on top of the old one.
Your data will not be lost.

See [Updating PPSSPP](/docs//reference/how-to-update) for a more detailed explanation.

## Save data

The original PSP saved data in a directory structure on the Memory Stick you had plugged into the device.
PPSSPP mimics this by designating a folder as the Memory Stick and handling it exactly the same as the original hardware.

To find this folder on Windows or macOS, simply choose `File -> Open Memory Stick...` in the menubar.

Alternatively, and on other platforms, go to <b class="inapp">Settings -> System -> Show Memory Stick folder</b>.

On Android, on Android 10 and older, the Memory Stick will simply be `/PSP` in your USB-accessible storage partition.
However, on Android 11 or newer, you have to create and choose a folder yourself.
I recommend creating a `PSP` folder in the root of your storage, but it's up to you.
See [Save data and storage](/docs/getting-started/save-data-and-storage) for more information.
