# Homebrew development with PPSSPP

PPSSPP can be a nice platform to test your in-development games and apps for the PSP.

Here, I'll document various tips and tricks.

## Use an up-to-date version of PPSSPP

The debugger has improved with leaps and bounds lately.

## Keep the ELF file next to your EBOOT.PBP, don't delete it

The debugger will read symbols and code line numbers from an elf like next to a PRX or EBOOT.PBP, which will be useful if you are diagnosing crashes.

Then you can remove it for distribution if you want.


## Use the modern PSPSDK

It's a bit of a hassle, but if you want to use modern C++ features and use a modern compiler, you're much better off with the modern WSL-based PSPSDK than the old Windows-based one.

However, don't forget the extra steps you have to do to get psplink going: https://pspdev.github.io/psplink/windows.html

A step is missing, too: `winget install --interactive --exact dorssel.usbipd-win`.
