# Emulator API

The emulator extends the emulated PSP with some additional functionality that can be used from homebrew apps, through `sceIoDevctl` and a couple of fake device names (`"emulator:"`/`"kemulator:"`). The plumbing details don't matter for normal use - PPSSPP ships a small, header-only C wrapper that takes care of all of that for you.

Important: None of this will work on the real PSP! So, better check that you are on the real emulator (see `ppsspp_is_emulator()` below) before relying on any of it, and always keep a working fallback path for real hardware.

## Getting the header

The wrapper is a single header, [`ppsspp_emu_api.h`](https://github.com/hrydgard/ppsspp/blob/master/docs/emu-api/ppsspp_emu_api.h), in the main PPSSPP source repository. Drop it into a PSPSDK-based homebrew project and `#include` it - no library to build or link, it's all `static inline` functions.

```c
#include "ppsspp_emu_api.h"

if (ppsspp_is_emulator()) {
    ppsspp_send_output_str("Hello from homebrew, running under PPSSPP!\n");
}
```

If you want to see exactly how it's implemented, or need to work with the raw protocol directly (e.g. from another language), the underlying `sceIoDevctl` commands are documented alongside the header at [`docs/emu-api/protocol.md`](https://github.com/hrydgard/ppsspp/blob/master/docs/emu-api/protocol.md) in the repository.

## Functions

| Function | Returns | Description |
|---|---|---|
| `ppsspp_is_emulator()` | `int` (bool) | Whether you're running under PPSSPP at all. Call this first - on real hardware, or an emulator that doesn't implement this API, everything below will just silently no-op/return zero. |
| `ppsspp_has_display()` | `int` (bool) | Whether there's an actual display to render to (false under `PPSSPPHeadless`). Useful to skip presentation/vblank-dependent work when there's nothing to show. |
| `ppsspp_send_output(const char *data, int len)` | - | Sends a block of text straight to PPSSPP's debug/log output, without going through a real file. |
| `ppsspp_send_output_str(const char *str)` | - | Same, for a NUL-terminated string. |
| `ppsspp_verify_state()` | - | Asks PPSSPP to do an internal savestate round-trip as a consistency check (asynchronous, result only logged on the PPSSPP side). Mainly useful when testing the emulator itself. |
| `ppsspp_emit_screenshot()` | - | Delivers the current framebuffer through PPSSPP's internal debug-screenshot hook, used by the pspautotests/frametest infrastructure. Doesn't write a file to the Memory Stick &ndash; not a general screenshot feature. |
| `ppsspp_toggle_fastforward(int enable)` | - | Turns PPSSPP's fast-forward mode on or off. |
| `ppsspp_get_aspect_ratio()` | `float` | Current display aspect ratio. Only correct in landscape orientation right now. |
| `ppsspp_get_scale()` | `float` | Current display scale factor. Only correct in landscape orientation right now. |
| `ppsspp_get_axis(int axisIndex)` | `float` | Reads an analog axis value injected by a PPSSPP-side input plugin, by `JOYSTICK_AXIS_*` index (`Common/Input/KeyCodes.h` in the PPSSPP source). Not normal controller input - use `sceCtrl*` for that. Returns 0 if no plugin has set it. |
| `ppsspp_get_vkey(int keyCode)` | `unsigned char` (bool) | Reads whether a virtual key injected by a PPSSPP-side input plugin is pressed, by `NKCODE_*` value (same header, mostly mirrors Android's key codes). Not normal controller input. Returns 0 if no plugin has set it. |

## Recommended use

Roughly in order of usefulness:

* **Debug logging** (`ppsspp_send_output`/`_str`) - a quick way to get text out of your homebrew during development without setting up file I/O.
* **`ppsspp_is_emulator()`** - guard any of the other calls with this, and don't let your app's behavior depend on them being available (real hardware, or an older PPSSPP build without a given command, should degrade gracefully).
* **`ppsspp_has_display()`** - lets the same binary behave sensibly under `PPSSPPHeadless` for automated testing.
* The rest (fast-forward toggling, aspect ratio/scale queries, screenshot emission, plugin axis/key reads, savestate verification) are more specialized - mostly useful for test tooling, TAS/plugin setups, or homebrew that wants to adapt its rendering to the host window shape.
