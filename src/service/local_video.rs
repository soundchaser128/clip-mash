use std::cmp::Reverse;
use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::bail;
use nanoid::nanoid;
use tokio::task::spawn_blocking;
use tracing::{debug, info};
use url::Url;
use walkdir::WalkDir;
use youtube_dl::YoutubeDl;

use super::directories::Directories;
use crate::data::database::{Database, DbVideo, LocalVideoWithMarkers};
use crate::server::handlers::AppState;
use crate::Result;

pub struct VideoService {
    database: Database,
    directories: Directories,
}

impl From<Arc<AppState>> for VideoService {
    fn from(value: Arc<AppState>) -> Self {
        VideoService {
            database: value.database.clone(),
            directories: value.directories.clone(),
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

    pub async fn list_videos(
        &self,
        path: impl AsRef<Utf8Path>,
        recurse: bool,
    ) -> Result<Vec<LocalVideoWithMarkers>> {
        let entries = self.gather_files(path.as_ref().to_owned(), recurse).await?;
        debug!("found files {entries:?} (recurse = {recurse})");
        let mut videos = vec![];
        for path in entries {
            if path.extension() == Some("mp4") {
                if let Some(video) = self.database.get_video_by_path(path.as_str()).await? {
                    debug!("found existing video {video:#?}");
                    videos.push(video);
                } else {
                    let interactive = path.with_extension("funscript").is_file();
                    let id = nanoid!(8);
                    let video = DbVideo {
                        id,
                        file_path: path.to_string(),
                        interactive,
                    };
                    info!("inserting new video {video:#?}");
                    self.database.persist_video(video.clone()).await?;
                    videos.push(LocalVideoWithMarkers {
                        video,
                        markers: vec![],
                    });
                }
            }
        }

        videos.sort_by_key(|v| Reverse(v.markers.len()));

        Ok(videos)
    }

    pub async fn download_video(&self, url: Url) -> Result<(String, Utf8PathBuf)> {
        use tokio::fs;

        let video_id = nanoid!(8);
        let folder = self
            .directories
            .video_dir()
            .join("downloads")
            .join(&video_id);
        if !folder.is_dir() {
            fs::create_dir_all(&folder).await?;
        }

        YoutubeDl::new(url)
            .output_directory(folder.as_str())
            .download(true)
            .run_async()
            .await?;

        let mut iterator = fs::read_dir(folder).await?;
        let entry = iterator.next_entry().await?;
        if let Some(entry) = entry {
            let path = Utf8PathBuf::from_path_buf(entry.path()).expect("path must be utf-8");
            info!("downloaded video to {path}");
            Ok((video_id, path))
        } else {
            bail!("could not find downloaded music file")
        }
    }

    pub async fn persist_video(&self, id: String, path: Utf8PathBuf) -> Result<DbVideo> {
        let video = DbVideo {
            id,
            file_path: path.as_str().to_string(),
            interactive: false,
        };
        self.database.persist_video(video.clone()).await?;
        Ok(video)
    }
}
