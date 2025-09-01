# Development tips & tricks

## Installing on very old phones that Android Studio won't talk to

For ancient phones that can no longer install directly from Android Studio, choose:

* Build->Build Bundles/APKs/Build APK

Then run:

```sh
adb install android/build/outputs/apk/normal/optimized/android-normal-optimized.apk
```

## Better ADB logs

Use the following very long command line to cleanly filter out PPSSPP-relevant logs:

```sh
adb logcat -s DEBUG AndroidRuntime PPSSPPNativeActivity PPSSPP NativeGLView NativeRenderer NativeSurfaceView PowerSaveModeReceiver InputDeviceState PpssppActivity CameraHelper
```

More instructions [here](/docs/troubleshooting/gettings-logs).

## Running PPSSPPHeadless in Docker on github CI

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

## Checking your code optimizations

The quickest way to check what machine code was generated when you compiled some C++ function, is to, in Visual Studio, set a breakpoint on the code, then run until it gets hit, and then press Ctrl+Alt+D to show disassembly. Best used to verify that your SIMD instrinsics don't do something silly, like reload a matrix for every vertex for example...

## Profiling on Android

In Android Studio, go Run->"Profiler: Run Android as profilable" (low overhead). Then choose "Find CPU Hotspots".

## Launch.json in vscode

Here's a quick `.vscode/launch.json` one for debugging on Mac:

```
{
        "version": "0.2.0",
        "configurations": [
                {
                        "name": "(lldb) Launch",
                        "type": "cppdbg",
                        "request": "launch",
                        "program": "",
                        "osx": {
                                "program": "${workspaceFolder}/build/PPSSPPSDL.app/Contents/MacOS/PPSSPPSDL"
                        },
                        "linux": {
                                "program": "${workspaceRoot}/build/PPSSPPSDL"
                        },
                        "args": [],
                        "stopAtEntry": false,
                        "cwd": "${workspaceFolder}",
                        "environment": [],
                        "externalConsole": false,
                        "MIMode": "lldb"
                }
        ]
}
```

Can be improved a lot, like currently it doesn't even perform the build.

## Running in WSL2 with full graphics support

See this stackoverflow post: https://stackoverflow.com/questions/79151910/how-to-fix-errors-in-wsl-gui

With this, both OpenGL and Vulkan will work in WSL. The latter works through dzn, a Vulkan-on-top-of-DX12 wrapper.

# Testing PPSSPP's websocket debugger support

From 1.20, there's a new script in scripts/ in the root of the code tree.

Use as follows:

```bash
python scripts\websocket-test.py 56244 gpu.stats.get 3
```

where `56244` is the configured port, and `gpu.stats.get` is the command you want to test, and 3 is the number of seconds
you want to wait for additional messages.