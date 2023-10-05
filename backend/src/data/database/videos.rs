use std::collections::HashSet;

use camino::Utf8Path;
use futures::TryStreamExt;
use sqlx::{FromRow, QueryBuilder, Row, SqlitePool};
use tracing::{debug, info};

use super::{AllVideosFilter, CreateVideo, DbMarker, DbVideo, LocalVideoWithMarkers, VideoUpdate};
use crate::server::types::{ListVideoDto, PageParameters};
use crate::service::video::TAG_SEPARATOR;
use crate::Result;

#[derive(Clone)]
pub struct VideosDatabase {
    pool: SqlitePool,
}

impl VideosDatabase {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_video(&self, id: &str) -> Result<Option<DbVideo>> {
        let video = sqlx::query_as!(DbVideo, "SELECT * FROM videos WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(video)
    }

    pub async fn delete_video(&self, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM markers WHERE video_id = $1", id)
            .execute(&self.pool)
            .await?;
        sqlx::query!("DELETE FROM videos WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_video(&self, id: &str, update: VideoUpdate) -> Result<()> {
        let mut query_builder = QueryBuilder::new("UPDATE videos SET ");
        let mut first = true;
        if let Some(title) = update.title {
            query_builder.push("video_title = ");
            query_builder.push_bind(title);
            first = false;
        }

        if let Some(tags) = update.tags {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("video_tags = ");
            query_builder.push_bind(tags.join(TAG_SEPARATOR));
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        let query = query_builder.build();
        query.execute(&self.pool).await?;

        Ok(())
    }

    pub async fn video_exists_by_path(&self, path: &str) -> Result<bool> {
        let records = sqlx::query!("SELECT * FROM videos v WHERE v.file_path = $1", path,)
            .fetch_all(&self.pool)
            .await?;

        Ok(!records.is_empty())
    }

    pub async fn get_video_with_markers(&self, id: &str) -> Result<Option<LocalVideoWithMarkers>> {
        let records = sqlx::query!(
            "SELECT *, m.rowid AS rowid FROM videos v LEFT JOIN markers m ON v.id = m.video_id WHERE v.id = $1",
            id,
        )
        .fetch_all(&self.pool)
        .await?;

        if records.is_empty() {
            Ok(None)
        } else {
            let video = DbVideo {
                id: records[0].id.clone(),
                file_path: records[0].file_path.clone(),
                interactive: records[0].interactive,
                source: records[0].source.clone().into(),
                duration: records[0].duration,
                video_preview_image: records[0].video_preview_image.clone(),
                stash_scene_id: records[0].stash_scene_id,
                video_created_on: records[0].video_created_on.clone(),
                video_tags: records[0].video_tags.clone(),
                video_title: records[0].video_title.clone(),
            };
            let markers = records
                .into_iter()
                .filter_map(|r| {
                    match (
                        r.video_id,
                        r.start_time,
                        r.end_time,
                        r.title,
                        r.rowid,
                        r.index_within_video,
                        r.marker_preview_image,
                        r.marker_created_on,
                    ) {
                        (
                            Some(video_id),
                            Some(start_time),
                            Some(end_time),
                            Some(title),
                            rowid,
                            Some(index),
                            marker_preview_image,
                            Some(marker_created_on),
                        ) => Some(DbMarker {
                            rowid,
                            title,
                            video_id,
                            start_time,
                            end_time,
                            index_within_video: index,
                            marker_preview_image,
                            marker_created_on,
                        }),
                        _ => None,
                    }
                })
                .collect();
            Ok(Some(LocalVideoWithMarkers { video, markers }))
        }
    }

    pub async fn get_videos(&self, filter: AllVideosFilter) -> Result<Vec<DbVideo>> {
        let query = match filter {
            AllVideosFilter::NoVideoDuration => {
                sqlx::query_as!(DbVideo, "SELECT * FROM videos WHERE duration = -1.0")
                    .fetch_all(&self.pool)
                    .await
            }
            AllVideosFilter::NoPreviewImage => {
                sqlx::query_as!(
                    DbVideo,
                    "SELECT * FROM videos WHERE video_preview_image IS NULL"
                )
                .fetch_all(&self.pool)
                .await
            }
            AllVideosFilter::NoTitle => {
                sqlx::query_as!(DbVideo, "SELECT * FROM videos WHERE video_title IS NULL")
                    .fetch_all(&self.pool)
                    .await
            }
        };
        query.map_err(From::from)
    }

    pub async fn cleanup_videos(&self) -> Result<u32> {
        let mut count = 0;
        let mut stream =
            sqlx::query!("SELECT * FROM videos WHERE source != 'stash'").fetch(&self.pool);

        while let Some(video) = stream.try_next().await? {
            info!("assessing video {} at {}", video.id, video.file_path);
            let path = Utf8Path::new(&video.file_path);
            if !path.exists() {
                info!("video {} does not exist, deleting", video.id);
                sqlx::query!("DELETE FROM markers WHERE video_id = $1", video.id)
                    .execute(&self.pool)
                    .await?;
                sqlx::query!("DELETE FROM videos WHERE id = $1", video.id)
                    .execute(&self.pool)
                    .await?;
                count += 1;
            }
        }

        Ok(count)
    }

    pub async fn persist_video(&self, video: &CreateVideo) -> Result<DbVideo> {
        let inserted = sqlx::query!(
            "INSERT INTO videos 
            (id, file_path, interactive, source, duration, video_preview_image, stash_scene_id, video_title, video_tags) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING video_created_on",
            video.id,
            video.file_path,
            video.interactive,
            video.source,
            video.duration,
            video.video_preview_image,
            video.stash_scene_id,
            video.title,
            video.tags,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DbVideo {
            id: video.id.clone(),
            file_path: video.file_path.clone(),
            interactive: video.interactive,
            source: video.source.clone(),
            duration: video.duration,
            video_preview_image: video.video_preview_image.clone(),
            stash_scene_id: video.stash_scene_id,
            video_created_on: inserted.video_created_on,
            video_tags: video.tags.clone(),
            video_title: video.title.clone(),
        })
    }

    async fn fetch_count(&self, query: Option<&str>) -> Result<i64> {
        match query {
            Some(query) => sqlx::query_scalar!(
                "SELECT COUNT(*) FROM videos_fts WHERE videos_fts MATCH $1",
                query
            )
            .fetch_one(&self.pool)
            .await
            .map_err(From::from),
            None => sqlx::query_scalar!("SELECT COUNT(*) FROM videos")
                .fetch_one(&self.pool)
                .await
                .map_err(From::from)
                .map(|c| c as i64),
        }
    }

    pub async fn list_videos(
        &self,
        query: Option<&str>,
        params: &PageParameters,
    ) -> Result<(Vec<ListVideoDto>, usize)> {
        #[derive(FromRow, Debug)]
        struct Row {
            id: String,
            file_path: String,
            interactive: bool,
            duration: f64,
            video_created_on: String,
            source: String,
            video_preview_image: Option<String>,
            stash_scene_id: Option<i64>,
            video_tags: Option<String>,
            video_title: Option<String>,
            marker_count: i64,
        }

        let count = self.fetch_count(query).await?;
        info!("count: {} for query {:?}", count, query);
        let limit = params.limit();
        let offset = params.offset();
        let order_by = match params.sort.as_deref() {
            Some("title") => "v.video_title COLLATE NOCASE ASC",
            Some("created") => "v.video_created_on DESC",
            _ => "marker_count DESC, v.video_title COLLATE NOCASE ASC",
        };

        let mut query_builder = QueryBuilder::new(
            "SELECT v.id, v.file_path, v.interactive, v.duration, v.video_created_on, v.source, v.video_preview_image, 
                    v.stash_scene_id, v.video_tags, v.video_title, COUNT(m.video_id) AS marker_count
            FROM videos v
            LEFT JOIN markers m ON v.id = m.video_id ",
        );

        if let Some(query) = query {
            query_builder
                .push("WHERE v.rowid IN (SELECT rowid FROM videos_fts WHERE videos_fts MATCH ");
            query_builder.push_bind(query);
            query_builder.push(") ");
        }
        query_builder.push("GROUP BY v.id ");
        query_builder.push("ORDER BY ");
        // security: technically this would be vulnerable to SQL injection, but
        // all the user would get is their own local sqlite database, I don't think
        // this is too much of an issue. with binds, this doesn't seem to work.
        query_builder.push(order_by);
        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);

        debug!("sql: '{}'", query_builder.sql());

        let query = query_builder.build();
        let records = query.fetch_all(&self.pool).await?;
        let records: Vec<_> = records.iter().map(|r| Row::from_row(r).unwrap()).collect();

        if records.is_empty() {
            Ok((vec![], count as usize))
        } else {
            let mut videos = vec![];
            for row in records {
                let video = DbVideo {
                    id: row.id,
                    file_path: row.file_path,
                    interactive: row.interactive,
                    source: row.source.into(),
                    duration: row.duration,
                    video_preview_image: row.video_preview_image,
                    stash_scene_id: row.stash_scene_id,
                    video_created_on: row.video_created_on,
                    video_tags: row.video_tags,
                    video_title: row.video_title,
                };
                videos.push(ListVideoDto {
                    video: video.into(),
                    marker_count: row.marker_count as usize,
                })
            }
            Ok((videos, count as usize))
        }
    }

    pub async fn set_video_duration(&self, id: &str, duration: f64) -> Result<()> {
        sqlx::query!(
            "UPDATE videos SET duration = $1 WHERE id = $2",
            duration,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn set_video_preview_image(&self, id: &str, preview_image: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE videos SET video_preview_image = $1 WHERE id = $2",
            preview_image,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Find videos in the database matching the given stash IDs
    pub async fn get_stash_scene_ids(&self, stash_ids: &[i64]) -> Result<HashSet<i64>> {
        let mut query_builder =
            QueryBuilder::new("SELECT stash_scene_id FROM videos WHERE stash_scene_id IN (");
        let mut list = query_builder.separated(",");
        for id in stash_ids {
            list.push_bind(id);
        }
        list.push_unseparated(") ");

        let query = query_builder.build();
        let rows = query.fetch_all(&self.pool).await?;

        let mut result = HashSet::new();
        for row in rows {
            let stash_id = row.get::<Option<i64>, _>(0);
            if let Some(id) = stash_id {
                result.insert(id);
            }
        }

        Ok(result)
    }

    pub async fn get_videos_by_ids(&self, ids: &[&str]) -> Result<Vec<DbVideo>> {
        let mut query_builder = QueryBuilder::new("SELECT * FROM videos WHERE id IN (");
        let mut list = query_builder.separated(",");
        for id in ids {
            list.push_bind(id);
        }
        list.push_unseparated(") ");
        query_builder.push(" ORDER BY id DESC");

        let query = query_builder.build();
        let rows = query.fetch_all(&self.pool).await?;
        let videos: Vec<_> = rows.iter().map(|v| DbVideo::from_row(v).unwrap()).collect();
        Ok(videos)
    }
}
