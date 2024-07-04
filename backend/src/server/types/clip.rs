use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Beats, MarkerGroup, SelectedMarker, VideoDto};
use crate::data::database::videos::VideoSource;
use crate::service::generator::PaddingType;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum ClipOrder {
    Random,
    Scene,
    NoOp,
    #[serde(rename_all = "camelCase")]
    Fixed {
        marker_title_groups: Vec<MarkerGroup>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub source: VideoSource,
    pub video_id: String,
    pub marker_id: i64,
    /// Start and endpoint inside the video in seconds.
    pub range: (f64, f64),
    pub index_within_video: usize,
    pub index_within_marker: usize,
    pub marker_title: String,
}

impl Clip {
    pub fn range_millis(&self) -> (u32, u32) {
        let start = (self.range.0 * 1000.0).round() as u32;
        let end = (self.range.1 * 1000.0).round() as u32;
        (start, end)
    }

    pub fn duration(&self) -> f64 {
        let (start, end) = self.range;
        end - start
    }

    pub fn duration_millis(&self) -> u32 {
        (self.duration() * 1000.0) as u32
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum VideoCodec {
    Av1,
    H264,
    H265,
}

impl fmt::Display for VideoCodec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Av1 => write!(f, "av1"),
            Self::H264 => write!(f, "h264"),
            Self::H265 => write!(f, "h265"),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum VideoQuality {
    Low,
    Medium,
    High,
    Lossless,
}

#[derive(Deserialize, Debug, Clone, Copy, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum EncodingEffort {
    Low,
    Medium,
    High,
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateVideoBody {
    pub video_id: String,
    pub file_name: String,
    pub clips: Vec<Clip>,
    pub selected_markers: Vec<SelectedMarker>,
    pub output_resolution: (u32, u32),
    pub output_fps: u32,
    pub song_ids: Vec<i64>,
    pub music_volume: Option<f64>,
    pub video_codec: VideoCodec,
    pub video_quality: VideoQuality,
    pub encoding_effort: EncodingEffort,
    pub padding: Option<PaddingType>,
    pub force_re_encode: bool,
}

#[derive(Deserialize, Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RandomizedClipOptions {
    pub base_duration: f64,
    pub spread: f64,
}

#[derive(Deserialize, Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum MeasureCount {
    Fixed { count: usize },
    Random { min: usize, max: usize },
}

#[derive(Deserialize, Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SongClipOptions {
    pub beats_per_measure: usize,
    pub cut_after_measures: MeasureCount,
    pub songs: Vec<Beats>,
}

#[derive(Deserialize, Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ClipLengthOptions {
    Randomized(RandomizedClipOptions),
    Songs(SongClipOptions),
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClipOptions {
    pub clip_picker: ClipPickerOptions,
    pub order: ClipOrder,
}

#[derive(Deserialize, Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ClipPickerOptions {
    RoundRobin(RoundRobinClipOptions),
    WeightedRandom(WeightedRandomClipOptions),
    EqualLength(EqualLengthClipOptions),
    NoSplit,
}

impl ClipPickerOptions {
    pub fn clip_lengths(&self) -> Option<&ClipLengthOptions> {
        match self {
            ClipPickerOptions::RoundRobin(opts) => Some(&opts.clip_lengths),
            ClipPickerOptions::WeightedRandom(opts) => Some(&opts.clip_lengths),
            ClipPickerOptions::EqualLength(_) => None,
            ClipPickerOptions::NoSplit => None,
        }
    }

    pub fn has_music(&self) -> bool {
        matches!(self.clip_lengths(), Some(ClipLengthOptions::Songs(_)))
    }

    pub fn songs(&self) -> Option<&[Beats]> {
        if let Some(ClipLengthOptions::Songs(songs)) = self.clip_lengths() {
            Some(&songs.songs)
        } else {
            None
        }
    }
}

#[derive(Deserialize, Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoundRobinClipOptions {
    pub length: f64,
    pub clip_lengths: ClipLengthOptions,
    pub lenient_duration: bool,
    pub min_clip_duration: Option<f64>,
}

#[derive(Deserialize, Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WeightedRandomClipOptions {
    pub weights: Vec<(String, f64)>,
    pub length: f64,
    pub clip_lengths: ClipLengthOptions,
    pub min_clip_duration: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EqualLengthClipOptions {
    pub clip_duration: f64,
    pub spread: f64,
    pub length: Option<f64>,
    pub min_clip_duration: Option<f64>,
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateClipsBody {
    pub markers: Vec<SelectedMarker>,
    pub seed: Option<String>,
    pub clips: ClipOptions,
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum InteractiveClipsQuery {
    MarkerTitles { data: Vec<String> },
    Performers { data: Vec<String> },
    VideoTags { data: Vec<String> },
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateInteractiveClipsBody {
    pub query: InteractiveClipsQuery,
    pub clip_duration: f64,
    pub order: ClipOrder,
    pub seed: Option<String>,
}

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClipsResponse {
    pub clips: Vec<Clip>,
    pub streams: HashMap<String, String>,
    pub videos: Vec<VideoDto>,
    pub beat_offsets: Option<Vec<f32>>,
}
