# Getting log output

Getting logs of crashes can make fixing them much easier.

## Getting ADB logs on Android

I'm sure there are other ways, but the simplest is:

* Install the `adb` command line tool on your PC or Mac. There are two ways:
  1. Just install the [platform tools](https://developer.android.com/tools/releases/platform-tools) somewhere, which should include `adb.exe`.
  2. Alternatively, install the full Android SDK on your PC or Mac. Figure out where adb.exe or the adb command is. On Mac, it should be something like `/Users/YOUR_USER_NAME/Library/Android/sdk/platform-tools/adb`. On PC, it'll be where you installed the SDK, and then `/sdk/platform-tools/adb.exe`.
* Make sure your device is in [developer mode](https://www.digitaltrends.com/mobile/how-to-get-developer-options-on-android/) (you can sideload APKs). Make sure "USB Debugging" is checked in developer settings.
* Connect your device to your PC/Mac via USB.
* In a command prompt, enter the following line (yes, all of it):

    ```cmd
    adb logcat -s DEBUG AndroidRuntime PPSSPPNativeActivity PPSSPP NativeGLView NativeRenderer NativeSurfaceView PowerSaveModeReceiver InputDeviceState PpssppActivity CameraHelper
    ```

* Find whatever looks like a crash dump, and paste it and some lines before it into your bug report.