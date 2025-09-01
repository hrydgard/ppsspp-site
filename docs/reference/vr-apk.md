# PPSSPP VR

PPSSPP VR is a special build of the emulator for Android-based VR devices such as the Quest 2 or Pico, based on OpenXR.

The goal is universal compatibility with such devices, but there are some device specific quirks requiring extra effort, so it might not work on others than the ones listed below (try it!).

The VR build was created and contributed by [Luboš V aka lvonasek](https://github.com/lvonasek).

The latest official version can be found as an APK on Sidequest or on the [Downloads page](/downloads), while the latest development version can be found [here](/devbuilds). It must be sideloaded, it cannot be installed from the official stores.

## Supported devices

* Meta Quest 2 and higher (Pro, 3, 3s, etc)
* Pico Neo 3 and higher (Pico 4, Pico 4 Ultra, Pro, etc)

## Luboš' notes

The goal is to have one APK for all. But the standard is still shaping and although OpenXR makes everything easier. The single vendors tend to add some additional requirements for their platform.

E.g. Quest 1 is unsupported because it requires a modified OpenXR loader. So we support Meta Quest 2 and higher (which means Quest 2, Pro, 3, 3s).

With Pico it is similar story. We support Pico Neo 3 and higher (which means Pico Neo 3, Pico 4 and Pico 4 Ultra + all the pro/enterprise variants).

Google Cardboard is not a proper VR platform and is a different thing.