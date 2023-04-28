CREATE TABLE local_videos (
    id VARCHAR NOT NULL,
    file_path VARCHAR NOT NULL UNIQUE,
    interactive BOOLEAN NOT NULL
);

CREATE TABLE markers (
    marker_id VARCHAR NOT NULL REFERENCES local_videos (id),
    start_time INTEGER NOT NULL,
    end_time INTEGER NOT NULL,
    title VARCHAR NOT NULL
);
