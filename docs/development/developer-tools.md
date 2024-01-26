---
position: 8
---

# Developer tools

You can reach the developer tools in Settings/Tools/Developer Tools.

A large amount of obscure settings and test screens that have been added over the years as needed to help development. Here are some guides to some of them that might be useful. This section will be expanded over time.

## Texture replacement

This is due to graduate out to the normal settings soon, although, "Save new textures" kind of still belongs here
so not sure what to do.

See [Texture replacement](/docs/reference/texture-replacement) and [Using replacement textures](/docs/reference/use-texture-replacement) for more detailed information.

### Save new textures

When this is checked, new textures encountered while running a game will be saved to `/PSP/TEXTURES/{GAME_ID}/new` under your memstick directory.

### Replace textures

This enables replacement of textures with textures in a folder, see above.

## General

### CPU Core

PPSSPP has multiple CPU emulation cores built-in, and you can choose between them here.

Generally, the best choice is Dynarec (JIT), since it's the most tested one and performs very well.

#### Interpreter

Interprets instructions one by one. Very simple and slow, mostly useful for debugging the JIT.

#### Dynarec (JIT)

Directly recompiles MIPS code to native code for the platform PPSSPP runs on. Performs very well and is the default choice.

#### IR Interpreter

Recompiles blocks of MIPS instructions into a specially designed "intermediate representation", which is faster to interpret than directly interpreting raw MIPS instructions. It also runs some optimization passes on this intermediate representation so it can often reduce the raw amount of instructions to interpret, compared to the regular interpreter.

#### JIT using IR

This takes the output from the IR interpreter and recompiles into native code. Theoretically, and also in practice most of the time, this can generate better code and thus runs faster than the Dynarec (JIT). However, it's not as well tested.

## JIT debug tools

Lets you selectively enable/disable different parts of the JIT. Useful for debugging, but checking these boxes will only make things slower, so not recommended for general use (like most of the things in developer tools).

### Show developer menu

This makes a "DevMenu" button show up during gameplay, which provides direct access to a bunch of handy tools. It's also the only way to access the JIT block viewer and the shader viewer.

### Dump decrypted EBOOT.BIN on game boot

Does what it says on the tin.

### Run CPU tests

Not generally useful.

### Debug overlay

Lets you choose one of a variety of overlays that shows various shows realtime information which can be useful for debugging games. It is primarily a developer option.

### Multi-threaded rendering

This turns on and off multi-threaded rendering when using the Vulkan backend. Turning it off will generally lose you some performance. In reality, this is only useful for debugging.

### GPU driver test

Shows a screen that tests some obscure state combinations that have been problematic in the past. Most modern phones pass these tests with flying colors.

### Enable driver bug workaround

Leave this on.

### Framedump tests

A set of tests for different graphics features, in the form of downloadable frame dumps. This is not meant for end-users.

### Touchscreen test

Interactive test of various aspects of input.

### Allow remote debugger

Starts up a server that lets you connect to PPSSPP via websockets to debug things programmatically. This is very sparsely documented so far.

### Show on-screen messages

Lets you globally turn off on-screen notification. Not a good idea to turn off.

### Enable debug logging

Globally turns on/off all logging.

### Logging channels

Lets you turn on and off logging for different sections of the emulator, and filter by level.

### Log dropped frame statistics

Whether to write some stats to the log from time to time.

### GPU log profiler

Not generally useful.

## Ubershaders

There's a tradeoff to make between generating many specialized shaders vs fewer more general shaders - the former perform better on older hardware. PPSSPP will autodetect the best option, but this allows you to override the choice.

## Stereo rendering

Experimental, don't enable.