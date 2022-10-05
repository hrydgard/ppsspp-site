# How to use the Rewind feature

## Enabling the Rewind feature

You may have noticed that in the *Settings > Controls > “Control mapping”* screen, one of the actions to map a key or button to is called “*Rewind*”. It’s between *Pause* and *Save State* on the list, and it is set to `kbd.Backspace` by default. And if you press the key bound to this action while playing a game, the emulator will just give an error: “No rewind saves states available.” How can you get this feature to work?

The answer is to go to *Settings > System* and look in the *Emulation* section. There is an option “*Rewind snapshot frequency (mem hog)*”. By default it is set to *Off* (0), which is why the Rewind key does nothing. Changing the option to something other than 0 will enable the rewind feature, which periodically makes save states that you can rewind to.

## Setting the “Rewind snapshot frequency”

What does the value of this option mean? As its editing interface shows, you can set any number from 0 to 1800 frames, and 0 means the feature is off. PSP games usually run at either 30 or 60 frames per second (you can tell by going to *Settings > Graphics > “Overlay information” section > “Show FPS counter”* and setting it to *FPS*). So if you set the option to 150 frames and then play a 30-FPS game, a save state will automatically be saved every 5 seconds.

## Using the Rewind key

If you make a mistake while playing, you can press the Rewind key, and the emulator will load the last automatic save state it made. In the example above, this save state could have been made anywhere from 0 to 5 seconds ago. After pressing the key, you can try that section of the game again. If your mistake was further back than one rewind frequency, you can press Rewind repeatedly – each quick press takes you back to the previous automatic save state.

## Memory usage

The “(mem hog)” part of the option name is telling you that enabling this feature causes PPSSPP to use up a lot more memory than normal. For example, on my computer, PPSSPP goes from 100MB to 800MB of memory used. So set the rewind snapshot frequency back to 0 when you’re not using the rewind feature. And note that you will have to quit and relaunch the emulator to free the memory that was used when you enabled the feature.

It’s not written anywhere in the interface or the documentation, but [according to PPSSPP’s code](https://github.com/hrydgard/ppsspp/blob/master/Core/SaveState.cpp#L212-L213), the number of rewindable states the emulator saves is 20:

    // TODO: Should this be configurable?
    static const int REWIND_NUM_STATES = 20;

So at least the memory taken up while the feature is on won’t grow forever.

That number means you can press Rewind up to 20 times in a row to go back. If you press Rewind a 21st time, it seems the emulator will just cycle back to the most recent save state.