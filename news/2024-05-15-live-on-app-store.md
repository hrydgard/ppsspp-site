---
slug: live-on-app-store
title: PPSSPP is approved for the App Store!
authors: hrydgard
tags: [releases]
---

## 2024-05-15

After nearly 12 years, PPSSPP has finally been approved for the iOS App Store! Thanks to Apple for relaxing their policies, allowing retro games console emulators on the store.

This initial App Store release of PPSSPP has some limitations compared to the earlier unofficial builds:

* Vulkan support through MoltenVK is not yet enabled
* Magic Keyboard (iPad Keyboard) is not supported
* The JIT recompiler is not supported
* RetroAchievements is temporarily disabled

In future updates, MoltenVK will be re-enabled (and I might even write a native Metal backend), and the Magic Keyboard will be supported through another method (the old method was using an undocumented API, so not usable in the App Store). RetroAchievements
will be back as well, with a better login UI.

However, the JIT compiler cannot be restored without a change in Apple’s rules. The loss of the JIT is unfortunate since without it, our CPU emulation performance is reduced. Fortunately, iOS devices are generally fast enough to run nearly all PSP games at full speed anyway, as the PSP CPU is not that expensive to emulate, thanks to our efficient IR-based caching interpreter, which also has further room for improvement.

Anyway, enjoy this new way of playing PPSSPP! And stay tuned for updates.

As I'm writing this, the app is in status "Processing for distribution". These two links should be active in the next 24h, if they aren't already by the time you are reading this:

[Buy PPSSPP Gold for iOS](https://apps.apple.com/us/app/ppsspp-gold-psp-emulator/id6502287918)

[Download PPSSPP for iOS](https://apps.apple.com/us/app/ppsspp-psp-emulator/id6496972903)
