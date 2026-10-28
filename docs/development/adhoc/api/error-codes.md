---
position: 9
---

# Error codes

All the error codes the networking libraries return, with when they were seen on a PSP.

## sceNet (0x804101xx)

| Code | Name | Seen |
|---|---|---|
| 0x80410001 | NO_SPACE (out of memory) | a 65523-byte PDP send, now and then |
| 0x80410103 | (invalid argument) | `sceNetGetMallocStat(NULL)` |
| 0x80410104 | (no such thread) | `sceNetThreadAbort` and `sceNetFreeThreadinfo` with 0 or a bad ID |
| 0x80410180 | (no ether address yet) | `sceNetGetLocalEtherAddr` before a scan or group operation |

## sceNetAdhoc (0x804107xx)

| Code | Name | Seen |
|---|---|---|
| 0x80410701 | INVALID_SOCKET_ID | any call with an ID that isn't (or is no longer) a socket of the right kind |
| 0x80410702 | INVALID_ADDR | a MAC that isn't the console's own as the local address; a zero or multicast destination; sockets before the console is in a group |
| 0x80410703 | INVALID_PORT | destination port 0 or 65535; local port 65535 |
| 0x80410704 | INVALID_BUFLEN | `sceNetAdhocctlGetPeerInfo` with a buffer smaller than the struct |
| 0x80410705 | INVALID_DATALEN | length over 65523, negative, or non-zero with NULL data; game mode buffer of size 0 |
| **0x80400706** | NOT_ENOUGH_SPACE | `PdpRecv` with a buffer smaller than the datagram |
| 0x80410707 | SOCKET_DELETED | a thread blocked on a socket that's deleted |
| 0x80410708 | SOCKET_ALERTED | a call matching an alert set on the socket |
| 0x80410709 | WOULD_BLOCK | non-blocking call that can't complete |
| 0x8041070A | PORT_IN_USE | `PdpCreate` on a port in use |
| 0x8041070B | NOT_CONNECTED | PTP send or receive on a listener |
| 0x8041070C | DISCONNECTED | PTP send or receive after the peer has gone |
| **0x8040070D** | NOT_OPENED | |
| **0x8040070E** | NOT_LISTENED | `PtpAccept` on a socket that isn't listening |
| 0x8041070F | SOCKET_ID_NOT_AVAIL | creating a 256th socket |
| 0x80410710 | PORT_NOT_AVAIL | |
| 0x80410711 | INVALID_ARG | NULL pointers, bad flags, bad counts |
| 0x80410712 | NOT_INITIALIZED | before `sceNetAdhocInit` or after `Term` |
| 0x80410713 | ALREADY_INITIALIZED | `sceNetAdhocInit` twice |
| 0x80410714 | BUSY | |
| 0x80410715 | TIMEOUT | a blocking call that ran out of time |
| 0x80410716 | NO_ENTRY | `GetPeerInfo`/`GetNameByAddr` for a MAC that isn't a peer, including the console's own |
| 0x80410717 | EXCEPTION_EVENT | `PollSocket` with a disconnected socket or a poll alert |
| 0x80410718 | CONNECTION_REFUSED | PTP connect to a port nobody listens on |
| 0x80410719 | THREAD_ABORTED | |
| 0x8041071A | ALREADY_CREATED | game mode master or replica created twice |
| 0x8041071B | NOT_IN_GAMEMODE | game mode data calls outside game mode |
| 0x8041071C | NOT_CREATED | deleting a game mode master or replica that doesn't exist; a bad replica ID |

The three in bold really are `0x8040...`.

## sceNetAdhocAuth (0x804106xx)

| Code | Name | Seen |
|---|---|---|
| 0x80410601 | ALREADY_INITIALIZED | `sceNetAdhocctlInit` after a failed second Init (see that function) |

## sceNetAdhocctl (0x80410Bxx)

| Code | Name | Seen |
|---|---|---|
| 0x80410B01 | NOT_LEFT_IBSS | |
| 0x80410B02 | ALREADY_CONNECTED | handler ERROR event after Create, Connect, Scan or game mode while connected |
| 0x80410B03 | WLAN_SWITCH_OFF | |
| 0x80410B04 | INVALID_ARG | bad group name, NULL ad hoc ID, type above 2, NULL handler, bad game mode arguments |
| 0x80410B05 | TIMEOUT | handler ERROR event when game mode members don't turn up in time |
| 0x80410B06 | ID_NOT_FOUND | `DelHandler` with an unknown ID |
| 0x80410B07 | ALREADY_INITIALIZED | `sceNetAdhocctlInit` twice |
| 0x80410B08 | NOT_INITIALIZED | before Init or after Term |
| 0x80410B09 | DISCONNECTED | `GetParameter` while not in a group |
| 0x80410B0A | NO_SCAN_INFO | |
| 0x80410B0B | INVALID_IBSS | `ExitGameMode` while in a normal group |
| 0x80410B0C | NOT_ENTER_GAMEMODE | `GetGameModeInfo` outside game mode |
| 0x80410B0D | CHANNEL_NOT_AVAILABLE | |
| 0x80410B0E | WLAN_BEACON_LOST | |
| 0x80410B0F | WLAN_SUSPENDED | |
| 0x80410B10 | BUSY | a request (Create, Connect, Scan) before the previous one's event; while game mode is waiting forever for members |
| 0x80410B11 | CHANNEL_NOT_MATCH | |
| 0x80410B12 | TOO_MANY_HANDLERS | a fifth handler |
| 0x80410B13 | STACKSIZE_TOO_SHORT | `sceNetAdhocctlInit` with a stack below 0xC00 |
| 0x80410682 | (none) | `GetPeerList` while not in a group |

## sceNetAdhocMatching (0x804108xx)

| Code | Name | Seen |
|---|---|---|
| 0x80410801 | INVALID_MODE | mode other than 1-3 |
| 0x80410802 | INVALID_PORT | |
| 0x80410803 | INVALID_MAXNUM | maxnum outside 2-16 |
| 0x80410804 | RXBUF_TOO_SHORT | rxbuflen below `6 * maxnum + 4` |
| 0x80410805 | INVALID_OPTLEN | |
| 0x80410806 | INVALID_ARG | zero intervals in host/P2P mode, negative init count, zero rexmt interval, NULL pool stat |
| 0x80410807 | INVALID_ID | unknown context ID |
| 0x80410808 | ID_NOT_AVAIL | a 16th context |
| 0x80410809 | NO_SPACE | |
| 0x8041080A | IS_RUNNING | `Start` or `Delete` on a running context |
| 0x8041080B | NOT_RUNNING | target, data and hello calls before `Start` |
| 0x8041080C | UNKNOWN_TARGET | a MAC that isn't a matching peer |
| 0x8041080D | TARGET_NOT_READY | `SelectTarget` on a request the client already cancelled |
| 0x8041080E | EXCEED_MAXNUM | |
| 0x8041080F | REQUEST_IN_PROGRESS | `SelectTarget` twice for the same peer |
| 0x80410810 | ALREADY_ESTABLISHED | |
| 0x80410811 | BUSY | |
| 0x80410812 | ALREADY_INITIALIZED | `Init` twice |
| 0x80410813 | NOT_INITIALIZED | `Create` before `Init` |
| 0x80410814 | PORT_IN_USE | two contexts on one port |
| 0x80410815 | STACKSIZE_TOO_SHORT | |
| 0x80410816 | INVALID_DATALEN | |
| 0x80410817 | NOT_ESTABLISHED | |
| 0x80410818 | DATA_BUSY | `SendData` before the previous send to that peer was acknowledged |

## sceNetAdhocDiscover (0x804113xx)

| Code | Seen |
|---|---|
| 0x80411301 | `InitStart` while discover is running |
| 0x80411303 | `RequestSuspend` while discovering with nobody around |

## The Wi-Fi driver (0x80410Dxx)

These only arrive through handler events.

| Code | Seen |
|---|---|
| 0x80410D01 | adhocctl's ERROR event when creating a group while apctl is connected to an access point |
| 0x80410D09 | apctl's ERROR event when joining an access point fails (on one console, while adhocctl was merely initialized) |
| 0x80410D13 | `sceNetAdhocGameModeCreateReplica` for a MAC that isn't a game mode member |

## sceNetApctl (0x80410Axx)

| Code | Name |
|---|---|
| 0x80410A01 | ALREADY_INITIALIZED |
| 0x80410A02 | INVALID_CODE (seen: `GetInfo` with code 19) |
| 0x80410A03 | INVALID_IP |
| 0x80410A04 | NOT_DISCONNECTED |
| 0x80410A05 | NOT_IN_BSS |
| 0x80410A06 | WLAN_SWITCH_OFF |
| 0x80410A07 | WLAN_BEACON_LOST |
| 0x80410A08 | WLAN_DISASSOCIATION |
| 0x80410A09 | INVALID_ID |
| 0x80410A0A | WLAN_SUSPENDED |
| 0x80410A0B | TIMEOUT |
| 0x80410A0D | (seen) apctl not initialized, from `GetState` |
| 0x80410A0F | (seen) too many handlers (the limit is 2) |
| 0x80410A80 | (seen) busy: `Connect` or `ScanUser` while one is in progress |
| 0x80410A82 | (seen) an ID that was never valid, in `DelHandler` and `GetBSSDescEntryUser` |
| 0x80110601 | (netparam) BAD_NETCONF: `sceNetApctlConnect` with no profile in that slot, or a slot out of range |
| 0x80110602 | (netparam, seen) `sceNetApctlConnect(-1)` |

## sceNetResolver (0x804104xx)

| Code | Name |
|---|---|
| 0x80410401 | NOT_TERMINATED |
| 0x80410402 | NO_DNS_SERVER |
| 0x80410403 | INVALID_PTR |
| 0x80410404 | INVALID_BUFLEN |
| 0x80410405 | INVALID_ID |
| 0x80410406 | ID_MAX |
| 0x80410407 | NO_MEM |
| 0x80410408 | BAD_ID |
| 0x80410409 | CTX_BUSY |
| 0x8041040A | ALREADY_STOPPED |
| 0x8041040B | NOT_SUPPORTED |
| 0x8041040C | BUF_NO_SPACE |
| 0x8041040D | INVALID_PACKET |
| 0x8041040E | STOPPED |
| 0x8041040F | SOCKET |
| 0x80410410 | TIMEOUT |
| 0x80410411 | NO_RECORD |
| 0x80410412 | RES_PACKET_FORMAT |
| 0x80410413 | RES_SERVER_FAILURE |
| 0x80410414 | INVALID_HOST (seen: a name that doesn't exist) |
| 0x80410418 | (seen) an empty host name |
| 0x80410419 | (seen) `PollAsync` with NULL |
| 0x8041041A | (seen) `PollAsync`/`WaitAsync` with no lookup pending |
| 0x80000020 | (seen) `sceNetResolverInit` twice |

## sceNetInet errno values

sceNetInet functions return -1 and set an errno (read with `sceNetInetGetErrno`). The numbers follow
newlib, not Linux or BSD:

| errno | Name | Seen |
|---|---|---|
| 4 | EINTR | the call was cut short by `sceNetInetSocketAbort` |
| 9 | EBADF | invalid or closed descriptor |
| 11 | EAGAIN | non-blocking call that can't complete; `MSG_WAITALL` timing out |
| 22 | EINVAL | bad lengths; bind twice; listen on an unbound socket; accept on a non-listener; connect after a refusal |
| 24 | EMFILE | a 256th socket |
| 28 | ENOSPC | `InetNtop` buffer too small |
| 32 | EPIPE | send after the peer closed, or after `shutdown(SHUT_WR)` |
| 95 | EOPNOTSUPP | listen or accept on UDP |
| 104 | ECONNRESET | receive after the peer reset the connection |
| 105 | ENOBUFS | a socket buffer over 1 MB |
| 106 | EAFNOSUPPORT | `InetPton`/`InetNtop` with a family other than AF_INET |
| 109 | ENOPROTOOPT | unknown or read-only socket options |
| 111 | ECONNREFUSED | connect to a port nobody listens on; receive on a connected UDP socket after an ICMP port unreachable |
| 112 | EADDRINUSE | bind to an address in use |
| 118 | EHOSTUNREACH | broadcast with no network interface up |
| 119 | EINPROGRESS | non-blocking connect |
| 121 | EDESTADDRREQ | send on an unconnected UDP socket |
| 122 | EMSGSIZE | UDP datagram larger than SO_SNDBUF |
| 123 | EPROTONOSUPPORT | bad domain, type or protocol in `socket` |
| 125 | EADDRNOTAVAIL | bind to an address the console doesn't have, or the wrong family |
| 127 | EISCONN | connect again; sendto another address on connected UDP |
| 128 | ENOTCONN | send, receive or getpeername on an unconnected socket |
