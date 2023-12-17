# Custom Adreno graphics drivers

On modern versions of Android, it's possible to load alternative Vulkan drivers for Adreno GPUs using something called [libadrenotools](https://github.com/bylaws/libadrenotools). These can sometimes perform better and with less bugs than the drivers shipped with the device.

PPSSPP 1.17 will gain support for this.

Note that generally, this feature is not as beneficial as it is for say Dolphin or Yuzu since PPSSPP intentionally uses quite a minimal GPU feature set, but it can still be worth experimenting with to get the best performance out of your device.

Note that this feature is not available for phones with ARM (Mali) or PowerVR GPUs. You can check what GPU
you have by going to `Settings/Tools/System Information` within PPSSPP.

Also note that these drivers only apply to Vulkan, not OpenGL. OpenGL always runs using the proprietary driver.

## How to download and use Adreno drivers

[AdrenoToolsDrivers Release page](https://github.com/K11MCH1/AdrenoToolsDrivers/releases/)

Download drivers to your device from the above link, and keep them as ZIP files (do not decompress).

Then go to `Settings/Tools/Developer Tools` in PPSSPP, and choose "Install custom driver" and pick the ZIP file.

After that, your new driver will be selectable under the "Custom Driver" setting.

## Different kinds of drivers

### Mesa Turnip Driver

Mesa is the Linux project that maintains most open-source GPU drivers. Turnip is the Adreno Vulkan driver in this suite of drivers.

It supports more features and extensions than the official driver. Performance varies but is generally a bit faster.

### Qualcomm Driver

These are official, proprietary drivers from Qualcomm, the company making Adreno GPUs. Sometimes bugs can be worked around by picking a different version.
