# Help, I get a black screen!

Or, your game won't load for one reason or other. Try these basic steps!

## Don't extract the ISO!

Just run the ISO file directly in PPSSPP, do not extract it into files.

Some games can actually work when played from a folder when extracted, which can be useful if you're dabbling in game modifications, but many require a specific disc layout, and will not work this way. You have to run the ISO file directly.

## Make sure your ISO file is OK

In PPSSPP, you can right-click (or long-press) the game icon, and on the screen that pops up, press "Calculate CRC". From PPSSPP 1.16 and later, it will tell you if your ISO file is known to be valid with a little green notice.

If not, it might still be a valid file, but it's worth looking into (re-dumping it from UMD, for example).

## Do not use graphics settings in the "Speedhacks" section

Settings like "Skip buffer effects" can completely break graphics in some games. Do not use them.

## Do other games run?

If not, there might be something wrong with your graphics driver, and it might be worth trying another rendering backend in settings.

## Is it a homebrew app?

Place it in the PSP/GAME directory in your memstick folder, don't try to run it from elsewhere, it might not find its data files.