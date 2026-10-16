# CwCheats

CwCheats is a cheat format originally created for real hacked PSPs, not emulators, inspired by classic
Action Replay codes. As such, it's very low-level and simple.

To use them, turn on [Enable Cheats](/docs/settings/system#enable-cheats), then open Cheats from the pause
menu in-game. From there you can download a cheat database, import the current game's cheats from it,
and turn individual cheats on and off.

Unfortunately, these cheats are often a bit unstable - if a game starts crashing when you're using them,
try a different cheat.

The rest of this page is for people who write their own codes. It describes the whole format as PPSSPP
implements it. Most of it is the standard CWCheat format, and codes written for the CWCheat plugin on a
real PSP work as they are. PPSSPP also has some extensions and quirks of its own, marked **[PPSSPP]**
throughout. Codes that rely on them won't work on a real PSP.

## Summary of the non-standard parts

- **[PPSSPP]** Code type `0xA` doesn't exist in CWCheat. PPSSPP uses it for controller vibration and for
  changing post-processing shader settings, see [PPSSPP-only codes](#ppsspp-only-codes-0xa).
- **[PPSSPP]** The `0x0` "8-bit write" also writes 16 or 32 bits when the value doesn't fit in a byte.
- **[PPSSPP]** The `0xB` delay code is accepted but does nothing.
- **[PPSSPP]** A cheat file may hold cheats for several games; only the section matching the running game is used.
- **[PPSSPP]** Lines starting with `//` or `#` are comments.
- **[PPSSPP]** Section titles (`_[>>>Title<<<]_`) and comments (`↑[#...]`) in cheat names are displayed
  specially in the cheat list, following the conventions of the CWCheat-Database-Plus `cheat.db`.
- **[PPSSPP]** Cheats run on a timer with an adjustable interval, and are switched off entirely in
  [RetroAchievements](/docs/reference/retro-achievements) hardcore mode.

## Where the cheats live

Each game has its own cheat file, `PSP/Cheats/<GAMEID>.ini` in the memory stick folder, for example
`PSP/Cheats/ULUS10336.ini`. PPSSPP creates an empty one the first time you start the game with cheats
enabled. The game ID is the one from the game's `PARAM.SFO`, without the dash. Homebrew that doesn't have
one gets an ID made up by PPSSPP; the easiest way to find the file is the **Edit Cheat File** button on the
cheat screen (desktop only), which opens it in your text editor.

The cheat screen can also:

- **Download cheat database**: downloads a community-maintained database to `PSP/Cheats/cheat.db`.
- **Import from PSP/Cheats/cheat.db**: copies the current game's cheats from that file into the game's
  own cheat file, skipping cheats whose names are already there.
- **Browse**: the same, from a database file anywhere.
- **Refresh interval**: how often the cheats run, see [How cheats run](#how-cheats-run).

While the cheat screen is open, PPSSPP checks the file about once a second and reloads it if it has
changed. So you can keep the screen open, edit the file in a text editor, save, and see the result.

## File format

A cheat file is a plain text file (UTF-8, a BOM is fine). Each line starts with a tag:

```
_S ULUS-10336
_G Crisis Core: Final Fantasy VII
_C0 Max Gil
_L 0x20123456 0x0098967F
_C1 Infinite HP
_L 0x10234560 0x0000270F
_L 0x10234562 0x0000270F
```

| Tag | Meaning |
|---|---|
| `_S` | Game ID. Starts a game's section. |
| `_G` | Game title. Ignored by PPSSPP. |
| `_C0 name` | Start of a cheat, disabled. |
| `_C1 name` | Start of a cheat, enabled. **[PPSSPP]** Any digit from `1` to `9` counts as enabled. |
| `_L 0xAAAAAAAA 0xBBBBBBBB` | One line of code, belonging to the cheat above it. A cheat can have any number of these. |
| `_M ...` | A TempAR code line. **Not supported**: PPSSPP reports an error and ignores it. |

Values are hexadecimal, and the `0x` prefix is optional. Leading and trailing whitespace is ignored.
Anything after the two values on an `_L` line is an error.

**[PPSSPP]** A line starting with `//` or `#` is a comment. Other lines that don't start with `_` are
errors. Errors are written to the log, not shown in the UI, so if a cheat doesn't seem to do anything,
check the log.

### Several games in one file **[PPSSPP]**

The ID on an `_S` line is compared with the running game's ID, ignoring dashes and surrounding spaces, so
`ULUS-10336` and `ULUS10336` both match. If a file has several `_S` sections, only the matching one is
used.

For compatibility with older files, a file with a single `_S` line is used even if the ID doesn't match, as
is a file with no `_S` line at all.

### Titles and comments **[PPSSPP]**

The cheat list in PPSSPP understands two conventions from the CWCheat-Database-Plus `cheat.db`:

- A cheat whose name contains `_[>>>` and, later, `<<<]_` is a **section title**. The text between the
  markers is shown as a heading instead of a checkbox. Underscores outside the markers are fine:
  `_C0 _________[>>>Debug Cheats<<<]_________`.
- A cheat whose name contains `↑[#` (an up arrow, U+2191) followed by text and a closing `]` is a
  **comment** on the cheat above it, and is shown as a hint underneath it:
  `_C0 ↑[#Change the display layout to Stretch first]`.

As far as the file format goes, both are still cheats. A cheat with no `_L` lines doesn't show up in the
list at all, so give them a dummy line, and leave them disabled (`_C0`):

```
_C0 _________[>>>Debug Cheats<<<]_________
_L 0x00000000 0x00000000
_C0 Debug Menu At Boot
_L 0x201D4AF8 0x00000000
_C0 ↑[#Must be enabled before the game boots]
_L 0x00000000 0x00000000
```

## Addresses

Addresses in codes are offsets from `0x08800000`, the start of user memory where games are loaded. So to
change the value that PPSSPP's debugger shows at `0x08912340`, use the offset `0x00112340`. The code type
goes in the top hex digit, so a 32-bit write there is `0x20112340`.

The usable offsets are `0x00000000`-`0x017FFFFF`, or up to `0x037FFFFF` for games that use the extra
memory of the PSP-2000 and later.

**[PPSSPP]** The vibration-from-memory (`0xA1`) and shader-setting-from-memory (`0xA3`) codes are the
exception: they take a full PSP address, like `0x08912340`, not an offset.

Writes to addresses outside valid memory are silently skipped. A condition that reads an invalid address
counts as false, except for the `C` code (see [Known issues](#known-issues)).

Writing over game code is fine; PPSSPP recompiles the code that changed.

## How cheats run

All enabled cheats run, in file order, every *refresh interval* (77 ms by default, adjustable from 1 to 1000
ms on the cheat screen, and per game with a [game config](/docs/reference/custom-game-configs)). Each
cheat's lines run from top to bottom. So a write isn't one-off: it's repeated every interval, which is how
"infinite" cheats keep a value frozen. If you need a write to happen only once, guard it with a condition
(a joker code, or a test of the value you're about to change).

If a line isn't a valid code (an unknown code type, or a multi-line code with its second line missing), the
rest of that cheat is skipped, but the cheats after it still run.

**[PPSSPP]** Cheats don't run at all while [RetroAchievements](/docs/reference/retro-achievements) hardcore
mode is active.

## Code types

In the tables, `A` is an address offset, `V` a value, `N` a count, and other letters are explained with
each code. A two-line code is written as two `_L` lines.

### Writes

| Code | Effect |
|---|---|
| `0AAAAAAA 000000VV` | Write the 8-bit value `VV` to `A`. |
| `1AAAAAAA 0000VVVV` | Write the 16-bit value `VVVV` to `A`. |
| `2AAAAAAA VVVVVVVV` | Write the 32-bit value `VVVVVVVV` to `A`. |

**[PPSSPP]** If the value of a `0` code doesn't fit in a byte, PPSSPP writes it as 16 bits (if it fits) or
32 bits instead of truncating it. Use `1` and `2` for 16 and 32-bit writes so the code works everywhere.

16-bit and 32-bit writes should be aligned to 2 and 4 bytes respectively.

### Increment and decrement

| Code | Effect |
|---|---|
| `301000VV AAAAAAAA` | Add `VV` to the byte at `A`. |
| `302000VV AAAAAAAA` | Subtract `VV` from the byte at `A`. |
| `3030VVVV AAAAAAAA` | Add `VVVV` to the 16-bit value at `A`. |
| `3040VVVV AAAAAAAA` | Subtract `VVVV` from the 16-bit value at `A`. |
| `30500000 AAAAAAAA`<br>`VVVVVVVV 00000000` | Add `VVVVVVVV` to the 32-bit value at `A`. |
| `30600000 AAAAAAAA`<br>`VVVVVVVV 00000000` | Subtract `VVVVVVVV` from the 32-bit value at `A`. |

Note that the address is in the second half of the line here. Since cheats run repeatedly, these add or
subtract again on every refresh; they're mostly useful behind a condition.

### Bitwise operations

`7AAAAAAA 000TVVVV`, where `T` is:

| `T` | Effect |
|---|---|
| `0` | OR the byte at `A` with `VV` (the low 8 bits). |
| `1` | OR the 16-bit value at `A` with `VVVV`. |
| `2` | AND the byte at `A` with `VV`. |
| `3` | AND the 16-bit value at `A` with `VVVV`. |
| `4` | XOR the byte at `A` with `VV`. |
| `5` | XOR the 16-bit value at `A` with `VVVV`. |

### Repeated writes

Write a value to a series of addresses, optionally changing the value each time:

```
_L 0x4AAAAAAA 0xNNNNSSSS
_L 0xVVVVVVVV 0xIIIIIIII
```

Writes the 32-bit value `V` to `A`, `N` times. After each write the address goes up by `S` *words*
(`S` × 4 bytes) and the value goes up by `I`.

```
_L 0x8AAAAAAA 0xNNNNSSSS
_L 0x000000VV 0xIIIIIIII     (8-bit)
_L 0x1000VVVV 0xIIIIIIII     (16-bit)
```

The same for 8-bit and 16-bit values. The step `S` is in units of the value size (1 or 2 bytes). PPSSPP
treats the code as 16-bit if any of the top 16 bits of the second line's first half are set.

### Copy

```
_L 0x5AAAAAAA 0xNNNNNNNN
_L 0x0BBBBBBB 0x00000000
```

Copies `N` bytes from `A` to `B`. The top digit of the second line is ignored.

### Delay

`B0000000 NNNNNNNN` delays the lines that follow. **[PPSSPP]** This is accepted but ignored; the following
lines run straight away.

### Conditions

Conditions skip lines (`_L` lines, not cheats) when they're false. A multi-line code counts as several
lines when you count what to skip.

All comparisons compare the value in memory against the value in the code: "less than" means the value at
`A` is less than `V`.

#### Stop the cheat unless equal

`CAAAAAAA VVVVVVVV`: if the 32-bit value at `A` isn't `V`, skip the rest of this cheat. Handy as a first
line, to make sure the right part of the game (or the right overlay) is loaded before writing anything.

#### Test, then run or skip the next line

| Code | Runs the next line if... |
|---|---|
| `DAAAAAAA 0000VVVV` | the 16-bit value at `A` equals `VVVV` |
| `DAAAAAAA 0010VVVV` | ...is not equal to `VVVV` |
| `DAAAAAAA 0020VVVV` | ...is less than `VVVV` |
| `DAAAAAAA 0030VVVV` | ...is greater than `VVVV` |
| `DAAAAAAA 200000VV` | the byte at `A` equals `VV` |
| `DAAAAAAA 201000VV` | ...is not equal to `VV` |
| `DAAAAAAA 202000VV` | ...is less than `VV` |
| `DAAAAAAA 203000VV` | ...is greater than `VV` |

If the test is false, exactly one line is skipped.

#### Test, then run or skip several lines

| Code | Runs the next `NN` lines if... |
|---|---|
| `E0NNVVVV TAAAAAAA` | the 16-bit value at `A` compares true against `VVVV` |
| `E1NN00VV TAAAAAAA` | the byte at `A` compares true against `VV` |

`T` is the comparison: `0` equal, `1` not equal, `2` less than, `3` greater than. Here too, the address is
in the second half.

#### Compare two addresses

```
_L 0xDAAAAAAA 0xTBBBBBBB
_L 0xNNNNNNNN 0x0000000Z
```

Compares the value at `A` with the value at `B`, and skips the next `N` lines if the comparison is false.
`T` is `4` equal, `5` not equal, `6` less than, `7` greater than. `Z` is the size: `0` 8-bit, `1` 16-bit,
`2` 32-bit.

**[PPSSPP]** 32-bit values are compared as signed numbers, so for less/greater, `0x80000000` and above count
as negative.

#### Button presses (joker codes)

| Code | Runs the next `NN`+1 lines if... |
|---|---|
| `D00000NN 1BBBBBBB` | all the buttons in `B` are held |
| `D00000NN 3BBBBBBB` | not all the buttons in `B` are held |

Add the button values together for a combination. The buttons are the ones the game sees, after PPSSPP's
control mapping.

| Button | Value |
|---|---|
| Select | `0x00000001` |
| Start | `0x00000008` |
| Up | `0x00000010` |
| Right | `0x00000020` |
| Down | `0x00000040` |
| Left | `0x00000080` |
| L | `0x00000100` |
| R | `0x00000200` |
| Triangle | `0x00001000` |
| Circle | `0x00002000` |
| Cross | `0x00004000` |
| Square | `0x00008000` |

The system buttons (Home `0x00010000`, Hold `0x00020000`, Screen `0x00400000`, Note `0x00800000`, the volume
buttons) are part of the format too, but PPSSPP doesn't normally pass them to the game, so codes that test
them don't fire.

For example, to set a value only while L + R + Select are held:

```
_L 0xD0000000 0x10000301
_L 0x2012A3C0 0x0000270F
```

### Pointer codes

Games often keep data in memory they allocate at runtime, so it isn't at the same address every time. A
pointer code reads a pointer from a fixed address, follows it, and writes at an offset from where it points.

```
_L 0x6AAAAAAA 0xVVVVVVVV
_L 0x000ZNNNN 0xIIIIIIII
```

Reads the 32-bit pointer at `A`, then writes `V` at that pointer plus `I`. `Z` sets the write:

| `Z` | Write |
|---|---|
| `0` | 8-bit, at pointer + `I` |
| `1` | 16-bit, at pointer + `I` |
| `2` | 32-bit, at pointer + `I` |
| `3` | 8-bit, at pointer - `I` |
| `4` | 16-bit, at pointer - `I` |
| `5` | 32-bit, at pointer - `I` |

The pointer read from memory is a full PSP address (`0x08...`), not an offset, and nothing is added to it.

`N` is the number of lines in the code after the first, so it's `1` for the simple two-line form. Further
lines extend it, and are processed in order before the final write:

| Line | Effect |
|---|---|
| `2XXXXXXX 00000000` | Follow another pointer: read the 32-bit value at pointer + `X`. |
| `3XXXXXXX 00000000` | Follow another pointer: read the 32-bit value at pointer - `X`. |
| `2XXXXXXX 2YYYYYYY` | Follow two levels in one line: first at + `X`, then at + `Y`. Either half can use `3` for minus. |
| `9XXXXXXX YYYYYYYY` | Add `X` to the pointer and `Y` to the value to write. |
| `1XXXXXXX 00000000` | Copy `V` bytes, from the first pointer + `I` to the first pointer + `X`, and skip the final write. Probably not what this line is meant to do, see [Known issues](#known-issues). |

For example, a two-level pointer: read the pointer at offset `0x0012A000`, read the next one 0x40 bytes into
what it points to, and write 999 as a 32-bit value 0x10 bytes into that:

```
_L 0x6012A000 0x000003E7
_L 0x00020002 0x00000010
_L 0x20000040 0x00000000
```

**[PPSSPP]** PPSSPP reads bits 20-31 of the second line (`BBB` in `0xBBBZNNNN`) as a signed offset, in
words, added to `A` before reading the first pointer. Keep them zero.

### PPSSPP-only codes (0xA)

**[PPSSPP]** The whole `0xA` code type is specific to PPSSPP. A real PSP running the CWCheat plugin won't
understand these.

#### Controller vibration

The PSP doesn't have a rumble motor, so PSP games never ask for vibration. These codes let a cheat drive a
gamepad's motors instead, for example by testing what the game is doing with a condition and vibrating
when it happens. Currently only XInput controllers on Windows are supported.

```
_L 0xA0TTLLLL 0x00ttRRRR
```

Sets the left motor to strength `LLLL` and the right one to `RRRR` (`0000`-`FFFF`). The strength then decays
every frame (1/60 s): it's multiplied by `TT`/256 for the left motor and `tt`/256 for the right. So `FF`
keeps it going for a long time, and `80` halves it every frame. A strength of `0000` leaves that motor
alone.

```
_L 0xA1000000 0xAAAAAAAA
```

The same, but with the parameters read from memory at the **full** PSP address `A` (such as `0x08912340`):

| Address | Size | Value |
|---|---|---|
| `A` | 16 bits | Left strength |
| `A` + 2 | 16 bits | Right strength |
| `A` + 4 | 8 bits | Left decay |
| `A` + 6 | 8 bits | Right decay |

#### Post-processing shader settings

Post-processing shaders can have up to four settings, the sliders under the shader in Display Layout &
Effects. These codes set one of them, so a cheat can for example change a shader's look depending on
what's going on in the game.

```
_L 0xA2SS00UU 0xVVVVVVVV
```

Sets setting `UU` (`00`-`03`, for settings 1-4) of shader `SS` to `V`, a 32-bit float (`0x3F800000` is
1.0). `SS` is the shader's position in the active post-processing chain, starting at `00`. Shaders that
another shader in the chain requires are counted too.

```
_L 0xA3SSFFUU 0xAAAAAAAA
```

The same, but with the value read from memory at the **full** PSP address `A`. `FF` is the format of the
value there: `00` unsigned 8-bit, `01` unsigned 16-bit, `02` unsigned 32-bit, `03` 32-bit float.

The value isn't clamped to the slider's range. These codes change the setting itself, so the value stays
after the cheat is turned off, and gets saved with the rest of the settings (see [Known issues](#known-issues)).

## Known issues

These are bugs or rough edges in PPSSPP's implementation, as of this writing. Write codes that don't depend
on them, since they may get fixed.

- **The `C` code doesn't stop the cheat when its address is invalid.** Every other condition counts an
  unreadable address as false, but `C` only stops the cheat when it can read the address and the value
  differs. Since a game's memory is always readable at offsets in range, this only matters for a wrong
  offset.
- **The `1` line in a pointer code copies within a single pointer.** Both the source and the destination are
  taken from the pointer read at `A` (plus `I` and `X`), so it can't copy between two different objects.
- **A failed step in a pointer walk doesn't abort the code.** If a `2`/`3` line would read from an invalid
  address, that step is skipped and the walk continues from the pointer it already had, so the final write can
  land somewhere other than intended. In practice a null pointer usually makes the final address invalid too,
  and the write is skipped.
- **Repeated writes (`4` and `8`) are all or nothing.** If any part of the range is outside valid memory, or
  the range is 24 MB or larger, nothing is written.
- **Comparing two addresses with a size `Z` of `3` or more** always counts as equal.
- **`E` codes other than `E0` and `E1`** are treated as 16-bit tests rather than rejected.
- **Vibration codes can't stop the motors**, since a strength of `0000` leaves a motor alone. Use a low decay
  value (like `01`) with a small non-zero strength to stop it quickly.
- **Vibration only works with XInput controllers on Windows.** On other platforms and controllers the
  `A0`/`A1` codes do nothing.
- **Shader settings set by `A2`/`A3` are saved to your config**, like a slider change, so they outlive the
  cheat. They also aren't clamped to the range the shader declares.
- **Errors in a cheat file aren't shown in the UI.** They only go to the log. An unknown tag, a malformed
  `_L` line, or a TempAR (`_M`) line is reported there and skipped; a line with junk after its two values is
  reported but still used.
- **A cheat with no `_L` lines disappears from the cheat list**, which is why titles and comments need a
  dummy line. If one of those dummy `_L 0x00000000 0x00000000` lines is enabled, it writes a zero byte to
  `0x08800000`, the start of the game's memory.
- **The delay code (`B`) is ignored**, as described above.
