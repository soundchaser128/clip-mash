# Unifying Stash and Local videos

Situation:
- Current model shows only markers, not videos
- Ability to find markers either via tags, scene(s) or performer(s)

Where do I want to end up:
- Unified library that lets you add videos via different sources:
    - Stash (can filter like before, tags, scenes, performers)
    - Local files
    - yt-dlp
- Adapt database table (rename it to just `videos` from `local_videos`)
    - `source` column already exists, just add another variant `stash`.
    - Need some additional columns, mostly stash scene ID?
    - Make path optional
- Allow adding all scenes from Stash in one go? (maybe not necessary)