use camino::{Utf8Path, Utf8PathBuf};
use nanoid::nanoid;
use serde::Serialize;
use tokio::fs;

use crate::Result;

#[derive(Serialize, Debug)]
pub struct LocalVideo {
    pub id: String,
    pub path: Utf8PathBuf,
    pub interactive: bool,
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
                path,
                interactive,
                id,
            })
        }
    }

    Ok(videos)
}
