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

## Checking your code optimizations

The quickest way to check what machine code was generated when you compiled some C++ function, is to, in Visual Studio, set a breakpoint on the code, then run until it gets hit, and then press Ctrl+Alt+D to show disassembly. Best used to verify that your SIMD instrinsics don't do something silly, like reload a matrix for every vertex for example...

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