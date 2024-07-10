# What are all those key binds?

## Analog limiter

Bind this if you want to control the analog stick with a digital controller like a keyboard, and you want to be able to move the stick only halfway out from the middle. Pressing this button does that.

## Rapid Fire

Holding this button will repeat all the normal PSP button inputs (Cross, Circle, etc) during the hold. This is useful for quickly skipping lots of dialog while fast-forwarding, for example.

## Fast-forward

Runs emulation as fast as possible. This is useful to quickly get past unskippable cutscenes, for example. Can even shorten load times in many cases.

## AxisSwap

Lets you temporarily switch the function of the analog stick and the d-pad. This is useful for controllers with a limited amount of sticks or buttons.

## RightAn.Left/Right/Up/Down

These are controller bindings for the right analog stick. The PSP doesn't actually have one, but support for one can be added via [plugins](docs/reference/plugins). A few plugins are available currently.

## Hold

From [the forums](https://github.com/hrydgard/ppsspp/pull/16293#issuecomment-1295237338):

On the PSP, the power switch had three positions: you pulled it upward to turn the device on or put it to sleep, and otherwise it sat in the middle. You could pull it downward to activate "hold" (it would stay in this position until moved out of hold.)

From memory, during "hold" it would continue to display the screen but other inputs were typically non-responsive (I assume at the OS level because I think this worked in all games I know of.) You could, for example, turn off the screen and switch to "hold" mode, then put it in your pocket to listen to music.

I'm not aware of any games that actually use "hold", but sometimes it was used to activate cheats.

-\[Unknown\]

## Note

This is the little Note button that sits below the screen on the real PSP, and can be used for muting audio by holding it down for a few seconds.

It doesn't do anything in most games, although they are free to listen to it. It's mostly used for controlling plugins like `prxshot`, though such plugins don't run in PPSSPP. So it's mostly pointless to bind, still, we want to have the ability.

## Screen

Similar to Note, it's not really useful. On the real PSP, holding it for a bit will turn off the screen. It's unclear why you would want to do that at all.

## Wlan

This is the WLAN switch on the side of the PSP. Binding it serves little purpose.

## Remote Hold

The PSP had headphones with a special little remote controller, which was useful if you were listening to music through your PSP while it was sitting in your backpack or pocket, for example. This is the button that was on that remote. I don't think any games or homebrew care about this button, so like most of the others here, it's pretty pointless.

## Vol +/Vol -

These are the volume buttons on the PSP. There's usually little reason to bind them in PPSSPP.
