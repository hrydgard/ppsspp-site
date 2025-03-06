# Scheduling

These are some notes written down by Nemoumbra in the chat (while he was looking at adding some tracing), later translated into this article.

There are (at least) 4 ways for PSP code to break the expected control flow and exit to a different destination from a JIT/IR basic block.

1. Switch threads: done by `__KernelSwitchContext`. PPSSPP (like the PSP) often switches to the two idle threads (idle0 and idle1), so these will also get captured, if you try to record thread switches. `__KernelSwitchOffThread` is used to switch off to idle0. `__KernelSwitchToThread` switches from any idle thread to the specified target. `__KernelReSchedule` with 1 arg is where the thread switch occurs normally. It calls `__KernelNextThread` for the argument to `__KernelSwitchContext`.

2. Running interrupts:
  - `CoreTiming::Advance` calls `CoreTiming::ProcessEvents`, which fires the scheduled events. Some of them are purely PPSSPP-related (for timing events management, duh) and most of them don't result in jumping to PSP code, but a few actually do.
The event handlers which actually consider running PSP code fire the interrupts by doing `__TriggerInterrupt`. At this point it's not clear if we'll jump anywhere, but we know what interrupt it is and what interrupt/subinterrupt handlers there are.
- `__TriggerRunInterrupts` gets called. Then we enter `__RunOnePendingInterrupt`. It may not do anything if there are no pending interrupts, of course, but if there are, we progress by switching off to idle0 (yeah, we also use it for running interrupts!), "do the job" and switch back.
"do the job" is complicated. We still don't know if we want to jump to PSP code at this point! PPSSPP makes a virtual dispatch `if (!handler->run(pend)) { /* code */ }`. From here we can either end up in the default implementation, which forwards the call to `copyArgsToCPU`... or in any of the children's implementations.
- There are 3 children: `GeIntrHandler`, `AlarmIntrHandler` and `VTimerIntrHandler`. The last 2 are supposed to jump (they only don't if there's an error, like... if the alarm is destroyed). Not just that, they ignore their pend arguments, because the destination is dictated by the alarm's or vtimer's data.
- The GE handler also doesn't use the pending interrupt argument. The difference is that it actually runs a subinterrupt (if there are no errors, obviously). The exact subinterrupt depends on the DL/GE state.

3. Thread callbacks, and MIPS actions: it's very cursed, but it all narrows down to `__KernelExecuteMipsCallOnCurrentThread`.
The first caller is `__KernelCallAddress`, which is itself called from `__KernelDirectMipsCall` (unused), `__KernelRunCallbackOnThread` (from `__KernelCheckThreadCallbacks`) and `__KernelThreadTriggerEvent` (way too many places, essentially all thread lifetime management routines invoke it with events `THREADEVENT_DELETE` / `THREADEVENT_CREATE` / `THREADEVENT_START` / `THREADEVENT_EXIT`).
The second caller is `__KernelExecutePendingMipsCalls`, which is itself called from `__KernelSwitchContext` (yeah, again we see it 💀), `__KernelReturnFromMipsCall` (fake emu HLE), `__KernelCheckCallbacks` (from `__KernelReSchedule`) and `__KernelForceCallbacks` (from `__KernelWaitCallbacksCurThread` and `sceKernelCheckCallback`).

So... Of all 3 found ways to handle it, the third one is the most "branchy" and complex. A lot of meaning is lost by the time we get to `__KernelExecuteMipsCallOnCurrentThread`, which modifies the regs, so I don't know where injecting tracing events would make sense the most.
By the way, `__KernelRunCallbackOnThread` goes quickly from 1 to 100. `PSPCallback` is easy, but then it creates `actionAfterCallback`, does `action->setCallback(cbId);`, then `__KernelCallAddress` goes crazy (with enqueueing (is this how it's spelled?) `doAfter`, etc)
Thankfully, `handleResult` of `IntrHandler` and its children don't require tracing.

Ooops, I'm wrong here. There's one more...

4. `sceKernelExtendThreadStack` also jumps. It goes back in `__KernelReturnFromExtendStack`.
Last note: entering and leaving the module will kinda get logged because of all the thread switching stuff, but we may wanna make special events for them. Leaving is done by `__KernelReturnFromModuleFunc` (fake emu HLE), entering is done by `__KernelStartModule` (from `__KernelLoadExec`, `sceKernelStartModule` or the plugin loader).

Also let's not forget about the fact that sceKernelStopModule can actually run the module's stop function on a new thread with the module's name.

## Function replacements

In a few cases, like when games read back from GPU memory using the CPU, we need to hook certain functions and perform an action before or after.

This is of course not needed when using the software renderer, but that one is not an option if you want more resolution and speed that in can provide.

First, the code is scanned to isolate functions, and each function is hashed to a value using a special hash function that discards things like offsets but keeps everything. This is inheritance from early Dolphin.

1. In tableEMU we have `INSTR("CallRepl", JITFUNC(Comp_ReplacementFunc), Dis_Emuhack, Int_Emuhack, 0),`.
2. It gets invoked:
  - Interpreter invokes this `Int_Emuhack` (the behavior depends on the flags)
  - IR Interpreter has a special instruction `IROp::CallReplacement`, where constant means the funcIndex and dest is... eh... some output register. It is assigned either -1 if the number of cycles is negative or 0 otherwise. Also has downcount, btw
  - JIT: It generates a call through the function directly using `ABI_CallFunction`
