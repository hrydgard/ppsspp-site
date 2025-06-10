---
slug: ppsspp-legacy
title: PPSSPP Legacy APK is now available
authors: hrydgard
tags: [releases]
---

![PPSSPP Legacy Edition icon](/static/img/platform/ppsspp-icon-legacy.png)

PPSSPP Legacy for Android is now available as an APK on the [buildbot](/devbuilds), as AndroidLegacy.

This special Android APK targets an older SDK version than the official builds, and is thus unaffected by the Scoped Storage API requirement. This can help various Android TV devices where you are currently unable to pick a folder to load an ISO from.

Note that this can not be uploaded onto Google Play since it doesn't meet current requirements. The only way to install it is to sideload it via ADB or similar. If you know how to use APK files, you know what you need to do.

For more information, see the [reference page](/docs/reference/legacy-edition).
