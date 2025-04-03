# GPI/GPO

PSP development kits have an extra set of inputs and outputs: 8 on/off switches, and 8 LEDs.

These are called the GPI and GPO, respectively, and can be read/written using the PSP kernel functions `sceKernelGetGPI`/`sceKernelSetGPO`.

On consumer hardware, `sceKernelGetGPI` always returns 0 and `sceKernelSetGPO` does nothing.

From 1.18, PPSSPP can return custom settings for the GPI switches, and display the GPO outputs. This functionality can be reached in developer settings and in the in-game DevMenu, if enabled. As it turns out, some games actually do things with these, as can be seen in [this YouTube video](https://www.youtube.com/watch?v=ioodSTApknM) by PtoPOnline.

## Known games with GPI/GPO functionality

* GhostBusters: The Video Game will enable a debug overlay if you turn on some GPI switches
* Parappa The Rapper: Displays a metronome on the GPO LEDs.
* LocoRoco/LocoRoco 2: Enables extra debug logging if you turn on some GPI switches
* Need for Speed Carbon: Own The City: Turn on GPI Switch 5 to get some performance debug info on-screen.
