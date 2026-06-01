# Translation

PPSSPP is translated into a multitude of languages, but as you may have noticed, many of the translations are far from complete, and there are many still missing.

Unfortunately it's not possible to have an on-duty team adding translations of every new string as they appear, so translations are dependent on volunteer efforts.

## Adding new strings

Since 2025, we've started to use AI to translate all new strings.

This can be done by running tools/langtool with `cargo run -- add-new-key-ai section "Key in English".`. You need to have the environment variable `OPENAI_API_KEY` set to, well, that. Or if you want to use another AI provider, ask your AI to modify the langtool code accordingly :P

Or, I guess, you can just use your AI agent to add the string to all the inis directly. This used to be far too efficent but maybe it can work today.

## How to update or contribute new translations

If you'd like to contribute to translation, the easiest way to get started and to be able to test directly in the app is to use the Windows or another desktop version, and go into `assets/lang` under the main PPSSPP directory, and open the .ini file for the language you want to help with, and make or copy your changes there. To add a new language, just add a new language .ini file, and also add an entry in `assets/langregion.ini`. The language ini file should be named according to [this table](https://docs.oracle.com/cd/E23824_01/html/E26033/glset.html), or similar. When you make changes here, you may have to restart PPSSPP for them to take effect.

Alternatively, you can directly to go to [the assets/lang folder on GitHub](https://github.com/hrydgard/ppsspp/tree/master/assets/lang) and start editing the .ini file for the language you know.

Once happy with your changes, you can submit a pull request on GitHub, or just e-mail the finished .ini file to [the maintainer](hrydgard+translations@gmail.com).

## Ini file editing

Do not touch section headers like `[Audio]`.

On each line, translate the right-hand side of the `=` sign, and leave the left side untouched.