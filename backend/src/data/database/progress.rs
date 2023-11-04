use sqlx::SqlitePool;
use tracing::info;

use super::Progress;
use crate::Result;

#[derive(Debug, Clone)]
pub struct ProgressDatabase {
    pool: SqlitePool,
}

impl ProgressDatabase {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_progress(&self, video_id: impl Into<String>) -> Result<Option<Progress>> {
        let video_id = video_id.into();
        sqlx::query_as!(
            Progress,
            "SELECT * FROM progress WHERE video_id = $1",
            video_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn insert_progress(
        &self,
        video_id: &str,
        items_total: f64,
        message: &str,
    ) -> Result<()> {
        sqlx::query!(
            "INSERT INTO progress (video_id, items_total, items_finished, message, done, timestamp)
             VALUES ($1, $2, 0, $3, false, CURRENT_TIMESTAMP)",
            video_id,
            items_total,
            message
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_progress(
        &self,
        video_id: &str,
        progress_inc: f64,
        eta: f64,
        message: &str,
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE progress SET items_finished = items_finished + $1, message = $2, eta_seconds = $3 WHERE video_id = $4",
            progress_inc,
            message,
            eta,
            video_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn finish_progress(&self, video_id: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE progress SET done = true, message = 'Finished!' WHERE video_id = $1",
            video_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn cleanup_progress(&self) -> Result<()> {
        info!("deleting all progress entries older than 7 days");
        sqlx::query!(
            "DELETE FROM progress WHERE done = true AND timestamp < datetime('now', '-7 day')"
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
