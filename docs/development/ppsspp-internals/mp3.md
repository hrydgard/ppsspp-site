# sceMp3

sceMp3 acts as an MP3 parser and wrapper around sceAudiocodec's MP3 decoding ability, adding lots of utility functions and helpers to make streaming from disk easy.

Here's a typical calling sequence (from Wipeout Pulse):

```
0=sceMp3InitResource()
startPos 14477 endPos a65e27 mp3buf 08af5580 mp3bufSize 00002000 PCMbuf 08af7580 PCMbufSize 00002400
00000000=sceMp3ReserveMp3Handle(09f94450)
0=sceMp3GetInfoToAddStreamData(00000000, 09f94430[08af5b40], 09f94434[00001a40], 09f94438[00014477])
0=sceMp3NotifyAddStreamData(00000000, 6720)
sceMp3Init(): channels=2, samplerate=32000Hz, bitrate=48kbps, layerBits=1 ,versionBits=3,HEADER: fffb3800
0=sceMp3Init(00000000)
-1=sceMp3GetLoopNum(00000000)
4608=sceMp3Decode(00000000, 09f94470[08af7580])
0=sceMp3SetLoopNum(00000000, 1)   << called from main thread for some reason?
1=sceMp3CheckStreamDataNeeded(00000000)
0=sceMp3GetInfoToAddStreamData(00000000, 09f94440[08af5b40], 09f94444[000000d8], 09f94448[00015eb7])
0=sceMp3NotifyAddStreamData(00000000, 216)
4608=sceMp3Decode(00000000, 09f94470[08af8780])
1=sceMp3CheckStreamDataNeeded(00000000)
0=sceMp3GetInfoToAddStreamData(00000000, 09f94440[08af5b40], 09f94444[00000360], 09f94448[00015f8f])
0=sceMp3NotifyAddStreamData(00000000, 864)
1=sceMp3GetLoopNum(00000000)
4608=sceMp3Decode(00000000, 09f94470[08af7580])
1=sceMp3CheckStreamDataNeeded(00000000)
0=sceMp3GetInfoToAddStreamData(00000000, 09f94440[08af5b40], 09f94444[000003f0], 09f94448[000162ef])
0=sceMp3NotifyAddStreamData(00000000, 1008)
4608=sceMp3Decode(00000000, 09f94470[08af8780])
1=sceMp3CheckStreamDataNeeded(00000000)
0=sceMp3GetInfoToAddStreamData(00000000, 09f94440[08af5b40], 09f94444[00000480], 09f94448[000166df])
0=sceMp3NotifyAddStreamData(00000000, 1152)
1=sceMp3GetLoopNum(00000000)
4608=sceMp3Decode(00000000, 09f94470[08af7580])
1=sceMp3CheckStreamDataNeeded(00000000)
0=sceMp3GetInfoToAddStreamData(00000000, 09f94440[08af5b40], 09f94444[000003f0], 09f94448[00016b5f])
0=sceMp3NotifyAddStreamData(00000000, 1008)
```

Basically, we ask what data to read from the file and where to write it, perform the I/O, then tell
sceMp3 that we're done. We call sceMp3Init after the first read (!).

Then we can call Decode, and then keep asking whether data is needed.

We now have a hardware test that shows how it works: [link](https://github.com/hrydgard/pspautotests/blob/master/tests/audio/mp3/stream.cpp)