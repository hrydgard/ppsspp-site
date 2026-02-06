---
position: 6
---

# System

## UI

### Language

Change the language of PPSSPP's user interface.

### UI size adjustment (DPI)

Lets you increase or decrease the size of PPSSPP's UI elements.

### Set UI background...

Pick an image for use as the background of PPSSPP.

### Transparent UI background

When browsing the menu while paused, this settings controls if the running game should "show through" behind the menu.

### Notification screen position

Controls where notifications show up. If you want to disable all notifications, set this to None.

### UI background animation

Controls what the menu background looks like when no game is being played.

### Theme

Pick a color theme you like!

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

## Save states

### Savestate slot backups

If this is active, extra work will be done at save/load state time to provide "Undo Load" and "Undo Save" operations.
Thus, saving and loading state is slower but safer.

### Savestate slot count

From PPSSPP 1.20, you can configure the number of save state slots here, up to a maximum of 30.

### Auto load savestate

Sets whether to load a savestate automatically on game launch. Can be set to a specific slot, or to always load the latest.

### Rewind Snapshot Interval

If you set a rewind snapshot interval, PPSSPP will make save states every X sections, where X is the value you
chose. You can go back to the last savestate by pressing Backspace.

## General

### Ask for exit confirmation after seconds

If you haven't saved your game, either in-game or savestate, within this time, PPSSPP will ask if you really want to exit if you try to do so. To turn this off, set this to zero.

### Restore PPSSPP's settings to default

Does what it says on the tin.

### Use system native keyboard

Lets you write using the PC keyboard (for example) when a game asks for a nickname.

### Cache full ISO in RAM

Reads the entire ISO into memory on startup. This is mainly useful when playing from hard drives that can spin
down to save energy - they won't have to spin up again in the middle of gameplay to read some file.

### Check for new versions of PPSSPP

If this is enabled, PPSSPP will occasionally check for new versions and alert you if one is available. Note that this does not include an auto-updater (yet).

### Save screenshots in PNG format

Normally, screenshots are saved as JPG. This lets you save them as .PNG instead for higher quality but larger files.

### Screenshot mode

Sets whether to take a picture of the internal rendered image, or after post-processing and scaling.

### Pause when not focused

Sets whether PPSSPP will pause when it loses focus. This doesn't work in multiplayer.

## Cheats

### Enable Cheats

Used to enable or disable [CwCheats](/docs/reference/cwcheats) in PPSSPP.

### Enable plugins

Used to enable or disable [Plugins](/docs/reference/plugins) in PPSSPP.

## PSP Settings

This replaces the settings menu of the real PSP, plus related things.

### Game language

Hints to games what language they should display. Only supported by some games, mainly European releases.

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

### Time format

This tells games which format to display times in.

### Confirmation button

In some games, this controls whether X or O is for confirmation. This is often different between Japan and other regions, Japanese games generally use O for confirmation. This lets you set the preference to what you're used to. However, many games ignore this setting and just do what they feel like.

## Recording

An old and limited screen and audio recording facility. Not really recommended for use.