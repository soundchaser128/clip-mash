use serde::{Deserialize, Serialize};
use sqlx::{FromRow, QueryBuilder, SqliteConnection, SqliteExecutor, SqlitePool};
use tracing::{debug, info};
use utoipa::ToSchema;

use super::videos::{DbVideo, VideoSource, tags_from_string};
use crate::Result;
use crate::data::database::unix_timestamp_now;
use crate::data::stash_api::MarkerLike;
use crate::server::types::{CreateMarker, UpdateMarker};

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct DbMarker {
    pub rowid: Option<i64>,
    pub video_id: String,
    pub start_time: f64,
    pub end_time: f64,
    pub title: String,
    pub index_within_video: i64,
    pub marker_preview_image: Option<String>,
    pub marker_created_on: i64,
    pub marker_stash_id: Option<i64>,
}

impl MarkerLike for DbMarker {
    fn start(&self) -> f64 {
        self.start_time
    }

    fn end(&self) -> f64 {
        self.end_time
    }
}

// TODO better name
#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct DbMarkerWithVideo {
    pub rowid: Option<i64>,
    pub video_id: String,
    pub start_time: f64,
    pub end_time: f64,
    pub title: String,
    pub file_path: String,
    pub index_within_video: i64,
    pub marker_preview_image: Option<String>,
    pub interactive: bool,
    pub marker_created_on: i64,
    pub video_title: Option<String>,
    pub video_tags: Option<String>,
    pub source: VideoSource,
    pub stash_scene_id: Option<i64>,
}

impl DbMarkerWithVideo {
    pub fn tags(&self) -> Vec<String> {
        tags_from_string(self.video_tags.as_deref())
    }
}

#[derive(Debug)]
pub struct VideoWithMarkers {
    pub video: DbVideo,
    pub markers: Vec<DbMarker>,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct MarkerCount {
    pub title: String,
    pub count: i64,
}

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
                v.interactive, m.marker_created_on, v.video_title, v.source AS \"source: VideoSource\",
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
                m.marker_created_on, v.video_title, v.source AS \"source: VideoSource\", v.video_tags,
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
                m.marker_created_on, v.video_title, v.source AS \"source: VideoSource\", v.video_tags,
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
        filter: Option<ListMarkersFilter>,
        sort: Option<&str>,
    ) -> Result<Vec<DbMarkerWithVideo>> {
        info!("fetching markers with filter {filter:?}");
        let mut query_builder = QueryBuilder::new(
            "SELECT m.video_id, m.rowid, m.start_time, m.end_time, 
                    m.title, v.file_path, m.index_within_video, m.marker_preview_image, 
                    v.interactive, m.marker_created_on, v.video_title, v.video_tags, v.source,
                    v.stash_scene_id
            FROM markers m
            INNER JOIN videos v ON m.video_id = v.id ",
        );
        if let Some(filter) = filter {
            match filter {
                ListMarkersFilter::VideoIds(video_ids) => {
                    query_builder.push("WHERE video_id IN (");
                    let mut list = query_builder.separated(",");
                    for video_id in video_ids {
                        list.push_bind(video_id);
                    }
                    list.push_unseparated(") ");
                }
                ListMarkersFilter::MarkerTitles(marker_titles) => {
                    query_builder.push("WHERE title IN (");
                    let mut list = query_builder.separated(",");
                    for title in marker_titles {
                        list.push_bind(title);
                    }
                    list.push_unseparated(") ");
                }
                ListMarkersFilter::VideoPerformers(performers) => {
                    query_builder.push(
                        "INNER JOIN video_performers pv ON v.id = pv.video_id INNER JOIN performers p ON pv.performer_id = p.id WHERE p.name IN (",
                    );

                    let mut list = query_builder.separated(",");
                    for performer in performers {
                        list.push_bind(performer);
                    }
                    list.push_unseparated(") ");
                }
                ListMarkersFilter::VideoTags(tags) => {
                    query_builder.push("JOIN json_each(v.video_tags) AS tag WHERE tag.value IN (");
                    let mut list = query_builder.separated(",");

                    for tag in tags {
                        list.push_bind(tag);
                    }
                    list.push_unseparated(") ");
                }
            }
        }

        query_builder.push("ORDER BY ");
        let order = match sort {
            Some("duration") => "(m.end_time - m.start_time) DESC",
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

        transaction.commit().await?;

        Ok(())
    }

    async fn fix_marker_video_indices(
        &self,
        video_id: &str,
        connection: &mut SqliteConnection,
    ) -> Result<Vec<DbMarker>> {
        use ordered_float::OrderedFloat;

        let mut markers = self
            .get_markers_for_video_inner(video_id, &mut *connection)
            .await?;
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
        let mut connection = self.pool.acquire().await?;
        self.get_markers_for_video_inner(video_id, &mut connection)
            .await
    }

    async fn get_markers_for_video_inner(
        &self,
        video_id: &str,
        connection: &mut SqliteConnection,
    ) -> Result<Vec<DbMarker>> {
        let markers = sqlx::query_as!(
            DbMarker,
            "SELECT *, rowid FROM markers WHERE video_id = $1 ORDER BY index_within_video ASC",
            video_id
        )
        .fetch_all(connection)
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
            WHERE title LIKE $1
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

#[derive(Debug)]
pub enum ListMarkersFilter {
    VideoIds(Vec<String>),
    MarkerTitles(Vec<String>),
    VideoPerformers(Vec<String>),
    VideoTags(Vec<String>),
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;
    use crate::data::database::Database;
    use crate::data::database::performers::{CreatePerformer, Gender};
    use crate::service::fixtures::{
        persist_marker, persist_performer, persist_video, persist_video_with,
    };

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

    #[sqlx::test]
    async fn test_list_markers_video_ids_filter(pool: SqlitePool) -> Result<()> {
        let database = Database::with_pool(pool);

        // Create two videos
        let video1 = persist_video(&database).await?;
        let video2 = persist_video(&database).await?;

        // Create markers for each video
        for i in 0..3 {
            persist_marker(&database, &video1.id, i, i as f64, (i + 5) as f64, false).await?;
        }

        for i in 0..2 {
            persist_marker(&database, &video2.id, i, i as f64, (i + 3) as f64, false).await?;
        }

        // Filter by the first video's ID
        let filter = ListMarkersFilter::VideoIds(vec![video1.id.clone()]);
        let result = database.markers.list_markers(Some(filter), None).await?;

        // Should only return markers from the first video
        assert_eq!(result.len(), 3);
        for marker in &result {
            assert_eq!(marker.video_id, video1.id);
        }

        // Filter by both video IDs
        let filter = ListMarkersFilter::VideoIds(vec![video1.id.clone(), video2.id.clone()]);
        let result = database.markers.list_markers(Some(filter), None).await?;

        // Should return all markers
        assert_eq!(result.len(), 5);

        Ok(())
    }

    #[sqlx::test]
    async fn test_list_markers_marker_titles_filter(pool: SqlitePool) -> Result<()> {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await?;

        // Create markers with specific titles
        let titles = ["Action", "Comedy", "Drama", "Action"];

        for (i, title) in titles.iter().enumerate() {
            let marker = CreateMarker {
                video_id: video.id.clone(),
                start: i as f64,
                end: (i + 1) as f64,
                index_within_video: i as i64,
                title: title.to_string(),
                preview_image_path: None,
                video_interactive: false,
                created_on: None,
                marker_stash_id: None,
            };
            database.markers.create_new_marker(marker).await?;
        }

        // Filter by "Action" title
        let filter = ListMarkersFilter::MarkerTitles(vec!["Action".to_string()]);
        let result = database.markers.list_markers(Some(filter), None).await?;

        // Should return 2 markers with title "Action"
        assert_eq!(result.len(), 2);
        for marker in &result {
            assert_eq!(marker.title, "Action");
        }

        // Filter by multiple titles
        let filter =
            ListMarkersFilter::MarkerTitles(vec!["Comedy".to_string(), "Drama".to_string()]);
        let result = database.markers.list_markers(Some(filter), None).await?;

        // Should return 2 markers (1 Comedy, 1 Drama)
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|m| m.title == "Comedy"));
        assert!(result.iter().any(|m| m.title == "Drama"));

        Ok(())
    }

    #[sqlx::test]
    async fn test_list_markers_video_tags_filter(pool: SqlitePool) -> Result<()> {
        let database = Database::with_pool(pool);

        // Create videos with specific tags
        let video1 = persist_video_with(&database, |v| {
            v.tags = Some(r#"["action", "thriller"]"#.to_string());
        })
        .await?;

        let video2 = persist_video_with(&database, |v| {
            v.tags = Some(r#"["comedy", "romance"]"#.to_string());
        })
        .await?;

        // Create markers for each video
        for i in 0..2 {
            persist_marker(&database, &video1.id, i, i as f64, (i + 5) as f64, false).await?;
        }

        for i in 0..3 {
            persist_marker(&database, &video2.id, i, i as f64, (i + 3) as f64, false).await?;
        }

        // Filter by "action" tag
        let filter = ListMarkersFilter::VideoTags(vec!["action".to_string()]);
        let result = database.markers.list_markers(Some(filter), None).await?;

        // Should return markers from video1
        assert_eq!(result.len(), 2);
        for marker in &result {
            assert_eq!(marker.video_id, video1.id);
        }

        // Filter by "comedy" tag
        let filter = ListMarkersFilter::VideoTags(vec!["comedy".to_string()]);
        let result = database.markers.list_markers(Some(filter), None).await?;

        // Should return markers from video2
        assert_eq!(result.len(), 3);
        for marker in &result {
            assert_eq!(marker.video_id, video2.id);
        }

        Ok(())
    }

    #[sqlx::test]
    async fn test_list_markers_sorting(pool: SqlitePool) -> Result<()> {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await?;

        // Create markers with different durations, creation times, and titles
        let markers_data = [
            // (title, start, end, created_on_offset)
            ("B-Title", 0.0, 10.0, 0),
            ("A-Title", 15.0, 20.0, 1),
            ("C-Title", 25.0, 55.0, 2),
        ];

        let base_timestamp = unix_timestamp_now();

        for (i, (title, start, end, time_offset)) in markers_data.iter().enumerate() {
            let marker = CreateMarker {
                video_id: video.id.clone(),
                start: *start,
                end: *end,
                index_within_video: i as i64,
                title: title.to_string(),
                preview_image_path: None,
                video_interactive: false,
                created_on: Some(base_timestamp + time_offset * 3600), // Add hours to timestamp
                marker_stash_id: None,
            };
            database.markers.create_new_marker(marker).await?;
        }

        // Test sort by duration
        let result = database
            .markers
            .list_markers(None, Some("duration"))
            .await?;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].title, "C-Title"); // Longest duration (30s) should be first
        assert_eq!(result[1].title, "B-Title"); // Middle duration (10s)
        assert_eq!(result[2].title, "A-Title"); // Shortest duration (5s)

        // Test sort by title
        let result = database.markers.list_markers(None, Some("title")).await?;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].title, "A-Title");
        assert_eq!(result[1].title, "B-Title");
        assert_eq!(result[2].title, "C-Title");

        // Test sort by created_on (most recent first)
        let result = database.markers.list_markers(None, Some("created")).await?;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].title, "C-Title"); // Most recent
        assert_eq!(result[1].title, "A-Title");
        assert_eq!(result[2].title, "B-Title"); // Oldest

        // Test default sort (by index_within_video)
        let result = database.markers.list_markers(None, None).await?;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].title, "B-Title"); // index 0
        assert_eq!(result[1].title, "A-Title"); // index 1
        assert_eq!(result[2].title, "C-Title"); // index 2

        Ok(())
    }

    #[sqlx::test]
    #[traced_test]
    async fn test_list_markers_video_performers_filter(pool: SqlitePool) -> Result<()> {
        let database = Database::with_pool(pool);

        // Create videos
        let video1 = persist_video(&database).await?;
        let video2 = persist_video(&database).await?;

        // Create performers
        let performer1 =
            persist_performer(&database, "Alice", Some(Gender::Female), None, None).await?;
        let performer2 =
            persist_performer(&database, "Bob", Some(Gender::Male), None, None).await?;

        // Associate performers with videos
        database
            .performers
            .insert_for_video(
                &[CreatePerformer {
                    name: performer1.name.clone(),
                    image_url: None,
                    stash_id: None,
                    gender: None,
                }],
                &video1.id,
            )
            .await?;

        database
            .performers
            .insert_for_video(
                &[CreatePerformer {
                    name: performer2.name.clone(),
                    image_url: None,
                    stash_id: None,
                    gender: None,
                }],
                &video2.id,
            )
            .await?;

        // Create markers for each video
        for i in 0..3 {
            persist_marker(&database, &video1.id, i, i as f64, (i + 5) as f64, false).await?;
        }

        for i in 0..2 {
            persist_marker(&database, &video2.id, i, i as f64, (i + 3) as f64, false).await?;
        }

        // Filter by the first performer
        let filter = ListMarkersFilter::VideoPerformers(vec![performer1.name.clone()]);
        let result = database.markers.list_markers(Some(filter), None).await?;

        // Should only return markers from video1
        assert_eq!(result.len(), 3);
        for marker in &result {
            assert_eq!(marker.video_id, video1.id);
        }

        // Filter by the second performer
        let filter = ListMarkersFilter::VideoPerformers(vec![performer2.name.clone()]);
        let result = database.markers.list_markers(Some(filter), None).await?;

        // Should only return markers from video2
        assert_eq!(result.len(), 2);
        for marker in &result {
            assert_eq!(marker.video_id, video2.id);
        }

        // Filter by both performers
        let filter = ListMarkersFilter::VideoPerformers(vec![
            performer1.name.clone(),
            performer2.name.clone(),
        ]);
        let result = database.markers.list_markers(Some(filter), None).await?;

        // Should return markers from both videos
        assert_eq!(result.len(), 5);

        Ok(())
    }
}
