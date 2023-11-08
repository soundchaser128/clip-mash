# Compilation '{{ video.title }}'
Created with [ClipMash](https://github.com/soundchaser128/clip-mash).

## Video information
- Resolution: **{{ video.width }} x {{ video.height }}**
- Frames per second: **{{ video.fps }}**
- Video codec: **{{ video.codec }}**

## Clips
| Video | Description | Start | End |
| ----- | ----------- | ----- | --- |
{% for clip in video.clips %}
  | {{ clip.video_title }} | {{clip.marker_title}} | {{ clip.start }} | {{ clip.end }} |
{% endfor %}