use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use futures::future;
use serde::Deserialize;
use tokio::task::spawn_blocking;
use tracing::{debug, info, warn};
use url::Url;
use utoipa::ToSchema;
use walkdir::WalkDir;

use super::commands::ffmpeg::FfmpegLocation;
use super::directories::Directories;
use crate::data::database::{CreateVideo, Database, DbVideo, VideoSource};
use crate::data::stash_api::StashApi;
use crate::server::handlers::AppState;
use crate::service::commands::{ffprobe, YtDlp, YtDlpOptions};
use crate::service::directories::FolderType;
use crate::service::preview_image::PreviewGenerator;
use crate::util::generate_id;
use crate::Result;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AddVideosRequest {
    Local { path: String, recurse: bool },
    Download { urls: Vec<String> },
    Stash { scene_ids: Vec<i64> },
}

pub struct VideoService {
    database: Database,
    directories: Directories,
    ffmpeg_location: FfmpegLocation,
    stash_api: StashApi,
}

impl VideoService {
    pub async fn new(state: Arc<AppState>) -> Result<Self> {
        Ok(VideoService {
            database: state.database.clone(),
            directories: state.directories.clone(),
            ffmpeg_location: state.ffmpeg_location.clone(),
            stash_api: StashApi::load_config().await?,
        })
    }

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

    async fn add_new_local_videos(
        &self,
        path: impl AsRef<Utf8Path>,
        recurse: bool,
    ) -> Result<Vec<DbVideo>> {
        let entries = self.gather_files(path.as_ref().to_owned(), recurse).await?;
        let mut videos = vec![];
        debug!("found files {entries:?} (recurse = {recurse})");
        for path in entries {
            if path.extension() == Some("mp4") || path.extension() == Some("m4v") {
                let video_exists = self
                    .database
                    .videos
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

                    let create_video = CreateVideo {
                        id,
                        file_path: path.to_string(),
                        interactive,
                        source: VideoSource::Folder,
                        duration: duration.unwrap_or_default(),
                        video_preview_image: Some(image_path.to_string()),
                        stash_scene_id: None,
                    };
                    info!("inserting new video {create_video:#?}");
                    let video = self.database.videos.persist_video(&create_video).await?;
                    videos.push(video);
                }
            }
        }
        Ok(videos)
    }

    async fn download_video(&self, url: Url) -> Result<(String, Utf8PathBuf)> {
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

    async fn persist_downloaded_video(&self, id: String, path: Utf8PathBuf) -> Result<DbVideo> {
        let ffprobe = ffprobe(&path, &self.ffmpeg_location).await?;
        let duration = ffprobe.duration();
        let preview_generator =
            PreviewGenerator::new(self.directories.clone(), self.ffmpeg_location.clone());
        let image_path = preview_generator
            .generate_preview(&id, &path, duration.map_or(0.0, |d| d / 2.0))
            .await?;

        let video = CreateVideo {
            id,
            file_path: path.as_str().to_string(),
            interactive: false,
            source: VideoSource::Download,
            duration: duration.unwrap_or_default(),
            video_preview_image: Some(image_path.to_string()),
            stash_scene_id: None,
        };
        info!("persisting downloaded video {video:#?}");

        self.database.videos.persist_video(&video).await
    }

    async fn persist_stash_video(&self, scene_ids: Vec<i64>) -> Result<Vec<DbVideo>> {
        let scenes = self.stash_api.find_scenes_by_ids(scene_ids).await?;
        let create_videos: Vec<_> = scenes
            .into_iter()
            .map(|scene| CreateVideo {
                id: generate_id(),
                file_path: scene.files[0].basename.clone(),
                interactive: scene.interactive,
                source: VideoSource::Stash,
                duration: scene.files[0].duration,
                // TODO
                video_preview_image: Some(self.stash_api.get_screenshot_url(&scene.id)),
                stash_scene_id: Some(scene.id.parse().unwrap()),
            })
            .collect();

        let mut videos = vec![];
        for request in &create_videos {
            let result = self.database.videos.persist_video(&request).await?;
            videos.push(result);
        }

        Ok(videos)
    }

    pub async fn add_videos(&self, request: AddVideosRequest) -> Result<Vec<DbVideo>> {
        match request {
            AddVideosRequest::Local { path, recurse } => {
                self.add_new_local_videos(path, recurse).await
            }
            AddVideosRequest::Download { urls } => {
                let futures = future::try_join_all(urls.into_iter().map(|url| async move {
                    let (id, path) = self.download_video(url.parse()?).await?;
                    self.persist_downloaded_video(id, path).await
                }));

                futures.await
            }
            AddVideosRequest::Stash { scene_ids } => self.persist_stash_video(scene_ids).await,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::data::stash_api::StashApi;
    use crate::service::fixtures;
    use crate::Result;

    #[tokio::test]
    #[ignore]
    async fn test_add_local_videos() -> Result<()> {
        let server = fixtures::stash_mock_server().await;

        Ok(())
    }
}
