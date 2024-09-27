# Strange games

These are a list of games that tend to cause questions, or do questionable things.

## Zenonia

This is a pixel-art game that nevertheless doesn't use exact PSP pixels for its graphics. It has its own strange resolution and relies on bilinear filtering to not look bad. This means that unlike with most other games, you can't get perfect pixels by the usual combination: Nearest texture filtering, 1x rendering, nearest scale factor and integer scaling. Instead, the result will be a distorted mess.

The likely cause of this is that the game is a port from a mobile phone game, likely a device with a very different resolution so just translating the graphics 1:1 made things too small on the PSP.

For this game, if you want the sharp pixel look, change texture filtering to nearest and crank up the rendering resolution to 5x or more. That's as close as you'll get to an ideal result.

## Darkstalkers

This game draws to the screen using the CPU. This is highly unusual outside homebrew software, and it even uses the GE to draw hardware accelerated graphics and then draws using the CPU on top. This is not possible to emulate efficiently using a PC GPU, so instead, for this game we force the use of our software renderer, with some special optimizations.

Additionally, this game runs at an unusual resolution, and it ends each frame by upscaling its lower resolution framebuffer to full screen. This is quite expensive to do with the software renderer due to it being a full screen bilinear filtered blit, plus it ruins the sharpness of the image, so instead we completely bypass this step, and use the low original resolution directly, for a much better result.

This means that in PPSSPP, it's the only game that doesn't actually run at 480x272 at 1x render resolution!
