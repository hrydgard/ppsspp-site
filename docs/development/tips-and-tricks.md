# Development tips & tricks

## Installing on very old phones that Android Studio won't talk to

For ancient phones that can no longer install directly from Android Studio, choose:

* Build->Build Bundles/APKs/Build APK

Then run:

```sh
adb install android/build/outputs/apk/normal/optimized/android-normal-optimized.apk
```