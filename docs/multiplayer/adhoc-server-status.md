---
position: 5
---

# Adhoc Server Status

Multiple adhoc servers have some kind of live status page, where you can see what's being played. Future versions of PPSSPP will be able to display this information in the server browser.

There are currently two standards, but PPSSPP will support `data.json` going forward.

## data.json

Examples:

http://psi-hate.com:27315/data.json

## status.xml

Examples:

https://www.socom.cc/status.xml

## Debug status

There's also: `http://server:27314/data_debug`. which shows internal state.

## For server owners

If you want your data.json page to be shown in ppsspp, contact me on `hrydgard+ppsspp@gmail.com` with data.json in the subject, and let me know the full URL that will display the data.json. Ideally this will be `yourhost:27315/data.json` but I'll accept other URLs.

aemu_postoffice already supports :27315/data.json, you just need to expose it on your domain.