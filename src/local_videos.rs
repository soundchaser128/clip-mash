use camino::{Utf8Path, Utf8PathBuf};
use nanoid::nanoid;
use serde::Serialize;
use tokio::fs;

use crate::{
    db::{Database, LocalVideo, LocalVideoWithMarkers},
    Result,
};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalVideoDto {
    pub id: String,
    pub file_name: String,
    pub interactive: bool,
    pub markers: Vec<MarkerDto>,
}
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkerDto {
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
                    start_time: m.start_time,
                    end_time: m.end_time,
                    title: m.title,
                })
                .collect(),
        }
    }
}

pub async fn list_videos(
    path: impl AsRef<Utf8Path>,
    database: &Database,
) -> Result<Vec<LocalVideoDto>> {
    let mut files = fs::read_dir(path.as_ref()).await?;
    let mut entries = vec![];
    while let Some(entry) = files.next_entry().await? {
        let path = Utf8PathBuf::from_path_buf(entry.path()).unwrap();
        entries.push(path);
    }
    let mut videos = vec![];
    for path in entries {
        if path.extension() == Some("mp4") {
            if let Some(video) = database.get_video_by_path(path.as_str()).await? {
                tracing::info!("found existing video {video:#?}");
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
