# Changelog

## Unrelease
- docs: Added updated screenshots
- chore: Fixed clippy warnings

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
