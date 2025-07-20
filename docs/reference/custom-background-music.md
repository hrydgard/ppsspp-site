# Custom background music

On the PSP, some games have support for playing audio files that you put on the memory stick as background music, replacing the default music that comes with the games.

Usually they require putting the music in a specific folder and then enabling the feature through the ingame sound options menu.

Some games also support selecting between the main folder and its subfolders.
This can be used to organize your songs into playlists or albums.

If you know some details that are missing below, [contact me](/contact).

More and better information will be available on this page over time.

## List of games that support this feature

This list should be pretty close to complete:
- ATV Offroad Fury: Blazin' Trails
- Beats
- Boom Beats
- Crazy Taxi: Fare Wars
- Dead or Alive Paradise
- Elminage Original (with patch 1.01)
- Gran Turismo
- Grand Theft Auto: Liberty City Stories
- Grand Theft Auto: Vice City Stories
- Gundam Assault Survive
- Gundam Memories \~Tatakai no Kioku\~
- Hatsune Miku: Project DIVA
- Hatsune Miku: Project DIVA 2nd
- Hatsune Miku: Project DIVA Extend
- Heroes' VS
- MLB 08: The Show
- MotorStorm: Arctic Edge
- NBA Live 07
- NBA Live 08
- NBA Live 09
- NBA Live 10
- Need for Speed Carbon
- Need for Speed Pro Street
- Need for Speed Undercover
- Need for Speed Shift
- Pro Evolution Soccer 2011 / World Soccer: Winning Eleven 2011
- Pro Evolution Soccer 2012 / World Soccer: Winning Eleven 2012
- Pro Evolution Soccer 2013 / World Soccer: Winning Eleven 2013
- Pro Evolution Soccer 2014 / World Soccer: Winning Eleven 2014
- SD Gundam G Generation Overworld
- Surf's Up
- Test Drive Unlimited <sup>(how? there's no option?)</sup>
- TOCA Race Driver 2 / DTM Race Driver 2 / V8 Supercars Australia 2 / Race Driver 2006
- TOCA Race Driver 3 Challenge / DTM Race Driver 3 Challenge / V8 Supercars Australia 3: Shootout
- Ultimate Board Game Collection
- Untold Legends: The Warrior's Code <sup>(how? there's no option?)</sup>
- WipEout Pulse

Homebrew are not listed.

## Games that read MP3 files

<div class="alert alert-warning">Several games require you to put the music into <code>ms:/MUSIC</code> at the root of the memory stick.
    Unfortunately this can't be done on Android if your device uses scoped storage and your memstick folder is named exactly <code>PSP</code>, because then <code>PSP</code> is the actual root.
    In the future, this will be possible by creating a <code>ROOT</code> folder in the <code>PSP</code> folder to simulate this.
</div>

Find the custom music folder for your game here: <sup>(table is not yet complete!)</sup>
| Game | Folder | Notes |
| --- | --- | --- |
| Beats | `ms:/PSP/MUSIC` | |
| Crazy Taxi: Fare Wars | `ms:/MUSIC` | |
| Gran Turismo | `ms:/MUSIC` | unlockable feature;<br />subfolder selection supported |
| SD Gundam G Generation Overworld | `ms:/MUSIC/OVERWORLD` | [explanation video](https://www.youtube.com/watch?v=LiNSQdUVjfU) |
| Surf's Up | `ms:/MUSIC/SURFSUP` | bitrate (kb/s): 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192;<br />sample rate: 44.1khz;<br />max 20 songs supported |
| TOCA Race Driver 3 Challenge,<br />DTM Race Driver 3 Challenge,<br />V8 Supercars Australia 3: Shootout | `ms:/PSP/MUSIC` | subfolder selection supported |
| Ultimate Board Game Collection | `ms:/PSP/MUSIC`,<br />`ms:/MUSIC` | subfolders supported |
| WipEout Pulse | `ms:/MUSIC/WIPEOUT` | |

In Gran Turismo to unlock custom music you must first [clear all Driving Challenges in blocks A and B](https://gran-turismo.fandom.com/wiki/Driving_Challenges_(GTPSP)), the rating doesn't matter.
Then it will be available through the options menu.

<div class="alert alert-info">Some games might have trouble reading MP3 files that have a sample rate different from 44,100 Hz.
    If you suspect that your file is affected, try opening and exporting it with 44,100 Hz sample rate using <a href="https://www.audacityteam.org/">Audacity</a>.
</div>

## Games that read ATRAC files

Some games read Atrac3 / Atrac3+ encoded files instead of MP3's for custom music.
For these games you need to convert your songs to the proper format first.
The guide here requires a Windows PC.

Originally tools such as **[Exact Audio Copy PSP Edition](https://archive.org/details/codemasters-eacsetup)** by Codemasters or **[Rockstar Custom Tracks](https://thegtaplace.com/downloads/f1123-rockstar-custom-tracks)** by Rockstar Games were intended to be used.
The two are essentially the same, however, RCT is limited to accepting only commercial audio CD's as input, while EACPE can convert MP3 files as well.

The issue with both however, is that they require your computer to have either a physical or an emulated optical disc drive, otherwise they freeze upon launch.
(Using the built-in Windows feature to mount a disk image is not sufficient.)

Nowadays we can just use **[ATRACTool Reloaded](https://github.com/XyLe-GBP/ATRACTool-Reloaded)** instead.
It can convert various audio formats into Waveform Audio File Format (`.WAV`) and then encode it into Atrac3+ (`.at3`).

Simply download and install the tool, along with the adequate .NET Desktop Runtime version.
You can safely ignore the warning about OpenMG upon starting it.

For the sake of convenience, enable these two settings under `Config -> Preferences` from the menubar:

![Enable the "Fixed output format for Wave conversion" setting and select the 44.1kHz from the list. Also enable the "Force conversion even if only wave files are read" setting.](/static/img/docs/custom_background_music/atractool_preferences.png)

In the scenario you have the `Force conversion even if only wave files are read` setting disabled and you want to load `.WAV` files directly, they must be encoded in 16-bit PCM mode with a sample rate of 44,100 Hz (44.1 kHz)!
If you get an error while loading or encoding a `.WAV` file, it means that it's encoded differently.

In case you want to try out the feature first before you commit yourself to encoding your files, we provide 2 songs for you, already in Atrac3+:
- [eu-short.at3](/static/img/docs/custom_background_music/eu-short.at3)
- [We_Wish_You_a_Merry_Christmas.at3](/static/img/docs/custom_background_music/We_Wish_You_a_Merry_Christmas.at3)

<div class="alert alert-info">Tip: Some games have their official soundtrack encoded in Atrac3 / Atrac3+.
    Try extracting a music file from Test Drive Unlimited and loading it as a custom song in GTA or TOCA Race Driver 2.
</div>

### Grand Theft Auto: Liberty City Stories & Vice City Stories

Custom music files go into a special folder next to the save data folders.

The filenames must end with the `.gta` extension!
Make sure to rename your files accordingly, e.g. `We_Wish_You_a_Merry_Christmas.gta`!

The games ignore songs that are shorter than 5 seconds.

Find the custom tracks folder for your Liberty City Stories version here:
| Game Version | Game Serial | Folder |
| --- | --- | --- |
| American,<br />Korean | ULUS10041 | `ms:/PSP/SAVEDATA/ULUS10041CUSTOMTRACKS` |
| European | ULES00151 | `ms:/PSP/SAVEDATA/ULES00151CUSTOMTRACKS` |
| German | ULES00182 | `ms:/PSP/SAVEDATA/ULES00182CUSTOMTRACKS` |
| Japanese CAPCOM,<br />Japanese Rockstar Classics | ULJM05255,<br />ULJM05885 | `ms:/PSP/SAVEDATA/ULJM05255CUSTOMTRACKS` |
| Sindacco Chronicles (romhack, all versions) | ULUS01826 | `ms:/PSP/SAVEDATA/ULUS01826CUSTOMTRACKS` |

Find the custom tracks folder for your Vice City Stories version here:
| Game Version | Game Serial | Folder |
| --- | --- | --- |
| American | ULUS10160 | `ms:/PSP/SAVEDATA/ULUS10160CUSTOMTRACKS` |
| European | ULES00502 | `ms:/PSP/SAVEDATA/ULES00502CUSTOMTRACKS` |
| German | ULES00503 | `ms:/PSP/SAVEDATA/ULES00503CUSTOMTRACKS` |
| Japanese CAPCOM,<br />Japanese Rockstar Classics | ULJM05297,<br />ULJM05884 | `ms:/PSP/SAVEDATA/ULJM05297CUSTOMTRACKS` |

### TOCA Race Driver 2

Custom music files go into the folder of save data and its subfolders.
It is recommended to organize your songs into subfolders so as not to mix them with the actual save files.
The game supports subfolder selection.

The filenames must end with the `.toc` extension!
Make sure to rename your files accordingly, e.g. `We_Wish_You_a_Merry_Christmas.toc`!

The game ignores songs that are shorter than 2 seconds.

Find the save folder for your game version here:
| Game Title | Region | Serial | Folder |
| --- | --- | --- | --- |
| Race Driver 2006 | America | ULUS10096 | `ms:/PSP/SAVEDATA/ULUS100960000` |
| TOCA Race Driver 2 | Europe | ULES00040 | `ms:/PSP/SAVEDATA/ULES000400000` |
| DTM Race Driver 2 | Germany | ULES00041 | `ms:/PSP/SAVEDATA/ULES000410000` |
| V8 Supercars Australia 2 | Australia | ULES00042 | `ms:/PSP/SAVEDATA/ULES000420000` |

The following versions of the game **DO NOT** support custom music:
| Game Title | Region | Serial |
| --- | --- | --- |
| TOCA Race Driver 2 | Japan | ULJM05160 |

The European and Japanese versions of the game are also known as **TOCA Race Driver 2: Ultimate Racing Simulator**.
