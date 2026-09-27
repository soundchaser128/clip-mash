use futures::{StreamExt, TryFutureExt, TryStreamExt, future};
use sqlx::SqlitePool;
use tokio::task::spawn_blocking;
use tracing::info;

use crate::Result;
use crate::service::commands::ffmpeg::FfmpegLocation;
use crate::service::music;
use crate::types::Beats;

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
    pub beats: Option<Beats>,
}

#[derive(Debug, Clone)]
pub struct MusicDatabase {
    pool: SqlitePool,
}

impl MusicDatabase {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn persist_song(&self, song: CreateSong) -> Result<DbSong> {
        let beats = serde_json::to_string(&song.beats)?;

        let rowid = sqlx::query_scalar!(
            "INSERT INTO songs (url, file_path, duration, beats) 
             VALUES ($1, $2, $3, $4)
             RETURNING rowid",
            song.url,
            song.file_path,
            song.duration,
            beats,
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

    pub async fn fetch_beats(&self, song_id: i64) -> Result<Option<Beats>> {
        let result = sqlx::query!("SELECT beats FROM songs WHERE rowid = $1", song_id)
            .fetch_optional(&self.pool)
            .await?;
        match result {
            Some(row) => match row.beats {
                Some(json) => Ok(serde_json::from_str(&json)?),
                None => Ok(None),
            },
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

    pub async fn generate_all_beats(&self, ffmpeg: FfmpegLocation) -> Result<()> {
        let rows = sqlx::query!("SELECT rowid, file_path FROM songs WHERE beats IS NULL")
            .fetch_all(&self.pool)
            .await?;
        if rows.is_empty() {
            return Ok(());
        }
        info!("generating beats for {} songs", rows.len());
        let mut handles = vec![];
        for row in rows {
            let ffmpeg = ffmpeg.clone();
            handles.push(spawn_blocking(move || {
                (music::detect_beats(row.file_path, &ffmpeg), row.rowid)
            }));
        }

        for handle in handles {
            let (beats, song_id) = handle.await?;
            self.persist_beats(song_id, &beats?).await?;
        }

        Ok(())
    }
}
