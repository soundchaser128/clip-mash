use camino::{Utf8Path, Utf8PathBuf};
use nanoid::nanoid;
use serde::Serialize;
use tokio::fs;

use crate::{db::LocalVideo, Result};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalVideoDto {
    pub id: String,
    pub file_name: String,
    pub interactive: bool,
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
        }
    }
}

pub async fn list_videos(path: impl AsRef<Utf8Path>) -> Result<Vec<LocalVideo>> {
    let mut files = fs::read_dir(path.as_ref()).await?;
    let mut entries = vec![];
    while let Some(entry) = files.next_entry().await? {
        let path = Utf8PathBuf::from_path_buf(entry.path()).unwrap();
        entries.push(path);
    }
    let mut videos = vec![];
    for path in entries {
        if path.extension() == Some("mp4") {
            let interactive = path.with_extension("funscript").is_file();
            let id = nanoid!(8);
            videos.push(LocalVideo {
                file_path: path.to_string(),
                interactive,
                id,
            })
        }
    }

    Ok(videos)
}
