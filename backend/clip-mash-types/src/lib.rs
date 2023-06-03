use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use typescript_type_def::TypeDef;

#[derive(Clone, Copy, Debug, Deserialize, TypeDef)]
#[serde(rename_all = "kebab-case")]
pub enum ClipOrder {
    Random,
    SceneOrder,
    Pmv,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub enum VideoSource {
    Stash,
    LocalFile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub source: VideoSource,
    pub video_id: VideoId,
    pub marker_id: MarkerId,
    /// Start and endpoint inside the video in seconds.
    pub range: (f64, f64),
    pub index_within_video: usize,
    pub index_within_marker: usize,
}

impl Clip {
    pub fn range_millis(&self) -> (u32, u32) {
        ((self.range.0 as u32) * 1000, (self.range.1 as u32) * 1000)
    }

    pub fn duration(&self) -> f64 {
        let (start, end) = self.range;
        end - start
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, TypeDef)]
#[serde(rename_all = "camelCase", tag = "type", content = "id")]
pub enum MarkerId {
    LocalFile(i64),
    Stash(i64),
}

impl MarkerId {
    pub fn inner(&self) -> i64 {
        match self {
            MarkerId::LocalFile(id) => *id,
            MarkerId::Stash(id) => *id,
        }
    }
}

impl fmt::Display for MarkerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarkerId::LocalFile(id) => write!(f, "{}", id),
            MarkerId::Stash(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, TypeDef)]
#[serde(rename_all = "camelCase", tag = "type", content = "id")]
pub enum VideoId {
    LocalFile(String),
    Stash(String),
}

impl VideoId {
    pub fn source(&self) -> VideoSource {
        match self {
            VideoId::LocalFile(_) => VideoSource::LocalFile,
            VideoId::Stash(_) => VideoSource::Stash,
        }
    }

    pub fn as_stash_id(&self) -> &str {
        if let Self::Stash(id) = self {
            id
        } else {
            panic!("this is not a stash ID")
        }
    }
}

impl fmt::Display for VideoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VideoId::LocalFile(id) => write!(f, "{}", id),
            VideoId::Stash(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Serialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct TagDto {
    pub name: String,
    pub id: String,
    pub marker_count: i64,
}

#[derive(Serialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct PerformerDto {
    pub id: String,
    pub scene_count: i64,
    pub name: String,
    pub image_url: Option<String>,
    pub tags: Vec<String>,
    pub rating: Option<i64>,
    pub favorite: bool,
}

#[derive(Serialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct MarkerDto {
    pub id: MarkerId,
    pub video_id: VideoId,
    pub primary_tag: String,
    pub stream_url: String,
    pub start: f64,
    pub end: f64,
    pub scene_title: Option<String>,
    pub performers: Vec<String>,
    pub file_name: Option<String>,
    pub scene_interactive: bool,
    pub tags: Vec<String>,
    pub screenshot_url: Option<String>,
    pub index_within_video: usize,
}

#[derive(Serialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct VideoDto {
    pub id: VideoId,
    pub title: String,
    pub performers: Vec<String>,
    pub file_name: String,
    pub interactive: bool,
}

#[derive(Deserialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct SelectedMarker {
    pub id: MarkerId,
    pub video_id: VideoId,
    pub selected_range: (f64, f64),
    pub index_within_video: usize,
    pub selected: Option<bool>,
}

#[derive(Deserialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct RandomizedClipOptions {
    pub base_duration: f64,
    pub divisors: Vec<f64>,
}

#[derive(Deserialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum MeasureCount {
    Fixed { count: usize },
    Random { min: usize, max: usize },
}

#[derive(Deserialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct SongClipOptions {
    pub beats_per_measure: usize,
    pub cut_after_measures: MeasureCount,
}

#[derive(Deserialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum PmvClipOptions {
    Randomized(RandomizedClipOptions),
    Songs(SongClipOptions),
}

#[derive(Deserialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ClipOptions {
    Pmv {
        song_ids: Vec<i64>,
        clips: PmvClipOptions,
    },
    Default(RandomizedClipOptions),
    NoSplit,
}

#[derive(Deserialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct CreateClipsBody {
    pub clip_order: ClipOrder,
    pub markers: Vec<SelectedMarker>,
    pub seed: Option<String>,
    pub clips: ClipOptions,
}

#[derive(Serialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct ClipsResponse {
    pub clips: Vec<Clip>,
    pub streams: HashMap<String, String>,
    pub videos: Vec<VideoDto>,
    pub beat_offsets: Option<Vec<f32>>,
}

#[derive(Serialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct ListVideoDto {
    pub video: VideoDto,
    pub markers: Vec<MarkerDto>,
}

#[derive(Debug, Clone, Copy, Deserialize, TypeDef)]
pub enum VideoResolution {
    #[serde(rename = "720")]
    SevenTwenty,
    #[serde(rename = "1080")]
    TenEighty,
    #[serde(rename = "4K")]
    FourK,
}

impl VideoResolution {
    pub fn resolution(&self) -> (u32, u32) {
        match self {
            Self::SevenTwenty => (1280, 720),
            Self::TenEighty => (1920, 1080),
            Self::FourK => (3840, 2160),
        }
    }
}

#[derive(Deserialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct CreateVideoBody {
    pub file_name: String,
    pub clips: Vec<Clip>,
    pub selected_markers: Vec<SelectedMarker>,
    pub output_resolution: VideoResolution,
    pub output_fps: u32,
    pub song_ids: Vec<i64>,
    pub music_volume: Option<f64>,
}

#[derive(Serialize, Debug, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct StashScene {
    pub id: String,
    pub performers: Vec<String>,
    pub image_url: Option<String>,
    pub title: String,
    pub studio: Option<String>,
    pub tags: Vec<String>,
    pub rating: Option<i64>,
    pub interactive: bool,
    pub marker_count: usize,
}

#[derive(Serialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct SongDto {
    pub song_id: i64,
    pub duration: f64,
    pub file_name: String,
    pub url: String,
    pub beats: Vec<f32>,
}

#[derive(Serialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct NewId {
    pub id: String,
}

pub type Api = (
    StashScene,
    CreateVideoBody,
    CreateClipsBody,
    ListVideoDto,
    ClipsResponse,
    VideoDto,
    MarkerDto,
    PerformerDto,
    TagDto,
    SongDto,
    NewId,
);
