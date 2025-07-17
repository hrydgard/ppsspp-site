# PPSSPP Website

This is the entire frontend for the official ppsspp.org website.

It's built using a super-minimal site generator written in Rust.

The design is inspired by Docusaurus but doesn't use anything from it anymore
other than the folder structure.

Turns out doing everything fully custom is easier to understand, debug and maintain for the long term,
plus the build process is way easier. `cargo run`, that's it.

## Backend and authentication

The backend is not (yet?) open source.

Authentication is done using httponly cookies, which means that client side javascript can't see them.
Thus, we also keep a record of the current login state in localstorage. Could also use a client-visible cookie
but localstorage was just easier.

We load the authentication data from localstorage on every page load, and apply visibility accordingly. All this
is managed from /static/script/account.js.

## Local testing instructions

Prerequisites:

- Rust must be installed.

Local testing:

- `cargo run`
- Go to `localhost:3000` in a browser.

That will launch the site on localhost:3000. A proxy is launched pointing `/api` at our dev backend (which
isn't always running).

To see options, `cargo run -- --help`.

### Testing in cloud

If you don't want to install Rust on your computer, you can also use a Chromium based web browser (such as Edge)
for testing through Codespaces on the GitHub website (for a limited time).

1. [Create a new codespace.](https://docs.github.com/en/codespaces/developing-in-a-codespace/creating-a-codespace-for-a-repository#creating-a-codespace-for-a-repository)

2. Once the shell is ready, download and run the Rust installer:  
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  
When prompted, press enter to proceed with standard installation.

3. When installation is finished, source the env file to the shell:  
`. "$HOME/.cargo/env"`  
(Mind the dot!)

5. To build and run the website: `cargo run`  
When the website is running, a [pop-up](https://docs.github.com/assets/cb-17218/mw-1440/images/help/codespaces/quickstart-port-toast.webp) will appear in the bottom right corner of the screen, prompting you to
open it in your browser.  
Alternatively, you can open it from the [Ports tab](https://docs.github.com/assets/cb-25086/mw-1440/images/help/codespaces/quickstart-forward-port.webp) by hovering on forwarded address and clicking on the globe
icon.

To stop the website go back into the Terminal and press `Ctrl+C`.

## Deploy instructions

NOTE: Currently, only hrydgard does this.

Prerequisites: See Local Testing Instructions.

If upgrading PPSSPP versions, regenerate data/downloads.json using util/dirtree-json.

That will have its own README.md one day..

Use one of the below as appropriate:

`./deploy.sh dev`  (deploys to `dev.ppsspp.org`)
`./deploy.sh prod` (deploys to `www.ppsspp.org`)

## Resources

- [Dark mode guide by Mads Stoumann](https://dev.to/madsstoumann/dark-mode-in-3-lines-of-css-and-other-adventures-1ljj)
- [Source of homepage's background animation by Warren Davies](https://alvarotrigo.com/blog/animated-backgrounds-css/)
- [Unused image for article headers](https://www.zupimages.net/up/22/08/uitq.png)
