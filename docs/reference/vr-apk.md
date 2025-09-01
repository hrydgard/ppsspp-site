# PPSSPP VR

PPSSPP VR is a special build of the emulator for Android-based VR devices such as the Quest 2 or PICO, based on OpenXR.

The goal is universal compatibility with such devices, but there are some device specific quirks requiring extra effort, so it might not work on others than the ones listed below. (Try it!)

The VR build was created and contributed by [Luboš V aka lvonasek](https://github.com/lvonasek).

The latest official version can be found as an APK on [SideQuest](https://sidequestvr.com/app/12379/ppsspp-vr) or on the [Downloads page](/download), while the latest development version can be found [here](/devbuilds).
It must be sideloaded, it cannot be installed from the official stores.

## Supported devices

* Meta Quest 2 or newer (Pro, 3, 3S)
* Play for Dream MR <sup>[since 1.19.3-570](https://github.com/hrydgard/ppsspp/pull/20769)</sup>
* PICO Neo3 or newer (PICO Neo3 Pro / ProEye, PICO Neo3 Link, PICO 4, PICO 4 Ultra, PICO 4 Ultra Enterprise)

Support for (Oculus) Quest 1 is discontinued. The last release to work on it is v1.17.1.

## Known limitations

- Currently there's no text input support. You need to manually edit `ms:/PSP/SYSTEM/ppsspp.ini` to change string settings for multiplayer etc.

## Luboš' notes

The goal is to have one APK for all. But the standard is still shaping and although OpenXR makes everything easier, the single vendors tend to add some additional requirements for their platform.

E.g. Quest 1 is unsupported because it requires a modified OpenXR loader. So we support Meta Quest 2 and higher (which means Quest 2, Pro, 3, 3S).

With PICO it is similar story. We support PICO Neo 3 and higher (which means PICO Neo 3, PICO 4 and PICO 4 Ultra + all the pro/enterprise variants).

Google Cardboard is not a proper VR platform and is a different thing.
