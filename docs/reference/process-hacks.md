# PPSSPP process hacking

If you want to search the memory of the emulated PSP using external tools by attaching to the PPSSPP process' memory space in Windows, you're gonna need a pointer to the base address.

To get it, you can use one of the following methods:
- read it from the log
- select `Debug -> Copy PSP memory base address` from the menubar <sup>[(since 1.15)](https://github.com/hrydgard/ppsspp/pull/16994)</sup>
- run [NABN00B's AutoHotkey script](https://github.com/NABN00B/AutoHotkey.PPSSPP/blob/main/ppsspp_WM.ahk) <sup>(for 1.14 and above)</sup>
- request it programmatically:
  - via the [WebSocket debugger API](/docs/reference/websocket-api) [^ws] <sup>[(since 1.7)](https://github.com/hrydgard/ppsspp/blob/9317fbdd5e8304de7fc0351b95cefbcc53228834/Core/Debugger/WebSocket/DisasmSubscriber.cpp#L255-L267)</sup>
  - by sending the `WM_USER_GET_BASE_POINTER` window message <sup>[(since 1.14)](https://github.com/hrydgard/ppsspp/pull/15748)</sup>

[^ws]: Note: The WebSocket debugger API request returns 8&nbsp;bytes for 32bit PPSSPPWindows process on 64bit Windows. Before v1.15, the return value was **bugged** and the upper 32 bits contained the value of another variable, so you need to get rid of that manually e.g. with `& 0xFFFF_FFFF`. See [#17266].

## Useful window messages

### `WM_USER_GET_BASE_POINTER`

Used to query a pointer to the base address of the PPSSPP process.
[Available since 1.14](https://github.com/hrydgard/ppsspp/pull/15748).

```cpp
const UINT WM_USER_GET_BASE_POINTER = WM_APP + 0x3118;  // 0xB118
```

You can send this message with the following values of `lParam` to the main window of PPSSPP, to get access to the following information:
- `0` - Lower 32 bits of pointer to the base of emulated memory
- `1` - Upper 32 bits of pointer to the base of emulated memory
- `2` - Lower 32 bits of pointer to the pointer to the base of emulated memory <sup>[(since 1.15)](https://github.com/hrydgard/ppsspp/commit/6f9d6c6c5fa333059d4cfed1f68e7121db8e7e6e)</sup>
- `3` - Upper 32 bits of pointer to the pointer to the base of emulated memory <sup>(since 1.15)</sup>

The point (hah) of the latter two is that that is valid across execution of multiple games, while the former is not, which may be useful in some cases.

### `WM_USER_GET_EMULATION_STATE`

Returns `1` if a game is running, `0` otherwise.
[Available since 1.15.4](https://github.com/hrydgard/ppsspp/pull/17461).

```cpp
const UINT WM_USER_GET_EMULATION_STATE = WM_APP + 0x3119; // 0xB119
```

## Cheat Engine tips and tricks by NABN00B

### Terminology

Note that all of the addresses and offsets below represent a `PPSSPPWindows64.exe` process running on x86-64 Windows.
A `PPSSPPWindows.exe` process running on x86-64 Windows has addressess and offsets that look "similar" enough to be recognizable, but they are all in 32 bits.

- **PSP Memory Base Address**: the address in Windows memory where the emulated PSP's memory starts from (`0x0` in PSP memory). Usually something like `0x0000_01??_????_???` or `0x0000_02??_????_????`.

- **PSP Memory Base Pointer**: the pointer to PSP Memory Base Address. The address where the value is the address in Windows memory where the emulated PSP's memory starts from. Its value is the `PSP Memory Base Address`. Its address is usually something like `0x0000_7ff?_????_????`.

- **PSP User Memory Address**: the address `0x0880_0000` in the emulated PSP's memory where game data starts from. (Mind the different characters!) The addresses of game variables (**offset1** in pointer paths) are higher than this (already contain `0x0880_0000`).

- **PPSSPP Process Base Address**: the address in Windows memory where the memory of a `PPSSPPWindows64.exe` process starts from. Often referred to as **PPSSPPWindows64.exe**. Usually something like `0x0000_7ff?_????_????`.

- **PPSSPP Base Offset (to PSP Memory Base Pointer)**: the offset in Windows memory, such that
`PPSSPP Process Base Address + PPSSPP Process Base Offset == address of PSP Memory Base Pointer`.
Seems to be between `0x00A0_0000` and `0x0100_0000`. A constant that is usually different for each build.

### Saving memory addresses

Memory addresses in Cheat Engine can be saved either in static address format or in pointer format.
Pointer format is preferred, because it will keep the memory address valid throughout multiple sessions, as long as you don't switch PPSSPP versions.

Here is an illustration of memory addresses using the terminology above:
<img src="/static/img/docs/process_hacking/ce_memory_address.png" alt="Visual illustration of Cheat Engine memory addresses using the aforementioned terminology">

### Working with ingame pointers and objects

Members of ingame composite data types (such as structs and objects) cannot be saved as memory addresses in a way that they would remain valid throughout multiple sessions.
The pointer to the start of an object can be saved as such, but in order to calculate the address of any object member we need to delve into Lua scripting.

In the example below we have the object members saved as dummy memory addresses, all children of a Lua script.
The idea is to calculate the addresses of the object members and display the memory addresses when the Lua script is activated.
<img src="/static/img/docs/process_hacking/ce_dummy_entries.png" alt="Ingame object members saved in Cheat Engine as dummy memory addresses">

Here is the said Lua script:
<img src="/static/img/docs/process_hacking/ce_pointer_script.png" alt="Example of a Lua script to calculate the addresses of object members">
(In the screenshot the base pointer is calculated using a manually retrieved base offset.)

This method will work throughout mutliple sessions.

Things to keep in mind:
- The memory addresses of object members become invalid whenever the object is deleted by the game.
- The script needs to be reactivated whenever the object is recreated by the game, in order to recalculate the addresses of object members.

### Cheat Engine script for getting the PPSSPP Process Base Address

```lua
-- By "25094" on GitHub
function GetEmuBase()
    local command = 0xB118
    local emuHWND = findWindow("PPSSPPWnd")

    local pointer = {}

    local result = sendMessage(emuHWND, command, 0, 0)
    pointer[1] = tonumber(result)

    result = sendMessage(emuHWND, command, 0, 1)
    pointer[2] = tonumber(result)

    local lower = pointer[1]
    local upper = pointer[2]
    local combinedPointer = (upper * 0x100000000) + lower

    return tonumber(combinedPointer)
end
```

You can add this inside of any auto assembler script at the start or inside the enable part and it should run fine if you remember to add `{$lua}` before the code.

## Additional resources

- **[AutoHotkey.PPSSPP](https://github.com/NABN00B/AutoHotkey.PPSSPP)** -- A collection of (work-in-progress) AutoHotkey scripts to manipulate PPSSPP memory.

More will be added in the future.
