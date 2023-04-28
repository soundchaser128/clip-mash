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

        Ok(Database { pool })
    }

    pub async fn init(&self) -> Result<()> {
        sqlx::query_file!("db/init.sql").execute(&self.pool).await?;

        Ok(())
    }

    pub async fn get_video(&self, id: &str) -> Result<LocalVideo> {
        let video = sqlx::query_as!(LocalVideo, "SELECT * FROM local_videos WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(video)
    }
}
