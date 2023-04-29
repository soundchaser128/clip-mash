CREATE TABLE local_videos (
    id VARCHAR NOT NULL PRIMARY KEY,
    file_path VARCHAR NOT NULL UNIQUE,
    interactive BOOLEAN NOT NULL
);

CREATE TABLE markers (
    video_id VARCHAR NOT NULL REFERENCES local_videos (id),
    start_time DOUBLE PRECISION NOT NULL,
    end_time DOUBLE PRECISION NOT NULL,
    title VARCHAR NOT NULL,
    PRIMARY KEY (video_id, start_time, end_time)
);
