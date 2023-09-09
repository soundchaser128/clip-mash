use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use tokio::task::spawn_blocking;
use tracing::{debug, info, warn};
use url::Url;
use walkdir::WalkDir;

use super::commands::ffmpeg::FfmpegLocation;
use super::directories::Directories;
use crate::data::database::{Database, DbVideo, LocalVideoSource};
use crate::server::handlers::AppState;
use crate::service::commands::{ffprobe, YtDlp, YtDlpOptions};
use crate::service::directories::FolderType;
use crate::service::preview_image::PreviewGenerator;
use crate::util::generate_id;
use crate::Result;

pub struct VideoService {
    database: Database,
    directories: Directories,
    ffmpeg_location: FfmpegLocation,
}

impl From<Arc<AppState>> for VideoService {
    fn from(value: Arc<AppState>) -> Self {
        VideoService {
            database: value.database.clone(),
            directories: value.directories.clone(),
            ffmpeg_location: value.ffmpeg_location.clone(),
        }
    }
}

impl VideoService {
    async fn gather_files(&self, path: Utf8PathBuf, recurse: bool) -> Result<Vec<Utf8PathBuf>> {
        spawn_blocking(move || {
            let files = WalkDir::new(path)
                .max_depth(if recurse { usize::MAX } else { 1 })
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .map(|e| Utf8PathBuf::from_path_buf(e.into_path()).expect("not a utf8 path"))
                .collect();

            Ok(files)
        })
        .await?
    }

    pub async fn add_new_videos(&self, path: impl AsRef<Utf8Path>, recurse: bool) -> Result<()> {
        let entries = self.gather_files(path.as_ref().to_owned(), recurse).await?;
        debug!("found files {entries:?} (recurse = {recurse})");
        for path in entries {
            if path.extension() == Some("mp4") || path.extension() == Some("m4v") {
                let video_exists = self
                    .database
                    .get_video_by_path(path.as_str())
                    .await?
                    .is_some();
                info!("video at path {path} exists: {video_exists}");
                if !video_exists {
                    let interactive = path.with_extension("funscript").is_file();
                    let ffprobe = ffprobe(&path, &self.ffmpeg_location).await;
                    if let Err(e) = ffprobe {
                        warn!("skipping video {path} because ffprobe failed with error {e}");
                        continue;
                    }
                    let ffprobe = ffprobe.unwrap();
                    let duration = ffprobe.duration();
                    let id = generate_id();
                    let preview_generator = PreviewGenerator::new(
                        self.directories.clone(),
                        self.ffmpeg_location.clone(),
                    );
                    let image_path = preview_generator
                        .generate_preview(&id, &path, duration.map_or(0.0, |d| d / 2.0))
                        .await?;

                    let video = DbVideo {
                        id,
                        file_path: path.to_string(),
                        interactive,
                        source: LocalVideoSource::Folder,
                        duration: duration.unwrap_or_default(),
                        video_preview_image: Some(image_path.to_string()),
                        stash_scene_id: None,
                    };
                    info!("inserting new video {video:#?}");
                    self.database.videos.persist_video(video.clone()).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn download_video(&self, url: Url) -> Result<(String, Utf8PathBuf)> {
        info!("downloading video {url}");
        let downloader = YtDlp::new(self.directories.clone());
        let options = YtDlpOptions {
            url,
            extract_audio: false,
            destination: FolderType::DownloadedVideo,
        };
        let result = downloader.run(&options, &self.ffmpeg_location).await?;
        Ok((result.generated_id, result.downloaded_file))
    }

    pub async fn persist_downloaded_video(&self, id: String, path: Utf8PathBuf) -> Result<DbVideo> {
        let ffprobe = ffprobe(&path, &self.ffmpeg_location).await?;
        let duration = ffprobe.duration();
        let preview_generator =
            PreviewGenerator::new(self.directories.clone(), self.ffmpeg_location.clone());
        let image_path = preview_generator
            .generate_preview(&id, &path, duration.map_or(0.0, |d| d / 2.0))
            .await?;

        let video = DbVideo {
            id,
            file_path: path.as_str().to_string(),
            interactive: false,
            source: LocalVideoSource::Download,
            duration: duration.unwrap_or_default(),
            video_preview_image: Some(image_path.to_string()),
            stash_scene_id: None,
        };
        info!("persisting downloaded video {video:#?}");
        self.database.videos.persist_video(video.clone()).await?;
        Ok(video)
    }
}
