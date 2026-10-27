---
position: 6
---

# sceNetAdhocDiscover

"Discover" mode, used by a few games to find another console without the game itself scanning. It runs as
a state machine the game steps every frame, like a utility dialog. Test: `net/adhoc/discover` (one
console, nobody else around, so a successful discovery hasn't been observed).

```c
typedef struct {
	u32 unknown1;
	char group[8];
	u32 unknown2;
	u32 result;   // written when it finishes: 1 = cancelled
} SceNetAdhocDiscoverParam;

int sceNetAdhocDiscoverInitStart(SceNetAdhocDiscoverParam *param); // NID 0x941B3877
int sceNetAdhocDiscoverUpdate(void);                                // NID 0x52DE1B97
int sceNetAdhocDiscoverGetStatus(void);                             // NID 0x944DDBC6
int sceNetAdhocDiscoverStop(void);                                  // NID 0xF7D13214
int sceNetAdhocDiscoverTerm(void);                                  // NID 0xA2246614
int sceNetAdhocDiscoverRequestSuspend(void);                        // NID 0xA423A21B
```

| Status | |
|---|---|
| 0 | NONE |
| 1 | IN_PROGRESS |
| 2 | COMPLETED |

The library is available after loading the ADHOC net module. The test ran it with adhocctl initialized
and not in a group.

- Before `InitStart`, `GetStatus`, `Update`, `Stop` and `Term` all return 0.
- **`InitStart(NULL)` hangs the console.**
- `InitStart` returns 0 and the status becomes IN_PROGRESS; shortly after, adhocctl's state becomes 4
  (DISCOVER). A second `InitStart` while running gives 0x80411301.
- With nobody to find, it stays IN_PROGRESS (at least 10 s), with `Update` called every frame returning 0.
- `Stop`: adhocctl goes back to DISCONNECTED, the status becomes COMPLETED, `result` becomes 1
  (cancelled), and adhocctl's handler gets a DISCONNECT event.
- `RequestSuspend` while discovering alone gives 0x80411303.
- `Term` returns 0, after which the status is 0.

**PPSSPP:** a stub that never finds anyone and never enters adhocctl state 4; `InitStart` while running
and `RequestSuspend` return 0, and `RequestSuspend` ends it as cancelled.
