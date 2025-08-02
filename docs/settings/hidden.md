---
position: 8
---
# Hidden settings

These are little-used settings that have not seemed worth creating specific UI for.

You can manually edit `PSP/SYSTEM/ppsspp.ini` in your memstick directory to set these.

To find the location of your memory stick on your device, navigate to <b class="inapp">Settings -> System -> PSP Memory Stick -> Show Memory Stick folder</b>.

Nice copy-paste-able lines below:

```ini
[General]
TopMost = True   # keeps PPSSPP in front of other windows
PauseWhenMinimized = True   # pauses emulation when PPSSPP is minimized to the taskbar
ShowMenuBar = False   # hides the menubar from the window
# use these to set the window position and size pixel perfectly
WindowX = -8   # -8 to offset invisible window edge in Windows 10
WindowY = 0
WindowWidth = 1296   # +16 to offset invisible window edges in Windows 10
WindowHeight = 720

[Recent]
MaxRecent = 0   # hides the Recent tab
```