# ClipMash

ClipMash is a video editing app that allows you to automate creating compilations from multiple videos. 
It's mostly made for, ahem, adult content, which is why it can connect to [Stash](https://stashapp.cc/) and fetch videos
and scene markers from there to guide the video creation process. You can also use local files and set the markers in 
ClipMash itself and then generate a compilation based on that.

## Usage
Download the binary for your OS from the releases page and run it. A new browser tab should open with the GUI
prompting you for your configuration information, then you can select filters, (de)select individual markers,
enter some video information and then generate the video. Should the download in the browser not work, the videos
are stored in the `videos` subdirectory of where the executable is stored.

The app requires `ffmpeg` to run, and will attempt to download it, if it isn't installed on your machine.
This currently only works for Windows and Linux systems, Mac users will have to install it either by 
downloading the executable and placing it into their `$PATH` or installing it with `brew install ffmpeg`.

## Building
Requires `cargo`, `rustc` (see http://rustup.rs/) and `node` and `npm` (see https://nodejs.org/en). When those
tools are installed, you should be able to build:

## Building for development

```shell
# Required to create the database and apply the schema
cargo install sqlx-cli
sqlx migrate run

cd frontend
npm install
npm run dev

# In a new shell, in the project root:
cargo run
```

## Building for production
```shell
cd frontend
npm install
npm run build

cd ..
# Required to create the database and apply the schema
cargo install sqlx-cli
sqlx migrate run
cargo build --release
```
