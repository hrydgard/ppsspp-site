---
position: 2
---

# sceNet, sceWlan and the net modules

The setup every networking game does before anything else: loading the modules, initializing sceNet,
and the console's own MAC address. Also the netconf dialog.

## sceUtilityLoadNetModule

```c
int sceUtilityLoadNetModule(int module);   // NID 0x1579A159
int sceUtilityUnloadNetModule(int module); // NID 0x64D50C56
```

| `module` | Loads |
|---|---|
| 1 (`PSP_NET_MODULE_COMMON`) | sceNet and the interface layer; load first |
| 2 (`PSP_NET_MODULE_ADHOC`) | sceNetAdhoc, sceNetAdhocctl, sceNetAdhocMatching (sceNetAdhocDiscover is available too) |
| 3 (`PSP_NET_MODULE_INET`) | sceNetInet, sceNetApctl, sceNetResolver |
| 4-7 | parse URI, parse HTTP, HTTP, SSL |

Returns 0 on success. Unload in the reverse order of loading, after terminating the libraries.

## sceNetInit

```c
int sceNetInit(int poolSize, int calloutPrio, int calloutStack, int netintrPrio, int netintrStack); // NID 0x39AF39A6
```

Creates the memory pool all networking allocates from, and two internal threads with the given
priorities and stack sizes. Games typically pass a pool of 0x20000 and priorities around 0x20.

**PPSSPP:** checks the pool size and that both priorities are within 0x08-0x77, but creates no threads.
Calling it again while initialized terminates first.

## sceNetTerm

```c
int sceNetTerm(void); // NID 0x281928A9
```

Frees the pool. Terminate the ad hoc and inet libraries first. (PPSSPP terminates them itself, since
some games don't.)

## sceNetGetLocalEtherAddr

```c
int sceNetGetLocalEtherAddr(SceNetEtherAddr *mac); // NID 0x0BF0A3AE
```

The console's own MAC address, the one used as the local address for every ad hoc socket.

**Fails with 0x80410180 until a scan or group operation has started**, even with adhocctl initialized.
Read the address from `sceWlanGetEtherAddr` before that.

## sceWlanGetEtherAddr

```c
int sceWlanGetEtherAddr(SceNetEtherAddr *mac); // NID 0x0C622081, library sceWlanDrv
```

The Wi-Fi chip's MAC address, available at any time (also with the WLAN switch off). It's the same
address `sceNetGetLocalEtherAddr` returns once that works.

**PPSSPP:** the MAC address setting, or for instance *N* > 1 on one machine, *N* repeated six times with
the low two bits of the first byte cleared.

## sceWlanGetSwitchState

```c
int sceWlanGetSwitchState(void); // NID 0xD7763699, library sceWlanDrv
```

1 if the WLAN switch is on, 0 if off. Ad hoc needs it on; what each call does with it off hasn't been
tested.

## sceWlanDevIsPowerOn

```c
int sceWlanDevIsPowerOn(void); // NID 0x93440B11, library sceWlanDrv
```

1 while the Wi-Fi chip is powered (it was, with the switch on and the net modules loaded).

## sceNetEtherNtostr / sceNetEtherStrton

```c
void sceNetEtherNtostr(const SceNetEtherAddr *mac, char *str); // NID 0x89360950
void sceNetEtherStrton(const char *str, SceNetEtherAddr *mac); // NID 0xD27961C9
```

`Ntostr` writes lower-case hex with colons (`00:1a:2b:c3:d4:ff`) and a terminating zero. `Strton` reads
six two-digit hex bytes, either case, with any single separator (`-` works as well as `:`). It doesn't
validate: one-digit bytes, a short string or non-hex characters give garbage bytes, not an error.

**PPSSPP:** malformed strings give different garbage (it stops at the end of the string, leaving the rest
of the address alone).

## sceNetGetMallocStat

```c
typedef struct {
	u32 pool;    // the pool size given to sceNetInit, less 32 bytes
	u32 maximum; // the most ever in use
	u32 free;
} SceNetMallocStat;

int sceNetGetMallocStat(SceNetMallocStat *stat); // NID 0xCC393E48
```

Returns 0; NULL gives 0x80410103. With `sceNetInit(0x20000, ...)`, `pool` is 0x1FFE0.

**PPSSPP:** NULL returns 0.

## sceNetThreadAbort / sceNetFreeThreadinfo

```c
int sceNetThreadAbort(int thid);     // NID 0xAD6844C6
int sceNetFreeThreadinfo(int thid);  // NID 0x50647530
```

`ThreadAbort` makes the networking call the thread is blocked in return at once, with EINTR for sceNetInet
calls (a blocked `sceNetInetRecv` was tested), like `sceNetInetSocketAbort` does for one socket. What
`FreeThreadinfo` frees wasn't tested. Both return 0 for any existing thread (also the current one, or one
that has ended), and 0x80410104 for 0 or an ID that isn't a thread.

**PPSSPP:** both return 0 for anything, and `ThreadAbort` doesn't wake the thread.

## The netconf dialog

```c
int sceUtilityNetconfInitStart(SceUtilityNetconfParam *param); // NID 0x4DB1E739
int sceUtilityNetconfUpdate(int animSpeed);                     // NID 0x91E70E35
int sceUtilityNetconfGetStatus(void);                            // NID 0x6332AA39
int sceUtilityNetconfShutdownStart(void);                        // NID 0xF88155F6
```

The system's "Connecting..." screen. The game fills in a parameter struct, calls `InitStart`, then calls
`Update` every frame (after drawing its own frame) until `GetStatus` returns 3 (FINISHED), then
`ShutdownStart`, and waits for `GetStatus` to return 0 again before calling anything else in the
utility library.

| `action` | For ad hoc |
|---|---|
| 2 | CONNECT_ADHOC: join the group if it exists, otherwise create it |
| 4 | CREATE_ADHOC: create the group |
| 5 | JOIN_ADHOC: scan for the group and join it |

For these, the parameter struct points to an ad hoc parameter `{ char group[8]; u32 timeout; }`. The
dialog calls adhocctl itself, so adhocctl has to be initialized first, and when it finishes, the `result`
field of the parameters says whether it succeeded. (This is how games use it; it hasn't been checked
against hardware in detail.)

The status values are 0 NONE, 1 INITIALIZE, 2 RUNNING, 3 FINISHED, 4 SHUTDOWN.

The dialog isn't covered by the pspautotests: a dialog left running wedges PSPLink, so it needs a
carefully bounded test.
