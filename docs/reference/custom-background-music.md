# Custom background music

On the PSP, some games have support for playing audio files that you put on the memory stick as background music, replacing the default music that comes with the games.

Usually they require putting the music in a specific folder and then enabling the feature through the ingame sound options menu.

To find the location of your memory stick on your device, navigate to <b class="inapp">Settings -> System -> PSP Memory Stick -> Show Memory Stick folder</b>.

<div class="alert alert-warning">Several games require you to put the music into <code>ms:/MUSIC</code> at the root of the memory stick.
    Unfortunately this can't be done on Android if your device uses scoped storage and your memstick folder is named exactly <code>PSP</code>, because then <code>PSP</code> is the actual root.
    In the future, this will be possible by creating a <code>ROOT</code> folder in the <code>PSP</code> folder to simulate this.
</div>

Some games also have the option of selecting to play music from subfolders.
This can be used to organize your songs into playlists or albums.

If you know some details that are missing below, [contact me](/contact).

More and better information will be available on this page over time.

## List of games that support this feature

This list should be pretty close to complete:
- ATV Offroad Fury: Blazin' Trails
- Beaterator
- Beats / Shouten Beat
- Crazy Taxi: Fare Wars / Crazy Taxi: Double Punch
- Dead or Alive Paradise
- Elminage Original<small> (with patch 1.01)</small>
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
- TOCA Race Driver 2 / DTM Race Driver 2 / V8 Supercars Australia 2 / Race Driver 2006
- TOCA Race Driver 3 Challenge / DTM Race Driver 3 Challenge / V8 Supercars Australia 3: Shootout
- Ultimate Board Game Collection
- WipEout Pulse

Homebrew are not listed.

<!--
### List of bogus games

The following games have been tested and DO NOT actually support custom music:
- ATV Offroad Fury Pro
- Boom Beats
- MX vs. ATV Reflex
- MX vs. ATV Unleashed: On the Edge
- MX vs. ATV Untamed / MX vs. ATV Extreme Limite
- SD Gundam G Generation Portable
- SD Gundam G Generation World
- Test Drive Unlimited
- <small>the Japanese release of </small>TOCA Race Driver 2<small> (maybe it's possible to patch the game to unlock it?)</small>
- Untold Legends: Brotherhood of the Blade
- Untold Legends: The Warrior's Code
- WipEout Pure<small> and </small>WipEout Pure: Stealth Edition

If someone claims otherwise, they should be asked to provide evidence and detailed instructions.
-->

## Games that read MP3 files

Find the custom music folder for your game here: <sup>(table is not yet complete!)</sup>
| Game | Folder | Notes |
| --- | --- | --- |
| Beats,<br />Shouten Beat | `ms:/PSP/MUSIC`,<br />`ms:/MUSIC` | &#x26A0;&#xFE0F;&nbsp;see issue [#14812];<br />subfolders supported;<br />also looks for ATRAC files |
| Crazy Taxi: Fare Wars,<br />Crazy Taxi: Double Punch | <small>primary:</small>`ms:/MUSIC/CRAZYTAXI`<br /><small>fallback:</small>`ms:/MUSIC` | &#x26A0;&#xFE0F;&nbsp;see issue [#15509];<br />fallback folder is only checked if no MP3s found in primary folder |
| Dead or Alive Paradise | `ms:/MUSIC` | &#x26A0;&#xFE0F;&nbsp;see issue [#8672];<br />you must first make progress in the game;<br />sample rate: any;<br />MP3s must contain a valid [ID3v1 or ID3v2 tag](https://en.wikipedia.org/wiki/ID3) |
| Gran Turismo | `ms:/MUSIC` | you must first make progress in the game;<br />subfolder selection supported |
| Heroes' VS | `ms:/MUSIC/HEROES' VS/*` | MP3s go into specific subfolders;<br />only the first MP3 is loaded from each subfolder |
| SD Gundam G Generation Overworld | `ms:/MUSIC/OVERWORLD` | you must first make progress in the game;<br />sample rate: 32,000&nbsp;Hz or 44,100&nbsp;Hz;<br />MP3s should NOT contain an [ID3v2 tag](https://en.wikipedia.org/wiki/ID3) |
| Surf's Up | `ms:/MUSIC/SURFSUP` | filenames must be at least 9 characters long, not counting the `.mp3` extension;<br />only the first 20 MP3s are loaded |
| TOCA Race Driver 3 Challenge,<br />DTM Race Driver 3 Challenge,<br />V8 Supercars Australia 3: Shootout | `ms:/PSP/MUSIC` | subfolder selection supported |
| Ultimate Board Game Collection | `ms:/PSP/MUSIC`,<br />`ms:/MUSIC` | subfolders supported |
| WipEout Pulse | `ms:/MUSIC/WIPEOUT` | |

<!-- For Surf's Up, the bitrate requirement mentioned by the game's manual is bogus. -->

<div class="alert alert-warning">Unless otherwise noted, games don't recognize or play MP3s that have a sample rate different from 44,100&nbsp;Hz.
    If your song is playing in incorrect pitch or speed in PPSSPP, or isn't playing at all on PSP, try opening and exporting it with 44,100&nbsp;Hz sample rate using <a href="https://www.audacityteam.org/">Audacity</a>.
</div>

### Additional details

#### Dead or Alive Paradise

If it's your first playthrough on your save, you must first complete Day 1.
Afterwards you can visit the Radio Station during daytime.

The message "Unable to play the selected song." shows up when the selected MP3 file doesn't contain a valid [ID3v1 or ID3v2 tag](https://en.wikipedia.org/wiki/ID3).
You can use any tag editor to add it, any respectful music player app has one built into them.
Even the one accessible through Windows file explorer is sufficient: right click on MP3 file, navigate to `Properties -> Details`, edit the Value of any Property, then click `Apply`.
The files don't need to contain any actual metadata, just the ID3 tag to exist.

#### Gran Turismo

You must first [clear all Driving Challenges in blocks A and either B or C](https://gran-turismo.fandom.com/wiki/Driving_Challenges_(GTPSP)), the rating doesn't matter.
Then it will be available through the options menu.

#### Heroes' VS

When you enable the custom soundtrack option, the game will prompt you to generate the required subfolders. Then you can place 1 MP3 inside each subfolder to replace the corresponding ingame song.

<details>
    <summary>The following subfolders are created:</summary>
    <ul>
        <li><code>01J_電王</code></li>
        <li><code>01V_ネガ電王</code></li>
        <li><code>02J_ダブル</code></li>
        <li><code>02V_エターナル</code></li>
        <li><code>03J_オーズ</code></li>
        <li><code>03V_恐竜グリード</code></li>
        <li><code>04_フォーゼ</code></li>
        <li><code>05J_ネクサス</code></li>
        <li><code>05V_ダークザギ</code></li>
        <li><code>06J_メビウス</code></li>
        <li><code>06V_エンペラ星人</code></li>
        <li><code>07J_ゼロ</code></li>
        <li><code>07V_カイザーベリアル</code></li>
        <li><code>08_サーガ</code></li>
        <li><code>09J_ターンＡ</code></li>
        <li><code>09V_ターンＸ</code></li>
        <li><code>10J_ダブルオー</code></li>
        <li><code>10V_リボーンズ</code></li>
        <li><code>11J_ゴッド</code></li>
        <li><code>11V_マスター</code></li>
        <li><code>12_ストライクフリーダム</code></li>
        <li><code>13_ストラガイア</code></li>
        <li><code>14_メインメニュー</code></li>
        <li><code>15_ストーリー</code></li>
        <li><code>16_対戦・マルチプレイ</code></li>
        <li><code>17_ヒーロー図鑑</code></li>
        <li><code>18_オープニング</code></li>
    </ul>
</details>

#### SD Gundam G Generation Overworld

You must first [complete the Prologue stage](https://ggen.fandom.com/wiki/SD_Gundam_G_Generation_Overworld_Stage:_Prologue) and select your Master character.
Then from the main menu navigate to `Gallery -> Custom BGM`.
The Gallery is the second option under red colored option.
From here you can select 1 custom song for each character from each series that plays when that character initiates an attack during gameplay.

If an MP3 file contains an [ID3v2 tag](https://en.wikipedia.org/wiki/ID3), the game sometimes plays weird sounds instead, so make sure to remove the tag first!

## Games that read ATRAC files

Some games read Atrac3 / Atrac3+ encoded files instead of MP3's for custom music.
For these games you need to convert your songs to the proper format first.
The guide here requires a Windows PC.

Originally tools such as **[Exact Audio Copy PSP Edition](https://archive.org/details/codemasters-eacsetup)** by Codemasters or **[Rockstar Custom Tracks](https://thegtaplace.com/downloads/f1123-rockstar-custom-tracks)** by Rockstar Games were intended to be used.
The two are essentially the same, but RCT is limited to accepting only commercial audio CD's as input, while EACPE can convert MP3 files as well.

The issue with both however, is that they require your computer to have either a physical or an emulated optical disc drive, otherwise they freeze upon launch.
(Using the built-in Windows feature to mount a disk image is not sufficient.)

Nowadays we can just use **[ATRACTool Reloaded](https://github.com/XyLe-GBP/ATRACTool-Reloaded)** instead.
It can convert various audio formats into Waveform Audio File Format (`.WAV`) and then encode it into Atrac3+ (`.at3`).

Simply download and install the tool, along with the adequate .NET Desktop Runtime version.
You can safely ignore the warning about OpenMG upon starting it.

For the sake of convenience, enable these two settings under `Config -> Preferences` from the menubar:

![Enable the "Fixed output format for Wave conversion" setting and select the 44.1kHz from the list. Also enable the "Force conversion even if only wave files are read" setting.](/static/img/docs/custom_background_music/atractool_preferences.png)

In the scenario where you have the `Force conversion even if only wave files are read` setting disabled and you want to load `.WAV` files directly, they must be encoded in 16-bit PCM mode with a sample rate of 44,100&nbsp;Hz!
If you get an error while loading or encoding a `.WAV` file, it means that it's encoded differently.

In case you want to try out the feature first before you commit yourself to encoding your files, we provide 2 songs for you, already in Atrac3+:
- [eu-short.at3](/static/img/docs/custom_background_music/eu-short.at3)
- [We_Wish_You_a_Merry_Christmas.at3](/static/img/docs/custom_background_music/We_Wish_You_a_Merry_Christmas.at3)

<div class="alert alert-info">Tip: Some games have their official soundtrack encoded in Atrac3 / Atrac3+.
    Try extracting a music file from Test Drive Unlimited and loading it as a custom song in any of the games below.
</div>

### ATV Offroad Fury: Blazin' Trails

Custom music files go into a special folder next to the save data folders.

The filenames must contain a dash `-` character in the middle, otherwise the game ignores the songs!
The game uses this to tell apart the name of the artist from the title of the song.
Make sure to rename your files accordingly, e.g. `United_States_Navy_Band-We_Wish_You_a_Merry_Christmas.at3`!

Find the music folder for your game version here:
| Game Version | Serial | Folder |
| --- | --- | --- |
| American | UCUS98603 | `ms:/PSP/SAVEDATA/UCUS98603MUSIC` |
| European | ULES00155 | `ms:/PSP/SAVEDATA/ULES00155MUSIC` |

### Beats

Custom music files go into `ms:/PSP/MUSIC`, `ms:/MUSIC` and any of their subfolders.

Although the game looks for ATRAC files, it rejects to actually play them on PSP with the error message "There is a problem with the selected track, the game cannot proceed. Please choose another track."
[In PPSSPP v1.19 and later however, the game plays them fine.](https://github.com/hrydgard/ppsspp/issues/20677)

The game also supports reading MP3 files, ~~so there's little reason to convert your files to Atrac3+~~.
You can play the soundtrack of other PSP games though.

<div class="alert alert-warning">The game currently doesn't work well with custom MP3 files in PPSSPP.
    Until <a href="https://github.com/hrydgard/ppsspp/issues/14812">issue #14812</a> is resolved, ATRAC format should be used.
</div>

### Grand Theft Auto: Liberty City Stories & Vice City Stories

Custom music files go into a special folder next to the save data folders.

The filenames must end with the `.gta` extension!
Make sure to rename your files accordingly, e.g. `We_Wish_You_a_Merry_Christmas.gta`!

The games ignore songs that are shorter than 5 seconds.

Find the custom tracks folder for your Liberty City Stories version here:
| Game Version | Serial | Folder |
| --- | --- | --- |
| American,<br />Korean | ULUS10041 | `ms:/PSP/SAVEDATA/ULUS10041CUSTOMTRACKS` |
| European | ULES00151 | `ms:/PSP/SAVEDATA/ULES00151CUSTOMTRACKS` |
| German | ULES00182 | `ms:/PSP/SAVEDATA/ULES00182CUSTOMTRACKS` |
| Japanese CAPCOM,<br />Japanese Rockstar Classics | ULJM05255,<br />ULJM05885 | `ms:/PSP/SAVEDATA/ULJM05255CUSTOMTRACKS` |
| Sindacco Chronicles<small> (romhack, all versions)</small> | ULUS01826 | `ms:/PSP/SAVEDATA/ULUS01826CUSTOMTRACKS` |

Find the custom tracks folder for your Vice City Stories version here:
| Game Version | Serial | Folder |
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

The following version of the game **DOES NOT** support custom music:
| Game Title | Region | Serial |
| --- | --- | --- |
| TOCA Race Driver 2 | Japan | ULJM05160 |

The European and Japanese versions of the game are also known as **TOCA Race Driver 2: Ultimate Racing Simulator**.

## Custom sound and music in Beaterator

Beaterator is a unique game in the sense that it revolves around creating new songs.
For this reason, it supports reading the more professional Waveform Audio File Format (`.WAV`) and MIDI (`.MID`) files.
The game itself is rather complex, so it won't be explained in detail here.

All custom sound and music files go into the `ms:/MUSIC/BEATERATOR` folder.

The game allows you to import `.WAV` files as sounds to use when creating or editing drum, melody and audio loops.
They must be encoded in 16-bit PCM mode with a sample rate of either 22,050&nbsp;Hz or 44,100&nbsp;Hz!

The game also allows you to record new sounds using the microphone when creating or editing loops.

<div class="alert alert-warning">Recording audio for the game using the microphone currently doesn't work in PPSSPP.
    The resulting sound will be junk.
    Until <a href="https://github.com/hrydgard/ppsspp/issues/19528">issue #19528</a> is resolved, this feature shouldn't be used.
</div>

You can also export your loops onto the memory stick in `.MID` format (with limitations).

The game also allows you to import `.MID` files (with limitations) as songs to play or edit.

You can also export your songs onto the memory stick in `.WAV` and (with limitations) in `.MID` formats.

All exported sound and music files are written into the `ms:/MUSIC/BEATERATOR` folder.
