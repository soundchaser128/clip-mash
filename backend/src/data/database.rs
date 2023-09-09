use std::str::FromStr;

use futures::{future, StreamExt, TryFutureExt, TryStreamExt};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::{FromRow, SqlitePool};
use tokio::task::spawn_blocking;
use tracing::{info, warn};

use crate::server::types::{
    Beats, CreateMarker, ListVideoDto, PageParameters, Progress, UpdateMarker,
};
use crate::service::commands::ffmpeg::FfmpegLocation;
use crate::service::music;
use crate::Result;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum LocalVideoSource {
    Folder,
    Download,
}

impl From<String> for LocalVideoSource {
    fn from(value: String) -> Self {
        match value.as_str() {
            "folder" => Self::Folder,
            "download" => Self::Download,
            other => {
                warn!("unknown enum constant {other}, falling back to LocalVideoSource::Folder");
                LocalVideoSource::Folder
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DbVideo {
    pub id: String,
    pub file_path: String,
    pub interactive: bool,
    pub source: LocalVideoSource,
    pub duration: f64,
    pub video_preview_image: Option<String>,
    pub stash_scene_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct DbMarker {
    pub rowid: Option<i64>,
    pub video_id: String,
    pub start_time: f64,
    pub end_time: f64,
    pub title: String,
    pub file_path: String,
    pub index_within_video: i64,
    pub marker_preview_image: Option<String>,
    pub interactive: bool,
}

#[derive(Debug)]
pub struct LocalVideoWithMarkers {
    pub video: DbVideo,
    pub markers: Vec<DbMarker>,
}

#[derive(Debug)]
pub struct DbSong {
    pub rowid: Option<i64>,
    pub url: String,
    pub file_path: String,
    pub duration: f64,
    pub beats: Option<String>,
}

#[derive(Debug)]
pub struct CreateSong {
    pub url: String,
    pub file_path: String,
    pub duration: f64,
    pub beats: Option<Beats>,
}

#[derive(Debug, Clone, Copy)]
pub enum AllVideosFilter {
    NoVideoDuration,
    NoPreviewImage,
}

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(path: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(&format!("sqlite:{path}?mode=rwc"))?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal);

        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!().run(&pool).await?;

        Ok(Database { pool })
    }

    #[cfg(test)]
    pub fn with_pool(pool: SqlitePool) -> Self {
        Database { pool }
    }

    pub async fn get_video(&self, id: &str) -> Result<Option<DbVideo>> {
        let video = sqlx::query_as!(
            DbVideo,
            "SELECT * FROM videos WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(video)
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

    pub async fn get_all_markers(&self) -> Result<Vec<DbMarker>> {
        let markers = sqlx::query_as!(DbMarker, "
        SELECT m.rowid, m.title, m.video_id, v.file_path, m.start_time, m.end_time, m.index_within_video, m.marker_preview_image, v.interactive
        FROM markers m INNER JOIN videos v ON m.video_id = v.id
        ORDER BY v.file_path ASC")
        .fetch_all(&self.pool)
        .await?;
        Ok(markers)
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

    pub async fn create_new_marker(&self, marker: CreateMarker) -> Result<DbMarker> {
        let new_id = sqlx::query_scalar!(
            "INSERT INTO markers (video_id, start_time, end_time, title, index_within_video, marker_preview_image) 
                VALUES ($1, $2, $3, $4, $5, $6) 
                ON CONFLICT DO UPDATE SET start_time = excluded.start_time, end_time = excluded.end_time, title = excluded.title
                RETURNING rowid",
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
            rowid: Some(new_id),
            start_time: marker.start,
            end_time: marker.end,
            title: marker.title,
            video_id: marker.video_id,
            file_path: "not-needed".to_string(),
            index_within_video: marker.index_within_video,
            marker_preview_image: marker.preview_image_path,
            interactive: marker.video_interactive,
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
            file_path: "not-needed".to_string(),
            index_within_video: record.index_within_video,
            marker_preview_image: record.marker_preview_image,
            // not needed for updating I think
            interactive: false,
        };

        Ok(marker)
    }

    pub async fn split_marker(&self, id: i64, split_time: f64) -> Result<Vec<DbMarker>> {
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

        let video = self
            .get_video_with_markers(&marker.video_id)
            .await?
            .expect("video for marker must exist");
        Ok(video.markers)
    }

    pub async fn delete_marker(&self, id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM markers WHERE rowid = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn persist_song(&self, song: CreateSong) -> Result<DbSong> {
        let beats = serde_json::to_string(&song.beats)?;

        let rowid = sqlx::query_scalar!(
            "INSERT INTO songs (url, file_path, duration, beats) 
             VALUES ($1, $2, $3, $4)
             RETURNING rowid",
            song.url,
            song.file_path,
            song.duration,
            beats,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DbSong {
            rowid: Some(rowid),
            url: song.url,
            file_path: song.file_path,
            duration: song.duration,
            beats: None,
        })
    }

    pub async fn get_song_by_url(&self, url: &str) -> Result<Option<DbSong>> {
        sqlx::query_as!(
            DbSong,
            "SELECT rowid, url, file_path, duration, beats FROM songs WHERE url = $1",
            url
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn get_song(&self, id: i64) -> Result<DbSong> {
        sqlx::query_as!(
            DbSong,
            "SELECT rowid, url, file_path, duration, beats FROM songs WHERE rowid = $1",
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn update_song_file_path(&self, id: i64, file_path: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE songs SET file_path = $1 WHERE rowid = $2",
            file_path,
            id
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
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM videos WHERE file_path LIKE $1",
            query
        )
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

    pub async fn list_songs(&self) -> Result<Vec<DbSong>> {
        use tokio::fs;

        let stream = sqlx::query_as!(
            DbSong,
            "SELECT rowid, url, file_path, duration, beats FROM songs"
        )
        .fetch(&self.pool);

        let videos = stream
            .try_filter(|row| fs::try_exists(row.file_path.clone()).unwrap_or_else(|_| false))
            .filter_map(|r| future::ready(r.ok()))
            .collect()
            .await;

        Ok(videos)
    }

    pub async fn get_songs(&self, song_ids: &[i64]) -> Result<Vec<DbSong>> {
        let mut songs = vec![];
        // TODO wait for SELECT ... FROM foo IN ... support in sqlx
        for id in song_ids {
            songs.push(self.get_song(*id).await?);
        }

        Ok(songs)
    }

    pub async fn fetch_beats(&self, song_id: i64) -> Result<Option<Beats>> {
        let result = sqlx::query!("SELECT beats FROM songs WHERE rowid = $1", song_id)
            .fetch_optional(&self.pool)
            .await?;
        match result {
            Some(row) => match row.beats {
                Some(json) => Ok(serde_json::from_str(&json)?),
                None => Ok(None),
            },
            None => Ok(None),
        }
    }

    pub async fn persist_beats(&self, song_id: i64, beats: &Beats) -> Result<()> {
        let json = serde_json::to_string(&beats)?;
        sqlx::query!(
            "UPDATE songs SET beats = $1 WHERE rowid = $2",
            json,
            song_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn generate_all_beats(&self, ffmpeg: FfmpegLocation) -> Result<()> {
        let rows = sqlx::query!("SELECT rowid, file_path FROM songs WHERE beats IS NULL")
            .fetch_all(&self.pool)
            .await?;
        if rows.is_empty() {
            return Ok(());
        }
        info!("generating beats for {} songs", rows.len());
        let mut handles = vec![];
        for row in rows {
            let ffmpeg = ffmpeg.clone();
            handles.push(spawn_blocking(move || {
                (music::detect_beats(row.file_path, &ffmpeg), row.rowid)
            }));
        }

        for handle in handles {
            let (beats, song_id) = handle.await?;
            self.persist_beats(song_id, &beats?).await?;
        }

        Ok(())
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

#[cfg(test)]
mod test {
    use sqlx::SqlitePool;
    use tracing_test::traced_test;

    use crate::data::database::{CreateMarker, Database};
    use crate::server::types::{PageParameters, SortDirection, UpdateMarker};
    use crate::service::fixtures::{persist_marker, persist_video, persist_video_with_file_name};
    use crate::util::generate_id;

    #[sqlx::test]
    async fn test_get_and_persist_video(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let expected = persist_video(&database).await.unwrap();

        let result = database.get_video(&expected.id).await.unwrap().unwrap();
        assert_eq!(result.id, expected.id);
        assert_eq!(result.file_path, expected.file_path);
        assert_eq!(result.interactive, expected.interactive);
    }

    #[sqlx::test]
    async fn test_create_marker(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await.unwrap();
        let expected = CreateMarker {
            title: "Some title".into(),
            video_id: video.id.clone(),
            start: 0.0,
            end: 17.0,
            index_within_video: 0,
            preview_image_path: None,
            video_interactive: false,
        };
        let result = database.create_new_marker(expected.clone()).await.unwrap();

        assert_eq!(result.start_time, expected.start);
        assert_eq!(result.end_time, expected.end);
        assert_eq!(result.video_id, video.id);
        assert_eq!(result.index_within_video, 0);
    }

    #[sqlx::test]
    async fn test_marker_foreign_key_constraint(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video_id = generate_id();
        let expected = CreateMarker {
            title: "Some title".into(),
            video_id,
            start: 0.0,
            end: 17.0,
            index_within_video: 0,
            preview_image_path: None,
            video_interactive: false,
        };
        let err = database
            .create_new_marker(expected.clone())
            .await
            .expect_err("must fail due to a foreign key constraint");
        let err: sqlx::Error = err.downcast().unwrap();
        let err = err.into_database_error().unwrap();
        assert_eq!(err.message(), "FOREIGN KEY constraint failed");
    }

    #[sqlx::test]
    async fn test_delete_marker(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await.unwrap();
        let marker = CreateMarker {
            title: "Some title".into(),
            video_id: video.id,
            start: 0.0,
            end: 17.0,
            index_within_video: 0,
            preview_image_path: None,
            video_interactive: false,
        };
        let result = database.create_new_marker(marker).await.unwrap();
        let id = result.rowid.unwrap();

        database.delete_marker(id).await.unwrap();
        let _ = database
            .get_marker(id)
            .await
            .expect_err("must not be in the database anymore");
    }

    #[sqlx::test]
    async fn test_get_video_by_path(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let expected = persist_video(&database).await.unwrap();
        let result = database
            .get_video_by_path(&expected.file_path)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result.video.id, expected.id);
        assert_eq!(result.video.file_path, expected.file_path);
        assert_eq!(result.video.interactive, expected.interactive);
        assert_eq!(result.markers.len(), 0);
    }

    #[sqlx::test]
    async fn test_list_videos(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        for _ in 0..45 {
            persist_video(&database).await.unwrap();
        }
        let page = PageParameters {
            page: Some(0),
            size: Some(10),
            sort: Some("file_path".into()),
            dir: Some(SortDirection::Asc),
        };
        let (result, total) = database.list_videos(None, &page).await.unwrap();
        assert_eq!(45, total);
        assert_eq!(10, result.len());
    }

    #[sqlx::test]
    #[traced_test]
    async fn test_list_videos_search(pool: SqlitePool) {
        let database = Database::with_pool(pool);

        for _ in 0..45 {
            persist_video(&database).await.unwrap();
        }
        for i in 0..10 {
            let path = format!("/some/path/{i}/sexy.mp4");
            persist_video_with_file_name(&database, &path)
                .await
                .unwrap();
        }

        for i in 0..5 {
            let path = format!("/some/path/{i}/cool.mp4");
            persist_video_with_file_name(&database, &path)
                .await
                .unwrap();
        }

        let page = PageParameters {
            page: Some(0),
            size: Some(10),
            sort: Some("file_path".into()),
            dir: Some(SortDirection::Asc),
        };
        let (result, total) = database.list_videos(Some("sexy"), &page).await.unwrap();
        assert_eq!(10, total);
        assert_eq!(10, result.len());
        let file_name = &result[0].video.file_name;
        assert_eq!(file_name, "sexy.mp4");

        let (result, total) = database.list_videos(Some("cool"), &page).await.unwrap();
        assert_eq!(5, total);
        assert_eq!(5, result.len());
        let file_name = &result[0].video.file_name;
        assert_eq!(file_name, "cool.mp4");
    }

    #[sqlx::test]
    async fn test_update_marker(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await.unwrap();
        let marker = persist_marker(&database, &video.id, 0, 0.0, 15.0, false)
            .await
            .unwrap();
        let title = marker.title.clone();
        let update = UpdateMarker {
            title: marker.title,
            rowid: marker.rowid.unwrap(),
            start: 5.0,
            end: 15.0,
        };
        database.update_marker(update).await.unwrap();
        let result = database.get_marker(marker.rowid.unwrap()).await.unwrap();

        assert_eq!(result.title, title);
        assert_eq!(result.end_time, 15.0);
        assert_eq!(result.start_time, 5.0);
    }

    #[sqlx::test]
    #[traced_test]
    async fn test_split_marker(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await.unwrap();
        let marker = persist_marker(&database, &video.id, 0, 0.0, 15.0, false)
            .await
            .unwrap();
        tracing::info!("inserted marker: {:?}", marker);
        let result = database
            .split_marker(marker.rowid.unwrap(), 5.0)
            .await
            .unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].start_time, 0.0);
        assert_eq!(result[0].end_time, 5.0);
        assert_eq!(result[1].start_time, 5.0);
        assert_eq!(result[1].end_time, 15.0);

        let all_markers = database.get_all_markers().await.unwrap();
        assert_eq!(all_markers.len(), 2);
    }

    #[sqlx::test]
    #[traced_test]
    async fn test_insert_progress(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video_id = generate_id();
        let items_total = 100.0;
        let message = "Some message";
        database
            .insert_progress(&video_id, items_total, message)
            .await
            .unwrap();
        let result = database.get_progress(&video_id).await.unwrap().unwrap();
        assert_eq!(result.items_total, items_total);
        assert_eq!(result.items_finished, 0.0);
        assert_eq!(result.message, message);
        assert_eq!(result.done, false);
        assert_eq!(result.eta_seconds, None);
    }

    #[sqlx::test]
    #[traced_test]
    async fn test_update_progress(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video_id = generate_id();
        let items_total = 100.0;
        let message = "Starting...";
        database
            .insert_progress(&video_id, items_total, message)
            .await
            .unwrap();

        database
            .update_progress(&video_id, 10.0, 60.0, "Encoding videos")
            .await
            .unwrap();

        let progress = database.get_progress(&video_id).await.unwrap().unwrap();
        assert_eq!(progress.items_finished, 10.0);
        assert_eq!(progress.eta_seconds, Some(60.0));
        assert_eq!(progress.message, "Encoding videos");
        assert_eq!(progress.done, false);

        database.finish_progress(&video_id).await.unwrap();

        let progress = database.get_progress(&video_id).await.unwrap().unwrap();
        assert_eq!(progress.done, true);
    }
}
