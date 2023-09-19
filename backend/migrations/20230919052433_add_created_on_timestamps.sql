ALTER TABLE
    videos
ADD
    COLUMN video_created_on VARCHAR NOT NULL DEFAULT "";

UPDATE
    videos
SET
    video_created_on = CURRENT_TIMESTAMP;

ALTER TABLE
    markers
ADD
    COLUMN marker_created_on VARCHAR NOT NULL DEFAULT "";

UPDATE
    markers
SET
    marker_created_on = CURRENT_TIMESTAMP;
