use camino::Utf8Path;
use serde::Deserialize;
use tracing::info;

use crate::util::commandline_error;
use crate::Result;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FfProbe {
    pub streams: Vec<Stream>,
    pub format: Format,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Stream {
    pub index: i64,
    pub codec_name: Option<String>,
    pub sample_aspect_ratio: Option<String>,
    pub display_aspect_ratio: Option<String>,
    pub color_range: Option<String>,
    pub color_space: Option<String>,
    pub bits_per_raw_sample: Option<String>,
    pub channel_layout: Option<String>,
    pub max_bit_rate: Option<String>,
    pub nb_frames: Option<String>,
    /// Number of frames seen by the decoder.
    /// Requires full decoding and is only available if the 'count_frames'
    /// setting was enabled.
    pub nb_read_frames: Option<String>,
    pub codec_long_name: Option<String>,
    pub codec_type: Option<String>,
    pub codec_time_base: Option<String>,
    pub codec_tag_string: String,
    pub codec_tag: String,
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
    pub disposition: Disposition,
    pub tags: Option<StreamTags>,
    pub profile: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub coded_width: Option<i64>,
    pub coded_height: Option<i64>,
    pub closed_captions: Option<i64>,
    pub has_b_frames: Option<i64>,
    pub pix_fmt: Option<String>,
    pub level: Option<i64>,
    pub chroma_location: Option<String>,
    pub refs: Option<i64>,
    pub is_avc: Option<String>,
    pub nal_length: Option<String>,
    pub nal_length_size: Option<String>,
    pub field_order: Option<String>,
    pub id: Option<String>,
    #[serde(default)]
    pub side_data_list: Vec<SideData>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SideData {
    pub side_data_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Disposition {
    pub default: i64,
    pub dub: i64,
    pub original: i64,
    pub comment: i64,
    pub lyrics: i64,
    pub karaoke: i64,
    pub forced: i64,
    pub hearing_impaired: i64,
    pub visual_impaired: i64,
    pub clean_effects: i64,
    pub attached_pic: i64,
    pub timed_thumbnails: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StreamTags {
    pub language: Option<String>,
    pub creation_time: Option<String>,
    pub handler_name: Option<String>,
    pub encoder: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

pub async fn ffprobe(path: impl AsRef<Utf8Path>) -> Result<FfProbe> {
    use tokio::process::Command;

    // TODO use ffmpeg path
    let args = &[
        "-v",
        "error",
        "-print_format",
        "json",
        "-show_format",
        "-show_streams",
        path.as_ref().as_str(),
    ];

    info!("running ffprobe with args {args:?}");
    let output = Command::new("ffprobe").args(args).output().await?;
    if output.status.success() {
        let json = serde_json::from_slice(&output.stdout)?;
        Ok(json)
    } else {
        commandline_error("ffprobe", output)
    }
}
