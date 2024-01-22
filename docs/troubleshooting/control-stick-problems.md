# Game controller problems?

Some steps to try if you have problems with your gamepad or joystick.

## Not getting any input at all?

Things to check:

* If on Android, can you navigate other apps with the connected gamepad? If not, troubleshoot that first.
* Go to Settings/Tools/Developer Tools and open the Touchscreen Test. It's a bit of a misnomer since it
  can also be used to check other input. Push some buttons and see if any messages show up on screen.
* If the above worked, you should be able to go to Settings/Controls/Control Mapping and use Auto mapping.
* If Auto mapping didn't work, try manually mapping the inputs one by one. Click "O" and then press the
  button you want to use for that, for example.

## D-Pad and analog stick won't work on Android

This is a common issue that appears to be because of a bug in Android itself.

[Here's the issue on Android Buganizer](https://issuetracker.google.com/issues/163120692).

Still not much of a resolution.

It's known that disabling various accessibility settings can fix it, such as:

* Accessibility / Magnification / Magnification Shortcut
* QuickCursor
* Longshot
* One-handed mode
* Just any accessibility option

It can also be that input comes on unexpected "axis". Go to the control mapping screen and re-map your stick and d-pad to make sure they're right.

Also try rebooting the phone, because why not..