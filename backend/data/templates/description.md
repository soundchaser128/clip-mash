# Compilation '{ title }'

Created with [ClipMash](https://github.com/soundchaser128/clip-mash).

## Video information

- Resolution: **{{ ctx.width }} x {{ ctx.height }}**
- Frames per second: **{{ ctx.fps }}**
- Video codec: **{{ ctx.codec }}**

## Clips

| Video | Description | Start | End |
| ----- | ----------- | ----- | --- |
{% for c in ctx.clips %}
| {{ c.video_title }} | {{ c.marker_title }} | {{ c.start }} | {{ c.end }} |
{% endfor %}

## Videos

<!-- | Source | Title | Interactive |
| ------ | ----- | ----------- |
{% for v in ctx.videos %}
| {{ v.source }} | {{ v.title }} | {{ v.interactive }}
{% endfor %} -->

<table>

</table>