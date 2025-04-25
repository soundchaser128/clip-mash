use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use tracing::info;

use super::commands::ffmpeg::{Ffmpeg, FfmpegLocation};
use super::directories::Directories;
use crate::Result;
use crate::server::handlers::AppState;

pub struct PreviewGenerator {
    directories: Directories,
    ffmpeg_location: FfmpegLocation,
}

impl From<Arc<AppState>> for PreviewGenerator {
    fn from(state: Arc<AppState>) -> Self {
        PreviewGenerator {
            directories: state.directories.clone(),
            ffmpeg_location: state.ffmpeg_location.clone(),
        }
    }
}

impl PreviewGenerator {
    pub fn new(directories: Directories, ffmpeg_location: FfmpegLocation) -> Self {
        PreviewGenerator {
            directories,
            ffmpeg_location,
        }
    }

    pub async fn generate_preview(
        &self,
        video_id: &str,
        video_path: impl AsRef<Utf8Path>,
        offset_seconds: f64,
    ) -> Result<Utf8PathBuf> {
        let destination = self
            .directories
            .preview_image_dir()
            .join(format!("{}_{}.webp", video_id, offset_seconds));

        info!("generating preview image at {destination}");

        if destination.exists() {
            info!("preview image already exists at {destination}");
            return Ok(destination);
        }

        Ffmpeg::new(&self.ffmpeg_location, destination.to_string())
            .input(video_path.as_ref().as_str())
            .extra_arg("-frames:v")
            .extra_arg("1")
            .start(offset_seconds)
            .video_filter("scale=800:-1")
            .extra_arg("-quality")
            .extra_arg("90")
            .run()
            .await?;

        Ok(destination)
    }
}
