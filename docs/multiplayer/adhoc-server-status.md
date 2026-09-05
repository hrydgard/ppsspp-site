---
title: Ad hoc live server status
position: 5
---

# Ad hoc live server status

Multiple ad hoc servers have some kind of live status page, where you can see what's being played. Future versions of PPSSPP will be able to display this information in the server browser.

There are currently two standards, but PPSSPP will support `data.json` going forward.

## data.json

Examples:

https://github.com/Kethen/aemu_postoffice/blob/main/server_cpp/data.json.sample

## status.xml

Examples:

https://www.socom.cc/status.xml


## For server owners

If you want your `data.json` page to be shown in ppsspp, contact me on hrydgard+ppsspp@gmail.com with "data.json" in the subject, and let me know the full URL that will display the `data.json`. Ideally this will be something like `yourhost/data.json` but I'll accept other URLs.

To enable `data.json` on your server, see [here](https://github.com/Kethen/aemu_postoffice/blob/main/hosting.md#customizing-the-http-status-page)
