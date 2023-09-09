# Plugin support

PPSSPP doesn't support traditional PSP plugins, because those often have a far too tight relationship to the PSP OS kernel to work in an emulator that only pretends the kernel is there.

However, PPSSPP now supports plugins specifically made for the emulator.

These will in the future be available to download and activate directly from the app, but for now, they're still a bit obscure both in discovery in usage.

## Installing a plugin

Copy it to PSP/PLUGINS in your memory stick directory.

## Save States

Plugins are compatible with save states, but updating/removing plugins will not affect the save state. While using plugins, prefer using in-game saves whenever possible.

## Dual Analog Support

Some plugins might add second analog stick support, in order to fully take advantage of this feature, second analog stick needs to be configured in the controls menu.

## Available plugins

### ThirteenAG's plugins

Multiple plugins are available here, for GTA, Splinter Cell and The Warriors. They add ultra-widescreen fixes, framerate improvements and more.

[ThirteenAG's plugins](https://github.com/ThirteenAG/WidescreenFixesPack/releases?q=PPSSPP&expanded=true)

### ermaccer's plugins

[Plugin for Mortal Kombat Unchained](https://ermaccer.github.io/posts/mkuhook/)

[Censorship plugin for Manhunt 2](https://ermaccer.github.io/posts/mh2censorshippatch/)

[Debug/Cheats menu plugin for Manhunt 2](https://ermaccer.github.io/posts/mh2menupsp/)

### Freakler's Plugins

[ppsspp-GTARemastered (second analog stick support for GTA games)](https://github.com/Freakler/ppsspp-GTARemastered)

[Cheat Device Remastered](http://cheatdeviceremastered.com/)


## Creating plugins

There are no official instructions, but the below resource can help.

These instructions are from the [original pull request](https://github.com/hrydgard/ppsspp/pull/13335) by [Unknown].

[PSPModeBase](https://github.com/xan1242/PSPModBase) is a code base for making plugins.

### Basic setup

1. Create a plugin directory inside `PSP/PLUGINS/`, i.e. `PSP/PLUGINS/mycrisiscoretranslation/`.
2. Add an ini file, such as:
```ini
[options]
version = 1
type = prx
filename = patch.prx

; Always specify games to indicate what games are supported.
[games]
; Normal usage: specify game IDs that are supported.
ULJM05275 = true
; Advanced usage: specify another ini to use a separate prx for a certain game ID.
; (in most cases, you won't use this.)
ULES01048 = spanish.ini
; General plugins for any game: use ALL to indicate all game IDs.
ALL = true

; Optional, specify ini for another prx for specific user interface languages.
; (in most cases, you won't use this.)
[lang]
hr_HR = croatian.ini
```

Create and compile your plugin, in this case named `patch.prx` using the pspsdk, and it'll be loaded. However, you cannot use many kernel or HEN functions, since they don't work in PPSSPP. The internal architecture is different. You can however call the same functions games can call, update RAM, etc.

### Additional ini options

You can use memory = 64 under [options] to allocate more RAM, up to 93 MB due to memory map limitations. Beware this gives the game overall more RAM, and may affect cheats and memory management in the game.

Under [games], you can use ULJM05275 = foo.ini to allow your plugin to use a different prx for different game IDs, i.e. multidisc games. true is the same as using plugin.ini, in other words use [options] in the same file. `ALL = true` or `ALL = otherfile.ini` can also be used to apply to all games. seplugins won't work, because HEN/kernel funcs are not supported.

You can also add [lang] and put something like se_SE = swedish.ini. This allows you to load a prx just for a user's specific language. This can be in addition to, or instead of, a main plugin.

## Future plans

* More plugins will be listed here
* Downloads will be available directly from the app