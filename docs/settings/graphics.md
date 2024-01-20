---
position: 1
---
# Graphics settings

PPSSPP has a large amount of settings to change the way graphics are rendered, but most of them are generally fine to leave at their defaults.

See the [Recommended settings](/docs/settings/recommended) for ideas on how to optimize for low-end, high-end and latency.

## Rendering mode

### Backend

Lets you choose which rendering API PPSSPP should use to draw graphics.

We have implemented support for a few of the most common graphics APIs:

* Vulkan: The recommended backend on most devices, if available. On Mac, this will use MoltenVK if available.
* OpenGL: The compatibility option on Android, and available on a lot of other devices too.
* D3D11: Windows-only. Good alternative to Vulkan if you're on Windows, very fast and high compatibility.
* D3D9: Windows-only. Mainly useful on very old laptops with Intel GPUs.

### Rendering resolution

Sets the resolution to render at as a multiplier of the PSP's original resolution. Setting it to higher values
than 1x can affect performance, depending on your hardware, but will create a much nicer and sharper image.

4x is almost exactly equal to 1080p (1920x1288 vs 1920x1280), and if you have your monitor set to that, PPSSPP will automatically cut off 4 pixels at the top and bottom of the screen to make it fit.

You might get a bit of extra antialiasing by setting the resolution higher than what your monitor can do, and letting the emulator downscale, there are also a few postprocessing filters that do a good job helping out with that. But it's better to use MSAA in Vulkan.

Not all games work perfectly in higher resolution, as PSP games were made to run at exactly 480x272 and only tested at that resolution. The most common artifact is thin lines between elements in menus and HUD. This can sometimes be worked around by using texture upscaling, but is not always practically fixable, and might have other problems.

Higher resolutions have obvious benefits for 3D games, but 2D games can benefit too, depending on their style. If it's mostly just pixel-aligned sprites art like in King of Fighters (typical of ports from 2D consoles) there won't be any benefits. If they scale and rotate stuff (think Patapon) or use vector graphics like loco roco, that will look better at higher rendering resolutions.

### Software rendering

The software renderer is mostly more accurate, but runs a lot slower than using the GPU to render. It also
doesn't allow higher resolutions, it's limited to 1x and ignores a lot of settings.
Can be useful for development, or a small number of games and homebrew apps that the hardware renderers
can't handle, or if you just like the look of authentic PSP rendering with dithering and so on.

### Antialiasing (MSAA)

This is the best quality antialiasing you can get in PPSSPP, but it's currently only implemented in the Vulkan backend, and only on desktop GPUs (this will change in the future). Use if if it's available, crank it up to 8x if your GPU can handle it for ultra smooth edges on things.

### VSync

Tries to avoid presenting a new image in the middle of your monitor's display refresh, by using whatever option is available to do so. This doesn't always make a difference at all, so if you don't see any issues with screen tearing, best to leave it off.

In Vulkan, instead of tearing, if the "Mailbox" present mode is available, latency will be almost as good as if we did tear. Still, no tearing will happen, instead the framerate might not be 100% smooth at all times.

### Display layout & effects

Allows you to move around and stretch the actual PSP display on the screen as desired, and additionally
apply various optional post-processing effects, some of which are documented below:

* FXAA Antialiasing - a cheap but not that effective method of antialiasing
* Vignette - darkens the image towards the corners, which looks a bit filmic
* Scanlines - creates a CRT screen like effect by drawing horizontal lines. Note that this doesn't make
  that much sense because the real PSP had an LCD screen, but I guess could be cool for some arcade ports.

## Framerate control

### Frame skipping

Especially on mobile platforms, the graphics rendering can be the performance bottleneck. Hence, speed
can be improved by simply skipping the rendering process on every other frame, or more. The drawback
is of course that the framerate will decrease, and in some cases there can be flickering issues.

The sub-setting Frame Skipping Type is generally not very useful.

### Auto frameskip

Not generally recommended for use, only use on very low-power devices. Will automatically vary frameskip from 0 up to the current frameskip setting.

### Alternative speed

Lets you set an alternative speed to play at, that can be toggled with a bindable key or button.

Useful for passing very tricky sections in games in slow motion, or for speeding things up in a
more predictable manner than using the unlimited fast-forward key.

Unfortunately due to how the way this works, audio will glitch and crackle at any other speed than 100%.

## Speedhacks

These are settings that can speed things up in games. The effect ranges from none to large, depending
on which game you're trying to play, and the hardware you're trying to play it on.

### Skip buffer effects

  The PSP can render to any location in its VRAM and use as either the scanout buffer (what you see on the screen) or textures. Many games use this to implement various special effects. We simulate this by representing each detected PSP framebuffer with a native framebuffer image.

Disabling it, and thus skipping rendering of everything that's not rendering directly to the backbuffer, is a speed hack, that may or may not speed up some games, and may cause severe graphical artifacts and/or screen flickering.

### Disable culling

The PSP will cull triangles that are outside a 4096x4096 clipping box, and also triangles crossing the Z=1 plane if clipping is enabled. Some games rely on this, but there are a few games where we get this subtly wrong. It can be useful to experiment with turning this off if you see missing or flickering geometry.

## Spline/Bezier curves quality

The PSP is capable of drawing curves using splines and bezier curves. This is not widely used but there are a few games out there that use it heavily, like Pursuit Force and Loco Roco.

This setting allows reducing the visual quality of these curves for higher performance.

## Performance

### Render duplicate frames

Does what it says - if a game runs at 30hz, we'll still push 60 frames to the monitor. A little more "work" for your device, but can improve timing on some (ie. reduce microstutter)

### Buffer graphics commands

How much latency is allowed between the CPU and GPU. A smaller value means that they might not be able to work in parallel as much, and you might get worse performance, but better latency. This may help in music games, such as Project Diva or Patapon. For more detail, see [Recommended settings](/docs/settings/recommended).

### Hardware transform

Do not turn this setting off, unless you are doing it for debugging purposes. With this enabled, PPSSPP runs the vertex transform pipeline (lighting, transform etc) on the GPU. Turning it off is always slower.

### Software skinning

Runs "skinning" on the CPU instead of the GPU, which does sound like it would be slower but it means that draw calls from the game can more easily be combined, which often improves things, especially on older devices. However, in some games, hardware (vertex shader) skinning can be faster on some devices so we keep the option around.

### Geometry shader culling

The PSP has some rather different behavior than PC GPUs about at what depth and in which positions triangles get "culled" (removed) and clipped.

Modern PC GPUs lets us emulate these behaviors efficiently if they support clip and cull planes, but not all GPUs do. For those that don't, we have added this option to use "geometry shaders" to get the same behavior. Unfortunately these have some performance overhead, so we offer the option to turn them off - most games don't in fact need this, though some do - if weird geometry is obscuring the camera, you're going to want to enable this.

This option is not visible if the hardware has the sufficient clip and cull plane features.

## Texture Scaling

Texture scaling improves texture detail. However, it is an expensive process and can cause slowdown.

PSP games generally have quite low texture detail, as more isn’t needed due to the low resolution of
the PSP display, and the lack of RAM and VRAM available. The Texture Scaling feature uses an upscaling
filter to give the illusion of sharper texture detail. Doesn’t work great with all art styles but
some games are very much improved.

There is generally no point in going beyond 4x texture scaling unless you are running at very high
resolutions.

If you are using the Vulkan backend, try the Texture Shader option for some GPU upscaling method,
than run without the performance penalty.

Types of Upscale algorithms:

* xBRZ - a sharpening upscale algorithm. [See here for examples](https://docs.libretro.com/shader/xbrz/).
* Bicubic - a smooth look
* Hybrid - uses bicubic for smooth areas. Can greatly improve sky textures over plain xBRZ.

## Texture Filtering

### Anisotropic Filtering

Improves sharpness of textures at shallow angles. See [wikipedia](https://en.wikipedia.org/wiki/Anisotropic_filtering) for some example pictures. There are generally no drawbacks to using it - PPSSPP's performance is rarely bottlenecked by texture sampling.

### Texture Filtering

* Auto - follows what the game sets
* Nearest - pixellates textures like on the PS1
* Linear - force all textures to be filtered smoothly
* Auto Max Quality - Like auto, but will generate extra mipmaps where the game doesn't supply them. Can cause artifacts with things like fence textures, but will generally give you a smoother look in the distance in 3D games, especially combined with anisotropic filtering.

For most games, max anisotropic + auto max quality will give you the best results. For a few games like Pursuit Force, switch back to just Auto. This may be made automatic in the future.

### Smart 2D texture filtering

Many 2D games render graphics with linear filtering enabled, even when drawing 2D where each texel maps exactly to a pixel, which we call "pixel-mapped". In this case, the filtering mode doesn't matter, but it does start to matter when you scale up the rendering to higher resolution.

With this option enabled we try to detect this case and switch to "nearest" (pixellated) filtering, which looks better than linear filtering for a lot of 2D art. Some more information can be found [in this PR](https://github.com/hrydgard/ppsspp/pull/18646). The detection is not working in all games yet though.

## Hack settings

### Lower resolution for effects

Sometimes games will draw effects that are meant to blur the screen in various ways, such as light bloom effects. PSP games are all designed for 480x272 though, so when it does the blurring, it basically just resizes the things it wants to blur down by a certain %. Now, enter PPSSPP: let's say we renders at 4x or even 8x the original resolution. That means that the "blurred" version is actually still pretty sharp and detailed. So then the supposedly blurred image is applied to the screen and... it kinda looks like a strange outline, after image or ghosting.

Lower resolution for effects forces images other than the main render targets to render at low resolution just like on the real PSP, working around the problem pretty well.

The different levels just represents a series of checks we add to identify this situation, with varying confidence.

For more details, see [The Bloom Problem](/docs/troubleshooting/the-bloom-problem.md).

## Overlay Information

### Show FPS Counter

Shows FPS at the top right corner in-game.

### Debug overlay

Lets you choose one of a variety of overlays that shows various shows realtime information which can be useful for debugging games. It is primarily a developer option.
