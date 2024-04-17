CREATE VIRTUAL TABLE videos_fts USING fts5(
    video_title,
    video_tags,
    content = videos,
    content_rowid = rowid
);

CREATE TRIGGER videos_fts_insert AFTER INSERT ON videos BEGIN
  INSERT INTO videos_fts(rowid, video_title, video_tags) VALUES (new.rowid, new.video_title, new.video_tags);
END;

CREATE TRIGGER videos_fts_delete AFTER DELETE ON videos BEGIN
  INSERT INTO videos_fts(videos_fts, rowid, video_title, video_tags) VALUES('delete', old.rowid, old.video_title, old.video_tags);
END;

CREATE TRIGGER videos_fts_update AFTER UPDATE ON videos BEGIN
  INSERT INTO videos_fts(videos_fts, rowid, video_title, video_tags) VALUES('delete', old.rowid, old.video_title, old.video_tags);
  INSERT INTO videos_fts(rowid, video_title, video_tags) VALUES (new.rowid, new.video_title, new.video_tags);
END;

INSERT INTO videos_fts (rowid, video_title, video_tags)
    SELECT rowid, video_title, video_tags
    FROM videos;
