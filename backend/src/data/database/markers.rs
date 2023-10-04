use sqlx::{FromRow, QueryBuilder, SqlitePool};
use tracing::{debug, info};

use super::{DbMarker, DbMarkerWithVideo};
use crate::server::types::{CreateMarker, UpdateMarker};
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
                v.interactive, m.marker_created_on, v.video_title, v.source
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
                m.marker_created_on, v.video_title, v.source
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

    pub async fn update_marker(&self, id: i64, update: UpdateMarker) -> Result<DbMarker> {
        let mut query_builder = QueryBuilder::new("UPDATE markers SET ");
        let mut first = true;

        if let Some(start) = update.start {
            query_builder.push("start_time = ");
            query_builder.push_bind(start);
            first = false;
        }

        if let Some(end) = update.end {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("end_time = ");
            query_builder.push_bind(end);
        }

        if let Some(title) = update.title {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("title = ");
            query_builder.push_bind(title);
        }

        query_builder.push(" WHERE rowid = ");
        query_builder.push_bind(id);
        query_builder.push(" RETURNING *, rowid");

        debug!("sql: '{}'", query_builder.sql());

        let query = query_builder.build();
        let row = query.fetch_one(&self.pool).await?;
        let record = DbMarker::from_row(&row)?;

        let marker = DbMarker {
            rowid: Some(id),
            video_id: record.video_id,
            start_time: record.start_time,
            end_time: record.end_time,
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

    #[allow(unused)]
    pub async fn get_all_markers(&self) -> Result<Vec<DbMarkerWithVideo>> {
        let markers = sqlx::query_as!(
            DbMarkerWithVideo,
            "
            SELECT 
                m.rowid, m.title, m.video_id, v.file_path, m.start_time, m.end_time, 
                m.index_within_video, m.marker_preview_image, v.interactive, 
                m.marker_created_on, v.video_title, v.source
            FROM markers m INNER JOIN videos v ON m.video_id = v.id
            ORDER BY v.file_path ASC"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(markers)
    }

    pub async fn list_markers(&self, video_ids: &[&str]) -> Result<Vec<DbMarkerWithVideo>> {
        info!("fetching markers with video ids {video_ids:?}");
        let mut query_builder = QueryBuilder::new(
            "SELECT m.video_id, m.rowid, m.start_time, m.end_time, 
                    m.title, v.file_path, m.index_within_video, m.marker_preview_image, 
                    v.interactive, m.marker_created_on, v.video_title, v.source
            FROM markers m
            INNER JOIN videos v ON m.video_id = v.id
            WHERE video_id IN (",
        );
        let mut list = query_builder.separated(",");
        for video_id in video_ids {
            list.push_bind(video_id);
        }
        list.push_unseparated(") ");
        query_builder.push("ORDER BY m.video_id ASC, m.index_within_video ASC");
        let query = query_builder.build();
        let records = query.fetch_all(&self.pool).await?;
        let markers = records
            .iter()
            .flat_map(DbMarkerWithVideo::from_row)
            .collect();

        debug!("found markers {markers:#?}");

        Ok(markers)
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
