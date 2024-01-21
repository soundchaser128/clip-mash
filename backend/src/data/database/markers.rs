use sqlx::{FromRow, QueryBuilder, SqliteConnection, SqliteExecutor, SqlitePool};
use tracing::{debug, info};

use super::{DbMarker, DbMarkerWithVideo, MarkerCount};
use crate::data::database::unix_timestamp_now;
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
                v.interactive, m.marker_created_on, v.video_title, v.source,
                v.video_tags, v.stash_scene_id
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
                m.marker_created_on, v.video_title, v.source, v.video_tags,
                v.stash_scene_id
            FROM markers m INNER JOIN videos v ON m.video_id = v.id
            WHERE m.marker_preview_image IS NULL"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(From::from)
    }

    async fn create_new_marker_inner(
        &self,
        marker: CreateMarker,
        connection: &mut SqliteConnection,
    ) -> Result<DbMarker> {
        let created_on = marker.created_on.unwrap_or_else(|| unix_timestamp_now());
        let inserted_value = sqlx::query!(
            "INSERT INTO markers (video_id, start_time, end_time, title, index_within_video, marker_preview_image, marker_created_on, marker_stash_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT DO UPDATE SET start_time = excluded.start_time, end_time = excluded.end_time, title = excluded.title
                RETURNING rowid, marker_created_on",
                marker.video_id,
                marker.start,
                marker.end,
                marker.title,
                marker.index_within_video,
                marker.preview_image_path,
                created_on,
                marker.marker_stash_id,

        )
        .fetch_one(&mut *connection)
        .await?;

        let mut marker = DbMarker {
            rowid: Some(inserted_value.rowid),
            start_time: marker.start,
            end_time: marker.end,
            title: marker.title,
            video_id: marker.video_id,
            index_within_video: marker.index_within_video,
            marker_preview_image: marker.preview_image_path,
            marker_created_on: inserted_value.marker_created_on,
            marker_stash_id: marker.marker_stash_id,
        };

        let new_markers = self
            .fix_marker_video_indices(&marker.video_id, &mut *connection)
            .await?;
        let actual_index = new_markers
            .iter()
            .position(|m| m.rowid == marker.rowid)
            .expect("marker must be in list of new markers");
        marker.index_within_video = actual_index as i64;

        info!("newly updated or inserted marker: {marker:?}");

        Ok(marker)
    }

    pub async fn create_new_marker(&self, marker: CreateMarker) -> Result<DbMarker> {
        let mut connection = self.pool.acquire().await?;
        self.create_new_marker_inner(marker, &mut connection).await
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

        if let Some(stash_id) = update.stash_marker_id {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("marker_stash_id = ");
            query_builder.push_bind(stash_id);
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
            marker_stash_id: record.marker_stash_id,
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
            created_on: None,
            marker_stash_id: None,
        };

        let new_marker_2 = CreateMarker {
            video_id: marker.video_id.clone(),
            start: split_time,
            end: marker.end_time,
            title: marker.title,
            index_within_video: marker.index_within_video + 1,
            preview_image_path: None,
            video_interactive: marker.interactive,
            created_on: None,
            marker_stash_id: None,
        };

        let rowid = marker.rowid.expect("marker must have rowid");

        let mut tx = self.pool.begin().await?;

        self.create_new_marker_inner(new_marker_1, &mut *tx).await?;
        self.create_new_marker_inner(new_marker_2, &mut *tx).await?;
        self.delete_marker_with_executor(rowid, &mut *tx).await?;

        tx.commit().await?;

        Ok(marker.video_id)
    }

    async fn delete_marker_with_executor<'a, E: SqliteExecutor<'a>>(
        &self,
        id: i64,
        executor: E,
    ) -> Result<()> {
        info!("deleting marker with id {}", id);
        sqlx::query!("DELETE FROM markers WHERE rowid = $1", id)
            .execute(executor)
            .await?;
        Ok(())
    }

    pub async fn delete_marker(&self, id: i64) -> Result<()> {
        self.delete_marker_with_executor(id, &self.pool).await
    }

    #[allow(unused)]
    pub async fn get_all_markers(&self) -> Result<Vec<DbMarkerWithVideo>> {
        let markers = sqlx::query_as!(
            DbMarkerWithVideo,
            "
            SELECT 
                m.rowid, m.title, m.video_id, v.file_path, m.start_time, m.end_time, 
                m.index_within_video, m.marker_preview_image, v.interactive, 
                m.marker_created_on, v.video_title, v.source, v.video_tags,
                v.stash_scene_id
            FROM markers m INNER JOIN videos v ON m.video_id = v.id
            ORDER BY v.file_path ASC"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(markers)
    }

    pub async fn list_markers(
        &self,
        video_ids: Option<&[String]>,
        sort: Option<&str>,
    ) -> Result<Vec<DbMarkerWithVideo>> {
        info!("fetching markers with video ids {video_ids:?}");
        let mut query_builder = QueryBuilder::new(
            "SELECT m.video_id, m.rowid, m.start_time, m.end_time, 
                    m.title, v.file_path, m.index_within_video, m.marker_preview_image, 
                    v.interactive, m.marker_created_on, v.video_title, v.video_tags, v.source,
                    v.stash_scene_id
            FROM markers m
            INNER JOIN videos v ON m.video_id = v.id ",
        );
        if let Some(video_ids) = video_ids {
            query_builder.push("WHERE video_id IN (");
            let mut list = query_builder.separated(",");
            for video_id in video_ids {
                list.push_bind(video_id);
            }
            list.push_unseparated(") ");
        }
        query_builder.push("ORDER BY ");
        let order = match sort {
            Some("duration") => "(m.end_time - m.start_time DESC)",
            Some("title") => "m.title ASC",
            Some("created") => "m.marker_created_on DESC",
            _ => "m.video_id ASC, m.index_within_video ASC",
        };
        query_builder.push(order);
        debug!("sql: '{}'", query_builder.sql());
        let query = query_builder.build();
        let records = query.fetch_all(&self.pool).await?;
        let markers = records
            .iter()
            .map(|m| DbMarkerWithVideo::from_row(m).unwrap())
            .collect();

        debug!("found markers {markers:#?}");

        Ok(markers)
    }

    pub async fn set_marker_preview_image(
        &self,
        id: i64,
        preview_image: Option<&str>,
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE markers SET marker_preview_image = $1 WHERE rowid = $2",
            preview_image,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn fix_all_video_indices(&self) -> Result<()> {
        let mut transaction = self.pool.begin().await?;
        let all_video_ids = sqlx::query!("SELECT id FROM videos")
            .fetch_all(&mut *transaction)
            .await?;

        for video_id in all_video_ids {
            let video_id = video_id.id;
            self.fix_marker_video_indices(&video_id, &mut *transaction)
                .await?;
        }

        Ok(())
    }

    async fn fix_marker_video_indices(
        &self,
        video_id: &str,
        connection: &mut SqliteConnection,
    ) -> Result<Vec<DbMarker>> {
        use ordered_float::OrderedFloat;

        let mut markers = self.get_markers_for_video(video_id).await?;
        markers.sort_by_key(|m| OrderedFloat(m.start_time));

        for (index, marker) in markers.iter_mut().enumerate() {
            let index = index as i64;

            if marker.index_within_video != index {
                let rowid = marker.rowid.expect("marker must have rowid");
                sqlx::query!(
                    "UPDATE markers SET index_within_video = $1 WHERE rowid = $2",
                    index,
                    rowid,
                )
                .execute(&mut *connection)
                .await?;

                info!(
                    "updated marker with id {} to have index {} (was {})",
                    rowid, index, marker.index_within_video
                );
                marker.index_within_video = index;
            }
        }

        Ok(markers)
    }

    pub async fn get_markers_for_video(&self, video_id: &str) -> Result<Vec<DbMarker>> {
        let markers = sqlx::query_as!(
            DbMarker,
            "SELECT *, rowid FROM markers WHERE video_id = $1 ORDER BY index_within_video ASC",
            video_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(markers)
    }

    pub async fn get_marker_titles(
        &self,
        count: i64,
        prefix: Option<&str>,
    ) -> Result<Vec<MarkerCount>> {
        let prefix = prefix
            .map(|s| format!("{}%", s))
            .unwrap_or_else(|| "%".to_string());
        let results = sqlx::query_as!(
            MarkerCount,
            "SELECT title, count(*) AS count
            FROM markers
            WHERE title != 'Untitled' AND title LIKE $1
            GROUP BY title
            ORDER BY count DESC
            LIMIT $2",
            prefix,
            count
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use std::marker;

    use super::*;
    use crate::data::database::Database;
    use crate::service::fixtures::{persist_marker, persist_video};

    #[sqlx::test]
    async fn test_fix_marker_video_indices(pool: SqlitePool) -> Result<()> {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await?;

        // uses create_new_marker, so the indices should be correct after inserting
        persist_marker(&database, &video.id, 3, 0.0, 5.0, false).await?;
        persist_marker(&database, &video.id, 2, 15.0, 25.0, false).await?;
        persist_marker(&database, &video.id, 1, 50.0, 55.0, false).await?;
        persist_marker(&database, &video.id, 0, 100.0, 110.0, false).await?;

        let all_markers = database.markers.get_markers_for_video(&video.id).await?;

        assert_eq!(all_markers[0].index_within_video, 0);
        assert_eq!(all_markers[0].start_time, 0.0);

        assert_eq!(all_markers[1].index_within_video, 1);
        assert_eq!(all_markers[1].start_time, 15.0);

        assert_eq!(all_markers[2].index_within_video, 2);
        assert_eq!(all_markers[2].start_time, 50.0);

        assert_eq!(all_markers[3].index_within_video, 3);
        assert_eq!(all_markers[3].start_time, 100.0);

        Ok(())
    }
}
