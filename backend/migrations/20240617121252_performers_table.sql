CREATE TABLE performers (
    id INTEGER PRIMARY KEY,
    "name" VARCHAR NOT NULL UNIQUE,
    created_on INTEGER NOT NULL,
    image_url VARCHAR,
    stash_id VARCHAR,
    gender VARCHAR
);

CREATE TABLE video_performers (
    performer_id INTEGER NOT NULL REFERENCES performers(id),
    video_id VARCHAR NOT NULL REFERENCES videos(id),
    PRIMARY KEY(performer_id, video_id)
);