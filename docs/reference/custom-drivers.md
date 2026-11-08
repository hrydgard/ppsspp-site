# Custom Adreno and Mali graphics drivers

On modern versions of Android, it's possible to load alternative Vulkan drivers for certain Adreno and Mali GPUs using something called [libadrenotools](https://github.com/bylaws/libadrenotools). These can sometimes perform better and with less bugs than the drivers shipped with the device.

This is supported from PPSSPP 1.17 and forward for Adreno. For Mali, it's supported from 1.21 and onwards.

Note that if your GPU isn't one of the supported GPUs, this option will simply not be visible.

RISK: This is an advanced feature. It may be that PPSSPP ends up unbootable after installation of a driver like this. It's recommended to only do this if you have placed your memory stick on accessible storage, so you can delete the driver or modify PPSSPP's config if needed.

## What's it useful for?

This feature is not nearly as beneficial as it is for say Dolphin, Yuzu or RPCS3 since PPSSPP intentionally uses a relatively minimal GPU feature subset, but it can still be interesting to experiment with to get the best performance out of your device. Don't expect miracles though.

Note that this feature is not available for phones PowerVR GPUs, nor any other type of GPU other than modern Adreno and some Mali GPUs. You can check what GPU you have by going to `Settings/Tools/System Information` within PPSSPP.

Also note that these drivers only apply to Vulkan, not OpenGL. OpenGL always uses the pre-installed proprietary driver.

## How to download and use Adreno drivers

Go to the [AdrenoTools driver release page](https://github.com/K11MCH1/AdrenoToolsDrivers/releases/)
and download drivers to your device from, and keep them as ZIP files in your downloads directory (do not decompress).

Then, in PPSSPP, go to `Settings/Tools/Developer Tools`, and choose "AdrenoTools Driver Manager" (if you can't find it, this functionality is not available for your device, sorry, see above). On the screen that pops up, choose "Install custom driver" and pick the ZIP file out of Downloads.

After that, your new driver will be selectable below. Just press the Select button next to the name of the driver.

You can also remove drivers by clicking the garbage can. This button will not be visible on the currently selected driver.

To restore you regular graphics driver, just select the "Default GPU Driver".

## Different kinds of drivers

### Adreno: Mesa Turnip Driver

Mesa is the Linux project that maintains most open-source GPU drivers. Turnip is the Adreno Vulkan driver in this suite of drivers.

It supports more features and extensions than the official driver. Performance varies but is often somewhat faster.

### Adreno: Qualcomm Driver

These are official, proprietary drivers from Qualcomm, the company making Adreno GPUs. Sometimes bugs can be worked around by picking a different version of this driver. Note that many of these can have problems on certain devices, and I generally do not recommend using these unless they specifically say that they're suitable for your device.

### Mali: PanVK drivers

Mesa is the Linux project that maintains most open-source GPU drivers. PanVK is the Mali Vulkan driver in this suite of drivers, for some generations of Mali GPUs.

It supports more features and extensions than the official driver. Performance varies but is often somewhat faster.

See [this PanVK repository](https://github.com/soulexit/panvk-exynos9611) for more info, for Mali-G72.