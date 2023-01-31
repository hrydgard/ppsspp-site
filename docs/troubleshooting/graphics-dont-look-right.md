# The graphics don't look right, what to do?

## Step 1: Don't panic

We're always working on improving graphics, but a lot of games look great already.  It's often just settings.

## Step 2: Change your settings

PPSSPP only keeps settings for things that have downsides.  Pretty much every setting that speeds things up can cause things to render wrong.

Under System, there's a way to reset PPSSPP's settings to defaults. Try this first of all.  Remember that you might have created "game specific" settings too.

Otherwise, some settings to keep in mind (under Graphics):

* Backend - try a different one
* Mode - anything but "Buffered rendering" draws things wrong, black screens or missing things, broken effects
* Simulate block transfer effects - some graphics require this, leave it on
* Frameskipping - sometimes results in glitches or flicker
* Rendering resolution - some games have weird lines above 1x PSP
* Vertex cache - causes things to update slowly or have wrong shapes
* Lazy texture caching - causes wrong graphics sometimse
* Retain changed textures - can cause black boxes, slowdown
* Disable slower effects - causes a lot of glitches, black screen, etc.
* Texture upscale level - sometimes causes glitches and weird lines
* Texture filtering - "Auto" is what the PSP does, try that first
* Hacks - cause bugs in most games, but improve some games - might be worth trying

And a couple under System:

* I/O timing method - sometimes changing this helps graphical issues
* Change emulated PSP's CPU clock - causes weird glitches, sometimes bad graphics - leave at auto

## Step 3: Stop using that save state

Sometimes even after we've fixed a bug, save states keep that bug around.  Save in game and restart the game.

Also, using save states only is kinda like playing on the PSP without ever turning it off.  Some games broke or had glitches after long play sessions, but most people didn't notice because its battery didn't last forever.

## Step 4: Update your drivers or OS

On desktop, make sure you're using the latest & greatest drivers.  On Android, try applying system updates.  Many weird rendering issues are caused by bugs in drivers.

PPSSPP does things most games don't, because it's trying to port old style game rendering tricks to modern graphics cards.  So we find all the good bugs in drivers.

## Step 5: Try the latest version

PPSSPP often gets fixes and updates.  Even if you're using the latest stable version, the [latest git build](/download#devbuilds) may work even better, and may have already fixed any issue you're hitting.

## Step 6: Report an issue

At this point, it seems that you're out of luck and it might be a bug in PPSSPP.

Go to [PPSSPP on Github](https://github.com/hrydgard/ppsspp/issues) and search for the game you're playing. There might be some issue report already describing your problem - if not, get a github account if you don't have one already, then click New Issue and make a report. It will be looked at!
