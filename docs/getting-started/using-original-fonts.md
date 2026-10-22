---
position: 10
---
# Using the original PSP fonts

Some games don't supply their own fonts, but instead make use of the fonts built-in to the PSP firmware.
PPSSPP ships with some free replacement fonts, but a few games will look better with the original fonts, and some homebrew doesn't work well with the replacements.

## Getting them automatically

From PPSSPP 1.21, you normally don't need to do anything. Almost every PSP game disc carries a firmware updater, and when you start a game, PPSSPP installs the firmware from it if you don't have one yet (or if the disc's is newer). The fonts come with it. Even if you've turned that off, PPSSPP still takes just the fonts from the disc when a game needs them.

You can also install a firmware yourself from an official PSP update file, in Settings / Tools / PSP Firmware. That screen also shows how many fonts you have. The firmware, fonts included, is stored in `/PSP/NAND/flash0` in your memstick directory.

See [How to run the XMB](/docs/getting-started/how-to-run-the-vsh) for more about installing firmware.

## Placing them manually

If you have dumped the fonts yourself from a real PSP, you can still place the files in this location in your memstick directory:

`/PSP/flash0/font`

Note that if this folder exists, PPSSPP uses it instead of the installed firmware's fonts, and won't install fonts from game discs.

<details>
    <summary>These are the files that should be placed there:</summary>
    <ul>
        <li><code>jpn0.pgf</code></li>
        <li><code>kr0.pgf</code></li>
        <li><code>ltn0.pgf</code></li>
        <li><code>ltn1.pgf</code></li>
        <li><code>ltn2.pgf</code></li>
        <li><code>ltn3.pgf</code></li>
        <li><code>ltn4.pgf</code></li>
        <li><code>ltn5.pgf</code></li>
        <li><code>ltn6.pgf</code></li>
        <li><code>ltn7.pgf</code></li>
        <li><code>ltn8.pgf</code></li>
        <li><code>ltn9.pgf</code></li>
        <li><code>ltn10.pgf</code></li>
        <li><code>ltn11.pgf</code></li>
        <li><code>ltn12.pgf</code></li>
        <li><code>ltn13.pgf</code></li>
        <li><code>ltn14.pgf</code></li>
        <li><code>ltn15.pgf</code></li>
    </ul>
</details>

On the desktop builds, you can also replace the fonts directly in the `assets/flash0/font` subdirectory, but this way they will be overwritten by updates.
