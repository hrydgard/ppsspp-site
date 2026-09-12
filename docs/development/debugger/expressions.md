---
position: 1
---

# Debugger expressions

Almost every place in the PPSSPP debugger where you can type something that isn't a plain
number accepts an *expression*: a small C-like formula that is evaluated against the current
state of the emulated machine. Expressions are how you say "break here, but only when `a0` is
this pointer", "watch the third field of that struct", or "jump to whatever `ra` points at".

There is one shared expression evaluator, but it comes in two flavours with different sets of
built-in variables: the **CPU** flavour, used by the main debugger, and the **GE** flavour,
used by the graphics (display list) debugger. The syntax is identical, only the names differ.

## Quick examples

If you just want a breakpoint condition and don't want to read the whole page, these cover most
of it. Remember that **numbers are hex by default**, so write `0d3` when you mean three.

| Expression | Breaks when |
|---|---|
| `a0 == 0d3` | The first argument is 3 |
| `a0 == 10` | The first argument is 0x10, that is 16 - not ten |
| `a0 == 0` | The first argument is a null pointer |
| `a0 == 08801234` | The first argument is that exact address |
| `ra == 08900abc` | The call came from that specific caller |
| `v0 != 0` | The function is about to return something non-zero |
| `[a0]` | The 32-bit word `a0` points at is non-zero |
| `[a0+8] == 1` | The field at offset 8 of the struct in `a0` is 1 |
| `[a0+24,2] > 0d1000` | A 16-bit field at offset 0x24 exceeds 1000 |
| `a0 == 0 && a1 > 0d16` | Both conditions hold |
| `f12 > 100.0` | The float in `f12` is over 100 |
| `threadid == 0d321` | Only this thread reaches here |
| `flipcount > 0d120` | Only after 120 more frames have been presented |

The same expressions work in a watch, where the result is simply displayed instead of compared.

## Where expressions can be used

| Place | Flavour | Notes |
|---|---|---|
| Breakpoint conditions | CPU | Execution, memory and register breakpoints. The breakpoint only trips when the expression evaluates to non-zero. |
| Breakpoint log format | CPU | The `{...}` parts of the log text, see [Log format strings](#log-format-strings). |
| Watch window | CPU | Re-evaluated continuously, and displayed as hex, decimal, float or string. |
| Struct viewer watches | CPU | The expression produces the *address* the struct is read from. Tick "Dynamic" to re-evaluate it every frame. |
| Address and size fields | CPU | "Go to address" in the disassembly and memory views, the memory dump dialog, the symbol editor, and the address/size fields of the breakpoint dialog. |
| Register editing | CPU | Typing a value into the register list, and the `register=expression` form in the assemble box. |
| GE display list breakpoints | GE | Conditions on display list addresses and on GE commands. Currently only exposed in the Windows GE debugger. |
| GE "go to address" | GE | The display list view's address field. |
| WebSocket `cpu.evaluate` | CPU | Evaluates an expression and returns it as both an unsigned integer and a float. |
| WebSocket breakpoint commands | CPU | The `condition` and `logFormat` parameters. |

The debugger also builds expressions internally. "Run to cursor" plants a temporary breakpoint
conditioned on `flipcount > N` so it only counts hits from the next frame onwards, and stepping
conditions its temporary breakpoint on `threadid == ...` so another thread running through the
same address can't complete your step.

## Values and types

Everything is a 32-bit value. There is no separate boolean type: comparisons produce 1 or 0,
and a condition counts as true when the result is non-zero. There is no string type either;
a string is just an address, which the `:s` log format or the string watch format then reads
from memory.

Floats exist, but only barely - see [Floating point](#floating-point) below.

## Numbers

**Numbers are hexadecimal by default.** `10` means 16. This trips up everyone at least once, so
write the radix out when it matters.

| Form | Example | Value |
|---|---|---|
| Bare | `10` | 16, hex is the default |
| `0x` prefix | `0x10` | 16 |
| `0d` prefix | `0d10` | 10, a PPSSPP invention since bare numbers are hex |
| `0b` prefix | `0b1010` | 10 |
| `0o` prefix | `0o17` | 15 |
| `h` suffix | `10h` | 16 |
| `i` or `u` suffix | `10i` | 10 |
| `o` suffix | `12o` | 10 |

A number has to start with a digit, so a hex literal beginning with a letter needs a prefix:
write `0xff` or `0ff`, not `ff`. There is no binary suffix, because `b` is a valid hex digit and
the default radix is hex - `101b` is the hex number 0x101b, not binary. Use the `0b` prefix
instead.

A token containing a single `.` surrounded by digits is a float: `1.5`, `0.25`. Floats are
always decimal.

## Names

A name starts with a letter or `@` and may contain letters, digits, `_`, `@`, `$` and `.`.
Names are case-insensitive and are resolved in two steps: first against the built-in variables
for the current flavour, and only then against the symbol map - the labels loaded from the
game's modules, from a `.sym` file, or created by hand in the debugger. A built-in name
therefore shadows a label of the same name.

A name that resolves to neither is an error: `Invalid symbol "foo"`.

## CPU variables

| Name | Meaning |
|---|---|
| `r0`-`r31` | General purpose registers by number |
| `zero`, `at`, `v0`-`v1`, `a0`-`a3`, `t0`-`t9`, `s0`-`s7`, `k0`-`k1`, `gp`, `sp`, `fp`, `ra` | The same registers by their conventional names |
| `f0`-`f31` | FPU registers, as floats |
| `fi0`-`fi31` | The same FPU registers, as raw 32-bit integers |
| `s000`-`s733` | VFPU registers in single notation, as floats |
| `vi0`-`vi127` | The same VFPU registers by index, as raw integers |
| `pc` | Program counter |
| `hi`, `lo` | The multiply/divide result registers |
| `threadid` | The currently running thread's ID |
| `moduleid` | The module ID the current thread belongs to |
| `usec` | Emulated time in microseconds, truncated to 32 bits |
| `ticks` | CPU ticks since boot, truncated to 32 bits |
| `vcount` | The PSP's vblank counter, as `sceDisplayGetVcount` returns it |
| `flipcount` | Frames actually presented to the screen |

`vcount` and `flipcount` are both useful for "only from now on" conditions, and they are not
interchangeable. A game rendering at 30fps produces two vblanks per frame, so a `vcount`-based
condition lets you through halfway into the frame you were trying to skip. `flipcount` only
advances when the framebuffer actually changed, which is exactly what you want - but it also
means it never advances at all if the game has stopped drawing, or is wedged in the very loop
you are debugging.

## Reading memory

Square brackets read the emulated PSP's memory.

| Form | Meaning |
|---|---|
| `[address]` | Read 32 bits |
| `[address,size]` | Read `size` bytes, where `size` is 1, 2 or 4 |

The address comes first and the size second, which is the opposite order from most assemblers.
Reads are little-endian and zero-extended, so `[sp,1]` is the byte at the stack pointer.

Reads from unmapped addresses quietly return 0 instead of failing. That is deliberate: a
breakpoint condition like `[a0+4] == 1` has to be *configurable* while `a0` still holds
something meaningless, long before the breakpoint is ever reached. The flip side is that a
typo'd address looks like a valid zero rather than an error.

Any size other than 1, 2 or 4 is an error.

## Operators

All of these behave as in C, and precedence is the same as in C. Listed tightest-binding first:

| Operators | Meaning |
|---|---|
| `(` `)` | Grouping |
| `[` `]` | Memory read |
| `+` `-` `~` `!` | Unary plus/minus, bitwise not, logical not |
| `*` `/` `%` | Multiply, divide, modulo |
| `+` `-` | Add, subtract |
| `<<` `>>` | Shift left, shift right |
| `<` `<=` `>` `>=` | Comparisons |
| `==` `!=` | Equality |
| `&` | Bitwise and |
| `^` | Bitwise xor |
| `\|` | Bitwise or |
| `&&` | Logical and |
| `\|\|` | Logical or |
| `? :` | Conditional |

Arithmetic is unsigned and wraps at 32 bits, so `-1` is `0xffffffff` and `>>` is always a
logical shift - there is no arithmetic shift and no signed comparison. Division or modulo by
zero is an error rather than a crash. Unlike C, `&&` and `||` do not short-circuit: both sides
are always evaluated, which matters if one of them reads memory.

**Nested conditionals need parentheses.** `1 ? 2 : 3 ? 4 : 5` evaluates to 4, not 2. Write
`1 ? 2 : (3 ? 4 : 5)` and it behaves as expected.

## Floating point

Float support is real but shallow, and it is easy to get meaningless numbers out of it. The
evaluator has no per-value types - it decides *once per expression* whether that expression is
a float expression, and it decides yes if any float constant or float register appears anywhere
in it. Every operand in the whole expression is then reinterpreted as a float, bit pattern and
all.

So `a0 + 1.5` does not add 1.5 to a pointer. It reinterprets the pointer's bits as a float and
adds 1.5 to that, which is nonsense. Equality is worse: `==` and `!=` always compare raw bits,
so `1 == 1.0` is false.

On top of that, the result of float arithmetic is truncated back to an integer. If `f0` holds
2.5, then `f0 * 2.0` gives 5, and `f0 + 0.5` gives 3.

What does work well, and is what floats are really there for:

- A float register on its own. `f12` yields the register's raw bits, which the `:f` log format
  and the float watch format then display correctly as a float.
- Comparing float registers against float constants, as in `f12 > 100.0`. Comparisons other
  than `==` and `!=` do compare as floats.

For anything else, prefer `fi12` and integer arithmetic on the bit pattern.

## Log format strings

A breakpoint can log a line instead of - or as well as - pausing. The log text is a template:
everything outside braces is copied literally, and each `{expression}` is evaluated and
substituted. An optional `:` and a format letter at the end of the braces picks the formatting.

| Suffix | Output |
|---|---|
| `:x` | Eight-digit hex. The default when no suffix is given. |
| `:d` | Signed decimal |
| `:f` | Float, from the value's bit pattern |
| `:p` | As a pointer: the address in hex, followed by the 32-bit value at that address in brackets, or `[invalid]` |
| `:s` | The zero-terminated string at that address, or `(invalid)` |

For example:

```
sceIoOpen name={a0:s} flags={a1:x} fd={v0:d}
```

An empty `{}` produces a literal `{}`, which is the only way to get a brace through. A `{`
without a matching `}` makes the whole format invalid, and the dialog will refuse it. Note
that `{a0 ? 1 : 2}` works - a trailing `:2` is not one of the format letters, so it is left
alone as part of the ternary.

The same format strings are available through the WebSocket debugger's `logFormat` parameter.

## GE debugger variables

Display list breakpoints and the GE debugger's address fields use the same syntax with a
completely different set of names. Memory reads, numbers and operators all work the same way.

**GE registers by name.** Every GE command can be referenced by its name, for instance
`vertextype`, `framebufptr`, `alphatest` or `prim`. The value is the low 24 bits of the
command, or the float bits for commands that hold floats.

**Bit fields.** Many commands can be broken up with a `.`, so you don't have to shift and mask
by hand: `prim.type`, `prim.count`, `vertextype.pos`, `texsize0.width`, `alphatest.func`. The
available field names depend on the command's format.

**Named constants.** The GE enumerations are available as symbols, both in their full form and
in a short form: `GE_PRIM_TRIANGLES` or `TRIANGLES`, `GE_TFMT_5650`, `GE_COMP_GEQUAL` or
`GEQUAL`, `GE_TFILT_LINEAR` or `LINEAR`. This makes conditions readable:

```
prim.type == TRIANGLES && prim.count > 100
```

**Display list state.**

| Name | Meaning |
|---|---|
| `pc` | Current display list PC |
| `stall` | The list's stall address |
| `op` | The full 32-bit command word at the PC |
| `data` | The low 24 bits of that command word |
| `vaddr`, `iaddr` | Current vertex and index addresses |
| `offset` | The current offset address |
| `bflag` / `boundflag` | Result of the last bounding box test |
| `clutaddr` | Full CLUT address |
| `texaddr0`-`texaddr7` | Full texture addresses per mip level |
| `transfersrc`, `transferdst` | Block transfer source and destination |
| `primcount`, `lastprimcount` | Primitives drawn this frame and last frame |

**Matrices**, as individual float elements: `bone0`-`bone95`, `world0`-`world11`,
`view0`-`view11`, `proj0`-`proj15`, and `tgen0`-`tgen11` (also spelled `texgen0`).

When a command breakpoint's condition runs, the register being written already holds the new
value, so a condition like `texaddr0 == 04000000` sees the address the command is about to set
rather than the previous one.

## Evaluating expressions from a script

The WebSocket debugger exposes the CPU evaluator directly, which is handy for automation:

```json
{"event": "cpu.evaluate", "expression": "[sp+10,4]"}
```

The reply carries `uintValue` and a `floatValue` string. See the WebSocket debugger
documentation for the full protocol.

## Examples

Break only on one caller:

```
ra == 08900abc
```

Break when a struct field crosses a threshold, reading a 16-bit field:

```
[a0+24,2] > 1000
```

Break the first time a function is called with a null pointer, only on the main thread:

```
a0 == 0 && threadid == 0x123
```

Skip the first frame of a hot path, then break every time:

```
flipcount > 0d120
```

Log the arguments of a call without stopping:

```
open path={a0:s} mode={a1:x} from={ra:x}
```

Watch a pointer chain in the watch window - two dereferences, with a field offset:

```
[[08801234]+10]
```

GE: break on a large triangle batch drawn with a specific texture format:

```
prim.type == TRIANGLES && prim.count > 0d200 && texformat.format == GE_TFMT_CLUT8
```
