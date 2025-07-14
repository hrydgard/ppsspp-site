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

Find the custom music folder for your game here: <sup>table is not yet complete</sup>
| Game | Folder | Notes |
| --- | --- | --- |
| Beats | `ms:/PSP/MUSIC` | |
| Crazy Taxi: Fare Wars | `ms:/MUSIC` | |
| Gran Turismo | `ms:/MUSIC` | unlockable feature;<br />subfolder selection supported |
| Surf's Up | `ms:/MUSIC/SURFSUP` | bitrate (kb/s): 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192;<br />sample rate: 44.1khz;<br />max 20 songs supported |
| TOCA Race Driver 3 Challenge,<br />DTM Race Driver 3 Challenge,<br />V8 Supercars Australia 3: Shootout | `ms:/PSP/MUSIC` | subfolder selection supported |
| Ultimate Board Game Collection | `ms:/PSP/MUSIC`,<br />`ms:/MUSIC` | subfolders supported |
| WipEout Pulse | `ms:/MUSIC/WIPEOUT` | |

In Gran Turismo to unlock custom music you must first [clear all Driving Challenges in blocks A and B](https://gran-turismo.fandom.com/wiki/Driving_Challenges_(GTPSP)), the rating doesn't matter.
Then it will be available through the options menu.

<div class="alert alert-info">Games might have trouble reading non-standard MP3 files.
    If you suspect that your file is impaired, try opening and exporting it with a constant bitrate using <a href="https://www.audacityteam.org/">Audacity</a>.
</div>

## Games that read Atrac3+ files

Some games read Atrac3+ encoded files instead of MP3's for custom music.
For these games you need to convert your songs to the proper format first.
The guide here requires a Windows PC.

Originally tools such as **[Exact Audio Copy PSP Edition](https://archive.org/details/codemasters-eacsetup)** by Codemasters or **[Rockstar Custom Tracks](https://thegtaplace.com/downloads/f1123-rockstar-custom-tracks)** by Rockstar Games were intended to be used.
However, EAC doesn't work on modern Windows and RCT only supports conversion from audio CD's.

Nowadays we can also use **[ATRACTool Reloaded](https://github.com/XyLe-GBP/ATRACTool-Reloaded)** for this purpose.
It can encode Waveform Audio File Format (`.WAV`) audio into Atrac3+ (`.at3`) and other Sony formats.

Simply download and install the tool, along with the adequate .NET Desktop Runtime version.
You can safely ignore the warning about OpenMG upon starting it.

ATRACTool Reloaded works with both mono and stereo `.WAV` files with most of the common bitrates.
However, the file must be encoded in 16-bit PCM mode with a sample rate of 44,100 Hz (44.1 kHz).

![ATRACTool Reloaded failed to encode the ATRAC file. Log: Not Supported Param](/static/img/docs/custom_background_music/custombgm_atrac3_atractool_fail.jpg)

If you get an error while loading or encoding a file, it either means that it's not in `.WAV` format or that it's encoded differently.
You have to re-encode it to such yourself, e.g. via [Audacity](https://www.audacityteam.org/).
Simply open the file, and select `File -> Export Audio...` from the menubar.

![When exporting to WAV, make sure to set Channels either to "Mono" or "Stereo", Sample Rate to "44100 Hz" and Encoding to "Signed 16-bit PCM".](/static/img/docs/custom_background_music/custombgm_atrac3_audacity_export.jpg)

In case you want to try out the feature first before you commit yourself to encoding your files, we provide 2 songs for you, already in Atrac3+:
- [eu-short.at3](/static/img/docs/custom_background_music/eu-short.at3)
- [We_Wish_You_a_Merry_Christmas.at3](/static/img/docs/custom_background_music/We_Wish_You_a_Merry_Christmas.at3)

<div class="alert alert-info">Tip: Some games have their official soundtrack encoded in Atrac3+.
    Try extracting a radio station file from GTA and loading it as a custom song in TOCA Race Driver 2.
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
