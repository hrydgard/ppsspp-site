# PPSSPP for iOS

Sometime in April-May 2024, Apple changed their App Store policy to allow emulator apps in the App Store. This finally made it possible to [release](/news/live-on-app-store)
 an official iOS version of PPSSPP!

This also means that going forward, the iOS version will be prioritized pretty much as highly as the Android and PC versions for features and performance work.

## Limitations

The one serious limitation of functionality that PPSSPP (and other emulators) are still subject to on iOS is the inability to "JIT", that is, to generate machine code at runtime and run it. This is very useful when emulating foreign CPU architectures such as the PSP's CPU (known as Allegrex). Fortunately, the PSP does not require a JIT compiler to be emulated at full speed on modern iOS devices, although it would reduce battery consumption to be able to do it.

## Sideloading

Before the policy change, you had to "sideload" PPSSPP onto an iPhone, which there are many ways to do. You still have to do this if you want JIT functionality, but since as mentioend it's not that critical for PSP emulation performance, I expect fewer people will pick this option.

## Unofficial sideload installation guide

IMPORTANT: This guide is unofficial - the project is not responsible for any negative results, and will not answer
questions about it. You're on your own!

Also, this is just one way to install it. There may be others.

[Sugoyga's SideStore guide for PPSSPP on iOS](https://suyogya.link/installing-sidestore-and-ppsspp-on-ios/)

## Missing features

The iOS build is missing a few features:

* Virtual keyboard input
* Camera input
* Microphone

## Logging in to RetroAchievements

Due to the missing keyboard, this is not possible. You'll have to login on another device, then transfer your login manually.

You'll have to figure out the exact paths yourself, can't help you with this.

What you need to transfer, to the corresponding place on the iOS device is:

* PSP/SYSTEM/ppsspp_retroachievements.dat
* In PSP/SYSTEM/ppsspp.ini, from the section `[Achievements]`, the line starting with `AchievementsUserName`
