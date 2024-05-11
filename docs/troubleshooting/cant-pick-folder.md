# I can't pick a folder on Android

If, when trying to pick a folder to store PSP data in, you get something that looks like this:

![Big red Java exception](/static/img/cant_pick_folder/exception.jpg)

Then, either your device is missing a folder picker entirely (which is unlikely if it's a phone, but likely on Android TV), or it's provided by a default app that has been disabled somehow.

## Things to try

### Android TV

If you have an Android TV and are getting this message, you may be out of luck for now since many ship without a proper file browser app. I'm considering making a special build for these older TVs that target andolder API version. This will need to be side-loaded over USB.

### Android phones and other devices

The problem seems to most often be that the "Files" app has been uninstalled somehow. This seems to be possible on some phones, although it shouldn't be allowed.

But, one user reported success with an app called [Brevent](https://play.google.com/store/apps/details?id=me.piebridge.brevent), which can help you reinstall the "Files" ("Arquivos") app if it's missing.

Another user sent in these screenshots (in Portuguese), showing how he managed to enable it again:

    It was the files app that got disabled. But for some reason,
    differently of other disabled apps that could be reactivated, I
    couldn't do anything.
    Reinstalling the same app from play store did not solved it .
    What worked: reset app preferences.
    I only found it a little bit strange cause that app were disable
    for a long time, and yet the problem happen just now, anyway, that
    solved the problem.

![Settings screen 1](/static/img/cant_pick_folder/settings1.jpg)

![Settings screen 2](/static/img/cant_pick_folder/settings2.jpg)
