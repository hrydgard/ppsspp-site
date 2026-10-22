# Command Line arguments

PPSSPP has a bunch of command line arguments, that are not documented very well elsewhere, so let's do it here.

Some arguments, like the window management ones, have no effect on mobile or other non-desktop devices, of course.

## How to run PPSSPP with command line arguments on Android

This can be useful from launchers and similar. Arguments go in the string extra `org.ppsspp.ppsspp.Args`. Here's an example ADB command line that boots a game and exits PPSSPP when the game ends. The URI is one copied with **Copy deep link** in the game's **...** menu:

```cmd
adb shell 'am start -S -n org.ppsspp.ppsspp/org.ppsspp.ppsspp.PpssppActivity -a android.intent.action.VIEW -d "content://com.android.externalstorage.documents/tree/primary%3APSP/document/primary%3APSP%2FRidge%20Racer.iso" --es org.ppsspp.ppsspp.Args --pause-menu-exit'
```

See [Front end integration on Android](/docs/reference/front-end-integration) for launcher configuration, and which URIs work.

## Generally supported command line arguments

### -d

Sets the log level to debug

### -v

Sets the log level to verbose

### -j, -i, -r, -J

Sets the CPU backend to JIT, Interpreter, IR Interpreter or JIT IR, respectively.

From PPSSPP 1.21 (and current [development builds](/devbuilds)), these are replaced by `--cpu=jit`, `--cpu=interpreter`, `--cpu=ir` and `--cpu=jit-ir`. The old forms give an error, and `-r` now means `--root`.

### --pause-menu-exit

PPSSPP exits when the game ends, instead of going to its menu. The "Exit to menu" choice on the pause menu becomes "Exit", and quitting the game from inside it, or dismissing a load error, also exits. This is most useful for frontends that also pass a file to launch. Before PPSSPP 1.21, only the pause menu choice exited.

### --escape-exit

The escape key (or whatever key you have mapped to pause) will exit the emulator immediately.

### --fullscreen

Forces full screen mode, ignoring saved configuration.

### --windowed

Forces windowed mode, ignoring saved configuration.

### --log=FILE, --loglevel=N, --state=FILE, --root=DIR

Write the log to FILE, set the log level, load a savestate from FILE on boot, and mount a directory as the root of `host0:/`.

### More options in PPSSPP 1.21

From PPSSPP 1.21 (and current [development builds](/devbuilds)), all platforms share one command line parser, and `--help` lists every option (except on Windows, where PPSSPP has no console to print to). Some new ones that can be handy:

* `--memstick=DIR` - use DIR as the memory stick.
* `--resolution-scale=N`, `--msaa=N` - override the rendering resolution and MSAA level (0 = off, 1 = 2x, 2 = 4x, 3 = 8x) for this run.
* `--cpu-mhz=N` - lock the emulated CPU clock (0 = let the game set it).
* `--vsh` - boot the [XMB](/docs/getting-started/how-to-run-the-vsh).
* `--debugger=PORT` - enable the [WebSocket debugger](/docs/reference/websocket-api) on this port.

## Development arguments

These are a bit esoteric and not that useful for end users.

### --gamesettings

Go directly to settings.

### --touchscreentest

Go directly to the touchscreentest screen

From PPSSPP 1.21 (and current [development builds](/devbuilds)), these two are replaced by `--start-screen=gamesettings` and `--start-screen=touchscreentest` (`--start-screen=developertools` also works).

### --appendconfig

Merge a different config file into the current configuration, as in `--appendconfig=FILE.ini`.

## Windows-only command line arguments

Some of these will be gneralized at some point.

### -l

Enables the Log Console (same as `Debug -> Log console` in the menu)

### -s

Prevent settings from being saved. Also disables auto-run. Kind of a weird one??

### --graphics=directx11, --graphics=vulkan, --graphics=gles, --graphics=software

Enforces this GPU backend.

From PPSSPP 1.21 (and current [development builds](/devbuilds)), `--graphics=` also accepts `d3d11`, `opengl` and, on Mac and iOS, `metal`.
