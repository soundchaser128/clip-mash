use std::str::FromStr;
use std::time::SystemTime;

use color_eyre::eyre::OptionExt;
use performers::PerformersDatabase;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use tracing::info;

use self::ffprobe::FfProbeInfoDatabase;
use self::markers::MarkersDatabase;
use self::music::MusicDatabase;
use self::progress::ProgressDatabase;
use self::settings::SettingsDatabase;
pub use self::settings::{HandyConfig, Settings};
use self::videos::VideosDatabase;
use crate::Result;
use crate::types::Progress;

pub mod ffprobe;
pub mod markers;
pub mod music;
pub mod performers;
pub mod progress;
pub mod settings;
pub mod videos;

#[derive(Clone)]
pub struct Database {
    pub videos: VideosDatabase,
    pub markers: MarkersDatabase,
    pub progress: ProgressDatabase,
    pub music: MusicDatabase,
    pub ffprobe: FfProbeInfoDatabase,
    pub settings: SettingsDatabase,
    pub performers: PerformersDatabase,
}

impl Database {
    pub async fn new(path: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(&format!("sqlite:{path}?mode=rwc"))?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal);

        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!().run(&pool).await?;
        info!("ran sqlx migrations");

        Ok(Database {
            markers: MarkersDatabase::new(pool.clone()),
            progress: ProgressDatabase::new(pool.clone()),
            music: MusicDatabase::new(pool.clone()),
            ffprobe: FfProbeInfoDatabase::new(pool.clone()),
            videos: VideosDatabase::new(pool.clone()),
            settings: SettingsDatabase::new(pool.clone()),
            performers: PerformersDatabase::new(pool.clone()),
        })
    }

    pub async fn sqlite_version(&self) -> Result<String> {
        let version = sqlx::query_scalar!("select sqlite_version()")
            .fetch_one(&self.progress.pool)
            .await?;

        version.ok_or_eyre("no version found")
    }

    #[cfg(test)]
    pub fn with_pool(pool: SqlitePool) -> Self {
        Database {
            markers: MarkersDatabase::new(pool.clone()),
            progress: ProgressDatabase::new(pool.clone()),
            music: MusicDatabase::new(pool.clone()),
            ffprobe: FfProbeInfoDatabase::new(pool.clone()),
            videos: VideosDatabase::new(pool.clone()),
            settings: SettingsDatabase::new(pool.clone()),
            performers: PerformersDatabase::new(pool.clone()),
        }
    }
}

pub fn unix_timestamp_now() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[cfg(test)]
mod test {
    use sqlx::SqlitePool;
    use tracing_test::traced_test;

    use super::videos::{VideoSearchQuery, VideoSource, VideoUpdate};
    use crate::Result;
    use crate::data::database::Database;
    use crate::data::database::markers::ListMarkersFilter;
    use crate::helpers::random::generate_id;
    use crate::service::fixtures::{persist_marker, persist_video, persist_video_with};
    use crate::types::{CreateMarker, PageParameters, SortDirection, UpdateMarker};

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
            created_on: None,
            marker_stash_id: None,
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
            created_on: None,
            marker_stash_id: None,
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
            created_on: None,
            marker_stash_id: None,
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
        assert!(exists);
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
        let (result, total) = database
            .videos
            .list_videos(VideoSearchQuery::default(), &page)
            .await
            .unwrap();
        assert_eq!(45, total);
        assert_eq!(10, result.len());
    }

    #[sqlx::test]
    #[traced_test]
    async fn test_list_videos_with_source(pool: SqlitePool) -> Result<()> {
        let database = Database::with_pool(pool);
        for _ in 0..5 {
            persist_video_with(&database, |v| {
                v.source = VideoSource::Stash;
            })
            .await?;
        }

        for _ in 0..5 {
            persist_video_with(&database, |v| {
                v.source = VideoSource::Folder;
            })
            .await?;
        }
        let params = PageParameters::new(0, 10);
        let (stash_videos, _) = database
            .videos
            .list_videos(
                VideoSearchQuery {
                    source: Some(VideoSource::Stash),
                    ..Default::default()
                },
                &params,
            )
            .await?;
        assert_eq!(5, stash_videos.len());
        for video in stash_videos {
            assert_eq!(video.video.source, VideoSource::Stash);
        }

        let (folder_videos, _) = database
            .videos
            .list_videos(
                VideoSearchQuery {
                    source: Some(VideoSource::Folder),
                    ..Default::default()
                },
                &params,
            )
            .await?;
        assert_eq!(5, folder_videos.len());
        for video in folder_videos {
            assert_eq!(video.video.source, VideoSource::Folder);
        }

        Ok(())
    }

    #[sqlx::test]
    #[traced_test]
    async fn test_list_videos_search(pool: SqlitePool) {
        let database = Database::with_pool(pool);

        for _ in 0..45 {
            persist_video(&database).await.unwrap();
        }
        for i in 0..10 {
            persist_video_with(&database, |v| {
                v.title = Some("sexy".into());
                v.file_path = format!("/path/{i}/sexy.mp4");
            })
            .await
            .unwrap();
        }

        for i in 0..5 {
            persist_video_with(&database, |v| {
                v.title = Some("cool".into());
                v.file_path = format!("/path/{i}/cool.mp4");
            })
            .await
            .unwrap();
        }

        let page = PageParameters {
            page: Some(0),
            size: Some(10),
            sort: None,
            dir: None,
        };
        let (result, total) = database
            .videos
            .list_videos(
                VideoSearchQuery {
                    query: Some("sexy".into()),
                    ..Default::default()
                },
                &page,
            )
            .await
            .unwrap();
        assert_eq!(10, total);
        assert_eq!(10, result.len());
        let file_name = &result[0].video.file_name;
        assert_eq!(file_name, "sexy.mp4");
        assert_eq!(result[0].video.title.as_str(), "sexy");

        let (result, total) = database
            .videos
            .list_videos(
                VideoSearchQuery {
                    query: Some("cool".into()),
                    ..Default::default()
                },
                &page,
            )
            .await
            .unwrap();
        assert_eq!(5, total);
        assert_eq!(5, result.len());
        let file_name = &result[0].video.file_name;
        assert_eq!(file_name, "cool.mp4");
        assert_eq!(result[0].video.title.as_str(), "cool");
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
            title: Some(marker.title),
            start: Some(5.0),
            end: Some(15.0),
            ..Default::default()
        };
        database
            .markers
            .update_marker(marker.rowid.unwrap(), update)
            .await
            .unwrap();
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
    async fn test_list_markers(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await.unwrap();
        for i in 0..5 {
            let start = i as f64;
            persist_marker(&database, &video.id, i, start, start + 5.0, false)
                .await
                .unwrap();
        }
        let result = database
            .markers
            .list_markers(Some(ListMarkersFilter::VideoIds(vec![video.id])), None)
            .await
            .unwrap();
        assert_eq!(5, result.len());

        let result = database.markers.list_markers(None, None).await.unwrap();
        assert_eq!(5, result.len());
    }

    #[sqlx::test]
    #[traced_test]
    async fn test_set_marker_preview_image(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await.unwrap();
        let marker = persist_marker(&database, &video.id, 0, 0.0, 15.0, false)
            .await
            .unwrap();
        let preview_image_path = "/some/path/to/image.png";
        database
            .markers
            .set_marker_preview_image(marker.rowid.unwrap(), Some(preview_image_path))
            .await
            .unwrap();
        let result = database
            .markers
            .get_marker(marker.rowid.unwrap())
            .await
            .unwrap();
        assert_eq!(result.marker_preview_image, Some(preview_image_path.into()));
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
            persist_video_with(&database, |v| {
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
        let (_videos, count) = database
            .videos
            .list_videos(VideoSearchQuery::default(), &params)
            .await
            .unwrap();
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
        let ids: Vec<&str> = videos.iter().map(|v| v.id.as_str()).collect();
        assert!(ids.contains(&video1.id.as_str()));
        assert!(ids.contains(&video2.id.as_str()));

        let videos = database
            .videos
            .get_videos_by_ids(&[&video1.id, &video3.id])
            .await
            .unwrap();
        assert_eq!(videos.len(), 2);
        let ids: Vec<&str> = videos.iter().map(|v| v.id.as_str()).collect();
        assert!(ids.contains(&video1.id.as_str()));
        assert!(ids.contains(&video3.id.as_str()));
    }

    #[sqlx::test]
    #[traced_test]
    async fn test_update_video(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await.unwrap();
        let update = VideoUpdate {
            title: Some("Some title".into()),
            tags: Some(vec!["tag1".into(), "tag2".into()]),
        };
        database
            .videos
            .update_video(&video.id, update)
            .await
            .unwrap();
        let video = database.videos.get_video(&video.id).await.unwrap().unwrap();
        assert_eq!(video.video_title, Some("Some title".into()));
        assert_eq!(video.tags(), vec!["tag1", "tag2"]);
    }

    #[sqlx::test]
    #[traced_test]
    async fn test_delete_video(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await.unwrap();
        database.videos.delete_video(&video.id).await.unwrap();
        let video = database.videos.get_video(&video.id).await.unwrap();
        assert!(video.is_none());
    }

    #[sqlx::test]
    #[traced_test]
    async fn list_videos_has_markers(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let video1 = persist_video(&database).await.unwrap();
        let video2 = persist_video(&database).await.unwrap();
        let video3 = persist_video(&database).await.unwrap();
        persist_marker(&database, &video1.id, 0, 0.0, 15.0, false)
            .await
            .unwrap();
        persist_marker(&database, &video2.id, 0, 0.0, 15.0, false)
            .await
            .unwrap();
        let params = PageParameters::new(0, 10);
        let (videos, _) = database
            .videos
            .list_videos(
                VideoSearchQuery {
                    has_markers: Some(true),
                    ..Default::default()
                },
                &params,
            )
            .await
            .unwrap();
        assert_eq!(videos.len(), 2);
        let ids: Vec<&str> = videos.iter().map(|v| v.video.id.as_str()).collect();
        assert!(ids.contains(&video1.id.as_str()));
        assert!(ids.contains(&video2.id.as_str()));
        assert!(!ids.contains(&video3.id.as_str()));
    }

    #[sqlx::test]
    #[traced_test]
    async fn list_video_tags(pool: SqlitePool) -> Result<()> {
        let database = Database::with_pool(pool);
        persist_video_with(&database, |v| {
            v.tags = Some(serde_json::to_string(&["tag1", "tag2"]).unwrap());
        })
        .await?;
        persist_video_with(&database, |v| {
            v.tags = Some(serde_json::to_string(&["tag1", "tag3"]).unwrap())
        })
        .await?;

        let tags = database.videos.list_tags(100).await?;
        assert_eq!(3, tags.len());
        assert_eq!("tag1", tags[0].tag);
        assert_eq!(2, tags[0].count);

        Ok(())
    }

    #[sqlx::test]
    #[traced_test]
    async fn get_video_with_markers_bug(pool: SqlitePool) -> Result<()> {
        let database = Database::with_pool(pool);
        let video = persist_video(&database).await?;
        let data = database
            .videos
            .get_video_with_markers(&video.id)
            .await?
            .unwrap();
        assert_eq!(data.markers.len(), 0);
        assert_eq!(data.video.id, video.id);
        Ok(())
    }
}
