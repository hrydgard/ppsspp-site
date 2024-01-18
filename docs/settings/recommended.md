---
position: 7
---

# Recommended settings

## High-end hardware

If you have a powerful device, to get the most out of your games, in Graphics settings:

* Set rendering resolution to match your device resolution. 4x = 1080p, 8x = 4K.
* Set Texture Filtering to Auto Max Quality
* Use the Vulkan backend, and set MSAA to 4x, or even 8x.
* Disable everything under Speedhacks.

That's about how good things will get, without delving into texture upscaling or replacement.

## Low-end hardware

If your device is struggling to keep up with the games, you might try the following:

* Lower rendering resolution to 2x or even 1x
* In some games, "Disable GPU readbacks" can fix severe slowdowns, but can also cause graphical issues.
* Turn off any post-processing effects you have enabled in Display layout and Effects.
* Experiment whether "Software skinning" helps or not, it can go either way depending on game and hardware.

As a last resort if nothing else helps:

* Set Frameskipping to 1 or 2
* In some games, you can even try "Skip buffer effects", although many games will not render correctly in this mode.

## Minimizing latency

Especially when playing rhythm-based games like Patapon, or even when playing any kind of action game, sometimes you care more about latency than raw performance or high resolutions, or you have spare power left over and want to spend it on something.

Here are the recommended settings to minimize input-to-screen latency:

* Use the Vulkan graphics backend
* Set "Buffer graphics commands" to 1 or even "No buffer".
* Turn off VSync.
* Experiment with the "System / Force Real Clock Sync" setting
