# PPSSPP process hacking

If you want to search the memory of the emulated PSP using external tools by attaching to the PPSSPP process' memory space in Windows, you're gonna need a pointer to the base address.

To get it, you can either use `Debug -> Copy PSP memory base address` in the window menu (in 1.15+),
read it from the log, or get it programmatically, by sending a custom window message to query the pointer.

That message is defined as:

```cpp
const UINT WM_USER_GET_BASE_POINTER = WM_APP + 0x3118;  // 0xB118
```

You can send this message with the following values of lParam to the main window of PPSSPP, to get access to the following information:

* `0` - Lower 32 bits of pointer to the base of emulated memory
* `1` - Upper 32 bits of pointer to the base of emulated memory
* `2` - Lower 32 bits of pointer to the pointer to the base of emulated memory
* `3` - Upper 32 bits of pointer to the pointer to the base of emulated memory

The point (hah) of the latter two is that that is stable across execution of multiple games, while the former is not, which may be useful in some cases.

2 and 3 are only available in builds starting from early February 2023.

## CheatEngine tips and tricks by NABN00B

Visual explanation of working with memory addresses in Cheat Engine (Windows x64):

<img src="/img/process-hacking/PPSSPP_CE.png" />

Scripting example (use Copy PSP memory base address to fill in the base pointer):

<img src="/img/process-hacking/PPSSPP_CE_Pointer_Lua.png" />

[An AutoHotkey script](https://github.com/NABN00B/AutoHotkey.PPSSPP).

### Terminology

* PSP Memory Base Address: the address in process memory where the emulated PSP's memory starts from (0x0 in PSP memory). Commonly something like `0x0000_01??_????_????` or `0x0000_02??_????_????` but that cannot be relied upon.
* PSP Memory Base Pointer: the pointer to PSP Memory Base Address. The address whose value is the address in Windows memory where the emulated PSP's memory starts from. Its value is the PSP Memory Base Address. Its address is higher than `0x0000_7ff?_????_????`.
* PSP User Memory Address: the address `0x0880_0000` in the emulated PSP's memory where game data starts from (mind the different characters!). The addresses of game variables (offset1 in our pointer paths) are higher than this (already contain `0x0880_0000`).
* PPSSPP Process Base Address: the address in Windows memory where the memory of a PPSSPPWindows64.exe process starts from. Referred to as PPSSPPWindows64.exe. Usually something like `0x0000_7ff?_????_????`.
* PPSSPP Base Offset (to PSP Memory Base Pointer): the offset in Windows memory, such that PPSSPP Process Base Address + PPSSPP Process Base Offset == address of PSP Memory Base Pointer. Higher than `0x00A0_0000`. A constant that is usually different for each build.
