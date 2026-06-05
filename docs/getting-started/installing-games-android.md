---
position: 5
---
# How to transfer PSP games to your mobile device

First, make sure you have your games as ISO (or CSO or CHD) files.
See [How to get games](how-to-get-games) and [How to dump games](dumping-games).

The following instructions apply identically for PPSSPP and PPSSPP Gold.

## Universal method (requires Wi-Fi)

This requires that your iOS or Android device, and the device where the games are, are connected to the same Wi-Fi network.
Additionally, this only works from PPSSPP 1.20 and upwards.

1. On the <b class="inapp">Games</b> tab, make sure you are in the folder where you want to put games.

1. Click the Upload icon (it has a folder with an up arrow on it).

1. Follow the on-screen instructions.
   You will be asked to go to a specific URL on the device that you want to copy files from.
   You will get a simple web interface where you can upload ISO files.

1. Use the web interface to upload your ISO files. Done!

## Android<img src="/static/img/icons/android.svg" aria-hidden="true" class="icon-36 icon-right">

This guide requires a PC.

1. Connect your Android device (phone, tablet, etc) to your PC using a USB cable.
   Likely, the one you charge the device with will work just fine.

1. On your device, pull down the menu and click the circled arrow:

    ![Open the notification about charging the device via USB.](/static/img/docs/move_games_android/step1.jpg)

1. Then click the circled item, "Tap for more options".

    ![Tap on the notification.](/static/img/docs/move_games_android/step2.jpg)

1. Next, you'll be at this screen, where you can now enable USB file sharing:

    ![Select File transfer.](/static/img/docs/move_games_android/step3.jpg)

1. On your PC, you'll now be able to see the device's storage in Explorer:

    ![Open device storage in Explorer.](/static/img/docs/move_games_android/step4.jpg)

    ![Open Internal shared storage.](/static/img/docs/move_games_android/step5.jpg)

1. You can now create a new folder to store your ISOs in, if you haven't already.
   A quick way is by pressing `Ctrl+Shift+N` and typing the name of your new folder.
   (It's recommended that it's separate from the `PSP` folder where system files and save games go, but it does work to use the same folder.)

1. Now simply copy the games over inside your new target folder.

Finally, after starting PPSSPP on your device, you'll be able to either directly browse to it (Android 10 or older, or old installs),
or you'll be able to tap <b class="inapp">Browse...</b> and select it.

## iOS<img src="/static/img/icons/ios.svg" aria-hidden="true" class="icon-36 icon-right">

The below instructions require a Mac.

1. Install PPSSPP on your iOS device (iPhone or iPad).

1. Connect your iOS device via USB to your Mac.

1. Open the device in Finder.
   Click the `Files` tab, then you should see PPSSPP listed.

1. Drag your ISO files directly into the app.

After this, the ISO files will be located on the virtual Memory Stick.
Simply browse them from the <b class="inapp">Games</b> tab.
(Click the Home icon if you can't find the files.)

If you don't have a Mac, use a Windows device and install [Apple Devices app](https://apps.microsoft.com/detail/9np83lwlpz9k).
