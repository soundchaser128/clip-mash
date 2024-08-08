use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use super::ffmpeg::FfmpegLocation;
use crate::util::commandline_error;
use crate::Result;

#[derive(Debug, PartialEq, Eq)]
pub struct VideoParameters {
    pub fps: String,
    pub width: i64,
    pub height: i64,
    pub codec: String,
}

#[derive(Deserialize, Serialize)]
pub struct FfProbe {
    pub streams: Vec<Stream>,
    pub format: Format,
}

impl FfProbe {
    pub fn duration(&self) -> Option<f64> {
        self.format
            .duration
            .as_deref()
            .and_then(|n| n.parse::<f64>().ok())
    }

    pub fn video_parameters(self) -> VideoParameters {
        let video_stream = self
            .streams
            .iter()
            .find(|s| s.codec_type.as_deref() == Some("video"));

        VideoParameters {
            fps: video_stream
                .map(|s| s.avg_frame_rate.clone())
                .unwrap_or_else(|| "N/A".into()),
            width: video_stream.and_then(|s| s.width).unwrap_or_default(),
            height: video_stream.and_then(|s| s.height).unwrap_or_default(),
            codec: video_stream
                .and_then(|s| s.codec_name.clone())
                .unwrap_or_else(|| "N/A".into()),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Stream {
    pub index: i64,
    pub codec_name: Option<String>,
    pub codec_long_name: Option<String>,
    pub codec_type: Option<String>,
    pub codec_time_base: Option<String>,

    pub sample_fmt: Option<String>,
    pub sample_rate: Option<String>,
    pub channels: Option<i64>,
    pub bits_per_sample: Option<i64>,
    pub r_frame_rate: String,
    pub avg_frame_rate: String,
    pub time_base: String,
    pub start_pts: Option<i64>,
    pub start_time: Option<String>,
    pub duration_ts: Option<i64>,
    pub duration: Option<String>,
    pub bit_rate: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub id: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Format {
    pub filename: String,
    pub nb_streams: i64,
    pub nb_programs: i64,
    pub format_name: String,
    pub format_long_name: String,
    pub start_time: Option<String>,
    pub duration: Option<String>,
    pub size: Option<String>,
    pub bit_rate: Option<String>,
    pub probe_score: i64,
    pub tags: Option<FormatTags>,
}

impl Format {
    pub fn duration(&self) -> Option<f64> {
        self.duration
            .as_ref()
            .and_then(|duration| duration.parse::<f64>().ok())
    }
}

#[derive(Deserialize, Serialize)]
pub struct FormatTags {
    #[serde(rename = "WMFSDKNeeded")]
    pub wmfsdkneeded: Option<String>,
    #[serde(rename = "DeviceConformanceTemplate")]
    pub device_conformance_template: Option<String>,
    #[serde(rename = "WMFSDKVersion")]
    pub wmfsdkversion: Option<String>,
    #[serde(rename = "IsVBR")]
    pub is_vbr: Option<String>,
    pub major_brand: Option<String>,
    pub minor_version: Option<String>,
    pub compatible_brands: Option<String>,
    pub creation_time: Option<String>,
    pub encoder: Option<String>,
}

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
        let json = serde_json::from_slice(&output.stdout)?;
        Ok(json)
    } else {
        commandline_error("ffprobe", output)
    }
}
