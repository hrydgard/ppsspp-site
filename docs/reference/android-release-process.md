# The Android release process

## Why do Android users get the updates later?

You may have noticed that within the first week after a new official release, your update still hasn't arrived.

This is because unlike on PC, on Android we are able to do something called a rolling release automatically, thanks to some nifty tools in the Google Play Console, the control panel for developers. First, we release it to 10% of users, then we can check crash reports.

They look like this:

![Screenshot of Play Console]("/static/img/blog/1.17-release/play-console-1.png")

ANR means "Application Not Responding", that is, a hang. These can be tricky to figure out.

In each of these, you can drill down, and if you're lucky there are details like a stack trace - sometimes there is, sometimes not.

Anyway, with these reports, we can easily see if there's some new serious crash or hang affecting a lot of users. If that's the case, we fix the bug and roll out another point release.

Once things look good, we turn the rollout up to a 100%, and you will receive your update within the next day or whenever you check for updates.

## What's different in 1.17 and later?

We started a [beta program](/docs/development/beta-testing), so you can now sign up to be an early tester of new versions. We now have enough that we've been able to fix a whole bunch of bugs before the release.

So hopefully there will be less point releases this time!