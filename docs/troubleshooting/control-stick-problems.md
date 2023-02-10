# Problems with Bluetooth controllers on Android

## D-Pad and joystick won't work

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