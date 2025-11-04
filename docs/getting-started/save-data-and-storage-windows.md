# Save data and storage on Windows

For general background, see the first part of [/docs/getting-started/save-data-and-storage].

## Moving the memory stick folder

Find out where PPSSPPWindows64.exe is located (that's the one you're running, right? Don't use the 32-bit one unless you have to!).

Then, create a file called "installed.txt" if it's not already there. Edit it with notepad. Here's how it works:

- No installed.txt: Memory stick is in a subfolder called `memstick` right next to the EXE file.
- Installed.txt is empty: Memory stick is in your user folder.
- Installed.txt contains a path: Memory stick is in that path. For example `E:\PSP` or whatever you want.