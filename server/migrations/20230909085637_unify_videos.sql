ALTER TABLE local_videos RENAME TO videos;
ALTER TABLE videos ADD COLUMN stash_scene_id INTEGER;

CREATE TABLE markers2 (
    video_id VARCHAR NOT NULL REFERENCES videos (id),
    start_time DOUBLE PRECISION NOT NULL,
    end_time DOUBLE PRECISION NOT NULL,
    title VARCHAR NOT NULL,
    index_within_video INTEGER NOT NULL, marker_preview_image VARCHAR,
    PRIMARY KEY (video_id, start_time, end_time)
);

INSERT INTO markers2 SELECT * FROM markers;
DROP TABLE markers;
ALTER TABLE markers2 RENAME TO markers;
