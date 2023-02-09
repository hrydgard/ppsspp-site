# Porting PPSSPP

It's great to see PPSSPP ported to new platforms, feel free to give it a go.

One of the next interesting main porting-related tasks will be to implement a JIT compiler for RISC-V processors, as they are expected to become popular in devices in the relatively near future.

## Platforms

Here's an incomplete list of platforms that PPSSPP could or could not be ported to (last updated October 2022):

| Platform  | State | Comments |
| --------- | ----- | -------- |
| Windows | Done | |
| Android | Done | |
| Linux | Done | |
| macOS | Done | Limited native UI |
| iOS | Done | JIT works sometimes. Not allowed in Apple Store. Unmaintained |
| Nintendo Switch | Done | by m4xw! |
| Raspberry Pi | Done | R4+ recommended |
| Windows UWP/Xbox One/WinPhone10 | Done | Windows Phone support is discontinued |

These platforms were supported once, but aren't anymore due to lack of maintainer interest:

| Platform  | State | Comments |
| --------- | ----- | -------- |
| Blackberry 10 | Worked | JIT for ARM |
| Symbian | Worked | JIT for ARM |
| Pandora | Worked | |
| Meego | Worked | |
| Xbox 360 | Proof-of-concept | Hacked consoles only |

These are ports that never happened or are deemed impossible:

| Platform  | State | Comments |
| --------- | ----- | -------- |
| Playstation 3 | Barely possible | Hacked consoles only. Unlikely to happen |
| Windows Phone 7.x | NOT POSSIBLE | No native code support |
| Windows Phone 8 | NOT POSSIBLE | JIT not allowed, would be way too slow |
| Wii | NOT POSSIBLE | Not enough RAM |
| Xbox Original | NOT POSSIBLE | Not enough RAM. Weak CPU? |
| Playstation 2 | NOT POSSIBLE | Not enough RAM, weak CPU. |
| Gamecube | NOT POSSIBLE | Not enough RAM, weak CPU. |
| Dreamcast | NOT POSSIBLE | Not enough RAM, weak CPU. |
