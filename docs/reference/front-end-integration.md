# Front end integration on Android

If you need this you know what it is, most others won't need to bother. This is here to be google-able.

The action is of course `android.intent.action.VIEW`.

The activity name is:

`org.ppsspp.ppsspp/org.ppsspp.ppsspp.PpssppActivity`

For PPSSPP Gold, use the following activity name:

`org.ppsspp.ppssppgold/org.ppsspp.ppsspp.PpssppActivity`

## Exiting directly to the frontend from the pause menu

This should work:

```sh
adb shell am start -n org.ppsspp.ppsspp/org.ppsspp.ppsspp.PpssppActivity --es org.ppsspp.ppsspp.Args "--pause-menu-exit"
```

But, according to issue [#18528] there still seem to be some problems.