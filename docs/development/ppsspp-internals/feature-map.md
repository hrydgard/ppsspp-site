# Game feature directory

This is a very incomplete list of games to test with if you want to debug a specific feature. Hopefully you can find one that you own.

## OS and hardware features

### OSK (On-screen keyboard)

- MotoGP (single-language)
- PES 2013 (single-language)
- Many more

### Microphone (hardware expansion)

- Invizimals
- Infected (Shouting into the mic pushes enemies back)
- Rock Band Unplugged
- DJ Max Fever
- Talkman
- Go Explore

### Camera (hardware expansion)

- Invizimals
- EyePet Adventures

### GPS

- ?

## GE rendering features and/or techniques

### Spline/Bezier patches

- Loco Roco (all games and variants)
- Pursuit Force (both games)
- Puzzle Bobble Pocket (completely unnecessary! Just used for rotating sprites)
- Namco Museum: Battle Collection (Pacman, more)

### Vertex skinning/bones

- Crisis Core
- Tekken 6
- Tekken 5
- God of War (both games)
- Many, many more.

### Vertex morphing

- Outrun 2006 (flame rings in 4th level on the left, 4 morph weights IIRC)
- Motorstorm (vehicles, 1-2 weights only)

### Depth buffer reads from the CPU

- Wipeout (sun lens flare). Multiple reads per frame to get an average.
- Syphon Filter (light halos (lots per frame!))
- Midnight Club: LA (sun lens flare)
- A few more

### Direct color buffer reads from the CPU

- Motorstorm (light adaptation)
- Dangan Ronpa, Dangan Ronpa 2 (renders a separate mini buffer to select clues, people)
- Coded Arms: Contagion (reads a single pixel from the alpha/stencil channel to identify enemies to lock-on)
- A few more

### Color buffer format aliasing

- Outrun 2
- Split/Second
- Spongebob
- Cars Race-o-rama

### Depth buffer manual swizzling through geometry

This is a mistake that a few games have made - the PSP has hardware to swizzle depth buffers to linear, but instead they use triangles. This applies to the particle effects in these games

- Ratchet & Clank (both games)
- Jak & Daxter

### Tricks requiring dual source blending

- Wipeout Pure (bloom effect)
- Armored Core 3 (issue #11430)
- Lens flares in Ridge Racer

### Intra-buffer block copies

- Tales of Phantasia X/Cross Edition (included as a retro game in Tales of Phantasia: Narikiri Dungeon X). See issue #21098

### Block copies to RAM, later used as texture

- Digimon
- Burnout Legends (sun lens flare)
- Tales of Phantasia X

### Memcpy framebuffer download/upload

- Everybody's Golf 2 (line by line, ugh)

### Zipped PRX

- Fired Up