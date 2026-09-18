# Optimizing code the GE

If you're writing a renderer for your own PSP game or a port/recomp, this page might be useful. Here I've collected tricks that I've noticed games to be using.

What tricks you need will depend on what bottlenecks you have, of course, so not all of these might actually give you a performance improvement.

## Command lists

Behind the scenes, sceGu records lists of raw 32-bit commands to memory, and then you can have the GE execute them by using sceGeEnqueue (which has a couple of variants). There are many patterns for use, and there's a function called sceGeUpdateStallAddr which tells the GE at which address to stop, which is useful when building a command list in parallel with having the GE execute it. This is generally not the most optimal pattern though.

Most big games bypass sceGu and do their own raw command writing, for reasons that we will see.

## Double buffer command lists

Most examples go draw a triangle, sync the gpu with sceGeUpdateStalllAddr, draw another thing, sync again. While this is a way to go if you really need to minimize latency, if you want to maximize speed you need the GE and CPU to be as decoupled as possible. You need to do something like this:

Frame 0:

Enqueue command list 1 on the GE.
Start recording to command list 0.
Wait for vblank. At vblank, sceGeDrawSync().

Frame 1:

Enqueue command list 0 on the GE.
Start recording to command list 1.
Wait for vblank. At vblank, sceGeDrawSync().

And continue flipping back and forth like this.

If you don't want to double buffer because of the extra frame of latency, you need to be really careful to interleave work between the CPU and the GE. Kick off the clear immediately at the start of the frame before you do any CPU scene processing, for example, to keep the GE as busy as possible. Even so, there's a large risk of wasting performance this way.

## Stripe your clears

Clears and other full screen fills are always done in vertical stripes, this is very important to maximize internal cache usage. 32 pixels wide stripes at 8888 color, 64 pixels wide for any 16-bit color format. This rule applies even if you are using depth-only clears, because the depth swizzle format in memory depends on the color format. Never clear using a single rectangle.

## Avoid indexed triangle lists

Indexing costs memory and provides little benefit on the PSP. Unless it makes tranlation from a ported game's internal data layout very simple, avoid, there are better alternatives.

## PRIM auto-advances the vertex pointer

Behind the scenes, PRIM commands update the vertex pointer, so if you have nine consecutive vertices and want to draw them as two strips of 4 and 5 triangles, after setting the vertex pointer you just go PRIM strip, 4; PRIM strip, 5; with no resetting of the vertex pointer in between.

## Shrink your vertex formats

Don't use Float32 positions if you can avoid it - instead use S16 or even S8 and bake a scale/offset into your world matrix at draw time. Normals should always be compressed to S16 or smaller, F32 is a waste. Same goes for UVs. For those, you have the flexibility of using the UVSCALE/UVOFFSET registers to fit smaller components to a range.

## Precompute lighting if possible

Hardware lights are pretty costly, especially if you use specular. If you need them you need them, but try to use 1 or 2, not all four, if at all possible.

## Precompute chunks of state that you set often

If you have large chunks of state that you set over and over again (such as setting up a texture with mipmaps, or a bunch of lighting state), write the commands to memory somewhere folllowed by a return, and then CALL it from your main command list.

## When using skinning, minimize the number of active bones

It's totally fine to mix PRIMs that use 3 bones with those that use 2 bones, so wheerever possible, where a bone weight is zero for a bunch of consecutive vertices, shrink the bone count.