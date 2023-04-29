# Vulkan Driver Bugs

A few years ago, I meant to start a website that documented Vulkan driver bugs. Never got around to it, but I did some writing for it, which I've collected here.

I've updated it slightly but there's more to be done.

## Introduction

Vulkan is the present and future of graphics, at least on some platforms. But closing in on seven years old at time of writing, it also has a past - early drivers for a new API often have crippling bugs, and Vulkan is no exception. Worse, drivers for mobile devices often stick around a long time in consumer devices due to the lack of updates on Android. This page is an incomplete attempt to collect information about driver bugs and performance advice for the whole ecosystem of Vulkan-enabled devices.

In the spirit of Vulkan, this site is meant to receive contributions from the community. If you have a bug that is not listed on this site,
feel free to open a pull request against this website [here](https://github.com/hrydgard/ppsspp-site/).

## Bugs

### Fragment discard broken when stencil test on, depth off

If you draw using a fragment shader that conditionally does a discard; operation, while having a pipeline bound that sets `pDepthStencilState->depthTestEnabled` to `false` and `stencilTestEnabled` to `true`, the discard operation will be ignored when the rasterizer writes to the stencil buffer on Adreno 500, a very common GPU in modern Android phones, and also multiple ARM Mali GPUs (though the condition is a bit more complex there).

This has caused multiple issues in PPSSPP:

[Graphic regression on Legend of Heroes III - Song of the ocean](https://github.com/hrydgard/ppsspp/issues/11535)

And many more, will collect the links at some point.

[Adreno forums report](https://developer.qualcomm.com/forum/qdn-forums/software/adreno-gpu-sdk/66526)

Reproduces in the GPU driver test screen of PPSSPP, on a multitude of older devices.

#### Workaround

Also enable depth test/write - or do the following:

```glsl
layout (depth_unchanged) out float gl_FragDepth;
...
gl_FragDepth = gl_FragCoord.z
```

Note that on all affected models of both Adreno and Mali, the above fixes the problem entirely, without
even having to enable depth on the pipeline state (!). Pretty remarkable coincidence.

None, unless you can get away with also writing depth.

Checking for affected hardware and/or drivers on Adreno:

```cpp
if (deviceProps.deviceID >= 0x05000000 && deviceProps.deviceID < 0x06000000) {
    if (deviceProps.driverVersion < 0x80180000) {
        // Affected by bug
    }
}
```

Checking for affected hardware and/or drivers on ARM Mali:

```cpp
int majorVersion = VK_API_VERSION_MAJOR(deviceProps.driverVersion);
if (majorVersion < 40) {
    // Affected by bug
}
```

#### Status

Fixed in the latest Adreno drivers since some time, supposedly fixed by Mali (but not yet confirmed).

### Color mask not applied

The color mask controls which channels of a color attachment are written. It's possible to write to only the depth buffer while still having both a color buffer and a depth buffer bound by setting the color mask to 0. Unfortunately, on Adreno 500, this does not produce the expected results.

It's unclear what exactly happens but it's clear that it doesn't work properly. See PPSSPP issue #10421.

#### Workaround

Simply enable alpha blending if you haven't already, and set pColorBlendState->pAttachments[i].colorBlendOp (and alphaBlendOp if desired) to src=VK_BLEND_FACTOR_ZERO, dst=VK_BLEND_FACTOR_ONE.

#### Affected hardware

Adreno 5xx generation, all drivers. No fix.

#### Checking for affected hardware and/or drivers

if (deviceProps.deviceID >= 0x05000000 && deviceProps.deviceID < 0x06000000)

### Dual-source blending broken

Dual source blending modes are not working on some drivers that claim to expose the feature.

Dual source blending is an optional feature that represents a path for fragment shaders to output an additinal color, which can then be used in the blend equation through VK_BLEND_FACTOR_SRC1_COLOR and its cousins. SRC1 signifies that it's the second color output, there's similarly ALPHA1 and their inverses. This is useful for, among other things, implementing ClearType-style font rendering where you need separate alpha values for each color component, or simulating strange blender behaviour in ideo game console emulators (Dolphin and PPSSPP both use dual source blending for this purpose).

#### Workaround

Simply avoid dual source blending through multipass tricks. You likely have a fallback path anyway, especially if you support mobile hardware.

#### Affected hardware

AMD: all drivers with deviceProps.driverVersion <= 0x00407000.
Intel: Old drivers, fix version unknown.

### Different behavior in vertex shader triangle culling on Mali

There's a pretty common (but unofficial), technique to cull triangles already in the vertex shader, by setting the w coordinate of
one of the output vertices to NaN. This works on all PC GPUs and most mobile GPUs that are not ancient. This turned out to be very useful when implementing a behavior of the PSP where triangles that have a point sticking out of a certain coordinate rectangle simply get dropped.

Setting just the w coordinate to NaN on all other modern GPUs confuses the rasterizer enough to drop the triangle, but on Mali chips, you have to set all four coordinates (xyzw) to NaN to avoid getting garbage triangles everywhere on the screen.

### Inverse depth in viewport not functioning correctly

If you set .minDepth to a larger value than .maxDepth when creating a VkViewport, depth is crushed to one of the values, or otherwise corrupted, resulting in bad or no rendering.

Expected result would be that values are simply inverted in a consistent way such that things will work the same if you just substitute VK_COMPARE_GREATER for VK_COMPARE_LESS in your VkPipeline depth config, and swap minDepth and maxDepth.

#### Affected hardware

* ARM Mali before R12
* AMD Radeon before some time in 2017.
* NVIDIA before 387.92 ( dolphin-emu/dolphin#6067 )

#### Current status

Fixed on all drivers.

### Primitive Restart broken for 16-bit indices

Repro
Draw using primitive restart with 16-bit indices and a vertexOffset that's non-zero.

Similar issue happens if you use indirect indexed draws (which might need vertex offset).

Workaround
Use vertexOffset = 0 and bind your vertex buffers at the appropriate offset.

Affected GPUS:
ARM Mali before R22 (Midgard) and R10 (Bifrost)
Current status
Fixed.

### `VK_REMAINING_*` enums broken

#### Description

Older Mali drivers do not handle these enums correctly in barriers.

`VK_REMAINING_ARRAY_LAYERS` and `VK_REMAINING_MIP_LEVELS` are both affected.

Example:

`image_memory_barrier.subresourceRange.levelCount = VK_REMAINING_MIP_LEVELS;`

#### Workaround

Simply specify the actual number of mip levels.

#### Source

[Report thread on ARM forums (requires login)](https://community.arm.com/support-forums/f/graphics-gaming-and-vr-forum/9344/vulkan-driver-issues---workaround-collection-thread/33606)

#### Current status

Fixed.

### Clear-only renderpass causes rendering artifacts

erpasses that are empty apart from the initial `CLEAR` operation cause corrupt rendering on certain Mali drivers.

That is, if you just do `vkCmdBeginRenderPass` and immediately afterwards `vkCmdEndRenderPass`, with a render pass where the attachment description has `loadOp=VK_ATTACHMENT_LOAD_OP_CLEAR`, things can break. Usually you get black blocky artefacts.

Additionally, similar things can happen if you mix `LOAD_OP_CLEAR` with `LOAD_OP_LOAD` in depth/stencil on Mali - use the same value for both depth and stencil channels.

#### Workaround

Combine your clearing renderpass with your rendering (as you should have been doing anyway),

Alternatively, use `VK_IMAGE_LAYOUT_GENERAL` on your framebuffers for a fix and slight performance penalty (prevents the driver hardware optimization that the bug breaks).

#### Source

[Report thread on ARM forums (requires login)](https://community.arm.com/support-forums/f/graphics-gaming-and-vr-forum/9344/vulkan-driver-issues---workaround-collection-thread/33606)

#### Affected drivers

Empty renderpasses: R14 (Bifrost) and R26 (Midgard). Notable devices: Galaxy S8/S8+.

Mixed LOAD/CLEAR: Unknown fix version.

#### Current status

Fixed.

### Passing samplers as parameters to functions doesn't work.

Passing samplers as parameters to functions doesn't work, and `textureQueryLod()` crashes the driver.

####  Workaround

Sampler: Avoid these constructs, or use SPIRV-Opt to inline all functions before calling `vkCreateShaderModule`.

`textureQueryLod()`: Don't use, or blacklist driver.

#### Affected GPUS:

ARM Mali and including R12 (Midgard) and R10 (Bifrost)

#### Current status

Fixed.

### Swapchain broken if width not divisible by 32

This is a bug only affecting older PowerVR Android drivers, which I'm really curious on how it got through
any kind of testing. If the width of the swapchain is not divisible by 32, the display output will be heavily corrupted.

#### Workaround

Force swapchain width to be divisible by 32 (!)

```cpp
swapChainExtent.width &= ~31;
```

This has some unfortunate consequences, like the scissor rectangle and viewport being off by a few pixels, leading to minor UI bugs. Better than crashing though!

### Rendering does not clip to viewport

Rendered geometry is not clipped properly to the current viewport.

Curiously, this is actually desired behavior when emulating the PSP, as its viewport also does not clip.

Affects Mali Midgard devices only, before driver version 15.

#### Workaround

Clip your scissor rectangle to the viewport rectangle.
This applies both for dynamic state and preset scissor/viewport rectangles.

```cpp
int majorVersion = VK_API_VERSION_MAJOR(deviceProps.driverVersion);
if (majorVersion < 15) {
    // Affected by bug
}
```
