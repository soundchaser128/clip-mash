# stash-compilation-maker

Connects to your [stash](https://github.com/stashapp/stash) instance and creates simple 
compilation videos from scene markers. You select one or more tags, or one or more performers
and it will take (currently) the first 15 seconds of video after the marker start and compile
all of the markers into one video.

## Usage
Download the binary for your OS from the releases page, download [ffmpeg](https://ffmpeg.org/download.html), 
put them both either in the same directory, or in your `PATH` and run `stash-compilation-maker` from your terminal.
You will need an API key for accessing your Stash instance, you can generate one in the settings page under security.
