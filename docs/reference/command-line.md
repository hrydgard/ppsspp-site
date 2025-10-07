# Command Line arguments

PPSSPP has a bunch of command line arguments, that are not documented very well elsewhere, so let's do it here.

Some arguments, like the window management ones, have no effect on mobile or other non-desktop devices, of course.

## How to run PPSSPP with command line arguments on Android

This can be useful from launchers and similar. Here's an example ADB command line, that causes PPSSPP to run in a mode where it exits directly from the pause menu:

```cmd
adb shell am start -n org.ppsspp.ppsspp/org.ppsspp.ppsspp.PpssppActivity --es org.ppsspp.ppsspp.Args "--pause-menu-exit"`
```

## Generally supported command line arguments

### -d

Sets the log level to debug

### -v

Sets the log level to verbose

### -j, -i, -r, -J

Sets the CPU backend to JIT, Interpreter, IR Interpreter or JIT IR, respectively.

### --pause-menu-exit

The "Exit to menu" choice on the pause menu is changed to just "Exit". This is most useful for frontends that also pass a file to launch.

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
