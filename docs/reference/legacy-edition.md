# PPSSPP Legacy Edition for Android

![Create a new folder](/static/img/platform/ppsspp-icon-legacy.png)

The new PPSSPP Legacy Edition is the same as a regular PPSSPP build, except it has been built specifying an old target Android SDK version (29, specifically).

This means that it's not affected by the Scoped Storage requirement of Android 11+, meaning that it can work without a folder browser app being available on your device, which helps on Android TV devices.

When you install it, you may be asked to confirm that you really want to install it, since this app does not live up to modern Android security requirements. This is fine.

However, PPSSPP built this way cannot be uploaded to the Play Store due to Google's restrictions, and must be side-loaded.

The Legacy Edition can be build from source code using the gradle command assembleLegacyOptimized. Alternatively, the [buildbot](/devbuilds) will soon build it for you.
