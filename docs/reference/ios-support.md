# PPSSPP for iOS

Apple does not allow emulators in their App Store (though, it seems they [may be reconsidering](/news/apple-announcement-comment)), and does not support side-loading like Android does,
so installing PPSSPP on iOS requires an ever-changing set of hacks.

Due to this, PPSSPP for iOS is not officially supported - though, we do try to keep it building. If it works, it works.

It's highly recommended that if you are interested in emulation on mobile, use Android instead.

However, there are some ways to install it.

## Unofficial installation guide

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
