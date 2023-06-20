CREATE TABLE github_release (
    fetched_at DATETIME DEFAULT current_timestamp,
    json_data VARCHAR NOT NULL
);
