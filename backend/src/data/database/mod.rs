use std::str::FromStr;

use color_eyre::eyre::bail;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::{FromRow, SqlitePool};
use tracing::warn;
use utoipa::ToSchema;

use self::markers::MarkersDatabase;
use self::music::MusicDatabase;
use self::progress::ProgressDatabase;
use self::videos::VideosDatabase;
use crate::server::types::{Beats, Progress, VideoLike};
use crate::service::video::TAG_SEPARATOR;
use crate::Result;

mod markers;
mod music;
mod progress;
mod videos;

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[sqlx(rename_all = "lowercase")]
pub enum VideoSource {
    Folder,
    Download,
    Stash,
}

impl FromStr for VideoSource {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "folder" => Ok(Self::Folder),
            "download" => Ok(Self::Download),
            "stash" => Ok(Self::Stash),
            other => bail!("unknown enum constant {other} for VideoSource"),
        }
    }
}

// needed for sqlx, I guess?
impl From<String> for VideoSource {
    fn from(value: String) -> Self {
        match value.as_str() {
            "folder" => Self::Folder,
            "download" => Self::Download,
            "stash" => Self::Stash,
            other => {
                warn!("unknown enum constant {other}, falling back to VideoSource::Folder");
                VideoSource::Folder
            }
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct DbVideo {
    pub id: String,
    pub file_path: String,
    pub interactive: bool,
    pub source: VideoSource,
    pub duration: f64,
    pub video_preview_image: Option<String>,
    pub stash_scene_id: Option<i64>,
    pub video_created_on: String,
    pub video_title: Option<String>,
    pub video_tags: Option<String>,
}

impl DbVideo {
    pub fn tags(&self) -> Option<Vec<String>> {
        self.video_tags
            .clone()
            .map(|s| s.split(TAG_SEPARATOR).map(|s| s.to_string()).collect())
    }
}

impl VideoLike for DbVideo {
    fn video_id(&self) -> &str {
        &self.id
    }

    fn stash_scene_id(&self) -> Option<i64> {
        self.stash_scene_id
    }
}

#[derive(Debug, Clone)]
pub struct CreateVideo {
    pub id: String,
    pub file_path: String,
    pub interactive: bool,
    pub source: VideoSource,
    pub duration: f64,
    pub video_preview_image: Option<String>,
    pub stash_scene_id: Option<i64>,
    pub title: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct DbMarker {
    pub rowid: Option<i64>,
    pub video_id: String,
    pub start_time: f64,
    pub end_time: f64,
    pub title: String,
    pub index_within_video: i64,
    pub marker_preview_image: Option<String>,
    pub marker_created_on: String,
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
    pub marker_created_on: String,
    pub video_title: Option<String>,
    pub source: VideoSource,
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
    pub videos: VideosDatabase,
    pub markers: MarkersDatabase,
    pub progress: ProgressDatabase,
    pub music: MusicDatabase,
}

impl Database {
    pub async fn new(path: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(&format!("sqlite:{path}?mode=rwc"))?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal);

        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!().run(&pool).await?;

        Ok(Database {
            markers: MarkersDatabase::new(pool.clone()),
            progress: ProgressDatabase::new(pool.clone()),
            music: MusicDatabase::new(pool.clone()),
            videos: VideosDatabase::new(pool),
        })
    }

    #[cfg(test)]
    pub fn with_pool(pool: SqlitePool) -> Self {
        Database {
            markers: MarkersDatabase::new(pool.clone()),
            progress: ProgressDatabase::new(pool.clone()),
            music: MusicDatabase::new(pool.clone()),
            videos: VideosDatabase::new(pool),
        }
    }
}

#[cfg(test)]
mod test {
    use sqlx::SqlitePool;
    use tracing_test::traced_test;

    use crate::data::database::Database;
    use crate::server::types::{CreateMarker, PageParameters, SortDirection, UpdateMarker};
    use crate::service::fixtures::{persist_marker, persist_video, persist_video_fn};
    use crate::util::generate_id;

    #[sqlx::test]
    async fn test_get_and_persist_video(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let expected = persist_video(&database).await.unwrap();

        let result = database
            .videos
            .get_video(&expected.id)
            .await
            .unwrap()
            .unwrap();
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
        let result = database
            .markers
            .create_new_marker(expected.clone())
            .await
            .unwrap();

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
            .markers
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
        let result = database.markers.create_new_marker(marker).await.unwrap();
        let id = result.rowid.unwrap();

        database.markers.delete_marker(id).await.unwrap();
        let _ = database
            .markers
            .get_marker(id)
            .await
            .expect_err("must not be in the database anymore");
    }

    #[sqlx::test]
    async fn test_get_video_by_path(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let inserted = persist_video(&database).await.unwrap();
        let exists = database
            .videos
            .video_exists_by_path(&inserted.file_path)
            .await
            .unwrap();
        assert_eq!(true, exists);
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
        let (result, total) = database.videos.list_videos(None, &page).await.unwrap();
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
            persist_video_fn(&database, |v| {
                v.file_path = path;
            })
            .await
            .unwrap();
        }

        for i in 0..5 {
            let path = format!("/some/path/{i}/cool.mp4");
            persist_video_fn(&database, |v| {
                v.file_path = path;
            })
            .await
            .unwrap();
        }

        let page = PageParameters {
            page: Some(0),
            size: Some(10),
            sort: Some("file_path".into()),
            dir: Some(SortDirection::Asc),
        };
        let (result, total) = database
            .videos
            .list_videos(Some("sexy"), &page)
            .await
            .unwrap();
        assert_eq!(10, total);
        assert_eq!(10, result.len());
        let file_name = &result[0].video.file_name;
        assert_eq!(file_name, "sexy.mp4");

        let (result, total) = database
            .videos
            .list_videos(Some("cool"), &page)
            .await
            .unwrap();
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
        database.markers.update_marker(update).await.unwrap();
        let result = database
            .markers
            .get_marker(marker.rowid.unwrap())
            .await
            .unwrap();

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
        let video_id = database
            .markers
            .split_marker(marker.rowid.unwrap(), 5.0)
            .await
            .unwrap();
        let video = database
            .videos
            .get_video_with_markers(&video_id)
            .await
            .unwrap();
        let result = video.unwrap().markers;
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].start_time, 0.0);
        assert_eq!(result[0].end_time, 5.0);
        assert_eq!(result[1].start_time, 5.0);
        assert_eq!(result[1].end_time, 15.0);

        let all_markers = database.markers.get_all_markers().await.unwrap();
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
            .progress
            .insert_progress(&video_id, items_total, message)
            .await
            .unwrap();
        let result = database
            .progress
            .get_progress(&video_id)
            .await
            .unwrap()
            .unwrap();
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
            .progress
            .insert_progress(&video_id, items_total, message)
            .await
            .unwrap();

        database
            .progress
            .update_progress(&video_id, 10.0, 60.0, "Encoding videos")
            .await
            .unwrap();

        let progress = database
            .progress
            .get_progress(&video_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(progress.items_finished, 10.0);
        assert_eq!(progress.eta_seconds, Some(60.0));
        assert_eq!(progress.message, "Encoding videos");
        assert_eq!(progress.done, false);

        database.progress.finish_progress(&video_id).await.unwrap();

        let progress = database
            .progress
            .get_progress(&video_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(progress.done, true);
    }

    #[sqlx::test]
    #[traced_test]
    fn test_has_stash_scene_ids(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        for idx in 0..20 {
            persist_video_fn(&database, |v| {
                if idx < 5 {
                    v.stash_scene_id = Some(idx);
                }
            })
            .await
            .unwrap();
        }

        let params = PageParameters {
            dir: None,
            page: Some(0),
            size: Some(100),
            sort: None,
        };
        let (_videos, count) = database.videos.list_videos(None, &params).await.unwrap();
        assert_eq!(20, count);

        let ids: Vec<_> = (0..20).collect();
        let results = database.videos.get_stash_scene_ids(&ids).await.unwrap();
        assert_eq!(results.len(), 5);

        let mut results: Vec<_> = results.into_iter().collect();
        results.sort();
        assert_eq!(results, vec![0, 1, 2, 3, 4]);
    }

    #[sqlx::test]
    #[traced_test]
    async fn test_fetch_videos_by_ids(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video1 = persist_video(&database).await.unwrap();
        let video2 = persist_video(&database).await.unwrap();
        let video3 = persist_video(&database).await.unwrap();

        let videos = database
            .videos
            .get_videos_by_ids(&[&video1.id, &video2.id])
            .await
            .unwrap();
        assert_eq!(videos.len(), 2);
        assert_eq!(videos[0].id, video1.id);
        assert_eq!(videos[1].id, video2.id);

        let videos = database
            .videos
            .get_videos_by_ids(&[&video1.id, &video3.id])
            .await
            .unwrap();
        assert_eq!(videos.len(), 2);
        assert_eq!(videos[0].id, video1.id);
        assert_eq!(videos[1].id, video3.id);
    }
}
