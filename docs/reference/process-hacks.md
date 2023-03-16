# PPSSPP process hacking

If you want to search the memory of the emulated PSP using external tools by attaching to the PPSSPP process' memory space in Windows, you're gonna need a pointer to the base address.

To get it, you can either use `Debug -> Copy PSP Memory Base Pointer` in the window menu (in 1.15+),
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