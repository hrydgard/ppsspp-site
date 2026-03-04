# Game feature directory

This is a very incomplete list of games to test with if you want to debug a specific feature. Hopefully you can find one that you own.

## OS and hardware features

### OSK (on-screen keyboard)

- MotoGP (single-language)
- PES 2013 (single-language)
- many more

### Microphone (hardware expansion)

- Beaterator
- DJ Max Fever
- Go Explore
- Infected (shouting into the mic pushes enemies back)
- Invizimals
- Rock Band Unplugged
- Talkman

### Camera (hardware expansion)

- EyePet Adventures
- Invizimals

### GPS

- ?

## GE rendering features and/or techniques

### Spline/Bezier patches

- Loco Roco (all games and variants)
- Namco Museum: Battle Collection (Pac-Man, more). This still exhibits a small bug (black patch on top of the ghosts)
- Pursuit Force (both games)
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

- Motorstorm (vehicles, 1-2 weights only)
- OutRun 2006 (flame rings in 4th level on the left, 4 morph weights IIRC)

### Depth buffer reads from the CPU

- Midnight Club: LA (sun lens flare)
- Syphon Filter (light halos (lots per frame!))
- WipEout (sun lens flare). Multiple reads per frame to get an average.
- a few more

### Direct color buffer reads from the CPU

- Coded Arms: Contagion (reads a single pixel from the alpha/stencil channel to identify enemies to lock-on)
- Dangan Ronpa, Dangan Ronpa 2 (renders a separate mini buffer to select clues, people)
- Motorstorm (light adaptation)
- a few more

### Color buffer format aliasing

- Cars Race-o-rama
- OutRun 2
- Split/Second
- Spongebob

### Depth buffer manual swizzling through geometry

This is a mistake that a few games have made - the PSP has hardware to swizzle depth buffers to linear, but instead they use triangles. This applies to the particle effects in these games

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

### Memcpy framebuffer download/upload

- Everybody's Golf 2 (line by line, ugh)

### Zipped PRX

- Fired Up

### Persistent render targets

- Mahjong Artifacts (Minis) relies on render targets sticking around.