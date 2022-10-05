---
slug: burnout-dominator-lens-flare
title: Burnout Dominator's lens flare
authors:
  name: Henrik Rydgård
  title: PPSSPP Founder
  url: https://github.com/hrydgard
  image_url: https://github.com/hrydgard.png
tags: [rendering-tricks]
---

## PSP rendering tricks - a new blog post series

In these posts, I'm going to look at some of the more interesting visual effects that games accomplish on the PSP, and how they do it. The PSP's GPU is very limited by modern standards, but it's still capable of quite a lot of interesting things if your really know how to use it.

First up, Burnout Dominator and its lens flare effect.

Lens flare effects are easy to do on modern GPUs. There are lots of different approaches, both image-based which can handle light sources of any shape but are generally not very detailed, and sprite-based, which are more limited in terms of the light source, but can look really complex and colorful. There are queries that can be used to figure out how many pixels actually got rendered when drawing the sun sprite, for example.

On limited, fixed-function hardware such as that of the PSP, where you can’t even do multitexturing, and definitely can’t write any shaders, one had to get imaginative to achieve these effects. And that’s what the authors of Burnout Dominator did, as we will see.

There are two main parts to rendering a lens flare:

- Determining the location, and measuring the coverage (how brightly the flare should render).
- Using the determined coverage value to adjust the brightness of the lens flare. If the source of light is fully covered, no lens flare should be drawn, while if it’s partially covered or not covered at all, the lens flare should be drawn but with a brightness adjusted by the coverage.

To achieve this, here’s what Burnout Dominator actually does to measure the coverage:

The game makes a backup copy of a 14x14 rectangle around the sun location on-screen, and then renders a black and a white rectangle on top, with only the black one being depth tested. The result is a binary mask of where the sun is visible.
It then downsamples (through bilinear filtering) this binary image to 8x8 and copies back the original color pixels, to reverse the damage.
Then, it downsamples the 8x8 image three more times, until we reach a single value, which is thus the average of the black and white pixels, and thus an effective measurement of the coverage. The final value is for some reason spread out over four pixels, for no obvious reasons.

So, we now have the coverage as a byte value, or rather multiple of them right next to each other. We could now treat this value as a 1x1 texture and let it influence the brightness of basic gouraud shaded geometry, like some simple hexagons and stuff, could look OK. Ridge Racer, for example, stops here (though uses a simpler accumulation method) - it draws simple shapes.

But Burnout gets fancier. A LOT fancier. We are now faced with the challenge of how to use this arbitrary value sitting in an image, to influence the brightness of the lens flare texture, using only the PSP GPU. We do not want to bring in the CPU to copy the brightness value to some vertex colors, for example, as that would require expensive synchronization, instead we need to coax the GPU to do the work directly.

If we had multitexturing with combiners, or even shaders, we’d just sample this texel and multiply the lens flare texture by it, but we don’t have that. The PSP has some other tricks up its sleeve though. Games on the PSP mostly use paletted textures, 4-bit or 8-bit. And that opens up for a really cute trick: Since on the PSP, VRAM is just VRAM, the game can actually overlay a render target on top of the palette memory (known as a CLUT for color look up table in Sony lingo) of the lens flare texture in memory, and render into the alpha channel of this render target, while texturing from a 1x1 (actually 4x1 for some reason) texture carefully made to overlap our brightness value from the coverage computation in memory! Then it can finally draw the lens flare using a texture with that modified CLUT loaded, which now has the correct alpha value in all colors, matching the coverage. 

Except… The PSP is actually not able to write arbitrary values into the alpha channel of a 32-bit render target, at all! Not sure exactly why that is, but it is likely related to the fact that on the PSP, the alpha and stencil buffers share bits. So how do we actually accomplish it anyway, given that we just stated that it’s impossible?

This is where we get to the next level of trickery. When the game renders to the CLUT, it doesn’t use 32-bit color, instead it uses 16-bit “565” color, setting the render width to be 512, twice the size of the palette. Now, every second 16-bit pixel can be used to control the A and B channels of a color, and the other ones will control the G and R channels. The mapping is not trivial, let’s try to see that:

Same bits in memory (each letter is 1 bit):

```
AAAAAAAA BBBBBBBB GGGGGGGG RRRRRRRR (one 32-bit color value)
bbbbbggg gggrrrrr bbbbbggg gggrrrrr (two 16-bit color values)
```

So when I said that it textures from the coverage value and writes it to the palette, what it really does is that it textures using another palette with a green-blue gradient to translate the coverage value into the appropriate green and blue components to properly fill the alpha channel of the 32-bit CLUT entries! The PSP GPU conveniently has a framebuffer bitmasking feature, where you can prevent writing to specific bits of each color value when rendering (modern GPUs don’t have that, they have a bit per channel instead).

That creates a new problem though, it will now write that value to green as well in the corresponding 32-bit color values, as we can see in the diagram, which we don’t want. The color write bitmask is applied individually to each 16-bit pixel here, so that’s not very useful. So what can we use to mask away the writes to every other pixel? Well we in fact are on a GPU, so why not initialize a Z buffer with 0, 32767, 0, 32767, and use depth testing to discard every second value? And that’s what it does, drawing to the palette, using a depth value and a depth compare function to make sure only every second 16-bit pixel actually gets written.

And we reach the final moment, where the coverage value sits properly in the alpha channel of each entry of the color palette of the lens flare texture, and we can just do a simple draw to get it on the screen with the appropriate brightness.

Phew! Getting this to work in my emulator PPSSPP was.. not trivial.
