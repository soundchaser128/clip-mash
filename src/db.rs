use crate::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    SqlitePool,
};
use std::str::FromStr;

#[derive(Debug)]
pub struct LocalVideo {
    pub id: String,
    pub file_path: String,
    pub interactive: bool,
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

    pub async fn get_video(&self, id: &str) -> Result<LocalVideo> {
        let video = sqlx::query_as!(LocalVideo, "SELECT * FROM local_videos WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(video)
    }

    pub async fn get_video_by_path(&self, path: &str) -> Result<Option<LocalVideo>> {
        let video = sqlx::query_as!(
            LocalVideo,
            "SELECT * FROM local_videos where file_path = $1",
            path,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(video)
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

    pub async fn persist_videos(&self, videos: &mut [LocalVideo]) -> Result<()> {
        for video in videos {
            let existing_video = self.get_video_by_path(&video.file_path).await?;
            match existing_video {
                Some(existing_video) => {
                    video.id = existing_video.id;
                }
                None => {
                    sqlx::query!(
                        "INSERT INTO local_videos (id, file_path, interactive) VALUES ($1, $2, $3)",
                        video.id,
                        video.file_path,
                        video.interactive
                    )
                    .execute(&self.pool)
                    .await?;
                }
            }
        }
        Ok(())
    }
}
