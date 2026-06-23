# PPSSPP for iOS

Sometime in April&ndash;May 2024, Apple changed their App Store policy to allow emulator apps in the App Store.
This finally made it possible to [release](/news/live-on-app-store) an official iOS version of PPSSPP!

This also means that now the iOS version is prioritized nearly as highly as the Android and PC versions for features and performance work.

## PPSSPP Gold for iOS

[See here for information.](/buygold_ios)

## Limitations

The one serious limitation of functionality that PPSSPP (and other emulators) are still subject to on iOS is the inability to "JIT",
that is, to generate machine code at runtime and run it.
This is very useful when emulating foreign CPU architectures such as the PSP's CPU (known as Allegrex).

Fortunately, the PSP does not require a JIT compiler to be emulated at full speed on modern iOS devices,
although it would reduce battery consumption to be able to do it.

## Installation

You can install PPSSPP through the App Store:

<a href="https://apps.apple.com/us/app/ppsspp-psp-emulator/id6496972903" class="download-button button-block">
    <div class="button-contents"><img src="/static/img/platform/appstore-badge.svg"
            alt="Download on the App Store" class="badge">Tap here to download PPSSPP from the App Store!</div>
</a>

### Sideloading

Before the policy change, you had to "sideload" PPSSPP onto an iPhone, which there are many ways to do.
You still have to do this if you want JIT functionality, but since as mentioend it's not that critical for PSP emulation performance,
I expect fewer people will pick this option.

#### Unofficial sideload installation guide

IMPORTANT: This guide is unofficial &ndash; the project is not responsible for any negative results,
and will not answer questions about it.
You're on your own!

Also, this is just one way to install it.
There may be others.

[Sugoyga's SideStore guide for PPSSPP on iOS](https://suyogya.link/installing-sidestore-and-ppsspp-on-ios/)

## Transferring files

First, make sure you have your games as ISO (or CSO or CHD) files.
See [How to get games](how-to-get-games) and [How to dump games](dumping-games).

Connect your device via USB to a Mac and use Finder to copy over games and savedata directly to the app.
After this, they will be located on the virtual Memory Stick.

If you don't have a Mac, use a Windows device and install [Apple Devices app](https://apps.microsoft.com/detail/9np83lwlpz9k).

From PPSSPP 1.20, you can also click the Upload Files icon (a folder with an up arrow on it) on the <b class="inapp">Games</b> tab
and follow the instructions, to transfer files from another device or computer on the same Wi-Fi network.

## Missing and upcoming features

### Missing features

The iOS build is missing a few features:

- Camera input
- Microphone

### Previously missing features

These have been fixed:

- UI keyboard input (now works, so RetroAchivement login works too)
- Background image replacement (the easy way) (1.19)
- Device rotation (1.20)
