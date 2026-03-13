# Save data and storage on Windows

For general background, see the first part of [the Android page](/docs/getting-started/save-data-and-storage).

## Moving the Memory Stick folder

Find out where `PPSSPPWindows64.exe` is located. (That's the one you're running, right? Don't use the 32-bit one unless you have to!)

You can find it easily in the emulator by navigating to <b class="inapp">Settings -> System -> PSP Memory Stick -> Show Memory Stick folder</b>.

Then, create a file called `installed.txt` if it's not already there. Edit it with Notepad. Here's how it works:

- No `installed.txt`: Memory Stick is in a subfolder called `memstick` right next to the EXE file.
- `installed.txt` is empty: Memory Stick is in your user folder.
- `installed.txt` contains a path: Memory Stick is in that path. For example `E:\PSP` or whatever you want.