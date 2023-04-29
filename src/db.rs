use crate::Result;
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    SqlitePool,
};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct LocalVideo {
    pub id: String,
    pub file_path: String,
    pub interactive: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbMarker {
    pub video_id: String,
    pub start_time: f64,
    pub end_time: f64,
    pub title: String,
}

#[derive(Debug)]
pub struct LocalVideoWithMarkers {
    pub video: LocalVideo,
    pub markers: Vec<DbMarker>,
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let options = SqliteConnectOptions::from_str("sqlite:videos.sqlite3")?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal);

        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!().run(&pool).await?;

        Ok(Database { pool })
    }

    pub async fn get_video(&self, id: &str) -> Result<Option<LocalVideo>> {
        let video = sqlx::query_as!(LocalVideo, "SELECT * FROM local_videos WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(video)
    }

    pub async fn get_video_by_path(&self, path: &str) -> Result<Option<LocalVideoWithMarkers>> {
        let records = sqlx::query!(
            "SELECT * FROM local_videos v LEFT JOIN markers m ON v.id = m.video_id WHERE v.file_path = $1",
            path,
        )
        .fetch_all(&self.pool)
        .await?;

        if records.is_empty() {
            Ok(None)
        } else {
            let video = LocalVideo {
                id: records[0].id.clone(),
                file_path: records[0].file_path.clone(),
                interactive: records[0].interactive,
            };
            let markers = records
                .into_iter()
                .filter_map(|r| match (r.video_id, r.start_time, r.end_time, r.title) {
                    (Some(video_id), Some(start_time), Some(end_time), Some(title)) => {
                        Some(DbMarker {
                            title,
                            video_id,
                            start_time,
                            end_time,
                        })
                    }
                    _ => None,
                })
                .collect();
            Ok(Some(LocalVideoWithMarkers { video, markers }))
        }
    }

    pub async fn video_exists(&self, path: &str) -> Result<bool> {
        let count = sqlx::query_scalar!(
            "SELECT count(*) FROM local_videos WHERE file_path = $1",
            path
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    pub async fn persist_video(&self, video: LocalVideo) -> Result<()> {
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

    pub async fn get_markers_for_video(&self, video_id: &str) -> Result<Vec<DbMarker>> {
        sqlx::query_as!(
            DbMarker,
            "SELECT * FROM markers WHERE video_id = $1",
            video_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn persist_markers(&self, markers: &[DbMarker]) -> Result<()> {
        for marker in markers {
            sqlx::query!(
                "INSERT INTO markers (video_id, start_time, end_time, title) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
                 marker.video_id,
                 marker.start_time,
                 marker.end_time,
                 marker.title
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }
}
