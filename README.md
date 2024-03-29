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

## Deploy instructions

NOTE: Currently, only hrydgard does this.

Prerequisites: See Local Testing Instructions.

If upgrading PPSSPP versions, regenerate data/downloads.json using util/dirtree-json.

That will have its own README.md one day..

Use one of the below as appropriate:

`./deploy.sh dev`  (deploys to `dev.ppsspp.org`)
`./deploy.sh prod` (deploys to `www.ppsspp.org`)

## Resources

https://dev.to/madsstoumann/dark-mode-in-3-lines-of-css-and-other-adventures-1ljj
