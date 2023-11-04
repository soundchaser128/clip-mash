CREATE TABLE ffprobe_info (
    video_id VARCHAR PRIMARY KEY NOT NULL REFERENCES videos (id),
    info VARCHAR NOT NULL
);
