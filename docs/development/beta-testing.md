---
position: 2
---

# Beta testing

A few weeks before each new official release, a beta release will be rolled out to participating people,
containing most of the new features that will be in the next release.
Additionally, there'll be a build every couple of months between major releases.

We only have a beta testing program for Android and iOS since Google Play and TestFlight provide convenient tools to automatically roll out new builds to participants.

We don't have anything like that yet on the other platforms.
On PC, if you want to help testing stuff before release, download the latest development build from the [buildbot](/devbuilds) and report results.

## Android<img src="/static/img/icons/android.svg" aria-hidden="true" class="icon-36 icon-right">

The beta testing program for PPSSPP on Android is open.

Tap the appropriate link to join:

<img src="/static/img/platform/ppsspp-icon.png" aria-hidden="true" class="icon-32 icon-left">[Join the beta test!](https://play.google.com/apps/testing/org.ppsspp.ppsspp)

<img src="/static/img/platform/ppsspp-icon-gold.png" aria-hidden="true" class="icon-32 icon-left">[Join the beta test for PPSSPP Gold!](https://play.google.com/apps/testing/org.ppsspp.ppssppgold)

## iOS<img src="/static/img/icons/ios.svg" aria-hidden="true" class="icon-36 icon-right">

The beta testing program for PPSSPP on iOS is open through TestFlight.

[Join the beta test on iOS!](https://testflight.apple.com/join/uNlhFG0m)

## Important information

Beta builds can have bugs and instabilities, but will usually be usable.
If you see unexpected behavior or odd crashes, please report them, as explained below.

If you make a save state in a beta build, it will not load if you go back to the previous "normal" build, but it will load in future versions.
Regular in-game save games are portable back and forth, assuming you keep the files around.

On Android 11 or higher, make sure you are using a manually-selected folder for memstick data.
[More information](/docs/getting-started/save-data-and-storage).

If you have installed PPSSPP through an APK previously, for example from the buildbot, you will have to uninstall,
and install again from Google Play, in order to get your automatic beta updates.

## Reporting bugs and issues

Either report bugs using our [GitHub issue tracker](https://github.com/hrydgard/ppsspp/issues),
or join the [Discord](/contact) and report your issues in the `#general` or `#ios-testflight` channel.

But before reporting, please do a search of the GitHub issue tracker to see if your problem is already known!

<!--
## Upcoming features

The pending `README.md` update can be found here: [Work-in-progress list](https://github.com/hrydgard/ppsspp/pull/20348/files)
-->
