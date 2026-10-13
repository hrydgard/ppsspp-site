# Rewind

Rewind lets you jump back a few seconds when you make a mistake. While it's on, PPSSPP quietly takes a snapshot (a save state kept in memory) every few seconds, and pressing Rewind loads the most recent one.

It's off by default, since it uses quite a bit of memory.

## Turning it on

Go to *Settings > System*, and in the *Emulation* section, set *Rewind Snapshot Interval (mem hog)* to the number of seconds you want between snapshots (1 to 60). *Off* turns the feature off.

The setting can be set per game, through a [game-specific config](/docs/reference/custom-game-configs), so you can keep it on only for the games where you want it.

Until it's on, pressing Rewind only shows "No rewind save states available".

## Rewinding

There are three ways to rewind:

- Press the key or button mapped to *Rewind* in *Settings > Controls > Control mapping*. On a keyboard it's Backspace by default. Controllers have no default mapping, so map a button yourself if you want one.
- Open the pause menu and press the *Rewind* button. It shows up when the feature is on, and is grayed out until the first snapshot has been taken.
- On touch screens, add a custom button to the touch layout and set it to *Rewind*.

PPSSPP shows how far back you went ("Rewind (4 seconds ago)"). Each snapshot can be anywhere between 0 and one interval old, so with a 5 second interval, the first rewind takes you back somewhere between 0 and 5 seconds.

Pressing Rewind again takes you back another step. Rewinding uses up the snapshot, so you can't go forward again afterwards.

## How far back you can go

PPSSPP keeps the 20 most recent snapshots, so you can go back up to 20 steps, 20 times the interval. With a 5 second interval, that's a bit over a minute and a half. When a new snapshot is taken, the oldest one is dropped, so the memory used doesn't keep growing the longer you play.

Due to how the snapshots are compressed (see below), the oldest few can sometimes become unusable, so count on about 17 steps rather than 20.

Some other details:

- The interval is measured in real time, not game time. Fast-forwarding doesn't make snapshots more frequent, and no snapshots are taken while the game is paused.
- Saving or loading a regular save state restarts the countdown to the next snapshot.
- All snapshots are thrown away when you exit the game. They're never written to disk, so they can't replace regular save states.

## Limitations

- Rewind is disabled in [RetroAchievements](/docs/reference/retro-achievements) hardcore mode, just like save states.
- It's also disabled when playing online in modes where save states aren't allowed.
- A snapshot is a save state, so it comes with the same caveats. It's taken at an arbitrary point, which in rare cases can be one where the game doesn't like being restored.

## Memory usage

A full PSP save state is roughly 35-70 MB, mostly the PSP's RAM. Rather than store 20 of those, PPSSPP keeps two full snapshots and stores the others as differences against them, which are usually much smaller. Still, expect the feature to use somewhere between 100 and a few hundred MB, depending on the game. That's not a problem on most PCs, but on phones with little memory it's worth leaving off.

The memory is freed when you exit the game.

## Older versions

Before PPSSPP 1.15, the interval was set in frames (0 to 1800) instead of seconds. PSP games usually run at 30 or 60 FPS, so for example 150 frames meant a snapshot every 5 seconds in a 30 FPS game. The setting was called *Rewind snapshot frequency (mem hog)*.

*Thanks to Rory O’Kane, whose [write-up](https://github.com/hrydgard/ppsspp/issues/8907) this page started from.*
