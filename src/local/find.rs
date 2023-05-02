use super::db::DbVideo;
use crate::{
    local::db::{Database, LocalVideoWithMarkers},
    Result,
};
use camino::{Utf8Path, Utf8PathBuf};
use nanoid::nanoid;
use tokio::task::spawn_blocking;
use walkdir::WalkDir;

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
) -> Result<Vec<LocalVideoWithMarkers>> {
    let entries = gather_files(path.as_ref().to_owned(), recurse).await?;
    tracing::debug!("found files {entries:?} (recurse = {recurse})");
    let mut videos = vec![];
    for path in entries {
        if path.extension() == Some("mp4") {
            if let Some(video) = database.get_video_by_path(path.as_str()).await? {
                tracing::debug!("found existing video {video:#?}");
                videos.push(video);
            } else {
                let interactive = path.with_extension("funscript").is_file();
                let id = nanoid!(8);
                let video = DbVideo {
                    id,
                    file_path: path.to_string(),
                    interactive,
                };
                tracing::info!("inserting new video {video:#?}");
                database.persist_video(video.clone()).await?;
                videos.push(LocalVideoWithMarkers {
                    video,
                    markers: vec![],
                });
            }
        }
    }

    Ok(videos)
}
