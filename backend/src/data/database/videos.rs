use itertools::Itertools;
use sqlx::SqlitePool;
use tracing::info;

use super::{AllVideosFilter, DbMarker, DbVideo, LocalVideoWithMarkers};
use crate::server::types::{ListVideoDto, PageParameters};
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

    pub async fn get_video_by_path(&self, path: &str) -> Result<Option<LocalVideoWithMarkers>> {
        let records = sqlx::query!(
            "SELECT *, m.rowid AS rowid FROM videos v LEFT JOIN markers m ON v.id = m.video_id WHERE v.file_path = $1",
            path,
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
                        r.file_path,
                        r.index_within_video,
                        r.marker_preview_image,
                        r.interactive,
                    ) {
                        (
                            Some(video_id),
                            Some(start_time),
                            Some(end_time),
                            Some(title),
                            rowid,
                            file_path,
                            Some(index),
                            marker_preview_image,
                            interactive,
                        ) => Some(DbMarker {
                            rowid,
                            title,
                            video_id,
                            start_time,
                            end_time,
                            file_path,
                            index_within_video: index,
                            marker_preview_image,
                            interactive: interactive,
                        }),
                        _ => None,
                    }
                })
                .collect();
            Ok(Some(LocalVideoWithMarkers { video, markers }))
        }
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
                        r.file_path,
                        r.index_within_video,
                        r.marker_preview_image,
                        r.interactive,
                    ) {
                        (
                            Some(video_id),
                            Some(start_time),
                            Some(end_time),
                            Some(title),
                            rowid,
                            file_path,
                            Some(index),
                            marker_preview_image,
                            interactive,
                        ) => Some(DbMarker {
                            rowid,
                            title,
                            video_id,
                            start_time,
                            end_time,
                            file_path,
                            index_within_video: index,
                            marker_preview_image,
                            interactive,
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
        };
        query.map_err(From::from)
    }

    pub async fn persist_video(&self, video: DbVideo) -> Result<()> {
        sqlx::query!(
            "INSERT INTO videos (id, file_path, interactive, source, duration, video_preview_image) VALUES ($1, $2, $3, $4, $5, $6)",
            video.id,
            video.file_path,
            video.interactive,
            video.source,
            video.duration,
            video.video_preview_image,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_videos(
        &self,
        query: Option<&str>,
        params: &PageParameters,
    ) -> Result<(Vec<ListVideoDto>, usize)> {
        let query = query
            .map(|q| format!("%{q}%"))
            .unwrap_or_else(|| "%".to_string());
        info!("using query '{}'", query);
        let count =
            sqlx::query_scalar!("SELECT COUNT(*) FROM videos WHERE file_path LIKE $1", query)
                .fetch_one(&self.pool)
                .await?;
        let limit = params.limit();
        let offset = params.offset();
        let sort = params.sort("file_path");

        let mut records = sqlx::query!(
            "SELECT *, m.rowid AS rowid 
            FROM videos v 
            LEFT JOIN markers m ON v.id = m.video_id 
            WHERE v.file_path LIKE $1 AND v.rowid IN 
                (SELECT rowid FROM videos WHERE file_path LIKE $1 LIMIT $2 OFFSET $3)
            ORDER BY $4",
            query,
            limit,
            offset,
            sort,
        )
        .fetch_all(&self.pool)
        .await?;
        if records.is_empty() {
            Ok((vec![], count as usize))
        } else {
            records.sort_by_key(|v| v.id.clone());

            let iter = records.into_iter().group_by(|v| v.id.clone());
            let mut videos = vec![];
            for (_, group) in &iter {
                let group: Vec<_> = group.collect();
                let video = DbVideo {
                    id: group[0].id.clone(),
                    file_path: group[0].file_path.clone(),
                    interactive: group[0].interactive,
                    source: group[0].source.clone().into(),
                    duration: group[0].duration,
                    video_preview_image: group[0].video_preview_image.clone(),
                    stash_scene_id: group[0].stash_scene_id,
                };
                let markers: Vec<_> = group
                    .into_iter()
                    .filter_map(|r| {
                        match (
                            r.video_id,
                            r.start_time,
                            r.end_time,
                            r.title,
                            r.rowid,
                            r.file_path,
                            r.index_within_video,
                            r.marker_preview_image,
                            r.interactive,
                        ) {
                            (
                                video_id,
                                start_time,
                                end_time,
                                title,
                                Some(rowid),
                                file_path,
                                index,
                                marker_preview_image,
                                interactive,
                            ) => Some(
                                DbMarker {
                                    rowid: Some(rowid),
                                    title,
                                    video_id,
                                    start_time,
                                    end_time,
                                    file_path,
                                    index_within_video: index,
                                    marker_preview_image,
                                    interactive,
                                }
                                .into(),
                            ),
                            _ => None,
                        }
                    })
                    .collect();
                videos.push(ListVideoDto {
                    video: video.into(),
                    markers,
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
}