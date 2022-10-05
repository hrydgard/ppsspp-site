# PPSSPP Website

This is the entire frontend for the official ppsspp.org website.

It's built using [Docusaurus 2](https://docusaurus.io/), a modern static website generator using React,
with some rather heavy customizations here and there to add a login system.

## Backend and authentication

The backend is not (yet?) open source.

Authentication is done using httponly cookies, which means that client side javascript can't see them.
Thus, we also keep a record of the current login state in localstorage. Could also use a client-visible cookie
but localstorage was just easier.

In root.js, we wrap everything in `<BrowserOnly>` so that we can have dynamic content. Ideally, we'd only use that in the navbar (for the login indicator) and on the select few pages where it matters - it's a future project to try to get to such a state.

## Local testing instructions

Prerequisites:

* On Windows, you probably should use a WSL shell.

* Node must be installed: `sudo apt npm install`
  If you are using an ubuntu WSL, install npm from here: https://github.com/nodesource/distributions/blob/master/README.md
  Also it must be a Ubuntu 22, so if you have an older one, uninstall and reinstall your ubuntu WSL.

Local testing:

`./switch.sh local`
`npm update`
`npm run start`  (not serve! that's for serving the build directory, after `npm build`)

That will launch the site on localhost:3000.

## Deploy instructions

NOTE: Currently, only hrydgard does this.

Prerequisites: See Local Testing Instructions.

If upgrading PPSSPP versions, regenerate static/downloads.json using util/dirtree-json.
That will have its own README.md.

Use one of the below as appropriate:

`./deploy.sh dev`  (deploys to dev.ppsspp.org)
`./deploy.sh prod` (deploys to www.ppsspp.org)