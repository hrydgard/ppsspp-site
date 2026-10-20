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

### --pause-menu-exit

PPSSPP exits when the game ends, instead of going to its menu. The "Exit to menu" choice on the pause menu becomes "Exit", and quitting the game from inside it, or dismissing a load error, also exits. This is most useful for frontends that also pass a file to launch. Before PPSSPP 1.21, only the pause menu choice exited.

### --escape-exit

The escape key (or whatever key you have mapped to pause) will exit the emulator immediately.

### --fullscreen

Forces full screen mode, ignoring saved configuration.

### --windowed

Forces windowed mode, ignoring saved configuration.

## Development arguments

These are a bit esoteric and not that useful for end users.

### --gamesettings

Go directly to settings.

### --touchscreentest

Go directly to the touchscreentest screen

### --appendconfig

Merge a different config file into the current configuration.

## Windows-only command line arguments

Some of these will be gneralized at some point.

### -l

Enables the Log Console (same as `Debug -> Log console` in the menu)

### -s

Prevent settings from being saved. Also disables auto-run. Kind of a weird one??

### -direct3d11 -vulkan -gles -software

Enforces this GPU backend.
