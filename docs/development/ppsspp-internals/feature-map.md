# Game feature directory

This is a very incomplete list of games to test with if you want to debug a specific feature. Hopefully you can find one that you own.

## OS and hardware features

### OSK (on-screen keyboard)

- MotoGP (single-language)
- PES 2013 (single-language)
- Metal Gear Solid: Portable Ops (see issue #21856)
- Fifa 07 (manager mode, create profile and name it)
- many more

### Microphone (hardware expansion)

- Beaterator
- DJ Max Fever
- Go Explore
- Infected (shouting into the mic pushes enemies back)
- Invizimals
- Rock Band Unplugged
- Talkman

These additional games are "headset compatible", possibly some support voice chat in multiplayer?

- Star Wars: Clone Wars
- Tron: Evolution
- SOCOM Fireteam Bravo 2
- SOCOM Fireteam Bravo 3
- Resistance
- Syphon Filter: Logan's Shadow
- Assassin's Creed
- Spiderman 3
- MACH
- Ford Racing Off Road

Additional reports:

- Traxxpad


Homebrew apps, possibly good for testing microphone functionality:

- [Audio Mechanica Kai](https://www.gamebrew.org/wiki/Audio_Mechanica_kai_PSP) ([video](https://www.youtube.com/watch?v=cvHWNBR9vNo))

### Camera (hardware expansion)

- EyePet Adventures
- Invizimals

### GPS (USB accessory)

Needs the PSP-290 GPS USB attachment.

- Go!Explore
- MapThis! (homebrew)
- Maplus Portable GPS

## Audio features

### VAudio, sceVaudioSetAlcMode

- SensMe music player. sceVaudioSetAlcMode controls automatic audio normalization, the algorithm for that is unclear.

## GE rendering features and/or techniques

### Spline/Bezier patches

- Loco Roco (all games and variants)
- Namco Museum: Battle Collection (Pac-Man, more). This still exhibits a small bug (black patch on top of the ghosts)
- Pursuit Force (both games, for the road surfaces and landscape)
- Test Drive Unlimited (landscape)
- Puzzle Bobble Pocket (completely unnecessary! Just used for rotating sprites)
- one of the snowboarding games

### Vertex culling / guardband

- TOCA Race Driver 2 / DTM Race Driver 2 / V8 Supercars Australia 2 / Race Driver 2006
- TOCA Race Driver 3 Challenge / DTM Race Driver 3 Challenge / V8 Supercars Australia 3: Shootout

### Vertex skinning/bones

- Crisis Core
- God of War (both games)
- Tekken 5
- Tekken 6
- many, many more.

### Vertex morphing

You can supply multiple copies of input vertex data per output vertex, and the GE will make a weighted average using the morph factors, specified separately directly in the GE context.

- MotorStorm: Arctic Edge (vehicles, 1-2 weights only)
- OutRun 2006 (flame rings in 4th level on the left, 4 morph weights IIRC)

### Depth buffer reads from the CPU

- Midnight Club: LA (sun lens flare)
- Syphon Filter (light halos (lots per frame!))
- Wipeout Pure (sun lens flare). Multiple reads per frame to get an average.
- a few more

### Direct color buffer reads from the CPU

- Coded Arms: Contagion (reads a single pixel from the alpha/stencil channel to identify enemies to lock-on)
- Dangan Ronpa, Dangan Ronpa 2 (renders a separate mini buffer to select clues, people)
- MotorStorm: Arctic Edge (light adaptation)
- a few more

### Color buffer format aliasing

- Cars Race-o-rama
- OutRun 2
- Split/Second
- Spongebob

### Depth buffer manual swizzling through geometry

This is a mistake that a few games have made - the PSP has hardware to swizzle depth buffers to linear, but instead they use triangles to simulate the swizzle to match. This applies to the particle effects in these games:

- Jak & Daxter
- Ratchet & Clank (both games)

### Tricks requiring dual source blending

- Armored Core 3 (issue [#11430])
- Ridge Racer (lens flares)
- WipEout Pure (bloom effect)

### Intra-buffer block copies

- Tales of Phantasia X/Cross Edition (included as a retro game in Tales of Phantasia: Narikiri Dungeon X), see issue [#21098]

### Block copies to RAM, later used as texture

- Burnout Legends (sun lens flare)
- Digimon
- Tales of Phantasia X

### CLUT lookups from color framebuffers ("depal")

- Dragon Ball Z: Tenkaichi Tag Team
- Test Drive (with "smooth" lookup workaround for better color precision)
- Burnout Dominator (lens flare effect)
- Tantalus games (Spongebob, etc)
- Many more

### CLUT lookups from depth buffers

- Fog effect in Katamari Damacy

### CLUT lookups with palette in framebuffer

- Ridge Racer 1/2 (digital speedometers)

### Reinterpret color framebuffers

- Outrun 2006 (however, worked around with "BlueToAlpha" method)
- Split/Second (however, worked around with "BlueToAlpha" method)

- Tantalus games (Spongebob, etc)

### Memcpy framebuffer download/upload

- Everybody's Golf 2 (line by line, ugh)

### Zipped PRX

- Fired Up

### Persistent render targets

- Mahjong Artifacts (Minis) relies on render targets sticking around.

### "3D textures" using malformed mipmap chains

The PSP doesn't have any restriction on the size relationships between successive mipmaps in a pyramid. Hence, it's possible
to put multiple images of the same size into the mipmap layers of a texture, and use trilinear filtering to interpolate between them.
Usually, for control, the mip level is controlled directly (instead of automatic from screen-space derivatives).

This is used by the following games:

* Macross series games (water effect, see #6357)
* Misshitsu no Sacrifice (background animation, also see #6357)
* Tactics Ogre: Let Us Cling Together (JAP/CN-patch) (font atlas, see #5350)

As far as I know, no other games use a "malformed" mip chain like this.

In Tactics Ogre it's to fit a lot of characters in a single font texture (for no good reason, really). In this case, every two successive mipmaps points to the same texture page, for a total of four. Not sure why it's done this way.