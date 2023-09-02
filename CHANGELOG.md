# Changelog

## Unreleased
- feat: allow removing and rearranging clips (unless the video has its cuts synced to music)

## 0.15.1
- fix(docker): ffmpeg wasn't working when the input source was a URL that had to be DNS resolved

## 0.15.0
- feat: Add beat-based funscript generator
- feat: Allow splitting markers
- feat: Add OpenAPI docs (WIP)
- feat: Improve progress handling
- chore: Split up videos folder into downloaded videos, generated clip fragments and finished compilations
- chore: Removed typescript interface generation, to be replaced by OpenAPI-generated client (WIP)
- fix: A song's detected beats weren't being persisted to the database in some cases
- feat: Add dark mode
- feat: Added Dockerfile and published generated image to `ghcr.io/soundchaser128/clip-mash`
- refactor: Change progress endpoints to require the ID of the generated video and store the progress in the database
- feat: Improve ETA calculations

## 0.14.0
- feat: Auotmated marker creation by detecting scene changes
- fix: Add encoding parameters (resolution and codec name) to clip filenames

## 0.13.0
- feat: show version number in footer
- chore: update frontend and backend dependency versions
- feat: add (almost) lossless encoding setting

## 0.12.0
- feat: Allow specifying video encoding parameters (codec, quality and encoding effort)
- feat: Change video list page to show all videos on a page instead of having to specify a path first
- feat: Show ETA during clip creation
- feat: Allow repeating markers
- fix: Changing a marker's range makes a duplicate
- fix: Funscript file was not available for download when generating from local files

## 0.11.0
- feat: Added browser notifications when downloads are finished or video generation is done
- feat: Generate video and marker preview images for local videos and show them in the UI
- feat: Added filter to local video list page

## 0.10.0
- feat: Allow specifying weights for markers
- feat: Added button to quickly add entire video as a marker
- feat: Use ffprobe to detect the video length for local videos

## 0.9.1
- fix: Show whether a video was downloaded via the app on the video list page
- fix: Fixed a few style issues with smaller screens

## 0.9.0
- feat: Allow downloading videos via `yt-dlp`
- feat: Allow editing the marker start/end time fields
- feat: Validate markers before saving them to the database
- fix: Beats were not being generated for music
- chore: move backend code into separate folder

## 0.8.0
- feat: Generate clips based on a song's beats
- feat: Add dedicated page for selecting markers for local videos
- feat: Add clip options to clip preview page
- fix: Improve frontend error handling
- refactor: generate typescript definitions based on Rust types

## 0.7.2
- fix: mismatch between length of generated clips and music
- feat: add logging to file, not just terminal

## 0.7.1
- fix: assets folder was not being created

## 0.7.0
- feat: Support for adding background music (#9)

## 0.6.0
- feat: Allow using local videos (#8)
- chore: Rename app to ClipMash
- chore: Added eslint setup for Typescript code

# 0.5.1
- docs: Added updated screenshots
- chore: Fixed clippy warnings
- fix: crash when reaching the end of the list of clips

## 0.5.0
- feat: Added a `.funscript` file generator.
- feat: Show more details about the scenes and performers used for selecting markers.
- fix: Improved frontend error handling (404s are handled differently from other errors, show more details)
- feat: Show existing configuration values when navigating to the configuration page
- feat: Improve clip preview page: Show colored segments signifying the different clips, allow playing through 
  all the clips in one go.
- fix: Fixed the "test credentials" button not showing an error when credentials were invalid
- feat: Added links to the steps in the UI
- chore: Use `color_eyre` for error handling in Rust
- fix: Improved UI for small screens (in particular, phones)
- feat: Added toggle for filter page to allow including any or all of the tags or performers selected.
- feat: Added toggle in video options to control whether to include the entire selected marker duration in one go, or split it up
  into shorter clips.
