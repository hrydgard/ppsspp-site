# Translation

PPSSPP is translated into a multitude of languages, but as you may have noticed, many of the translations are far from complete, and there are many still missing.

Unfortunately it's not possible to have an on-duty team adding translations of every new string as they appear, so translations are dependent on volunteer efforts.

## How to update or contribute new translations

If you'd like to contribute to translation, the easiest way to get started and to be able to test directly in the app is to use the Windows or another desktop version, and go into `assets/lang` under the main PPSSPP directory, and open the .ini file for the language you want to help with, and make or copy your changes there. To add a new language, just add a new language .ini file, and also add an entry in `assets/langregion.ini`. The language ini file should be named according to [this table](https://docs.oracle.com/cd/E23824_01/html/E26033/glset.html), or similar. When you make changes here, you may have to restart PPSSPP for them to take effect.

Alternatively, you can directly to go to [the assets/lang folder on GitHub](https://github.com/hrydgard/ppsspp/tree/master/assets/lang) and start editing the .ini file for the language you know.

Once happy with your changes, you can submit a pull request on Github, or just email the finished .ini file to [the maintainer](hrydgard+translations@gmail.com).

## Ini file editing

Do not touch section headers like `[Audio]`.

On each line, translate the right-hand side of the `=` sign, and leave the left side untouched.