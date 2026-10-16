---
position: 10
---
# Display list control

A display list is a sequence of 32-bit commands in RAM: a command number in the top 8 bits and a 24-bit parameter. Most commands set a register or draw. This page is about the others, which decide where the GE reads next and when it stops.

| Command | | What it does |
|---|---|---|
| NOP | 0x00 | Nothing. |
| BASE | 0x10 | Sets bits 24-27 of the addresses that follow, from bits 16-19 of its parameter. |
| OFFSETADDR | 0x13 | Sets the offset added to addresses, to its parameter shifted left by 8. |
| ORIGIN | 0x14 | Sets the offset to the address of the ORIGIN command itself. |
| JUMP | 0x08 | Continues at an address. |
| BJUMP | 0x09 | Continues at an address if the last bounding box test failed. |
| BOUNDINGBOX | 0x07 | Tests a box of vertices against the view. |
| CALL | 0x0A | Continues at an address, and comes back with RET. Two levels only. |
| RET | 0x0B | Returns from a CALL. |
| SIGNAL | 0x0E | Together with the END after it, stops the GE and interrupts the CPU. |
| FINISH | 0x0F | Together with the END after it, ends the list. |
| END | 0x0C | Completes a SIGNAL or FINISH. |

## Addresses

JUMP, BJUMP and CALL, like the vertex and index addresses (VADDR 0x01, IADDR 0x02), take the low 24 bits of their address from the parameter, and bits 24-27 from the last BASE. The offset is added to that.

The offset is 0 unless the list sets it. OFFSETADDR sets it directly, and ORIGIN sets it to where the ORIGIN command is, which makes position-independent lists possible: put an ORIGIN at the start, and address everything relative to it.

## Jumps and the bounding box

JUMP just continues at the target.

BOUNDINGBOX takes a vertex count, and tests that many vertices from the vertex address, in the current vertex format, against the view. It's meant for a box of 8 corners around an object. If the object can't be on screen, the following BJUMP is taken, usually to skip the draw. A count of 0 resets the result, so BJUMP isn't taken.

## CALL and RET

CALL continues at the target and remembers where it came from, and RET goes back. The offset is saved and restored along with the return address, and BASE isn't.

The GE has two registers for return addresses: RADR1 and RADR2 (with OADR1 and OADR2 for the matching offsets). So a list can CALL a subroutine, which can CALL another one, and that's it. **A CALL from the second subroutine does nothing**: the GE doesn't jump, and carries on with the command after the CALL. Nothing is reported, and the list finishes normally.

The firmware's GE driver shows the same thing from its side. Two bits in the GE control register, 0x100 and 0x200 at `0xBD400100`, say whether RADR1/OADR1 and RADR2/OADR2 are in use. When `ge.prx` saves a list's state it reads those bits and zeroes the slots that aren't.

This was measured on a PSP with lists that CALL 1 to 4 levels deep, with each level writing a register before and after its CALL ([`gpu/callstack/depth`](https://github.com/hrydgard/pspautotests/tree/master/tests/gpu/callstack)). The main list and the first two levels run in full, and nothing deeper runs.

### Soul Calibur: Broken Destiny

Soul Calibur's weapon trails are drawn with per-vertex colors, color doubling, and the texture function set to modulate. Before each trail is drawn, the game CALLs its effect setup, which CALLs a few small state blocks. One of them CALLs a third, shared block that turns lighting on and sets the material colors to white without taking them from the vertices.

That third CALL is the one the GE skips. On a PSP, lighting stays off from the block before it, and the trail takes the colors in its vertices, which differ by weapon. An emulator with a deeper call stack runs the shared block too, and draws every trail white. PPSSPP did that for over a decade ([#7395]).

## SIGNAL, FINISH and END

The 24-bit parameters of SIGNAL and FINISH aren't used by the GE itself. When it reaches the END after one of them, it stops and raises an interrupt, and the firmware's GE driver (`ge.prx`) decides what happens next. For SIGNAL it goes by bits 16-23 of the parameter, the behavior, and hands the low 16 bits to the game's signal callback. The low 16 bits of the END are a second argument, used to build an address by the behaviors that need one.

| Behavior | What happens |
|---|---|
| 0x01 | The game's signal callback runs with the GE stopped, then the list continues. |
| 0x02 | The list continues, and the signal callback runs alongside it. |
| 0x03 | The list continues, but is paused when it reaches its FINISH. |
| 0x08 | The list's next FINISH is ignored. |
| 0x10, 0x13, 0x15 | Jump. The address is absolute, relative to the list, or relative to the offset. |
| 0x11, 0x14, 0x16 | Call, with the same three kinds of address. |
| 0x12 | Return from a signal call. |
| 0x20 - 0x2F, 0x30, 0x38 | Set a texture or CLUT address relative to the list or to the offset. |

A FINISH followed by END ends the list. The finish callback runs, and then the next list on the queue starts.

The way the queue, the callbacks and the waiting functions interact is described in detail in [PPSSPP's notes on sceGe](https://github.com/hrydgard/ppsspp/blob/master/docs/sceGe.md), all of it measured with the tests in `gpu/ge` and `gpu/signals`.

### Signal calls

A signal call isn't done by the GE. The firmware's interrupt handler saves the control register, the list address, RADR1/RADR2 and the OADRs into the next entry of the stack the game passed to `sceGeListEnQueue`, points the GE at the target, and restarts it with the control register set to just 1, so both depth bits are clear. A signal return puts all of it back.

So a list that makes a signal call two CALLs deep gets two new levels of CALL inside it, and once the signal return brings it back, it's two deep again and a further CALL does nothing. `gpu/callstack/depth` covers both cases.

## The stall address

Besides the list itself, `sceGeListEnQueue` takes a stall address. The GE stops when it gets there, and waits for `sceGeListUpdateStallAddr` to move it further. That lets a game have the GE draw while it's still writing the rest of the list. A stall address of 0 means none.
