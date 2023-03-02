---
sidebar_position: 2
---

# FAQ - Frequently Asked Questions

Quick answers to questions that are frequently asked.

## How can I run my real PSP games in PPSSPP?

You need to have your PSP games as .CSO or .ISO files. I do not have the right to distribute those with the app, so you'll have to provide them on your own.

To convert your real PSP games for use with PPSSPP, see [this article](docs/getting-started/dumping-games.md).

## PPSSPP is awesome! How do I donate to the project?

Buy [PPSSPP Gold](/buygold)! Available for Android and PC. It's the same as the regular version functionally (see [Why Gold?](/docs/reference/whygold)), but by buying it you support the development of PPSSPP.

## If I buy PPSSPP Gold for Android, can I also download PPSSPP Gold for PC? Or vice versa?

See the [Cross Licensing](/requestgold) page!

## Why is the emulator called PPSSPP?

Why not? The domain name ppsspp.org was available, unlike the corresponding domains for many other alternatives I considered. Today I probably would have named it something different and more pronounceable. At least it's kinda fun watching various YouTube personalities try to pronounce it - sorry!

## Where can I get PPSSPP for iOS?

PPSSPP can run on most modern iOS versions. On some versions, the JIT works. See the [Downloads page](/download) for more info.

<a name="vita"></a>

## Will PPSSPP be able to emulate the PS Vita in the future?

No. PS Vita has a completely different machine architecture, much more powerful than the PSP and with different security technologies. I don't have either the information needed nor the time.

Do look into the [Vita3K](https://vita3k.org/) project though! They are making good progress, although game compatibility is still quite low.

## How do I install game DLC?

Install it exactly the same way as you would on a PSP, that is, copy the files to PSP/GAME or PSP/SAVEDATA (depending on the DLC) on the memory stick. In the Android version of PPSSPP, the memory stick is simply the SD card or USB storage of your phone, PPSSPP will create a PSP folder in the root of that. On Windows without installer, the memory stick is the "memstick" subdirectory in the PPSSPP folder. On iOS, it's in /User/Documents/PSP/ . On Mac and Linux, it's in ~/.config/PPSSPP.

## Do I need a BIOS file to run PPSSPP, like I would with PSX/PS1 and PS2 emulators?

No. PPSSPP simulates the BIOS and the entire internal operating system. It does not currently emulate enough of the hardware for the actual PSP operating system to run inside of PPSSPP, so even if you have a copy of it, PPSSPP can't run it.

This is also why PPSSPP will not show the cross media bar (XMB) interface of the real PSP, it won't run.

<a name="gold"></a>

## Can I use my gamepad to control PPSSPP?

Yes, PPSSPP has built-in XInput and DirectInput support on Windows so it will "just work" with any Xbox 360 pad and most other pads that you plug into your PC.
On Android, many Bluetooth gamepads like iPega Red Knight work just fine, sometimes with a few limitations.

## My XBox or PlayStation joystick doesn't work on Android

Apparently, some accessibility options can interfere with joystick functionality. Try turning any accessibility settings off in Android settings. This behavior has been seen on Google Pixel phones.

It seems like apps like Quick Cursor that draw over other apps can also cause this, by seemingly taking over joystick input.

The bug has been reported to Google, still no fix: [issue report](https://issuetracker.google.com/issues/163120692?pli=1)

## Can I play adhoc multiplayer locally with two instances of PPSSPP?

Yes, although it's not a super smooth experience. Follow this:

* Set "Pro adhoc server IP address" to localhost
* Enable "Built-in proadhocserver"
* Start a second instance (File -> Open New Instance on Windows).

## Can PPSSPP play UMD video discs?

No. On the real hardware, the player app for these is built into the PPSSPP firmware, and since PPSSPP is a HLE emulator, we don't run the firmware so we'd have to write our own player. There's a scripting language for menus and stuff, it's pretty complicated. So it has not been a priority to figure out, especially as UMD Video is today an outdated, low-definition format and there are better ways to watch movies. If you really want to play UMD video, use a real PSP.

## Why do cutscenes and videos look so blurry?

The prerecorded video cutscenes on the PSP are of wildly varying quality, but the common denominator is that they were all judged to be "good enough" for the small low-resolution screen of the real actual PSP. They were never meant to be played on modern devices with large, bright screens, so quality is often just good enough to be passable on a real PSP screen, in order to save disk space for example, unfortunately. Not much that can be done.

<a name="xmb"></a>

## Can you get PPSSPP's menu to look like the real PSP?

The "Cross media bar" (XMB) is patented by Sony so the idea is iffy - and we'd have to write an
imitation, rather than the real thing, since PPSSPP doesn't use the original firmware of the PSP
that contains the menu, and is missing a lot of functionality that only the menu, but not
games, uses.

Additionally, even if we wrote our own, it would be hard to use on touchscreens or with
a mouse. PPSSPP tries to have the same UI everywhere for practical reasons.

<a name="arm64win"></a>

## What is PPSSPP Windows for ARM64?

Since a few years, Windows is not only available for x86 and x86-64 compatible PCs, but also for so-called "Windows on ARM" laptops, such as the Surface Pro 9 with 5G, or the Lenovo ThinkPad X13s. These can run x86 apps but since they have to be runtime-translated, this has a performance penalty.

PPSSPP Windows for ARM64 is a native build of PPSSPP for these devices, enabling utilizing their full performance potential.

## Is it generally better to use stable or dev (git) builds?

Usually, git builds are pretty stable and include all the recent fixes, but sometimes they have bugs that haven't been fixed yet and weren't there in the last stable version. So it's up to you. Beware that save states saved in dev builds may not be compatible with the stable builds.

## Save states seem slower in 1.12+. What can I do?

Disabling save state backups will make save/load faster, but also disables save/load undo.

<a name="memstick"></a>

## Where is the memory stick folder?

If you have PPSSPP 1.12 or later and are on desktop, you can open it directly from within the emulator. Just go to Settings/System and choose Open Memstick Folder.

Where it is depends on the platform:

* Windows: Either in the directory "memstick" in PPSSPP, or in your documents directory. There's also an additional shortcut, just choose "File->Open Memstick Folder..." to find it.
* Mac/Linux: Look in .config/PPSSPP in your home directory. Some distributions may have it in other places.
* Android: In older versions it was always in the /PSP directory at the root of your shared storage. In Android 11 and later with PPSSPP 1.12 or later, it's configurable.

Sharing controls between the two instances can be an issue though..

## What are the PC CPU and GPU requirements?

Any reasonably modern CPU will be just fine, and any GPU that can handle OpenGL 3.0 should have no issue. You should make sure to install the latest graphics drivers available though. Windows Vista or later is required, Windows 7 or higher is recommended. Vulkan will likely help performance where available, also try D3D9 or D3D11 if OpenGL is slow by changing the backend in settings. On some older computers, you may need to use the D3D9 backend, but it should be a last resort as it's slightly less compatible than the others.

## Where are the "git" versions people are talking about?

[Here](/download#devbuilds).

## What are CSO files?

CSO are compressed ISO files that can be played directly, decompressing on the fly. Very useful to save space on your Android device, for example. [MaxCSO](https://github.com/unknownbrackets/maxcso/releases) is a great program to create CSO files. Of course, there are others around the web, too.

## I've managed to fix a bug, how do I get the fix into PPSSPP?

If you know GitHub, you know the drill - just make a pull request with the changes, in a clone of the [PPSSPP repository](https://github.com/hrydgard/ppsspp). If you don't know Git(Hub), feel free to [ask for help](/contact).

## My favorite game doesn't work in PPSSPP. What can I do?

You can either [help out with fixing it](/docs/category/development), or wait until someone does.

## What is the JIT and why can't we use it on iOS?

To emulate advanced systems like the PSP fast, the emulator needs to translate the machine code language of the PSP to the machine code language of your PC or mobile device at runtime. This is done with a **"Just-In-Time recompiler"** or JIT, also known as a Dynarec. PPSSPP has JITs for x86 and ARM, 32-bit and 64-bit.

For a JIT to function, an app needs to have the ability to generate machine code at runtime. This is allowed on Windows, Mac, Linux and Android, while it is disallowed on many versions of iOS.

## Does PPSSPP work on Chromebooks?

Chromebooks can run the Android version of PPSSPP. However it has not yet been adapted for keyboard input, so you'll want to use an external bluetooth controller for now.

## How do I turn off buffered rendering in PPSSPP 1.14 or later?

The option is still there, but now it's under "Speedhacks" and called "Skip buffer effects".

Note that it may cause various rendering issues and missing graphics depending on the game, just like before.

## How do I get the IPEGA Red Knight (and similar IPEGA pads) to work with PPSSPP?

First, make sure you have charged it to the max once. If you don't, the normal Android mode will not work!

Then, just flip the power switch to on, and press Home+X to start it in Android mode. After that, things should just work! You may want to tweak the controls a little bit in Control Mapping but the defaults are mostly okay.

## My app is on the PPSSPP Homebrew Store and I do not approve!

Shoot me an e-mail (hrydgard at gmail dot com) and I'll remove it.

## Where can I find the privacy policy?

Here: [The PPSSPP privacy policy](/privacy)

## Can PPSSPP play GTA 5?

No, it can't. Silly Youtubers.

## Does PPSSPP work on Chromecast?

This is not currently actively supported. In theory it should work, but many people have reported problems with folder selection and file permissions.

## My SD card won't work for loading games after upgrading to Android 13!

I've had a couple of reports of this, and one customer reported being able to solve the problem by reformatting the SD card from scratch, and copying back the stuff on it. Still unclear what was going on.

In general, SD cards are a bit iffy on the more recent Android versions and can have some really strange problems, unfortunately.