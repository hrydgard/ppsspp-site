# Upgrading PPSSPP

New versions are released from time to time. Depending on the platform, upgrading is usually pretty easy, but there are some concerns to be aware of.

## Upgrading PPSSPP for Windows

If you've installed PPSSPP by downloading the zip file and unzipped it somewhere, you have two paths:

* Unzip the new version in another folder, and move or copy over the "memstick" subfolder to the new PPSSPP folder. That's what contains the save games.
* Unzip the new version directly into your existing PPSSPP folder, overwriting it.

Either will work, matter of taste.

If you've been using the installer before and are using the installer again, it should just work to install the new version on top of the old one.

## Upgrading PPSSPP for Android

If you've installed from Google Play, upgrading is handled automatically by Google Play on your devices.

However, if you installed the APK downloaded from ppsspp.org, it will not automatically update, so you have to manually download an install a new APK to upgrade.

Additionally, if you have downloaded a development build from the [build bot](https://buildbot.orphis.net/ppsspp/), it's not compatible with either APKs from ppsspp.org or builds from Google Play, so you have to first uninstall and then reinstall to switch type of build. To avoid losing save data, make sure to have manually specified a memory stick folder so that it won't be deleted along with the app, when you do.

## PPSSPP Gold

Both on Android and PC, PPSSPP Gold can be installed side by side with the regular PPSSPP. If you make them share PSP (memstick) directories though, they may have problems sharing save states due to the above-mentioned backwards compatibility issue.

## General things to consider

* Save states from newer versions are usually not compatible with older versions. Regular, in-game saves are though, so it's highly recommended to do a real save before switching versions (or at any time, really).
