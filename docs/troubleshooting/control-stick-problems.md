# Game controller problems?

Some steps to try if you have problems with your gamepad, joystick or touch controls.

## Not getting any controller input at all!

Things to check:

* If on Android, can you navigate other apps with the connected gamepad? If not, troubleshoot that first.
* Go to Settings/Tools/Developer Tools and open the Touchscreen Test. It's a bit of a misnomer since it
  can also be used to check other input. Push some buttons and see if any messages show up on screen.
* If the above worked, you should be able to go to Settings/Controls/Control Mapping and use Auto mapping.
* If Auto mapping didn't work, try manually mapping the inputs one by one. Click "O" and then press the
  button you want to use for that, for example.

## It's difficult to rotate the stick on the touch screen or with a controller

The God of War games have a few events where you need to rotate the stick a few full laps. Doing that on the touchscreen, or especially with keyboard inputs, can be a little tricky but there's another way.

IMPORTANT: Before doing the below, check that you have disabled Frameskipping on the Graphics tab in settings - this can interfere with the move. This might make it possible for you to perform the move without issue.

Anyway, here are the steps to enable doing it the easy way:

* If you are using a controller, just map two buttons to "Rotate Analog (CW)" and CCW. CW refers to clockwise,
  CCW to counter-clockwise. Then press and hold the mapped buttons to rotate the stick.

* If you are using only touchscreen, try this:

  1. Go to Settings/Controls/Edit touch control layout.../Customize
  1. Check the checkboxes next to Custom 1 and Custom 2 (or whichever ones you want)
  1. For each of these, click it, give them an icon of your choosing, and check Rotate Analog (CW) for one and CCW for the other
  1. Back out to the `Edit touch control layout...` screen, and place them on the touchscreen somewhere.
  1. In the game, press and hold to spin the stick. This should let you pass the event.

## D-Pad and analog stick won't work on Android

This is a common issue that appears to be because of a bug in Android itself.

[Here's the issue on Android Buganizer](https://issuetracker.google.com/issues/163120692).

Still not much of a resolution.

It's known that disabling various accessibility settings can fix it, such as:

* Accessibility / Magnification / Magnification Shortcut
* QuickCursor (and other apps that layer themselves on top of other apps)
* Longshot
* One-handed mode
* Just about any accessibility option

It can also be that input comes on unexpected "axis". Go to the control mapping screen and re-map your stick and d-pad to make sure they're right.

Also try rebooting the phone, it can't hurt.