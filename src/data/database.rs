use crate::{service::beats::Beats, Result};
use futures::{future, StreamExt, TryFutureExt, TryStreamExt};
use serde::Deserialize;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    SqlitePool,
};
use std::str::FromStr;
use tracing::info;

#[derive(Debug, Clone)]
pub struct DbVideo {
    pub id: String,
    pub file_path: String,
    pub interactive: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DbMarker {
    pub rowid: Option<i64>,
    pub video_id: String,
    pub start_time: f64,
    pub end_time: f64,
    pub title: String,
    pub file_path: String,
    pub index_within_video: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMarker {
    pub video_id: String,
    pub start: f64,
    pub end: f64,
    pub title: String,
    pub index_within_video: i64,
}

#[derive(Debug)]
pub struct LocalVideoWithMarkers {
    pub video: DbVideo,
    pub markers: Vec<DbMarker>,
}

#[derive(Debug)]
pub struct DbSong {
    pub rowid: Option<i64>,
    pub url: String,
    pub file_path: String,
    pub duration: f64,
    pub beats: Option<String>,
}

#[derive(Debug)]
pub struct CreateSong {
    pub url: String,
    pub file_path: String,
    pub duration: f64,
}

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(path: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(&format!("sqlite:{path}?mode=rwc"))?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal);

        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!().run(&pool).await?;

        Ok(Database { pool })
    }

    #[cfg(test)]
    pub fn with_pool(pool: SqlitePool) -> Self {
        Database { pool }
    }

    pub async fn get_video(&self, id: &str) -> Result<Option<DbVideo>> {
        let video = sqlx::query_as!(DbVideo, "SELECT * FROM local_videos WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(video)
    }

    pub async fn get_marker(&self, id: i64) -> Result<DbMarker> {
        let marker = sqlx::query_as!(
            DbMarker,
            "SELECT m.rowid, m.title, m.video_id, v.file_path, m.start_time, m.end_time, m.index_within_video
                FROM markers m INNER JOIN local_videos v ON m.video_id = v.id
                WHERE m.rowid = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(marker)
    }

    pub async fn get_video_by_path(&self, path: &str) -> Result<Option<LocalVideoWithMarkers>> {
        let records = sqlx::query!(
            "SELECT *, m.rowid AS rowid FROM local_videos v LEFT JOIN markers m ON v.id = m.video_id WHERE v.file_path = $1",
            path,
        )
        .fetch_all(&self.pool)
        .await?;

        if records.is_empty() {
            Ok(None)
        } else {
            let video = DbVideo {
                id: records[0].id.clone(),
                file_path: records[0].file_path.clone(),
                interactive: records[0].interactive,
            };
            let markers = records
                .into_iter()
                .filter_map(|r| {
                    match (
                        r.video_id,
                        r.start_time,
                        r.end_time,
                        r.title,
                        r.rowid,
                        r.file_path,
                        r.index_within_video,
                    ) {
                        (
                            Some(video_id),
                            Some(start_time),
                            Some(end_time),
                            Some(title),
                            rowid,
                            file_path,
                            Some(index),
                        ) => Some(DbMarker {
                            rowid,
                            title,
                            video_id,
                            start_time,
                            end_time,
                            file_path,
                            index_within_video: index,
                        }),
                        _ => None,
                    }
                })
                .collect();
            Ok(Some(LocalVideoWithMarkers { video, markers }))
        }
    }

    pub async fn persist_video(&self, video: DbVideo) -> Result<()> {
        sqlx::query!(
            "INSERT INTO local_videos (id, file_path, interactive) VALUES ($1, $2, $3)",
            video.id,
            video.file_path,
            video.interactive
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn persist_marker(&self, marker: CreateMarker) -> Result<DbMarker> {
        let new_id = sqlx::query_scalar!(
            "INSERT INTO markers (video_id, start_time, end_time, title, index_within_video) 
                VALUES ($1, $2, $3, $4, $5) 
                ON CONFLICT DO UPDATE SET start_time = excluded.start_time, end_time = excluded.end_time, title = excluded.title
                RETURNING rowid",
                marker.video_id,
                marker.start,
                marker.end,
                marker.title,
                marker.index_within_video,
        )
        .fetch_one(&self.pool)
        .await?;

        let marker = DbMarker {
            rowid: Some(new_id),
            start_time: marker.start,
            end_time: marker.end,
            title: marker.title,
            video_id: marker.video_id,
            file_path: "not-needed".to_string(),
            index_within_video: marker.index_within_video,
        };

        info!("newly updated or inserted marker: {marker:?}");

        Ok(marker)
    }

    pub async fn delete_marker(&self, id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM markers WHERE rowid = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn persist_song(&self, song: CreateSong) -> Result<DbSong> {
        let rowid = sqlx::query_scalar!(
            "INSERT INTO songs (url, file_path, duration) 
             VALUES ($1, $2, $3)
             RETURNING rowid",
            song.url,
            song.file_path,
            song.duration
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DbSong {
            rowid: Some(rowid),
            url: song.url,
            file_path: song.file_path,
            duration: song.duration,
            beats: None,
        })
    }

    pub async fn get_song_by_url(&self, url: &str) -> Result<Option<DbSong>> {
        sqlx::query_as!(
            DbSong,
            "SELECT rowid, url, file_path, duration, beats FROM songs WHERE url = $1",
            url
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn get_song(&self, id: i64) -> Result<DbSong> {
        sqlx::query_as!(
            DbSong,
            "SELECT rowid, url, file_path, duration, beats FROM songs WHERE rowid = $1",
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn update_song_file_path(&self, id: i64, file_path: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE songs SET file_path = $1 WHERE rowid = $2",
            file_path,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_songs(&self) -> Result<Vec<DbSong>> {
        use tokio::fs;

        let stream = sqlx::query_as!(
            DbSong,
            "SELECT rowid, url, file_path, duration, beats FROM songs"
        )
        .fetch(&self.pool);

        let videos = stream
            .try_filter(|row| fs::try_exists(row.file_path.clone()).unwrap_or_else(|_| false))
            .filter_map(|r| future::ready(r.ok()))
            .collect()
            .await;

        Ok(videos)
    }

    pub async fn get_songs(&self, song_ids: &[i64]) -> Result<Vec<DbSong>> {
        let mut songs = vec![];
        // TODO wait for SELECT ... FROM foo IN ... support in sqlx
        for id in song_ids {
            songs.push(self.get_song(*id).await?);
        }

        Ok(songs)
    }

    pub async fn sum_song_durations(&self, song_ids: &[i64]) -> Result<f64> {
        let duration = self
            .get_songs(song_ids)
            .await?
            .into_iter()
            .map(|s| s.duration)
            .sum();
        Ok(duration)
    }

    pub async fn fetch_beats(&self, song_id: i64) -> Result<Option<Beats>> {
        let result = sqlx::query!("SELECT beats FROM songs WHERE rowid = $1", song_id)
            .fetch_one(&self.pool)
            .await?;
        match result.beats {
            Some(json) => Ok(serde_json::from_str(&json)?),
            None => Ok(None),
        }
    }

    pub async fn persist_beats(&self, song_id: i64, beats: &Beats) -> Result<()> {
        let json = serde_json::to_string(&beats)?;
        sqlx::query!(
            "UPDATE songs SET beats = $1 WHERE rowid = $2",
            json,
            song_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::data::database::{CreateMarker, Database, DbVideo};
    use crate::Result;
    use fake::{faker::filesystem::en::FilePath, Fake};
    use nanoid::nanoid;
    use sqlx::SqlitePool;

    async fn persist_video(db: &Database) -> Result<DbVideo> {
        let expected = DbVideo {
            file_path: FilePath().fake(),
            id: nanoid!(8),
            interactive: false,
        };
        db.persist_video(expected.clone()).await?;
        Ok(expected)
    }

    #[sqlx::test]
    async fn test_get_and_persist_video(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let expected = persist_video(&database).await.unwrap();

        let result = database.get_video(&expected.id).await.unwrap().unwrap();
        assert_eq!(result.id, expected.id);
        assert_eq!(result.file_path, expected.file_path);
        assert_eq!(result.interactive, expected.interactive);
    }

    #[sqlx::test]
    async fn test_persist_marker(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await.unwrap();
        let expected = CreateMarker {
            title: "Some title".into(),
            video_id: video.id.clone(),
            start: 0.0,
            end: 17.0,
            index_within_video: 0,
        };
        let result = database.persist_marker(expected.clone()).await.unwrap();

        assert_eq!(result.start_time, expected.start);
        assert_eq!(result.end_time, expected.end);
        assert_eq!(result.video_id, video.id);
        assert_eq!(result.index_within_video, 0);
    }

    #[sqlx::test]
    async fn test_marker_foreign_key_constraint(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video_id = nanoid!(8);
        let expected = CreateMarker {
            title: "Some title".into(),
            video_id,
            start: 0.0,
            end: 17.0,
            index_within_video: 0,
        };
        let err = database
            .persist_marker(expected.clone())
            .await
            .expect_err("must fail due to a foreign key constraint");
        let err: sqlx::Error = err.downcast().unwrap();
        let err = err.into_database_error().unwrap();
        assert_eq!(err.message(), "FOREIGN KEY constraint failed");
    }

    #[sqlx::test]
    async fn test_delete_marker(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await.unwrap();
        let marker = CreateMarker {
            title: "Some title".into(),
            video_id: video.id,
            start: 0.0,
            end: 17.0,
            index_within_video: 0,
        };
        let result = database.persist_marker(marker).await.unwrap();
        let id = result.rowid.unwrap();

        database.delete_marker(id).await.unwrap();
        let _ = database
            .get_marker(id)
            .await
            .expect_err("must not be in the database anymore");
    }

    #[sqlx::test]
    async fn test_get_video_by_path(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let expected = persist_video(&database).await.unwrap();
        let result = database
            .get_video_by_path(&expected.file_path)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result.video.id, expected.id);
        assert_eq!(result.video.file_path, expected.file_path);
        assert_eq!(result.video.interactive, expected.interactive);
        assert_eq!(result.markers.len(), 0);
    }
}
