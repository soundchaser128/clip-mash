use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::data::database::{DbMarker, DbVideo, VideoSource};
use crate::util::expect_file_name;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ClipOrder {
    Random,
    SceneOrder,
    NoOp,
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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagDto {
    pub name: String,
    pub id: String,
    pub marker_count: i64,
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MarkerDto {
    pub id: i64,
    pub video_id: String,
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
    pub source: VideoSource,
}

impl MarkerDto {
    pub fn from_db(marker: DbMarker, video: &DbVideo) -> Self {
        MarkerDto {
            id: marker.rowid.expect("marker must have rowid"),
            video_id: video.id.clone(),
            primary_tag: marker.title,
            stream_url: format!("/api/local/video/{}/file", video.id),
            start: marker.start_time,
            end: marker.end_time,
            scene_title: video.video_title.clone(),
            performers: vec![],
            file_name: Some(expect_file_name(&video.file_path)),
            scene_interactive: video.interactive,
            tags: video.tags().unwrap_or_default(),
            screenshot_url: marker.marker_preview_image,
            index_within_video: marker.index_within_video as usize,
            source: video.source,
        }
    }
}

#[derive(Serialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoDto {
    pub id: String,
    pub title: String,
    pub performers: Vec<String>,
    pub file_name: String,
    pub interactive: bool,
    pub source: VideoSource,
    pub duration: f64,
    pub stash_scene_id: Option<i64>,
    pub tags: Option<Vec<String>>,
}

impl VideoLike for VideoDto {
    fn video_id(&self) -> &str {
        &self.id
    }

    fn stash_scene_id(&self) -> Option<i64> {
        self.stash_scene_id
    }
}

#[derive(Serialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StashVideoDto {
    pub id: String,
    pub title: String,
    pub performers: Vec<String>,
    pub file_name: String,
    pub interactive: bool,
    pub source: VideoSource,
    pub duration: f64,
    pub stash_scene_id: Option<i64>,
    pub exists_in_database: bool,
}

impl StashVideoDto {
    pub fn from(dto: VideoDto, exists_in_database: bool) -> Self {
        Self {
            id: dto.id,
            title: dto.title,
            performers: dto.performers,
            file_name: dto.file_name,
            interactive: dto.interactive,
            source: dto.source,
            duration: dto.duration,
            stash_scene_id: dto.stash_scene_id,
            exists_in_database,
        }
    }
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectedMarker {
    pub id: i64,
    pub video_id: String,
    pub selected_range: (f64, f64),
    pub index_within_video: usize,
    pub selected: Option<bool>,
    pub title: String,
    pub loops: usize,
    pub source: VideoSource,
}

#[derive(Deserialize, Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RandomizedClipOptions {
    pub base_duration: f64,
    pub divisors: Vec<f64>,
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
pub enum PmvClipOptions {
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
    pub fn clip_lengths(&self) -> Option<&PmvClipOptions> {
        match self {
            ClipPickerOptions::RoundRobin(opts) => Some(&opts.clip_lengths),
            ClipPickerOptions::WeightedRandom(opts) => Some(&opts.clip_lengths),
            ClipPickerOptions::EqualLength(_) => None,
            ClipPickerOptions::NoSplit => None,
        }
    }

    pub fn has_music(&self) -> bool {
        matches!(self.clip_lengths(), Some(PmvClipOptions::Songs(_)))
    }

    pub fn songs(&self) -> Option<&[Beats]> {
        if let Some(PmvClipOptions::Songs(songs)) = self.clip_lengths() {
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
    pub clip_lengths: PmvClipOptions,
}

#[derive(Deserialize, Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WeightedRandomClipOptions {
    pub weights: Vec<(String, f64)>,
    pub length: f64,
    pub clip_lengths: PmvClipOptions,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EqualLengthClipOptions {
    pub clip_duration: f64,
    pub divisors: Vec<f64>,
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateClipsBody {
    pub clip_order: ClipOrder,
    pub markers: Vec<SelectedMarker>,
    pub seed: Option<String>,
    pub clips: ClipOptions,
}

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClipsResponse {
    pub clips: Vec<Clip>,
    pub streams: HashMap<String, String>,
    pub videos: Vec<VideoDto>,
    pub beat_offsets: Option<Vec<f32>>,
}

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListVideoDto {
    pub video: VideoDto,
    pub markers: Vec<MarkerDto>,
}

#[derive(Debug, Clone, Copy, Deserialize, ToSchema)]
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

impl fmt::Display for VideoResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SevenTwenty => write!(f, "720"),
            Self::TenEighty => write!(f, "1080"),
            Self::FourK => write!(f, "4K"),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy, ToSchema)]
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
    pub output_resolution: VideoResolution,
    pub output_fps: u32,
    pub song_ids: Vec<i64>,
    pub music_volume: Option<f64>,
    pub video_codec: VideoCodec,
    pub video_quality: VideoQuality,
    pub encoding_effort: EncodingEffort,
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Beats {
    pub offsets: Vec<f32>,
    pub length: f32,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SongDto {
    pub song_id: i64,
    pub duration: f64,
    pub file_name: String,
    pub url: String,
    pub beats: Vec<f32>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewId {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone, Copy, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Deserialize, Debug, Clone, IntoParams)]
pub struct PageParameters {
    pub page: Option<usize>,
    pub size: Option<usize>,
    pub sort: Option<String>,
    pub dir: Option<SortDirection>,
}

impl PageParameters {
    pub const DEFAULT_PAGE: i64 = 0;
    pub const DEFAULT_SIZE: i64 = 20;

    pub fn limit(&self) -> i64 {
        self.size.map(|s| s as i64).unwrap_or(Self::DEFAULT_SIZE)
    }

    pub fn offset(&self) -> i64 {
        self.page
            .map(|p| p as i64 * self.limit())
            .unwrap_or(Self::DEFAULT_PAGE)
    }

    #[allow(unused)]
    pub fn size(&self) -> i64 {
        self.size.map(|s| s as i64).unwrap_or(Self::DEFAULT_SIZE)
    }

    #[allow(unused)]
    pub fn page(&self) -> i64 {
        self.page.map(|p| p as i64).unwrap_or(Self::DEFAULT_PAGE)
    }

    pub fn sort(&self, default_col: &str, default_direction: SortDirection) -> String {
        let sort = self.sort.as_deref().unwrap_or(default_col);
        let direction = self.direction(default_direction);
        format!("{} {}", sort, direction)
    }

    fn direction(&self, default: SortDirection) -> &str {
        let dir = self.dir.unwrap_or(default);
        match dir {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub video_id: String,
    pub items_finished: f64,
    pub items_total: f64,
    pub done: bool,
    pub eta_seconds: Option<f64>,
    pub message: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateMarker {
    pub video_id: String,
    pub start: f64,
    pub end: f64,
    pub title: String,
    pub index_within_video: i64,
    pub preview_image_path: Option<String>,
    pub video_interactive: bool,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMarker {
    pub rowid: i64,
    pub start: f64,
    pub end: f64,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum StrokeType {
    /// Creates a stroke every `n` beats
    EveryNth { n: usize },
    /// Steadily accelerates the strokes from `start_strokes_per_beat` to `end_strokes_per_beat`
    Accelerate {
        start_strokes_per_beat: f32,
        end_strokes_per_beat: f32,
    },
}

impl StrokeType {
    #[allow(unused)]
    pub fn initial_acceleration(&self) -> Option<f32> {
        match self {
            Self::Accelerate {
                start_strokes_per_beat,
                ..
            } => Some(*start_strokes_per_beat),
            _ => None,
        }
    }
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBeatFunscriptBody {
    pub song_ids: Vec<i64>,
    pub stroke_type: StrokeType,
}

pub trait VideoLike {
    fn video_id(&self) -> &str;

    fn stash_scene_id(&self) -> Option<i64>;
}
