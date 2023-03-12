# stash-compilation-maker

Connects to your [stash](https://github.com/stashapp/stash) instance and creates simple 
compilation videos from scene markers. You select one or more tags, or one or more performers
and it will take (currently) the first 15 seconds of video after the marker start and compile
all of the markers into one video.

## Usage
Download the binary for your OS from the releases page and run it. A new browser tab should open with the GUI
prompting you for your configuration information, then you can select filters, (de)select individual markers,
enter some video information and then generate the video. Should the download in the browser not work, the videos
are stored in the `videos` subdirectory of where the executable is stored.
