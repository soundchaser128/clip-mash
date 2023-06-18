use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use tracing::info;

use super::commands::ffmpeg::Ffmpeg;
use super::directories::Directories;
use crate::server::handlers::AppState;
use crate::Result;

pub struct PreviewGenerator {
    directories: Directories,
}

impl From<Arc<AppState>> for PreviewGenerator {
    fn from(state: Arc<AppState>) -> Self {
        PreviewGenerator {
            directories: state.directories.clone(),
        }
    }
}

impl PreviewGenerator {
    pub fn new(directories: Directories) -> Self {
        PreviewGenerator { directories }
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
            .join(format!("{}_{}.png", video_id, offset_seconds));
        self.generate_preview_inner(video_path, destination, offset_seconds)
            .await
    }

    async fn generate_preview_inner(
        &self,
        video_path: impl AsRef<Utf8Path>,
        preview_image_path: Utf8PathBuf,
        offset_seconds: f64,
    ) -> Result<Utf8PathBuf> {
        info!("generating preview image at {preview_image_path}");

        if preview_image_path.exists() {
            info!("preview image already exists at {preview_image_path}");
            return Ok(preview_image_path);
        }

        Ffmpeg::new(
            self.directories.ffmpeg_executable(),
            preview_image_path.to_string(),
        )
        .input(video_path.as_ref().as_str())
        .extra_arg("-frames:v")
        .extra_arg("1")
        .start(offset_seconds)
        .run()
        .await?;

        Ok(preview_image_path)
    }
}
