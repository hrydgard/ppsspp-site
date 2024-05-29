# Porting PPSSPP

It's great to see PPSSPP ported to new platforms, feel free to give it a go.

## Platforms

Here's an incomplete list of platforms that PPSSPP could or could not be ported to (last updated 2024):

| Platform  | State | Comments |
| --------- | ----- | -------- |
| Windows | Done | |
| Android | Done | |
| Linux | Done | x86-32/64, ARM32/64, RISCV-64 |
| macOS | Done | Limited native UI |
| iOS | Done | JIT works but not on the App Store builds |
| Nintendo Switch | Done | by m4xw! |
| Raspberry Pi | Done | R4+ recommended |
| Windows UWP/Xbox One/WinPhone10 | Done | Windows Phone support is discontinued, too slow |

These platforms were supported once, but aren't anymore due to lack of maintainer interest:

| Platform  | State | Comments |
| --------- | ----- | -------- |
| Blackberry 10 | Worked | JIT for ARM |
| Symbian | Worked | JIT for ARM |
| Pandora | Worked | |
| DragonBox Pyra | Worked | |
| Meego | Worked | |
| Wii U | Proof-of-concept | |
| Xbox 360 | Proof-of-concept | Hacked consoles only |

These are ports that never happened or are deemed impossible:

| Platform  | State | Comments |
| --------- | ----- | -------- |
| Playstation 3 | Barely possible | Hacked consoles only. Unlikely to happen |
| Windows Phone 7.x | NOT POSSIBLE | No native code support |
| Windows Phone 8 | NOT POSSIBLE | JIT not allowed, way too slow |
| Wii | NOT POSSIBLE | Not enough RAM |
| Xbox Original | NOT POSSIBLE | Not enough RAM. Weak CPU? |
| Playstation 2 | NOT POSSIBLE | Not enough RAM, weak CPU. |
| Gamecube | NOT POSSIBLE | Not enough RAM, weak CPU. |
| Dreamcast | NOT POSSIBLE | Not enough RAM, weak CPU. |

### Why not the PlayStation 3?

It's a common question why there's no homebrew port of PPSSPP for the PlayStation 3. Unfortunately, while techically possible, it's a lot harder than it seems, and would need somebody doing months of dedicated work, maybe a year, to make it run well.

The main issue is that the PS3 uses "[big-endian](https://en.wikipedia.org/wiki/Endianness)" CPUs. All platforms that PPSSPP currently run on are little-endian, just like the PSP itself. With all the memory copying and low level bit manipulation that happens during emulation, it's very easy for code that assumes little-endian byte order to slip in. So, first of all somebody would need to bring up PPSSPP on some other easier-to-debug big-endian platform (like old PowerPC Mac, perhaps) and fix all the endian-ness bugs.

Second, the PS3 uses the CELL processor, which is really unsuitable for emulation due to being terrible at executing branchy code, the main core being PowerPC-based, and the SPUs would be nearly useless. The IR interpreter would run far too slowly, so JIT compilation would be the only option. PowerPC is simply unsuitable because PPSSPP doesn't have any PowerPC JIT backends, and writing a good one would be a major effort.

Third, programming the PS3 GPU isn't very easy, and we'd need to link a full shader compiler into the app. I don't know if there's an existing suitable one.

Finally, the number of people who are interested enough in the PS3 and PSP to do this, and have the skills to do a good emulator port (including JIT), appears to be zero.
