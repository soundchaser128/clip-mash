use sqlx::SqlitePool;
use tracing::info;

use super::{DbMarker, DbMarkerWithVideo};
use crate::server::types::{CreateMarker, PageParameters, SortDirection, UpdateMarker};
use crate::Result;

#[derive(Debug, Clone)]
pub struct MarkersDatabase {
    pool: SqlitePool,
}

impl MarkersDatabase {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_marker(&self, id: i64) -> Result<DbMarkerWithVideo> {
        let marker = sqlx::query_as!(
            DbMarkerWithVideo,
            "SELECT 
                m.rowid, m.title, m.video_id, v.file_path, m.start_time, 
                m.end_time, m.index_within_video, m.marker_preview_image, 
                v.interactive, m.marker_created_on, v.video_title
            FROM markers m INNER JOIN videos v ON m.video_id = v.id
            WHERE m.rowid = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(marker)
    }

    pub async fn get_markers_without_preview_images(&self) -> Result<Vec<DbMarkerWithVideo>> {
        sqlx::query_as!(
            DbMarkerWithVideo,
            "SELECT 
                m.rowid, m.title, m.video_id, v.file_path, m.start_time, m.end_time, 
                m.index_within_video, m.marker_preview_image, v.interactive, 
                m.marker_created_on, v.video_title
            FROM markers m INNER JOIN videos v ON m.video_id = v.id
            WHERE m.marker_preview_image IS NULL"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn create_new_marker(&self, marker: CreateMarker) -> Result<DbMarker> {
        let inserted_value = sqlx::query!(
            "INSERT INTO markers (video_id, start_time, end_time, title, index_within_video, marker_preview_image) 
                VALUES ($1, $2, $3, $4, $5, $6) 
                ON CONFLICT DO UPDATE SET start_time = excluded.start_time, end_time = excluded.end_time, title = excluded.title
                RETURNING rowid, marker_created_on",
                marker.video_id,
                marker.start,
                marker.end,
                marker.title,
                marker.index_within_video,
                marker.preview_image_path,
        )
        .fetch_one(&self.pool)
        .await?;

        let marker = DbMarker {
            rowid: Some(inserted_value.rowid),
            start_time: marker.start,
            end_time: marker.end,
            title: marker.title,
            video_id: marker.video_id,
            index_within_video: marker.index_within_video,
            marker_preview_image: marker.preview_image_path,
            marker_created_on: inserted_value.marker_created_on,
        };

        info!("newly updated or inserted marker: {marker:?}");

        Ok(marker)
    }

    pub async fn update_marker(&self, update: UpdateMarker) -> Result<DbMarker> {
        let record = sqlx::query!(
            "UPDATE markers SET start_time = $1, end_time = $2, title = $3 WHERE rowid = $4
            RETURNING *",
            update.start,
            update.end,
            update.title,
            update.rowid
        )
        .fetch_one(&self.pool)
        .await?;

        let marker = DbMarker {
            rowid: Some(update.rowid),
            video_id: record.video_id,
            start_time: update.start,
            end_time: update.end,
            title: record.title,
            index_within_video: record.index_within_video,
            marker_preview_image: record.marker_preview_image,
            marker_created_on: record.marker_created_on,
        };

        Ok(marker)
    }

    pub async fn split_marker(&self, id: i64, split_time: f64) -> Result<String> {
        let marker = self.get_marker(id).await?;
        let new_marker_1 = CreateMarker {
            video_id: marker.video_id.clone(),
            start: marker.start_time,
            end: split_time,
            title: marker.title.clone(),
            index_within_video: marker.index_within_video,
            preview_image_path: None,
            video_interactive: marker.interactive,
        };

        let new_marker_2 = CreateMarker {
            video_id: marker.video_id.clone(),
            start: split_time,
            end: marker.end_time,
            title: marker.title,
            index_within_video: marker.index_within_video + 1,
            preview_image_path: None,
            video_interactive: marker.interactive,
        };

        let rowid = marker.rowid.expect("marker must have rowid");

        futures::try_join!(
            self.create_new_marker(new_marker_1),
            self.create_new_marker(new_marker_2),
            self.delete_marker(rowid),
        )?;

        Ok(marker.video_id)
    }

    pub async fn delete_marker(&self, id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM markers WHERE rowid = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    #[cfg(test)]
    pub async fn get_all_markers(&self) -> Result<Vec<DbMarkerWithVideo>> {
        let markers = sqlx::query_as!(
            DbMarkerWithVideo,
            "
            SELECT 
                m.rowid, m.title, m.video_id, v.file_path, m.start_time, m.end_time, 
                m.index_within_video, m.marker_preview_image, v.interactive, 
                m.marker_created_on, v.video_title
            FROM markers m INNER JOIN videos v ON m.video_id = v.id
            ORDER BY v.file_path ASC"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(markers)
    }

    pub async fn list_markers(
        &self,
        query: Option<&str>,
        params: &PageParameters,
    ) -> Result<(Vec<DbMarkerWithVideo>, i64)> {
        info!("fetching markers with query {query:?} and page {params:?}");
        let query = query
            .map(|q| format!("%{q}%"))
            .unwrap_or_else(|| "%".to_string());
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM markers WHERE title LIKE $1", query)
            .fetch_one(&self.pool)
            .await?;
        let limit = params.limit();
        let offset = params.offset();
        let sort = params.sort("marker_created_on", SortDirection::Desc);

        let markers = sqlx::query_as!(
            DbMarkerWithVideo,
            "SELECT m.video_id, m.rowid, m.start_time, m.end_time, 
                    m.title, v.file_path, m.index_within_video, m.marker_preview_image, 
                    v.interactive, m.marker_created_on, v.video_title
            FROM markers m
            INNER JOIN videos v ON m.video_id = v.id
            WHERE m.title LIKE $1
            ORDER BY $2
            LIMIT $3
            OFFSET $4
        ",
            query,
            sort,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        info!("found markers {markers:#?}");

        Ok((markers, count.into()))
    }

    pub async fn set_marker_preview_image(&self, id: i64, preview_image: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE markers SET marker_preview_image = $1 WHERE rowid = $2",
            preview_image,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
