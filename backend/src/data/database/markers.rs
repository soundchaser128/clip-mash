use sqlx::SqlitePool;

use super::DbMarker;
use crate::Result;

#[derive(Debug, Clone)]
pub struct MarkersDatabase {
    pool: SqlitePool,
}

impl MarkersDatabase {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_marker(&self, id: i64) -> Result<DbMarker> {
        let marker = sqlx::query_as!(
            DbMarker,
            "SELECT m.rowid, m.title, m.video_id, v.file_path, m.start_time, m.end_time, m.index_within_video, m.marker_preview_image, v.interactive
                FROM markers m INNER JOIN videos v ON m.video_id = v.id
                WHERE m.rowid = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(marker)
    }

    pub async fn get_markers_without_preview_images(&self) -> Result<Vec<DbMarker>> {
        sqlx::query_as!(
            DbMarker,
            "SELECT m.rowid, m.title, m.video_id, v.file_path, m.start_time, m.end_time, m.index_within_video, m.marker_preview_image, v.interactive
            FROM markers m INNER JOIN videos v ON m.video_id = v.id
            WHERE m.marker_preview_image IS NULL"
        )
            .fetch_all(&self.pool)
            .await
            .map_err(From::from)
    }
}
