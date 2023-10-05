ALTER TABLE
    videos
ADD
    COLUMN video_created_on INTEGER NOT NULL DEFAULT 0;

UPDATE
    videos
SET
    video_created_on = strftime('%s', 'now');

ALTER TABLE
    markers
ADD
    COLUMN marker_created_on INTEGER NOT NULL DEFAULT 0;

UPDATE
    markers
SET
    marker_created_on = strftime('%s', 'now');
