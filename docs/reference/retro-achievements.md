# RetroAchievements - setup and use

[RetroAchievements](https://retroachievements.org/) are similar to XBox achievements or PlayStation trophies, except that they're created and defined by a volunteer community. Just like other retro consoles, the PSP itself has no native concept of achievement/trophies. However, this can be added with emulation!

Not only achievements are supported, but also global leaderboards in games that have scores or times.

RetroAchievements are supported from [PPSSPP version 1.16](1-16-release-announcement). The feature is not supported in 1.15.4 and older.

From 1.16.1, you can find RetroAchievements under Tools in settings, previously it was under System.

## How to use

First, create an account on the RetroAchievements website. Then, in PPSSPP's RetroAchievements settings, enter the same username and password.

After that, simply run games, and if they have achievement sets made for them, they'll be activated. Also, you might want to enable Hardcore Mode, see below.

## Settings documentation

### Hardcore Mode

In Hardcore Mode, a number of the emulator's features that are particularly prone to cheating are disabled, such as save states and slowing down time. Speeding up time is still allowed.

NOTE: Leaderboards are only available in Hardcore mode.

We previously called it Challenge Mode, but we have now switched to the RetroAchievements standard, which is Hardcore mode.

### Encore Mode

Lets you unlock achievements again. Note that this does not affect the real unlock status - you'll just get the notification again if you fulfill the conditions, such as completing a level or whatever it was. This is mainly useful for development - for regular gameplay, it gets confusing.

### Unofficial achievements

These are achievements that have been removed, or are in development, or have simply been considered not really worth having.

Usually, turning on this option is not very useful - if you need it, you know you need it.

## FAQ

## What games are supported?

Find the list of PSP games with achievement support [here](https://retroachievements.org/gameList.php?c=41).

Unfortunately, it's often the case that only one or two regions of a game are supported. Click the link "Supported game files" on the game's page on that site to find out.

## Leaderboards are not working!

Turn on hardcore mode.

## My achievement unlocked at the wrong time!

All existing achievements were created for the PPSSPP core in [RALibRetro](https://github.com/RetroAchievements/RALibretro), which has the same timing and plugins as RetroArch. Unfortunately, due to deficiencies in libretro's design, achievement checks don't run at the correct time in RALibRetro or RetroArch - so if your achievement works correctly in RetroArch and not in PPSSPP standalone, the achievement is broken.

## Developing achievements for PPSSPP using RAIntegration

Achievements are best developed with emulators supporting something called [RAIntegration](https://github.com/RetroAchievements/RAIntegration). The official builds of PPSSPP will start supporting that in version 1.18, but it's already available in the latest [development builds](/devbuilds).

To use RAIntegration, you need a fresh version of the DLL. Drop it next to PPSSPPWindows64.exe, then enable RAIntegration support in `Settings/Tools/RetroAchievements/Development/Enable RAIntegration`. Then restart the emulator.

There are sometimes some drawing issues with the RA menu, which should show up next to the Help menu. If it's not there but should be, try just clicking in the empty space next to "Help".

## How do I register?

Simply go to the [RetroAchievements website](https://retroachievements.org/) and sign up for an account. You can then sign into that account in PPSSPP.

## Additional resources

[RetroAchievements on YouTube](https://www.youtube.com/@retrocheevos)
[RANews - the digital magazine](https://news.retroachievements.org/)
[RA User Documentation](https://docs.retroachievements.org/)
[RA Developer Documentation](https://docs.retroachievements.org/Developer-Docs/)