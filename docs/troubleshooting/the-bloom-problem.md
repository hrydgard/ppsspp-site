# The Bloom Problem

Bloom is a post-processing image-based effect that aims to replicate how very bright light sources tend to affect surrounding pixels in the image, due to a variety of optical phenomena. More details can be found in the [Wikipedia article](https://en.wikipedia.org/wiki/Bloom_(shader_effect)).

On the PSP, as on other consoles of its generation, it's more of an artistic choice than an accurate simulation, but it can still look pretty good.

Blooms are usually accomplished by taking a copy of the rendered image, blurring it and then adding the blurred result back to the image. Blurring on the PSP is accomplished by downscaling with a bilinear filter repeatedly, then summing those smaller images together, getting a rough approximation of a gaussian curve. The PSP has no shaders nor does it do multitexturing, but repeated downscales work well enough.

PSP games always run at a fixed resolution of 480x272 (except a few like GTA that use higher internal resolutions and then downscale). They thus usually do a hardcoded number for steps. Unfortunately, as resolution increases, more blurring steps are needed, but the games just won't perform them.

In PPSSPP, we have a mitigation for this problem where we render buffers other than the main framebuffer at the standard PSP resolution instead of at upscaled resolutions. This means that only the initial copy from the rendered game image to the first blurring target will induces aliasing, further steps will result in a smooth blur, creating an acceptable bloom effect.

In some games, though, we are not able to use this since other effects in the game require full resolution buffers. An example of this is Burnout Dominator, where the bloom does not look good but is currently unfixable.
