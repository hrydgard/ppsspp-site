# Texture replacement

You can create texture replacement packs in PPSSPP to replace textures.

It's also possible to include a textures.ini file to customize them, but defaults are used if the file doesn't exist.

## Basic structure

```ini title="textures.ini"
[options]
# 1 is the only version, as of writing
version = 1
# Quick is the default hash.  Other options: xxh32, xxh64 (better hashes, but slower)
# ALL hashes will change if you change this option, so pick one at the beginning and stick with it.
hash = quick

# By default, a file with .png at the end of the hash is used.
# Override below to share filenames or use custom filenames.
# To skip a texture (like videos) use a blank filename on the right side.
# Syntax: hash = filename.png
[hashes]
# Ignore the intro video textures.
099bf1c000000909 =
0993698000000909 =
09a47a0000000909 =
09ad024000000909 =
# Water animation.
099c0db096c0500ecd2f3e6e = water/frame1.png
099c2db0d26dc9a7966195cf = water/frame2.png
099c4db0fa2cbcfec0bd3e0f = water/frame3.png
099c6db0d17d9a67c7591d4f = water/frame4.png
# This texture is loaded at two addresses.  The content is the same.
094b89907dcca1a5ee284131 = 094b5a707dcca1a5ee284131.png
# Alias the extra mip level too.
094b89907dcca1a5ee284131_1 = 094b5a707dcca1a5ee284131_1.png

# This allows customizing the range of a texture that is hashed.
# Careful: sometimes the same address is reused later for a texture that needs more hashing.
# Syntax: address,w,h = w,h
[hashranges]
# Force the intro video to hash 480x272.
0x09936980,512,512 = 480,272
0x099bf1c0,512,512 = 480,272
0x09a47a00,512,512 = 480,272
0x09ad0240,512,512 = 480,272
# Clip logos to actual texture size.
0x090056d0,256,256 = 176,160
0x0900c4d0,256,64 = 208,56
```

## Compatibility with Windows, Android, Linux, Mac, etc.

If you're making a texture pack for others to use - that's great.  Otherwise, ignore this section.

Filenames are the tricky part here.  Specifically:

* Always use `/` not `\` for directories.  All platforms (even Windows) support `/`, but `\` doesn't work anywhere but Windows.
* Always use lowercase filenames.  On Windows and Mac, "Hello.png" is the same as "hello.png", but that's not the case everywhere.  Using only lowercase is safest.
* Avoid special characters: Windows doesn't support a bunch, and they can cause problems for others too.

If you follow those guidelines, even more people will be able to appreciate your hard work.

## Texture sizes on the PSP

TL;DR: Some things are hard for robots, and easy for humans.  Unfortunately this makes replacing textures harder and more annoying.

The PSP hardware limited textures to power-of-two sizes, so games always use those sizes (e.g. 512x512 or 512x256.)

When a game wants to show a full screen image (480x272), it has to use a texture that's 512x512, because the PSP hardware just can't process a texture actually 480x272 in size.  And the extra space that isn't used?  The game doesn't care - it usually has garbage.

Unfortunately, this means that hashes may be different even though any human (sorta like a captcha) could obviously tell they are identical.  It's often very difficult for PPSSPP to detect this: sometimes 512x512 images are sprite sheets, fonts, or just scaled down.  And it happens with smaller images (even 32x64) too.

If you solve this without making PPSSPP slow, please send a pull request.

Workaround: you can add `reduceHash = True` under `[options]` (this cannot be used with `hash = quick`.  This will just ASSUME that only the top half of any texture is important.  If a game draws text a letter at a time into a texture or ever changes the bottom half, this will definitely break drawing in those cases.  And if you run into this... your only option will be to start over from scratch (with `reduceHash` off), or live with the brokenness.  You've been warned.

## Parts of the hash

The hash simply uses the same hashing PPSSPP internally uses to tell textures apart.  Aside from a hash of the image data, it also uses the hash of the palette (for palette-swapped images), and the address of image in memory.

Keep in mind that the `quick` hash especially is not perfect.  If you rely only on it, you might find yourself replacing a completely unrelated texture by accident (oops.)  Decide your risk carefully.

It's possible to ignore some of these parts, but if you do, don't use `hash = quick`.  Here are the options (includes mipmap levels, see below):

```ini
[hashes]
# Address + CLUT (palette) + data hash + mipmap level 0.
094b89907dcca1a5ee284131_0 = very/organized/things/texture1.png

# Level defaults to 0.
094b89907dcca1a5ee284131 = very/organized/things/texture2.png

# Ignore the texture with this CLUT regardless of data.
094b89907dcca1a500000000_0 =

# The same CLUT / data at any address:
000000007dcca1a5ee284131 = very/organized/things/texture3.png

# The same address / data with any CLUT:
094b899000000000ee284131 = very/organized/things/texture4.png

# The same data at any address (collisions might happen):
0000000000000000ee284131 = very/organized/things/texture3.png
```

## Mipmaps

Mipmapping is a technique games use to make faraway textures look better.

Imagine you have a HD (1920x1080) image of a car, but that car is very far away.  It's so far away, that on your screen, it's only 19x11 pixels (1%.)

Because of how GPUs work, they're going to only pick a few random pixels from the 1920x1080 image to draw on screen.  You might get unlikely, and these could all be from the tail-lights.  Then the "car" would just become a red blur when it's far away (even if the actual car is black.)

Mipmapping fixes this by giving the GPU smaller images for farther away.  Some, but not all, PSP games use these.

The "original size" image is "mip level 0", and every size after that must be *exactly* half the width and height.  So, if "mip level 0" is 1920x1080, then "mip level 1" should be 960x540.

```ini
094b89907dcca1a500000000_0 = mip_level_0.png
094b89907dcca1a500000000_1 = mip_level_1.png
094b89907dcca1a500000000_2 = mip_level_2.png
# etc.
```

You don't need an infinite number of mip levels.  Even a few will often help, especially if the player has anisotropic texturing enabled.

Mipmapping will make the game load a little slower, but usually look better.  Sometimes it can make drawing (after initial load) faster too.

## Zipped texture packs

You can install zipped texture packs directly, there's no need to unzip them. This will even improve performance, especially if you zip without compression (images are generally already compressed, compression on top of compression doesn't do much good).

## More info

See [#8715](https://github.com/hrydgard/ppsspp/pull/8715), [#8792](https://github.com/hrydgard/ppsspp/pull/8792), [#4630](https://github.com/hrydgard/ppsspp/issues/4630), and [#9668](https://github.com/hrydgard/ppsspp/pull/9668).