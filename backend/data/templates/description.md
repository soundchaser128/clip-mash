# Compilation '{ title }'

Created with [ClipMash](https://github.com/soundchaser128/clip-mash).

## Video information

- Resolution: **{ width } x { height }**
- Frames per second: **{ fps }**
- Video codec: **{ codec }**

## Clips

| Video | Description | Start | End |
| ----- | ----------- | ----- | --- |

{{ for value in clips }}
| { clip.video_title } | {clip.marker_title} | { clip.start } | { clip.end } |
{{ endfor }}

## Videos

| Source | Title | Interactive |
| ------ | ----- | ----------- |

{{ for v in videos }}
| { v.source } | { v.title } | { v.interactive }
{{ endfor }}
