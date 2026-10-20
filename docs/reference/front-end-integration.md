# Front end integration on Android

If you need this you know what it is, most others won't need to bother. This is here to be google-able.

The action is of course `android.intent.action.VIEW`.

The activity name for the free edition is:

`org.ppsspp.ppsspp/org.ppsspp.ppsspp.PpssppActivity`

For PPSSPP Gold, use the following activity name:

`org.ppsspp.ppssppgold/org.ppsspp.ppsspp.PpssppActivity`

And for [PPSSPP Legacy](/docs/reference/legacy-edition), use this one instead:

`org.ppsspp.ppsspplegacy/org.ppsspp.ppsspp.PpssppActivity`

## Passing command line arguments

PPSSPP reads [command line arguments](/docs/reference/command-line) from the string extra `org.ppsspp.ppsspp.Args`, on all editions. They only take effect when PPSSPP starts. If it's already running, the game still boots but the arguments are ignored, so launch with a flag that restarts it (`-S` in `am start`, or `--activity-clear-task`).

The game to boot can be given either way:

- As the intent's data URI, the way launchers normally do it. The arguments in the extra are added after it. Before PPSSPP 1.21, the extra was ignored when there was a data URI ([#18528]).
- Inside the extra itself, quoted if it has spaces: `--pause-menu-exit "/sdcard/PSP/GAME/My Game.iso"`.

## Returning to the frontend when the game ends

`--pause-menu-exit` makes PPSSPP quit instead of going to its own menu when the game ends: from the pause menu's Exit, when the game quits by itself, or after a load error. Before PPSSPP 1.21, it only changed the pause menu's Exit ([#18456]).

A launcher configuration, in the style of Daijishou:

```
-n org.ppsspp.ppsspp/org.ppsspp.ppsspp.PpssppActivity
-a android.intent.action.VIEW
-c android.intent.category.DEFAULT
-d {file.uri}
-t application/octet-stream
--es org.ppsspp.ppsspp.Args --pause-menu-exit
--activity-clear-task
--activity-clear-top
--activity-no-history
```

`{file.uri}` is typically a `content://` URI. That works because the launcher has been given access to your ROM folder and passes it on to PPSSPP with the intent.

## Getting a URI PPSSPP can open by itself

A launch that doesn't pass access along (`adb`, scripts, a frontend's custom per-game command) needs a URI that PPSSPP already has access to: a game inside a folder you've opened in PPSSPP. To get one, open the game's info screen, tap the **...** menu, and choose **Copy deep link** (PPSSPP 1.21 and later). This copies a URI of this form:

```
content://com.android.externalstorage.documents/tree/primary%3APSP/document/primary%3APSP%2FRidge%20Racer.iso
```

The `tree/.../document/...` form matters: it says which of PPSSPP's folders the file is in. The shorter `content://com.android.externalstorage.documents/document/...` form of the same file is refused.

Launching it from a computer:

```sh
adb shell 'am start -S -n org.ppsspp.ppsspp/org.ppsspp.ppsspp.PpssppActivity -a android.intent.action.VIEW -d "content://com.android.externalstorage.documents/tree/primary%3APSP/document/primary%3APSP%2FRidge%20Racer.iso" --es org.ppsspp.ppsspp.Args --pause-menu-exit'
```

The whole command is in single quotes so that the device's shell, not your local one, sees the inner quotes.

On older Android versions (before 11, or with legacy storage), PPSSPP can also open plain file paths, either as a `file://` data URI or in the extra:

```sh
adb shell 'am start -S -n org.ppsspp.ppsspp/org.ppsspp.ppsspp.PpssppActivity --es org.ppsspp.ppsspp.Args "--pause-menu-exit \"/sdcard/PSP/GAME/My Game.iso\""'
```