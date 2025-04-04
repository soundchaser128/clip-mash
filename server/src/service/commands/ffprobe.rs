use serde::{Deserialize, Serialize};
use tracing::{debug, info, Level};

use super::ffmpeg::FfmpegLocation;
use crate::util::commandline_error;
use crate::Result;

pub async fn ffprobe(path: impl AsRef<str>, location: &FfmpegLocation) -> Result<FfProbe> {
    use tokio::process::Command;

    info!("running ffprobe on {}", path.as_ref());

    let args = &[
        "-v",
        "error",
        "-print_format",
        "json",
        "-show_format",
        "-show_streams",
        path.as_ref(),
    ];
    debug!("running ffprobe with args {args:?}");
    let output = Command::new(location.ffprobe()).args(args).output().await?;
    if output.status.success() {
        if tracing::event_enabled!(Level::DEBUG) {
            let stdout = String::from_utf8_lossy(&output.stdout);
            debug!("ffprobe stdout: {}", stdout);
        }

        let json = serde_json::from_slice(&output.stdout)?;
        Ok(json)
    } else {
        commandline_error("ffprobe", output)
    }
}

pub async fn get_version(location: &FfmpegLocation) -> Result<String> {
    use tokio::process::Command;

    let output = Command::new(location.ffmpeg())
        .arg("-version")
        .output()
        .await?;
    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout);
        let version = version.lines().next().unwrap_or_default();
        Ok(version.to_string())
    } else {
        commandline_error("ffmpeg", output)
    }
}
