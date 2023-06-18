use camino::{Utf8Path, Utf8PathBuf};
use tracing::info;

use super::commands::ffmpeg::Ffmpeg;
use crate::Result;

pub async fn generate_preview_image(
    video_path: impl AsRef<Utf8Path>,
    offset_seconds: f64,
) -> Result<Utf8PathBuf> {
    let preview_image_path = video_path.as_ref().with_extension("png");
    if preview_image_path.exists() {
        info!("preview image already exists at {preview_image_path}");
        return Ok(preview_image_path);
    }

    Ffmpeg::new("ffmpeg", preview_image_path.to_string())
        .input(video_path.as_ref().as_str())
        .extra_arg("-frames:v")
        .extra_arg("1")
        .start(offset_seconds)
        .run()
        .await?;

    Ok(preview_image_path)
}
