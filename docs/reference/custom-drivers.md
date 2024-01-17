# Custom Adreno graphics drivers

On modern versions of Android, it's possible to load alternative Vulkan drivers for Adreno GPUs using something called [libadrenotools](https://github.com/bylaws/libadrenotools). These can sometimes perform better and with less bugs than the drivers shipped with the device.

PPSSPP 1.17 will gain support for this.

## What's it useful for?

Note that generally, this feature is not as beneficial as it is for say Dolphin or Yuzu since PPSSPP intentionally uses quite a minimal GPU feature set, but it can still be interesting to experiment with to get the best performance out of your device.

Note that this feature is not available for phones with ARM (Mali) or PowerVR GPUs. You can check what GPU
you have by going to `Settings/Tools/System Information` within PPSSPP.

Also note that these drivers only apply to Vulkan, not OpenGL. OpenGL always runs using the pre-installed proprietary driver.

## How to download and use Adreno drivers

In PPSSPP 1.17 or later, go to [AdrenoTools driver release page](https://github.com/K11MCH1/AdrenoToolsDrivers/releases/)
and download drivers to your device from, and keep them as ZIP files in your downloads directory (do not decompress).

Then go to `Settings/Tools/Developer Tools` in PPSSPP, and choose "AdrenoTools Driver Manager". On the screen that pops up, choose "Install custom driver" and pick the ZIP file out of Downloads.

After that, your new driver will be selectable below. Just press the Select button next to the name of the driver.

You can also remove drivers by clicking the garbage can. This button will not be visible on the currently selected driver.

To restore you regular graphics driver, just select the "Default GPU Driver".

## Different kinds of drivers

### Mesa Turnip Driver

Mesa is the Linux project that maintains most open-source GPU drivers. Turnip is the Adreno Vulkan driver in this suite of drivers.

It supports more features and extensions than the official driver. Performance varies but is generally a bit faster.

### Qualcomm Driver

These are official, proprietary drivers from Qualcomm, the company making Adreno GPUs. Sometimes bugs can be worked around by picking a different version. Note that many of these can have problems on certain devices, and I generally do not recommend using these unless they specifically say that they're suitable for your device.
