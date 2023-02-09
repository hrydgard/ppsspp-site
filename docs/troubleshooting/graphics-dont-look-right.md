# The graphics don't look right, what to do?

## Step 1: Don't panic

We're always working on improving graphics, but a lot of games look great already.  It's often just settings.

## Step 2: Check for known problems

Are you seeing font characters getting substituted with others, or sprites being swapped? Very likely all you need to do is to turn off the speedhack "Lazy texture caching".

Thin glitchy lines in the UI? These are usually an artifact of rendering at higher resolution than 1x and are not always fixable. Cranking up "Texture scaling" can help (but has its own drawbacks).

Are you seeing cracks in the ground? Turn up the spline/bezier quality.

Strange flickering here and there? Turn off vertex caching.

Black screen, missing effects? Make sure "Skip buffer effects" isn't checked.

Try turning off everything under "Speed hacks".

## Step 3: Change your settings

Under System, there's a way to reset PPSSPP's settings to defaults. Try this first of all.  Remember that you might have created "game specific" settings too.

Otherwise, some settings to keep in mind (under Graphics):

* The settings under "Speed hacks", as mentioned above - turn them off.
* Backend - try a different one if you can
* Mode - anytehing but "Buffered rendering" draws things wrong, black screens or missing things, broken effects
* Frameskipping - sometimes results in glitches or flicker
* Rendering resolution - some games have weird lines above 1x PSP
* Texture upscale level - sometimes causes glitches and weird lines
* Texture filtering - "Auto" is what the PSP does, try that first

And a couple under System:

* I/O timing method - sometimes changing this helps graphical issues
* Change emulated PSP's CPU clock - causes weird glitches, sometimes bad graphics - leave at auto

## Step 4: Stop using that save state

Sometimes even after we've fixed a bug, save states keep that bug around.  Save in game and restart the game.

Also, using save states only is kinda like playing on the PSP without ever turning it off.  Some games broke or had glitches after long play sessions, but most people didn't notice because its battery didn't last forever.

## Step 5: Update your drivers or OS

On desktop, make sure you're using the latest & greatest drivers.  On Android, try applying system updates.  Many weird rendering issues are caused by bugs in drivers.

PPSSPP does things most games don't, because it's trying to port old style game rendering tricks to modern graphics cards.  So we find all the good bugs in drivers.

## Step 6: Try the latest version

PPSSPP often gets fixes and updates.  Even if you're using the latest stable version, the [latest git build](/download#devbuilds) may work even better, and may have already fixed any issue you're hitting.

## Step 7: Report an issue

At this point, it seems that you're out of luck and it might be a bug in PPSSPP.

Go to [PPSSPP on Github](https://github.com/hrydgard/ppsspp/issues) and search for the game you're playing. There might be some issue report already describing your problem - if not, get a github account if you don't have one already, then click New Issue and make a report. It will be looked at!
