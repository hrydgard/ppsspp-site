# How to update PPSSPP

New versions are released from time to time. Depending on the platform, upgrading is usually pretty easy, but there are some concerns to be aware of.

## Android

If you've installed from Google Play, upgrading is handled automatically by Google Play on your device.

However, if you installed the APK downloaded from ppsspp.org, it will not automatically update, so you have to manually download an install a new APK to upgrade.

In summary:

* If you updated by Google Play last time, do the same now.
* If you installed an APK from the [buildbot](/devbuilds), you'll have to install another one from the build bot.
* Or if you downloaded an APK directly from the [Downloads page](/download), do that again.

If you want to install by another method than before, or downgrade to an older version, you need to uninstall first (in which case you should make sure to [backup your save games](/docs/getting-started/save-data-and-storage)), since the digital signatures are not compatible.

## Upgrading PPSSPP for GNU/Linux

### AppImage (suitable for most GNU/Linux distributions)

#### AM

[AM](https://github.com/ivan-hc/AM) is a powerful command-line package manager for managing AppImage software. It allows users to install, update, and remove AppImages either system-wide or locally without manual setup.

1. Installation

There are no system packages available for AM in common repositories, so it must be installed manually.

Follow the official installation instructions provided at: https://github.com/ivan-hc/AM

During setup, you’ll be prompted to choose one of two installation modes:

* Option 1: AM – for system-wide AppImage management (requires root privileges).
* Option 2: appman – for local, user-level AppImage management (no root access required).

Choose the option that best suits your environment and permissions.


2. Searching for AppImages

Once installation is complete, verify that AM or AppMan is working by performing a search.
Use the appropriate command depending on your installation:


```
# For system-wide installation
am -q ppsspp

# For user-level installation
appman -q ppsspp
```

This query searches for available AppImages matching “ppsspp” and displays a list of results.
Example output:

```
$ appman -q ppsspp

 SEARCH RESULTS FOR "PPSSPP":

 ◆ ppsspp : PSP emulator written in C++.
```

## Upgrading PPSSPP for Windows

If you've installed PPSSPP by downloading the zip file and unzipped it somewhere, you have two paths:

* Unzip the new version in another folder, and move or copy over the "memstick" subfolder to the new PPSSPP folder. That's what contains the save games.
* Unzip the new version directly into your existing PPSSPP folder, overwriting it.

Either will work, matter of taste.

If you've been using the installer before and are using the installer again, it should just work to install the new version on top of the old one.

## PPSSPP Gold

Both on Android and PC, PPSSPP Gold can be installed side by side with the regular PPSSPP. If you make them share PSP (memstick) directories though and keep different version of them, they may have problems sharing save states due to the above-mentioned backwards compatibility issue.

## General things to consider

* Save states from newer versions are usually not compatible with older versions, although saves from older versions are generally compatible with newer versions. Regular, in-game saves are though, so it's highly recommended to do a real save before switching versions (or as often as possible, really).
