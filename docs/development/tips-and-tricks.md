# Development tips & tricks

## Installing on very old phones that Android Studio won't talk to

For ancient phones that can no longer install directly from Android Studio, choose:

* Build->Build Bundles/APKs/Build APK

Then run:

```sh
adb install android/build/outputs/apk/normal/optimized/android-normal-optimized.apk
```

# Running PPSSPPHeadless in Docker

Here's an example setup. Note that `--shm-size=8g` (or bigger) is critically important!

```yml
build_test_headless_alpine:
  runs-on: ubuntu-latest
  container:
    image: alpine:latest
    options: --shm-size=8g
```

## Compat.ini on Android

It's inconvenient to directly edit compat.ini since it's built into the APK's asset subdirectory,
but you can put a separate compat.ini in your PSP/SYSTEM directory. It will be automatically merged
with the one from assets in the APK.
