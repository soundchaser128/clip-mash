# Changelog

## Unreleased
- feat: Allow specifying video encoding parameters (codec, quality and encoding effort)
- feat: Change video list page to show all videos on a page instead of having to specify a path first
- feat: Show ETA during clip creation
- feat: Allow repeating markers
- fix: Changing a marker's range makes a duplicate

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
