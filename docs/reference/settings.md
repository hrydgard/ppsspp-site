# Settings documentation

Work-in-progress, incomplete. Started with the old outdated /help documentation and filled it out
a bit, but there's still lots missing here.

<a name="graphics"></a>

## Graphics settings

### Backend

We have implemented rendering code for a few common graphics APIs.

* Vulkan: The recommended backend on most devices, if available. On Mac, this will use MoltenVK.
* OpenGL: The compatibility option on Android, and available on a lot of other devices too.

Windows-only options:

* D3D11: A good alternative to Vulkan if you're on Windows, very fast and high compatibility.
* D3D9: Mainly useful on very old laptops with Intel GPUs

### Rendering resolution

Sets the resolution to render at as a multiplier of the PSP's original resolution. Setting it to higher values
than 1x can affect performance, depending on your hardware, but will create a much nicer and sharper image.

Not all games work perfectly in higher resolution, as PSP games were made to run at exactly 480x272. The most common artifact is thin lines between elements in menus and HUD. This can sometimes be worked around by using texture upscaling, but is not always practically fixable.

### Software Rendering

The software renderer is mostly more accurate, but runs a lot slower than using the GPU to render. It also
doesn't allow higher resolutions. Can be useful for development, or a small number of games and homebrew apps
that the hardware renderers can't handle.

### VSync

The display of your device refreshes many times per second (usually 60 times per second). If this option is enabled, then PPSSPP will only draw during every refresh. This will cause animation to look a bit smoother in some circumstances, assuming your display is set to 60hz.

### Display layout and effects

Allows you to move around and stretch the actual PSP display on the screen as desired, and additionally
apply various optional post-processing effects, some of which are documented below:

* FXAA Antialiasing - a cheap but not that effective method of antialiasing
* Vignette - darkens the image towards the corners, which looks a bit filmic
* Scanlines - creates a CRT screen like effect by drawing horizontal lines. Note that this doesn't make
  that much sense because the real PSP had an LCD screen, but I guess could be cool for some arcade ports.

### Framerate control settings

Especially on mobile platforms, the graphics rendering can be the performance bottleneck. Hence, speed
can be improved by simply skipping the rendering process on every other frame, or more. The drawback
is of course that the framerate will decrease, and in some cases there can be flickering issues.
The setting to control this is called *Frameskipping*.

### Speedhacks

These are settings that can speed things up in games. The effect ranges from none to large, depending
on which game you're trying to play, and the hardware you're trying to play it on.

* Vertex caching

  Saves some CPU work every frame translating vertex from the PSP's formats to PC-compatible format. Usually around a 5% speed bost at most. Most games are fine with this but there's a small number where it can cause glitches.

* Skip buffer effects

  The PSP can render to any location in its VRAM and use as either the scanout buffer (what you see on the screen) or textures. Many games use this to implement various special effects. We simulate this by representing each detected PSP framebuffer with a native framebuffer image.

Disabling it, and thus skipping rendering of everything that's not rendering directly to the backbuffer, is a speed hack, that may or may not speed up some games, and may cause severe graphical artefacts and/or screen flickering.

#### Alternative speed

Lets you set an alternative speed to play at, that can be toggled with a bindable key or button.

Useful for passing very tricky sections in games in slow motion, or for speeding things up in a
more predictable manner than using the unlimited fast-forward key.

Unfortunately due to how the way this works, audio will glitch and crackle at any other speed than 100%.

### Spline/Bezier curves quality

The PSP is capable of drawing curves using splines and bezier curves. This is not widely used but there are a few games out there that use it heavily, like Pursuit Force and Loco Roco.

This setting allows reducing the visual quality of these curves for higher performance.

### Texture Scaling

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

### Texture Filtering

* Anisotropic Filtering: Improves sharpness of textures at shallow angles. See [wikipedia](https://en.wikipedia.org/wiki/Anisotropic_filtering) for some example pictures.

* Texture Filtering - how textures are drawn on the screen
  * Auto - follows what the game desires
  * Nearest - pixellate textures like on the PS1
  * Linear - force all textures to be filtered smoothly
  * Auto Max Quality - Like auto, but will generate extra mipmaps where the game doesn't supply them, for a smoother look. Can cause artifacts with things like fence textures.

### Overlay Information

* Show FPS Counter - shows FPS at the top right corner in-game.
* Show debug statistics - shows realtime information which can be useful for debugging games. It is primarily a developer option.

## Audio

* Global volume - sets the overall volume
* Reverb volume - lets you separately control the volume of reverb and echo effects

<a name="controls"></a>

## Controls

### Enable standard shortcut keys

If this is checked, keys like the Windows key, alt key etc can be used as normal.
If you instead want to map those, uncheck this option.

### Control Mapping

Rebind PSP controls to keyboard, game pad and virtual buttons.

### Haptic Feedback

If this is enabled, the device will vibrate every time a button is pressed.
Useful of mobile devices for tactile feedback.

### On-screen touch controls

This option can be enabled to use on-screen controls. This is useful on
devices such as phones and tablets with no hardware buttons.

### Button Opacity

Used to change the opacity of buttons. An opacity of 0 is completely transparent.
Opacity of 100 is fully opaque

### Auto-hide buttons after seconds

After this number of seconds of inactivity, touch-screen buttons are hidden. This is
nice when playing on the gamepad or watching long cutscenes.

### Edit touch control layout

Used to move on-screen buttons to custom positions.

Hold down a button and drag the button to the desired location. Then click on
Back to save changes.

To Reset changes, press the *Reset* button.

<a name="system"></a>

## System

### Language

Change the default language of PPSSPP's user interface.

### UI Sound

Enables/disables little sound effects when navigating the UI.

### Emulation

#### Fast memory

Fast Memory (unstable)

This option changes PPSSPP such that memory accesses that the game makes are not checked before they are performed. Thus, there is an increase in speed.

### Cheats

#### Enable Cheats

Used to enable or disable cheats in PPSSPP.

<a name="tools"></a>

## Tools

### System Information

Shows device-related information such as the system specifications, OpenGL extensions and OpenGL ES extensions.
