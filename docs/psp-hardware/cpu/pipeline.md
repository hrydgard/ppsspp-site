---
position: 3
---
# Pipeline and instructions

The Allegrex issues **one instruction per cycle, in order**. Integer, FPU and VFPU instructions never issue together: a loop of independent `addu`, `add.s` and `vadd.q` instructions takes one cycle per instruction, the same as a loop of `addu` alone. The FPU and VFPU are pipelined, so they keep up with that rate as long as nothing waits for their results.

All figures are CPU cycles at 222 MHz.

## Latencies

The latency here is how many cycles after a producer a consumer of its result can issue without stalling. With a latency of 3, two unrelated instructions in between hide it completely, and a consumer right after the producer stalls for 2 cycles.

| producer | consumer | latency |
|---|---|---|
| integer ALU op, including `clz`, `ext`, `ins`, `min`, `max`, `seb`, `wsbh`, `bitrev`, `movz` | anything, including a load or store address or a branch | 1 |
| `lw`, `lh`, `lb`, `lbu` | integer use, load address | 3 |
| `lwl` + `lwr` | integer use | 3 after the `lwr` |
| `mult`, `multu` | `mflo`, `mfhi` | 3 |
| `mflo` | integer use | 1 |
| `lwc1` | FPU op, `mfc1` | 4 |
| `mtc1` | FPU op | 4 |
| `mfc1` | integer use | 3 |
| FPU op (`add.s`, `mul.s`, `cvt.w.s`...) | FPU op, `mfc1`, `swc1`, `c.lt.s` | 5 |
| `c.cond.s` | `bc1t`, `bc1f` | 1 |
| `lv.s`, `lv.q` | VFPU op | 5 |
| VFPU `vadd`, `vmul` and similar | VFPU op | 5 |
| `vmov` | VFPU op | 3 |
| `vdot`, `vhdp`, `vrcp`, `vsin` and the other special functions | VFPU op | 7 |

**A taken branch's two lost cycles count towards a latency.** A load in a taken branch's delay slot whose value is used at the branch target doesn't stall at all, and an FPU operation there stalls 2 cycles less. Compilers put loads in delay slots often, so this matters for modelling real code.

**An FPU operation that writes a register an earlier one is still computing waits for it**, even when it doesn't read it: eight `trunc.w.s $f0, $f9` in a row take about 4.5 cycles each, almost as long as a dependent chain, and the same eight writing eight different registers take one cycle each.

## Branches and jumps

| | cycles |
|---|---|
| branch not taken, including branch-likely | 1, plus its delay slot |
| branch taken (`b`, `bal`, `beq`, `bne`, `bc1t`, `bvt` and the rest), forward or backward | 3, plus its delay slot |
| `j`, `jal`, `jr`, `jalr` | 2, plus its delay slot |
| call to a function that returns right away (`jal`, `nop`, `jr $ra`, `nop`) | 6 |

- **There's no dynamic branch prediction.** A branch following a 32-step pattern costs exactly 2 cycles per taken branch whatever the pattern: random, alternating, or periodic with any period up to 32 all cost the same as long as the number of taken branches is the same.
- A branch-likely that isn't taken still costs the cycle of its annulled delay slot.
- Where the target falls within a cache line doesn't matter.
- Comparing a value produced by the previous instruction costs nothing extra, but comparing a loaded value pays the load's latency like any other use.

## Integer

All ALU operations, including the Allegrex additions (`ext`, `ins`, `min`, `max`, `seb`, `seh`, `wsbh`, `wsbw`, `bitrev`, `clz`, `clo`, `rotr`, `rotrv`, `movz`, `movn`), take one cycle with a latency of one. `sync` and `mfc0` take one cycle as well.

### Multiply and divide

| | cycles |
|---|---|
| `mult`, `multu` | issue every cycle; the product is ready 3 cycles later |
| dependent `mult` → `mflo` → `mult` chain | 4 per multiply |
| `madd`, `msub`, back to back | ~3.6 each |
| `mtlo` + `mflo` | ~2.9 |
| `div`, `divu`, large dividend (`0x7FFFFFF0 / 7`) | 35 |
| `div`, `divu`, small dividend (`100 / 7`) | 19 |

- **The divide time depends on the data.**
- **A divide runs in the background.** Integer work issued after the `div` and before the `mflo` costs nothing extra: a `div`, 32 ALU instructions and an `mflo` take the same time as a `div` and an `mflo`.
- **A new `div` or `mult` cancels a divide that's still running.** `div`, `div`, `mflo` takes 36 cycles, not 70, and `div`, `mult`, `mflo` takes 5.
- `mtlo` waits for a running divide to finish.

## Loads and stores

- A load that hits the D-cache issues in one cycle. Its value is ready 3 cycles later.
- **A store keeps the memory unit busy for about 3 cycles.** Back-to-back `sw`, `sb` or `swc1` cost about 2.9 cycles each. One ALU instruction between two stores still costs 2.9 for the pair, and two hide it completely.
- **A load right after a store waits about 2 cycles**, whatever address it reads: the same word, the same line or somewhere else entirely.
- `sv.s` costs about 6.9 and `sv.q` about 6.75 back to back. `lv.q` can issue every ~3.4 cycles.
- `swl` + `swr` cost 5.75 per pair, `ll` + `sc` 3.75.
- A miss or an uncached access stalls the whole CPU at the load or store itself, whether its value is used or not. See [Memory](/docs/psp-hardware/cpu/memory).

## The FPU

| | latency | a new one every |
|---|---|---|
| `add.s`, `sub.s`, `mul.s`, `abs.s`, `neg.s`, `mov.s`, `cvt.*`, `trunc.w.s` | 5 | 1 |
| `div.s` | ~31 | ~29 |
| `sqrt.s` | ~29 | ~28 |

- **`div.s` and `sqrt.s` aren't pipelined, and they hold up the whole FPU.** Integer work runs alongside them, but every further FPU instruction waits until they're done, whether it needs the result or not. Two `div.s` in a row take 62 cycles.
- `mul.s` followed by a dependent `add.s` waits the same 5 cycles as two `add.s`: there's no fused path.
- A compare and the branch that tests it can be back to back: `c.lt.s` + `bc1f` + delay slot is 3 cycles. A compare of a fresh FPU result waits the usual 5.

## The VFPU

The vector width (`.s`, `.p`, `.t`, `.q`) doesn't change any of these.

| operations | latency | a new one every |
|---|---|---|
| `vadd`, `vsub`, `vmul`, `vmin`, `vmax`, `vscl`, `vsgn`, `vi2f`, `vf2in`, `vf2iz` | 5 | 1 |
| `vmov` | 3 | 1 |
| `vidt`, `vi2uc` | ~2.5 | 1 |
| `vfim` | 1 | 1 |
| `vdot`, `vhdp`, `vavg` | 7 | 1 |
| `vrcp`, `vrsq`, `vsqrt`, `vsin`, `vcos`, `vexp2`, `vlog2`, `vasin` | 7 | 1 |
| `vdiv.s` | ~17 | ~13 |
| `vcrsp.t` | ~9 | ~3 |
| `vqmul.q`, `vtfm4.q` | ~10 | ~4 |
| `vmmul.q` | ~16 | ~15 |
| `vmmov.q`, `vmscl.q` | ~4 | ~4 |
| `lv.q` | 5 to a VFPU op | ~3.4 |
| `sv.q` | | ~6.75 |

- **The special functions are fully pipelined.** A `vsin.s` or `vrsq.s` can start every cycle, like a `vadd`.
- A prefix (`vpfxs`, `vpfxt` or `vpfxd`) costs one cycle of issue and adds no latency.
- `vdiv.s` holds up the VFPU while it runs, the way `div.s` holds up the FPU. The two divides don't hold each other up, and an FPU divide doesn't stall VFPU work.
- `vcmp` + `vcmovt`: ~5.4. `vcmp` + `bvt` + delay slot: 10.
- Matrix operations cost about as much as the vector operations they're made of: `vmmul.q` is four `vtfm4.q`.

## Moving values between units

Every transfer between the GPRs, the FPU and the VFPU has a latency of its own, so a value that crosses over and back costs a lot more than the instructions suggest. Round trips, in cycles:

| path | cycles |
|---|---|
| FPU → GPR → FPU (`mfc1`, `addu`, `mtc1`) | ~6.75 |
| `add.s` → `mfc1` → `mtc1` | 12.5 |
| `mtc1` → dependent `add.s` | 9.5 |
| VFPU → GPR → VFPU (`mfv`, `addu`, `mtv`) | ~11.75 |
| `vadd.s` → `mfv` → `mtv` | 15.75 |
| `lwc1` → `add.s` → `swc1` | ~12.8 |
| `lv.q` → `vadd.q` → `sv.q` | 14.75 |
| `add.s` → `swc1` → `lw` → `mtc1` | 15.5 |
| `add.s` → `c.lt.s` → `bc1t` | 8 |
| `vadd.s` → `vcmp` → `bvt` | 15 |

Going through memory is no faster than the direct moves, and the VFPU side is slower than the FPU side throughout.
