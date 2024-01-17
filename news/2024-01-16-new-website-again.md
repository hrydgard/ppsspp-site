---
slug: beta-1.17
title: Android Beta program now open, site update!
authors: hrydgard
tags: [releases]
---
## Beta program

Android has a convenient way for people to opt-in to receive beta releases of apps. I'm finally making use of it, and already lots of people have signed up. Thanks to people installing the beta app, I get crash and hang reports that I can use to make the next release more stable than ever.

[Join the Android PPSSPP Beta program!](/docs/development/beta-testing)

## New website

In other news, I've ripped out Docusaurus and React from the website and rebuilt the whole thing from scratch. I kept getting reports that the site was loading poorly on various devices, and integrations like Analytics had issues with it - plus, every time I wanted to make a change I had to re-learn a lot of stuff.

For long lived projects with small maintenance budgets, simplicity is key. I seem to have to re-learn this concept over and over...

So now, the site is statically generated HTML, all produced with a simple Rust program that stitches together the pages from markdown, Handlebars templates and raw CSS and Javascript. It's easier to manage, easier to change and easier to debug - plus, deploys in 3 seconds instead of 30. Win-win!

This time, I kept most of the visual design (but had to re-implement it, of course), and could re-use the work I previously did to merge the old "PPSSPP Central" with the main site. All the content is the same, too.

## 1.17 coming soon

Yes, version 1.17 is indeed coming soon, with many fixes and improvements - stay tuned.