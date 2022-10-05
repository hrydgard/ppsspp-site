# Porting PPSSPP

You are encouraged to port PPSSPP to new platforms.

One of the next main porting-related tasks will be to implement a JIT compiler for RISC-V processors, as they are expected to become popular in devices in the near future.

## Platforms

Here's an incomplete list of platforms that PPSSPP could or could not be ported to (last updated October 2022):

| Platform  | Done | Comments |
| --------- | ---- | -------- |
| Windows | Done | JIT for x86 and x64 |
| Android | Done | JIT for ARM, ARM64, x86, x86-64 |
| Linux | Done | JIT for ARM, ARM64, x86, x86-64 |
| macOS | Done | JIT for x86 and x64. Port still pretty bad. |
| iOS | Done | JIT works sometimes. Not allowed in Apple Store. Unmaintained |
| Nintendo Switch | Done | by m4xw! |
| Raspberry Pi | Done | JIT working |
| Windows UWP/Xbox One/WinPhone10 | Done | JIT for x86 and x64 |

These platforms were supported once, but aren't due to lack of maintainer interest:

| Platform  | Done | Comments |
| --------- | ---- | -------- |
| Blackberry 10 | Done | JIT for ARM |
| Symbian | Done | JIT for ARM |
| Pandora | Done | |
| Meego | Done | |
| Xbox 360 | Proof-of-concept | Hacked consoles only |
| Playstation 3 | Possible | Hacked consoles only. Unlikely to happen |

These are ports that never happended or are deemed impossible:

| Platform  | Done | Comments |
| --------- | ---- | -------- |
| Windows Phone 7.x | NOT POSSIBLE | No native code support |
| Windows Phone 8 | NOT POSSIBLE | JIT not allowed, would be way too slow |
| Wii | NOT POSSIBLE | Not enough RAM |
| Xbox Original | NOT POSSIBLE | Not enough RAM. Weak CPU? |
| Playstation 2 | NOT POSSIBLE | Not enough RAM, weak CPU. |
| Gamecube | NOT POSSIBLE | Not enough RAM, weak CPU. |
| Dreamcast | NOT POSSIBLE | Not enough RAM, weak CPU. |
