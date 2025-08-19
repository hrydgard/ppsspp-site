# PPSSPP keeps crashing!

## Step 1: Don't panic

There are still some games that just don't work yet in PPSSPP, and we're always fixing bugs.

If PPSSPP crashes right away, before you even start a game, [scroll down to the bottom](#crashing-on-startup).

## Step 2: Change your settings

Some settings, especially ones meant for speed, can cause crashes.  Under System, there's a way to reset PPSSPP's settings to defaults.  Try this first of all.

Otherwise, try turning off (in order):

* Enable cheats (under System)
* Enable networking/WLAN (under Network)
* Fast memory (under System)
* Change CPU Clock speed (under System)
* Retain changed textures (under Graphics)
* Disable slow framebuffer effects (under Graphics - uncheck it to turn off disabling)
* Anything under hacks in Audio or Graphics
* Frameskipping (under Graphics)
* Simulate block transfer effects (under Graphics - only if you have to)

The first four are the most common causes, by far.

If you had to turn off "Simulate block transfer effects", "Frameskipping", or "Fast memory" - you've found a bug.  The other settings can be unstable.

## Step 3: Check your disc file

Check the CRC of your ISO, and make sure it's a good copy.  Sometimes files can become corrupt, and if you downloaded it, you may easily get a hacked/broken/partial disc.  See [the FAQ](/docs/faq) to learn how to get games that will work properly.

## Step 4: Update your drivers (PC only)

Even if you play other amazing games which don't crash, your video drivers can still be the problem.  Emulators tend to use features of video drivers that most other games don't.  Authors of emulators routinely discover bugs in video drivers for this reason.

Updating your operating system or drivers (if they come separately) often tends to help.  With some old video cards, there may be specific versions that work best - search the forum or Google for your card.

Also, if you changed graphics cards - especially from an AMD or ATI card - you may have old drivers on your system that can cause problems.  There are driver removal tools to uninstall these drivers, which may fix issues.

## Step 5: Try the latest version

PPSSPP often gets fixes and updates.  Even if you're using the latest stable version, the [latest git build](/download#devbuilds) may work even better, and may have already fixed any issue you're hitting.

## Step 6: Check the forum and reports

[PPSSPP's forum](https://forums.ppsspp.org/) has info on whether the game works for other people, and [PPSSPP's reporting section](http://report.ppsspp.org/games) logs reports people made from PPSSPP, along with their system details.

This can help you find if there's a certain version of PPSSPP that used to work, or a certain CRC that's known broken, or even if it just has never worked for anyone.

## If all else fails

Try to find a good way to reproduce the crash, and create a log of the crash.  On Android, an "adb logcat" can help a lot, and on Windows, running DebugLog from a git build will generate a log.

Make sure to search for an issue first, and if you can't find one, create one.  Remember to include as much information as you can.

## If you can't get any log/error message

### For Windows

Try to run PPSSPP in safe mode or with clean boot enabled

Turn off any programs that might cause issues

### For Android/iOS

Turn off any apps that might cause issues. Turn off accessibility settings.

Restart your Device

## Crashing on startup?

If you're getting crashes on startup, before even launching a game, the best things to try are:

1. [Try the latest version](#step-5-try-the-latest-version) - it may already be fixed
2. Restart your device
3. PC only - [update your drivers](#step-4-update-your-drivers-pc-only), and uninstall drivers from all devices
4. Linux only - check for platform bugs, often Mesa or driver issues are known
5. Windows only - try switching to DirectSound (described below)

### Switching to DirectSound on Windows

In 2020, new crashes started occurring for some users which were found to be related to WASAPI.  It's possible as you're reading this, the bug may already have been found and fixed.  But it started causing mysterious startup crashes for users.

A workaround is to switch from WASAPI to DirectSound, which performs worse but doesn't have the crash.

To do this:

1. If you unzipped PPSSPP, look for a folder "memstick" next to PPSSPPWindows64.
2. If you installed PPSSPP, look under My Documents for PPSSPP.
3. Inside the folder you found, open PSP and then SYSTEM inside that.  If these don't exist, create them.
4. Look for a "ppsspp.ini" file.  It it doesn't exist, save one using Notepad - type `"ppsspp.ini"` as the filename, with quotes.
5. Replace the contents of that file with:

   ```ini
   [Sound]
   AudioBackend = 1
   ```

After this, you should be using DirectSound (that's what 1 means) for your audio.
