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

#### zsync

To upgrade from PPSSPP 1.20.2 to 1.20.3 using `zsync`, you let `zsync` patch your existing 1.20.2 AppImage from the 1.20.3 `.zsync` metadata file, so you reuse most of the old download and only fetch the changed bytes. [github](http://github.com/hrydgard/ppsspp/releases/tag/v1.19.3)

##### 1. Put the files in the same directory

Assume you already have:

- `PPSSPP-v1.20.2-anylinux-x86_64.AppImage` (your current one)
- `PPSSPP-v1.20.3-anylinux-x86_64.AppImage.zsync` (the 1.20.3 delta file)

Put both in the same folder, for example:

```bash
~/Downloads/PPSSPP/
```

##### 2. Run `zsync` from the terminal

zsync is a delta‑sync tool that lets you efficiently update an AppImage by downloading only the changed parts of the file, instead of re‑downloading the entire new version. PPSSPP distributes .zsync files alongside its AppImage releases so users can incrementally update to newer versions without redownloading the full binary.

To upgrade from PPSSPP 1.20.2 to 1.20.3 using zsync, you let zsync patch your existing 1.20.2 AppImage from the 1.20.3 .zsync metadata file, so you reuse most of the old download and only fetch the changed bytes.

1. Put the files in the same directory

Assume you already have:
* PPSSPP-v1.20.2-anylinux-x86_64.AppImage (your current one)
* PPSSPP-v1.20.3-anylinux-x86_64.AppImage.zsync (the 1.20.3 delta file)

Put both in the same folder, for example:

```
~/Downloads/PPSSPP/
```

2. Run zsync from the terminal

Install `zsync` if you don’t have it:

- Debian/Ubuntu/etc.:
```
sudo apt install zsync
```

Then navigate to the folder and run:

```
cd ~/Downloads/PPSSPP
zsync PPSSPP-v1.20.3-anylinux-x86_64.AppImage.zsync
```

This will:

- Use the existing `PPSSPP-v1.20.2-anylinux-x86_64.AppImage` as the base.
- Download only the differences referenced by the `.zsync` file.
- Overwrite the old file (or produce `PPSSPP-v1.20.3-anylinux-x86_64.AppImage`) depending on how the `.zsync` is configured. [github](https://github.com/hrydgard/ppsspp/actions/runs/16994040280/workflow)

##### 3. Mark the new file executable and run it

If the result is a new filename (e.g., `PPSSPP-v1.20.3-anylinux-x86_64.AppImage`):

```bash
chmod +x PPSSPP-v1.20.3-anylinux-x86_64.AppImage
./PPSSPP-v1.20.3-anylinux-x86_64.AppImage
```

If you want to keep 1.20.2 around, manually copy the old file before running `zsync`:

```bash
cp PPSSPP-v1.20.2-anylinux-x86_64.AppImage PPSSPP-v1.20.2-old.AppImage
zsync PPSSPP-v1.20.3-anylinux-x86_64.AppImage.zsync
```

That’s it; you’ve effectively “upgraded” the AppImage from 1.20.2 to 1.20.3 using the zsync file.


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
