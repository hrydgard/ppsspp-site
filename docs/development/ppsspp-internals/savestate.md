# Savestate backwards compatibility

PPSSPP uses a chunk-based savestate format, where each chunk has its own versioning.

Thanks to this, it's possible to load a years-old savestate in the latest build, even though many things have changed and more things have been found that need saving.

Each system uses a "DoState" function to do both loading and saving (and also measuring and verifying, but that's for another article). This is called hiearchically to cover everything.

Here's an example chunk, first version:

```cpp

int g_stateVar1;
int g_stateVar2;


void SampleSystem_DoState(PointerWrap &p) {
    auto s = p.Section("SampleSystem", 1, 1);
    if (!s)
        return;
    Do(p, g_stateVar1);
    Do(p, g_stateVar2);
}

```

And when we need to add some additional state in the next version:

```cpp


int g_stateVar1;
int g_stateVar2;
int g_stateVar3;


void SampleSystem_DoState(PointerWrap &p) {
    auto s = p.Section("SampleSystem", 1, 2);
    if (!s)
        return;
    Do(p, g_stateVar1);
    Do(p, g_stateVar2);
    if (s >= 2) {
        Do(p, g_stateVar3);
    }
}

```

Note that the arguments to Section are:

```cpp
Section(const char *title, int minVer, int ver)
```

So you can set minVer if you want to reject savestates that are older than a certain version.

We have almost never used that though, it's usually possible to upgrade information, and it would be bad UX to reject old states.

We've been inconsistent with the numbering, some sections have started at 0, some at 1. This doesn't really matter, though.