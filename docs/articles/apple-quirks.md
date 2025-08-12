# Apple development quirks

Here I'll write strange things I've learned about iOS/macOS development.

## The alternate icon conundrum

If you name an alternate icon (.xcassets method) AppIconGold, it doesn't work - it'll show the placeholder. However, renaming the same icon GoldIcon, it works! It seems something is looking for AppIcon as a prefix... Sigh. This wasted like 5 hours of debugging and talking to LLMs in desperation. Finally one suggested this solution...

## Popping the keyboard on iOS

This is absolutely ridiculous. TODO: Write up...
