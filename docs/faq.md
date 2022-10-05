---
sidebar_position: 2
---

# Frequently Asked Questions

Quick answers to questions that are frequently asked.

## How can I run my PSP games in PPSSPP?

You need to have your PSP games as .CSO or .ISO files. I do not have the right to distribute those with the app, so you'll have to provide them on your own. To convert your real PSP games for use with PPSSPP, you need to install a "Custom Firmware" on your PSP. Google for that. Then follow these steps:

* Insert the UMD into your PSP.
* Connect your PSP to your PC with a USB cable.
* At the PSP main menu, press Select. In the menu that pops up (only on Custom Firmware), choose to UMD as USB Device.
* Still in the PSP menu, choose USB Connection in the Settings menu (to the left).
* On your PC, a folder will pop up, containing a virtual ISO file. To copy the game to your PC, simply drag this to somewhere on your harddrive and the copy will start. Done!

There are tools to turn ISO files into CSO (compressed ISO) files to save space, such as maxcso by [Unknown].

If you have digital downloads on your real PSP, they can be used directly on PPSSPP. Just copy the EBOOT.PBP over. Note that this has not been tested as much as ISO loading so there may still be issues with some games.

## PPSSPP is awesome! How do I donate to the project?

Buy [PPSSPP Gold](https://central.ppsspp.org/buygold)! Available for Android and PC. It's the same as the regular version functionally (see [Why Gold?](https://central.ppsspp.org/whygold)), but by buying it you support the development of PPSSPP.

## Where can I get PPSSPP for iOS?

PPSSPP can run on most modern iOS versions. On some versions, the JIT works. See the [Downloads page](/downloads.html) for more info.

## How do I install game DLC?

Install it exactly the same way as you would on a PSP, that is, copy the files to PSP/GAME or PSP/SAVEDATA (depending on the DLC) on the memory stick. In the Android version of PPSSPP, the memory stick is simply the SD card or USB storage of your phone, PPSSPP will create a PSP folder in the root of that. On Windows without installer, the memory stick is the "memstick" subdirectory in the PPSSPP folder. On iOS, it's in /User/Documents/PSP/ . On Mac and Linux, it's in ~/.config/PPSSPP.

<a name="vita"></a>

## Will PPSSPP be able to emulate the PSP Vita in the future?

No. PSP Vita is a completely different machine, far more powerful than the PSP and with different security technologies. I don't have neither the information needed nor the time.

## Do I need a BIOS file to run PPSSPP, like with PSX and PS2 emulators?

No. PPSSPP simulates the BIOS and the internal OS. It does not currently emulate enough of the hardware for the actual PSP operating system to run, so even if you have a copy of it, PPSSPP can't run it.

## Why is the emulator called PPSSPP?

Why not? The domain name ppsspp.org was available, unlike the corresponding domains for many other alternatives I considered. Today I probably would have named it something different and more memorable.

## If I buy PPSSPP Gold for Android, can I also download PPSSPP Gold for PC? Or vice versa?

Contact me at hrydgard+ppssppgold@gmail.com and I'll set you up.

## Can I use my gamepad to control PPSSPP?

Yes, PPSSPP has built-in XInput and DirectInput support on Windows so it will "just work" with any Xbox 360 pad and most other pads that you plug into your PC.  
On Android, many Bluetooth gamepads like iPega Red Knight work just fine, sometimes with a few limitations.

## Can I play adhoc multiplayer locally with two instances of PPSSPP?

Yes, although it's not a super smooth experience. Follow this:

* Set "Pro adhoc server IP address" to localhost
* Enable "Built-in proadhocserver"
* Start a second instance (File -> Open New Instance on Windows).

## Savestates seem slower in 1.12+. What can I do?

Disabling savestate backups will make save/load faster, but also disables save/load undo.

<a name="memstick"></a>

## Where is the memory stick folder?

If you have PPSSPP 1.12 or later and are on desktop, you can open it directly from within the emulator. Just go to Settings/System and choose Open Memstick Folder.

Where it is depends on the platform:

*   Windows: Either in the directory "memstick" in PPSSPP, or in your documents directory. There's also an additional shortcut, just choose "File->Open Memstick Folder..." to find it.
*   Mac/Linux: Look in .config/PPSSPP in your home directory. Some distros may have it in other places.
*   Android: In older versions it was always in the /PSP directory at the root of your shared storage. In Android 11 and later with PPSSPP 1.12 or later, it's configurable.

Sharing controls between the two instances can be an issue though..

## What are the PC CPU and GPU requirements?

Any reasonably modern CPU will be just fine, and any GPU that can handle OpenGL 2.0 should have no issues. You should make sure to install the latest graphics drivers available though. Windows Vista or later is required, Windows 7 or higher is recommended. Vulkan will likely help performance where available, also try D3D9 or D3D11 if OpenGL is slow by changing the backend in settings. On some older computers, you may need to use the D3D9 backend.

## Where are the "git" versions people are talking about?

[Here](/downloads.html#devbuilds).

## What are CSO files?

CSO are compressed ISO files that can be played directly, decompressing on the fly. Very useful to save space on your Android device, for example. [MaxCSO](https://www.github.com/unknownbrackets/maxcso/releases) is a great program to create CSO files. Of course, there are others around the web, too.

## I've managed to fix a bug, how do I get the fix into PPSSPP?

If you know GitHub, you know the drill - just make a pull request with the changes, in a clone of the [PPSSPP repository](development.html). If you don't know Git(Hub), feel free to [ask for help](contact.html).

## My favorite game doesn't work in PPSSPP. What can I do?

You can either [help out with fixing it](development.html), or wait until someone does.

## What is the JIT and why can't we use it on iOS?

To emulate advanced systems like the PSP fast, the emulator needs to translate the machine code language of the PSP to the machine code language of your PC or mobile device at runtime. This is done with a **"Just-In-Time recompiler"** or JIT, also known as a Dynarec. PPSSPP has JITs for x86 and ARM, 32-bit and 64-bit.

For a JIT to function, an app needs to have the ability to generate machine code at runtime. This is allowed on Windows, Mac, Linux and Android, while it is disallowed on many versions of iOS.

## Does PPSSPP work on Chromebooks?

Chromebooks can run the Android version of PPSSPP. However it has not yet been adapted for keyboard input, so you'll want to use an external bluetooth controller for now.

## How do I get the IPEGA Red Knight (and similar IPEGA pads) to work with PPSSPP?

First, make sure you have charged it to the max once. If you don't, the normal Android mode will not work!

Then, just flip the power switch to on, and press Home+X to start it in Android mode. After that, things should just work! You may want to tweak the controls a little bit in Control Mapping but the defaults are mostly okay.

## My XBOX or PlaySTation joystick doesn't work on Android!

Apparently, accessibility options can interfere with joystick functionality. Try turning any accessibility settings off in Android settings. This behavior has been seen on Google Pixel phones.

It seems like apps like Quick Cursor that draw over other apps can also cause this, by seemingly taking over joystick input.

The bug has been reported to Google, still no fix: [issue report](https://issuetracker.google.com/issues/163120692?pli=1)

## My app is on the PPSSPP Homebrew Store and I do not approve!

Shoot me an e-mail (hrydgard at gmail dot com) and I'll remove it.

## Where can I find the privacy policy?

Here: [The PPSSPP privacy policy](/privacy.html)
