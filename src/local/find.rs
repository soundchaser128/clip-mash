use camino::{Utf8Path, Utf8PathBuf};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;
use walkdir::WalkDir;

use crate::{
    local::db::{Database, LocalVideo, LocalVideoWithMarkers},
    Result,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalVideoDto {
    pub id: String,
    pub file_name: String,
    pub interactive: bool,
    pub markers: Vec<MarkerDto>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkerDto {
    pub rowid: Option<i64>,
    pub start_time: f64,
    pub end_time: f64,
    pub title: String,
}

impl From<LocalVideo> for LocalVideoDto {
    fn from(video: LocalVideo) -> Self {
        LocalVideoDto {
            id: video.id,
            file_name: Utf8PathBuf::from(video.file_path)
                .file_name()
                .expect("video must have a file name")
                .into(),
            interactive: video.interactive,
            markers: vec![],
        }
    }
}

impl From<LocalVideoWithMarkers> for LocalVideoDto {
    fn from(video: LocalVideoWithMarkers) -> Self {
        LocalVideoDto {
            id: video.video.id,
            file_name: Utf8PathBuf::from(video.video.file_path)
                .file_name()
                .expect("video must have a file name")
                .into(),
            interactive: video.video.interactive,
            markers: video
                .markers
                .into_iter()
                .map(|m| MarkerDto {
                    rowid: m.rowid,
                    start_time: m.start_time,
                    end_time: m.end_time,
                    title: m.title,
                })
                .collect(),
        }
    }
}

async fn gather_files(path: Utf8PathBuf, recurse: bool) -> Result<Vec<Utf8PathBuf>> {
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
    path: impl AsRef<Utf8Path>,
    recurse: bool,
    database: &Database,
) -> Result<Vec<LocalVideoDto>> {
    let entries = gather_files(path.as_ref().to_owned(), recurse).await?;
    tracing::debug!("found files {entries:?} (recurse = {recurse})");
    let mut videos = vec![];
    for path in entries {
        if path.extension() == Some("mp4") {
            if let Some(video) = database.get_video_by_path(path.as_str()).await? {
                tracing::debug!("found existing video {video:#?}");
                videos.push(video.into());
            } else {
                let interactive = path.with_extension("funscript").is_file();
                let id = nanoid!(8);
                let video = LocalVideo {
                    id,
                    file_path: path.to_string(),
                    interactive,
                };
                tracing::info!("inserting new video {video:#?}");
                database.persist_video(video.clone()).await?;
                videos.push(video.into());
            }
        }
    }

    Ok(videos)
}
