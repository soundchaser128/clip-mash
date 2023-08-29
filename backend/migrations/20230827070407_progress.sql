CREATE TABLE progress (
    video_id VARCHAR NOT NULL,
    items_finished DOUBLE PRECISION NOT NULL,
    items_total DOUBLE PRECISION NOT NULL,
    done BOOLEAN NOT NULL,
    "message" VARCHAR NOT NULL,
    eta_seconds DOUBLE PRECISION,
    PRIMARY KEY (video_id)
)