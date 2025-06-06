---
slug: stuck-in-app-store-review
title: Talking to an App Store Review brick wall
authors: hrydgard
tags: [ppsspp]
---

# UPDATE!!!

The separate Gold app has now been retired, and replaced with an in-app purchase. [More information here](/news/ios-changes).

# Previous article below

## The App Store situation

Here's an overview, followed by the actual conversation with App Store Review.

PPSSPP is an open source PSP emulator, that lets you run your own [PlayStation Portable](https://en.wikipedia.org/wiki/PlayStation_Portable) games on your various devices. PPSSPP is officially available on Android through [Google Play](https://play.google.com/store/apps/details?id=org.ppsspp.ppsspp), [PC](/downloads), [Mac](/downloads), and recently iOS through the [App Store](https://apps.apple.com/us/app/ppsspp-psp-emulator/id6496972903). There is also a [Linux flatpak build](https://flathub.org/apps/org.ppsspp.PPSSPP). The project is ongoing for more than 11 years now, and has been downloaded over 100M times. It has millions of active users on Android.

PPSSPP is completely free to download (and compile if you want, since it's open source) but there's also an optional paid version to finance the development and maintenance of the project, buildbot, website, etc.

The paid "Gold" version is, except cosmetically, identical to the free version. I've chosen this monetization model on the App Store to keep it consistent with the other platforms, where it's often the most practical one.

In May this year, Apple changed its policies and started allowing game console emulators on the App Store. The change was of course followed by a bunch of emulator releases. Here are a few examples:

* [Delta](https://apps.apple.com/us/app/delta-game-emulator/id1048524688) ([Website](https://faq.deltaemulator.com/)) - Nintendo consoles (NES, SNES, GB, GBC, GBA, N64, DS). Open source.
* [RetroArch](https://apps.apple.com/us/app/retroarch/id6499539433) ([Website](https://www.retroarch.com/)) - Multisystem emulator supporting a large amount of game consoles. Open source.
* [BigPEmu](https://apps.apple.com/us/app/bigpemu/id6737359949) ([Website](https://www.richwhitehouse.com/jaguar/)) - Atari Jaguar emulator. Closed source.
* [Folium](https://apps.apple.com/us/app/folium/id6498623389) ([Website](https://folium.emuplace.app/)) - multisystem emulator supporting various portable Nintendo consoles. Open source.
* [ArcEmu](https://apps.apple.com/us/app/arcemu-watch-emulator/id6496282733) - Nintendo GB/GBC/GBA emulator for Apple Watch.

For some time now, I have simply not been able to update the paid iOS version on Apple's App Store. The free version flies through review in a few hours, while the near-identical paid version is just stuck.

Below is an authentic conversation with App Store Review. This is the second conversation, I previously had a much longer one, but it disappeared when I submitted a new build. The arguments were the same, just more rounds of back-and-forth.

<hr/>

## Apple App Store Rejection, initial message

<i>
Submission ID: 7fe4cdbf-80a6-43ba-a3d4-e54b73b14267

Review date: November 04, 2024

Version reviewed: 1.18

### Guideline 4.1 - Design - Copycats

Your app's metadata contains content that is similar to third-party content, which may create a misleading association with another developer's app or intellectual property.

We understand that the mentioning of the console is to provide more context to the users, including trademarked terms such as "PSP" in a non-referential way is still not appropriate.


#### Next Steps

It would be appropriate to revise the app and metadata to remove this third-party content before resubmitting for review.

If you have the necessary rights to distribute an app with this third-party content, attach documentary evidence in the App Review Information section in App Store Connect and reply to this message.


#### Resources

Many factors may contribute to a guideline 4.1 rejection, including but not limited to the following examples:

- Using the app metadata or developer account information to create a misleading association with another app.

- Including irrelevant references to popular apps in an app's name or subtitle.

- Copying the content, features, and user interface of popular apps.


Learn more about [guideline 4.1](https://developer.apple.com/app-store/review/guidelines/#copycats).


### Guideline 4.3(a) - Design - Spam

We noticed your app shares a similar binary, metadata, and/or concept as apps submitted to the App Store by other developers, with only minor differences.


Submitting similar or repackaged apps is a form of spam that creates clutter and makes it difficult for users to discover new apps.


#### Next Steps

Since we do not accept spam apps on the App Store, we encourage you to review your app concept and submit a unique app with distinct content and functionality.


#### Resources

Some factors that contribute to a spam rejection may include:

- Submitting an app with the same source code or assets as other apps already submitted to the App Store

- Creating and submitting multiple similar apps using a repackaged app template

- Purchasing an app template with problematic code from a third party

- Submitting several similar apps across multiple accounts

Learn more about our requirements to prevent spam in [App Review Guideline 4.3(a)](https://developer.apple.com/app-store/review/guidelines/#spam).


### Guideline 5.2.2 - Legal

The app appears to contain copyrighted video game files,

Apps and their content should not infringe upon the rights of another party. In the event an app infringes another party’s rights, the app's developers are responsible for any liability to Apple because of a claim.


#### Next Steps

Either remove the copyrighted third-party content from the app and its metadata or provide a written affirmation that you have the appropriate rights or license to use and distribute the third-party copyrighted materials.


#### Resources

Learn more about intellectual property requirements in [guideline 5.2.2](https://developer.apple.com/app-store/review/guidelines/#intellectual-property).


### Support

- Reply to this message in your preferred language if you need assistance. If you need additional support, use the [Contact Us](https://developer.apple.com/contact/topic/#!/topic/select) module.

- Consult with fellow developers and Apple engineers on the [Apple Developer Forums](https://developer.apple.com/forums/).

- Provide feedback on this message and your review experience by [completing a short survey](https://essentials.applesurveys.com/jfe/form/SV_esVePfih7uqt4NM?taskid=2ce1ec41-879a-48ff-8f12-4018e7b106eb&campaignid=0001).

</i>
<hr/>

## My initial response

Please read! If you are not a robot, please indicate that you read the below, and respond to it properly.

Five minutes after this rejection was made, my near-identical free version of this very same app ("PPSSPP - PSP Emulator"), on the same account, with the same metadata, and same files, and essentially the same functionality, was APPROVED by App Review. Why not this one? It's 99.9% the same app, on the same account.

OK, so once again, I will refute your complaints point by point.

#### 4.1 - Design - Copycats

Any games console emulator obviously needs to be able to mention what system it emulates. Anything else is unreasonable. Other emulators like Folium do this too.

#### 4.3(a) - Design - Spam

I am the original author of this app. It is not spam, it's just the paid version of PPSSPP - PSP Emulator.

#### Guideline 5.2.2 - Legal

"The app appears to contain copyrighted video game files"

There are no such files. If there's a file you have a complaint about. PLEASE LET ME KNOW EXACTLY WHICH ONE. The filename please.

<hr/>

## Apple App Store Reply

<i>

Hello,

Thank you for information.

However, to be in compliance with guideline 5.2.2, the app can open video games files without the appropriate authorisation from the right holder. it would still be appropriate to either remove the copyrighted third-party content from the app and its metadata or provide a written affirmation that you have the appropriate rights or license to use and distribute the third-party copyrighted materials.

To be in compliance with guideline 4.1, the app uses copyrighted terminology in the app name (PSP) in a non - referential manner, it would still be appropriate to revise the app and metadata to remove this third-party content before resubmitting for review.

To be in compliance with guideline 4.3, it would still be appropriate to review your app concept and submit a unique app with distinct content and functionality.

We look forward to reviewing your resubmission.

Best regards,

App Review

</i>

<hr/>

## My second response

Hi again..

First, can you tell me what changed? This app was APPROVED by App Review a few months ago, but suddenly App Review started rejecting updates - but only for this paid version of the app, the free version still gets every update approved. I am simply trying to submit a bugfix update, no substantial change has been made to what the app does. Additionally, you APPROVED the free version of the app ("PPSSPP - PSP emulator"), on this very same developer account, just two days ago. It has all the same functionality.

Second, I believe that you have an automated system that has somehow started triggering on just this app. Is this the case? The three points are simply wrong. Again, point by point:

As for 5.2.2, there are, again, no copyrighted game files being shipped with the app. As for playing unauthorized files, just like an MP3 music player will play files that the user owns, this app will play retro PSP games that the user owns. This app is not in any way a violation of 5.2.2. Apple started allowing game console emulators with this type of functionality earlier this year, some other examples that are live on the App Store are Folium and RetroArch, and of course PPSSPP, the free version.

As for 4.1, for a user to understand what retro game console the app emulates, we really have to mention the name PSP. This is not any kind of copyright violation, and the free version of the app does this as well and was approved.

As for 4.3, it's really quite insulting to be told to "review my app concept".
This app is serious software and is one of the most popular retro game console emulators in the world, with hundreds of millions of downloads on other platforms, and four million downloads of the free version on iOS . It has a great reputation in emulation circles. Here's the wikipedia page, which confirms that I, Henrik Rydgård, am the original author: https://en.wikipedia.org/wiki/PPSSPP

So please, consider the facts above.

<hr/>

## Apple App Store Reply #2

<i>

Thank you for your reply.

Please note that we are unable to share with you the review process or other information regarding other apps.

All apps, including updates, undergo a complete review to ensure compliance with the most current version of the App Review Guidelines.

To be in compliance with guideline 5.2.2, the app can open video games files without the appropriate authorisation from the right holder. it would still be appropriate to either remove the copyrighted third-party content from the app and its metadata or provide a written affirmation that you have the appropriate rights or license to use and distribute the third-party copyrighted materials.

To be in compliance with guideline 4.1, the app uses copyrighted terminology in the app name( PSP) in a non - referential manner, it would still be appropriate to revise the app and metadata to remove this third-party content before resubmitting for review.

To be in compliance with guideline 4.3, it would still be appropriate to review your app concept and submit a unique app with distinct content and functionality.

Let us know if you have any further questions.

Best regards,

App Review

</i>

<hr/>

## Conclusion

There seems to be no progress possible, despite Apple's complaints being entirely invalid:

* The essentially-identical free version of PPSSPP flies through review every time, while the paid version is stuck. Why aren't the two treated the same?
* Most of the other approved emulators also mention the names of the consoles they emulate in the App Store description - otherwise it's kind of hard for users to know what to download.
* There are no copyrighted game files included. Emulators playing user-provided games are allowed on the App Store now.
* It's the original app, not a cheap re-upload of someone else's content.

I tried appealing the previous conversation to the App Store review board, with no result.

It's just so frustrating. I want to get a bugfix update out, and I can't.

If you are an Apple employee and have any way to help, or any information or tips that might be helpful to get past this roadblock, contact me at hrydgard+ppssppgold@gmail.com.

At this point, I'm starting to think that the best way forward might be ditching the separate Gold app and switching to in-app purchase, though there are some practical issues with that.
