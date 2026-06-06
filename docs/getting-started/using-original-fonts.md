---
position: 10
---
# Using the original PSP fonts

Some games don't supply their own fonts, but instead make use of the fonts built-in to the PSP firmware.
PPSSPP ships with some free replacement fonts, but a few games will look better if you use the original fonts instead.

If you have dumped the PSP firmware yourself from a real PSP, you can place the files in this location in your memstick directory:

`/PSP/flash0/font`

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

On the Windows build, you can also replace the fonts directly in the `assets` subdirectory, but this way they will be overwritten by updates.
