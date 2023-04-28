CREATE TABLE IF NOT EXISTS local_videos (
    id VARCHAR NOT NULL,
    file_path VARCHAR NOT NULL UNIQUE,
    interactive BOOLEAN NOT NULL
)