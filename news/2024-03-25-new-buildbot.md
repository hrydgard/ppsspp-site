---
slug: new-buildbot
title: PPSSPP has a new buildbot, and Android builds now include VR
authors: hrydgard
tags: [releases]
---

Due to a 3+ week downtime of Orphis' buildbot, I went ahead and set up my own to cover.

The builds that it produces can be found here: [Development builds](/devbuilds).

Currently, it builds for Android, Windows and Windows for ARM64 processors. More platforms will be added
in the future.

Due to using different signing keys, you can't directly upgrade from an Orphis-build to these, but you can upgrade from an official release APK downloaded from this website to a buildbot build.

## But what's a buildbot?

A classic emulator buildbot makes new builds (emulator executables) for every single change to the emulator source code. This can be very useful when tracking down regressions, that is, finding out when specifically something that was working before now broke.

By using the halving or bisection method, you can often track down errors over huge ranges of commits in a small number of tries. Simply try two builds that you know work and don't, then try the one in the middle between them. Then if it's still broken, try the range from the first commit to the one you just tried, and if not try the second half. Repeat!

## Android builds now include VR

From build v1.17.1-60, our Android builds are cross-compatible between phones and VR setups like the Quest 2. So there's no longer any need for a separate downloadable VR app.