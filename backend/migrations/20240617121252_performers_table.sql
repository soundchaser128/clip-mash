CREATE TABLE performers (
    "name" VARCHAR NOT NULL,
    created_on INTEGER NOT NULL,
    image_url VARCHAR,
    stash_id VARCHAR,
    gender VARCHAR
);

CREATE TABLE video_performers (
    performer_id INTEGER NOT NULL REFERENCES performers(rowid),
    video_id INTEGER NOT NULL REFERENCES videos(id),
    PRIMARY KEY(performer_id, video_id)
);