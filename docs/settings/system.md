---
position: 6
---

# System

## UI

### Language

Change the default language of PPSSPP's user interface.

### UI Sound

Enables/disables little sound effects when navigating the UI.

### Set UI background...

Lets you pick an image for use as the background of PPSSPP.

### Transparent UI background

When browsing the menu while paused, this settings controls if the running game should "show through" behind the menu.

### UI background animation

Controls what the menu background looks like when no game is being played.

### Color Tint / Color Saturation

Lets you control the color of the menu.

## PSP Memory Stick

### Show Memory Stick Folder

Opens up your memory stick folder. Only available on desktops.

### Memory Stick inserted

Some games ask you to remove and insert the memory stick. This allows you to make the game believe that this happened.

### Memory Stick size

Sets the memory stick size that the emulator reports to the game. A few games get confused by very large memory sticks, but we already have built-in workarounds for most of them. In case that doesn't work, this option is here.

## Help the PPSSPP team

### Enable compatibility server reports

This will report some information to PPSSPP's report server, that can sometimes help with diagnosing bugs.

If this is not checked, PPSSPP never sends any information to the server, except looking for new versions - which it
only does if "Check for new versions of PPSSPP" is checked.

## Emulation

### Fast memory

Fast Memory (unstable)

This option omits checks that memory accesses that the game makes are valid. Thus, there is an increase in speed.

This doesn't mean inaccurate emulation, it's how the real hardware worked, games would crash if they accessed invalid addresses - though turning it off can sometimes paper over crashes caused by other inaccuracies in emulation, if you also turn on Ignore bad memory accesses.

Generally recommend to keep it on.

### Ignore bad memory accesses

As described under Fast Memory, in combination with turning off Fast Memory this can sometimes save a crashing game. Apart from that, there's no real reason to use this feature.

### I/O timing method

PPSSPP isn't able to accurately simulate the exact times that read operations from UMD discs took on the real hardware - but usually that doesn't matter very much as access times were variable due to scratches etc and games mostly tolerate that. Simulating the accesses as faster than on the real hardware cuts loading times drastically, too.

However, there are a few games that are not happy if I/O is too fast. So, we provide a few options here for how to
deal with timing. More details will be added here later.

### Force real clock sync

When enabled, PPSSPP tries to keep the emulated CPU in sync with the host CPU at all times by sleeping many times per frame, instead of running a big burst of emulation at the start of each frame and then waiting. It's not otherwise about accuracy, but can help latency slightly and multiplayer can behave very slightly better.

### Change emulated PSP's CPU clock (unstable)

This lets you effectively overclock or underclock the emulated CPU. Set it to 0 to autodetect (some games
overclock the PSP CPU from 222 to 333mhz on their own).

Overclocking can be useful to avoid framedrops that would happen on the real hardware (approximately). Underclocking can sometimes help games run on really slow CPUs by triggering internal frameskipping in games, but doesn't always do that.

### Rewind Snapshot Interval

If you set a rewind snapshot interval, PPSSPP will make save states every X sections, where X is the value you
chose. You can go back to the last savestate by pressing Backspace.

## General

### Restore PPSSPP's settings to default

Does what it says on the tin.

### Savestate slot backups

If this is active, extra work will be done at save/load state time to provide "Undo Load" and "Undo Save" operations.
Thus, saving and loading state is slower but safer.

### Auto load savestate

Sets whether to load a savestate automatically on startup. Not recommended.

### Use system native keyboard

Lets you write using the PC keyboard (for example) when a game asks for a nickname.

### Cache full ISO in RAM

Reads the entire ISO into memory on startup. This is mainly useful when playing from hard drives that can spin
down to save energy - they won't have to spin up again in the middle of gameplay to read some file.

### Check for new versions of PPSSPP

If this is enabled, PPSSPP will occasionally check for new versions and alert you if one is available. Note that this does not include an auto-updater (yet).

### Save screenshots in PNG format

Normally, screenshots are saved as JPG. This lets you save them as .PNG instead for higher quality but larger files.

## Cheats

### Enable Cheats

Used to enable or disable CwCheats in PPSSPP.

CwCheats is a cheat format originally created for real hacked PSPs, not emulators, inspired by classic
Action Replay codes. As such, it's very low-level and simple.

To use them, you have to download a cheat database and load it in cheat settings, which you can access
from the pause menu in-game if this setting is enabled.

Unfortunately, these cheats are often a bit unstable - if a game starts crashing when you're using them,
try a different cheat.

## PSP Settings

This replaces the settings menu of the real PSP, plus related things.

### PSP model

Lets you choose whether PPSSPP will report PSP-1000 (the "phat" PSP) or PSP-2000/3000 (PSP Go, other models)
to games. Generally it doesn't matter, games get the same amount of memory anyway, etc.

### Change nickname

This changed the default when games ask you for your name. It's convenient to put in your preferred nickname
here.

### Daylight savings

This doesn't actually do anything, but games can check what you have chosen here. Most don't, though.

### Date format

This tells games which format to display dates in.

### Time

This tells games which format to display times in.

### Confirmation button

In some games, this controls whether X or O is for confirmation. This is often different between Japan and other
regions, Japanese games generally use O for confirmation. This lets you set the preference to what you're used to.
However, many games ignore this setting and just do what they feel like.
