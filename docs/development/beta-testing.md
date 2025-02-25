---
position: 2
---

# Beta Testing on Android

The beta testing program for PPSSPP for Android is now open!

A few weeks before each new official release, a beta release will be rolled out to participating people, containing most of the new features that will be in the next release. Additionally, there'll be a build every couple of months between major releases.

We only have a beta testing program for Android since Google Play provides convenient tools to automatically roll out new builds to participants. We don't have anything like that yet on the other platforms. On PC, if you want to help testing stuff before release, download the latest development build from the [buildbot](/devbuilds) and report results.

## Joining the Android beta test

Click the appropriate link to join:

<img src="/static/img/platform/ppsspp-icon.png" width="32" alt="icon"> [Join the beta test!](https://play.google.com/apps/testing/org.ppsspp.ppsspp)

<img src="/static/img/platform/ppsspp-icon-gold.png" width="32" alt="icon">[Join the beta test for PPSSPP Gold!](https://play.google.com/apps/testing/org.ppsspp.ppssppgold)

# Beta Testing on iOS through TestFlight

The beta testing program for iOS is now open as well (although only for the free version currently)!

[Join here](https://testflight.apple.com/join/uNlhFG0m).

## Important information

Beta builds can have bugs and instabilities, but will usually be usable. If you see unexpected behavior or odd crashes, please report them, as explained below.

If you make a save state in a beta build, it will not load if you go back to the previous "normal" build, but it will load in future versions. Regular in-game save games are portable back and forth, assuming you keep the files around.

On Android 11 or higher, make sure you are using a manually-selected folder for memstick data. [More information](/docs/getting-started/save-data-and-storage).

If you have installed PPSSPP through an APK previously, for example from the buildbot, you will have to uninstall, and install again from Google Play, in order to get your automatic beta updates.

## Reporting bugs and issues

Either report bugs using our [GitHub issue tracker](https://github.com/hrydgard/ppsspp/issues), or join
the discord ([instructions here](/contact)) and report your issues in the #general channel.

But before reporting, please do a search of the GitHub issue tracker to see if your problem is already known!

## Upcoming features in 1.17

* Performance improvements
* CHD ISO support
* Many bug fixes in for example MLB Baseball games, Tokimeki Memorial 4, Tiger Woods 06, Naruto UNH 2, etc etc.
* AdrenoTools custom driver loading support for Vulkan
* Keep the game running behind the pause menu for less desyncs in multiplayer
* Deadzone fixes for joystick and tilt input, other input fixes
* Bugfixes in texture replacement

And much more! [Work-in-progress list](https://github.com/hrydgard/ppsspp/pull/18677/files#diff-b335630551682c19a781afebcf4d07bf978fb1f8ac04c6bf87428ed5106870f5R28)