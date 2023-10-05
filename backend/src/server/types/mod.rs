use std::collections::HashMap;
use std::fmt;

use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::data::database::{
    DbMarker, DbMarkerWithVideo, DbVideo, LocalVideoWithMarkers, VideoSource,
};
use crate::data::stash_api::find_scenes_query::FindScenesQueryFindScenesScenes;
use crate::data::stash_api::StashApi;
use crate::service::video::TAG_SEPARATOR;
use crate::util::{add_api_key, expect_file_name};

pub struct StashSceneWrapper<'a> {
    pub scene: FindScenesQueryFindScenesScenes,
    pub api_key: &'a str,
}

impl<'a> From<StashSceneWrapper<'a>> for StashScene {
    fn from(value: StashSceneWrapper<'a>) -> Self {
        let StashSceneWrapper { scene, api_key } = value;
        StashScene {
            id: scene.id,
            performers: scene.performers.into_iter().map(|p| p.name).collect(),
            image_url: scene.paths.screenshot.map(|url| add_api_key(&url, api_key)),
            title: scene.title.unwrap_or_default(),
            studio: scene.studio.map(|s| s.name),
            tags: scene.tags.into_iter().map(|t| t.name).collect(),
            rating: scene.rating100,
            interactive: scene.interactive,
            marker_count: scene.scene_markers.len(),
        }
    }
}

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
#[aliases(
    ListVideoDtoPage = Page<ListVideoDto>,
    StashVideoDtoPage = Page<StashVideoDto>,
    MarkerDtoPage = Page<MarkerDto>,
)]
pub struct Page<T> {
    pub content: Vec<T>,
    pub total_items: usize,
    pub page_number: usize,
    pub page_size: usize,
    pub total_pages: usize,
}

impl<T> Page<T> {
    pub fn empty() -> Self {
        Page {
            content: vec![],
            total_items: 0,
            page_number: 0,
            page_size: 0,
            total_pages: 0,
        }
    }
}

impl<T: Serialize + ToSchema<'static>> Page<T> {
    pub fn new(content: Vec<T>, size: usize, page: PageParameters) -> Self {
        let page_number = page.page.unwrap_or(PageParameters::DEFAULT_PAGE as usize);
        let page_size = page.size.unwrap_or(PageParameters::DEFAULT_SIZE as usize);
        let total_pages = (size as f64 / page_size as f64).ceil() as usize;

        Page {
            content,
            total_items: size,
            page_number,
            page_size,
            total_pages,
        }
    }
}

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
    pub file_name: Option<String>,
    pub scene_interactive: bool,
    pub tags: Vec<String>,
    pub screenshot_url: String,
    pub index_within_video: usize,
    pub source: VideoSource,
}

pub struct MarkerDtoConverter {
    stash_api: StashApi,
}

impl MarkerDtoConverter {
    pub async fn new() -> Self {
        Self {
            stash_api: StashApi::load_config().await,
        }
    }

    fn stream_url(&self, source: VideoSource, video_id: &str, stash_id: Option<i64>) -> String {
        match source {
            VideoSource::Stash => {
                let stash_id = stash_id.expect("stash video must have scene id");
                self.stash_api.get_stream_url(stash_id)
            }
            VideoSource::Folder | VideoSource::Download => {
                format!("/api/library/video/{}/file", video_id)
            }
        }
    }

    fn screenshot_url(&self, marker_id: i64) -> String {
        format!("/api/library/marker/{}/preview", marker_id)
    }

    pub fn from_db(&self, marker: DbMarker, video: &DbVideo) -> MarkerDto {
        MarkerDto {
            id: marker.rowid.expect("marker must have rowid"),
            video_id: video.id.clone(),
            primary_tag: marker.title,
            stream_url: self.stream_url(video.source, &video.id, video.stash_scene_id),
            start: marker.start_time,
            end: marker.end_time,
            scene_title: video.video_title.clone(),
            file_name: Some(expect_file_name(&video.file_path)),
            scene_interactive: video.interactive,
            tags: video.tags().unwrap_or_default(),
            screenshot_url: self.screenshot_url(marker.rowid.unwrap()),
            index_within_video: marker.index_within_video as usize,
            source: video.source,
        }
    }

    pub fn from_db_with_video(&self, value: DbMarkerWithVideo) -> MarkerDto {
        let tags = value.tags();

        MarkerDto {
            id: value.rowid.expect("marker must have a rowid"),
            start: value.start_time,
            end: value.end_time,
            file_name: Utf8Path::new(&value.file_path)
                .file_name()
                .map(|s| s.to_string()),
            primary_tag: value.title,
            scene_interactive: value.interactive,
            scene_title: value.video_title,
            stream_url: self.stream_url(value.source, &value.video_id, value.stash_scene_id),
            tags,
            screenshot_url: self.screenshot_url(value.rowid.unwrap()),
            index_within_video: value.index_within_video as usize,
            video_id: value.video_id,
            source: value.source,
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
    pub file_path: Option<String>,
    pub interactive: bool,
    pub source: VideoSource,
    pub duration: f64,
    pub stash_scene_id: Option<i64>,
    pub tags: Vec<String>,
    pub created_on: i64,
}

impl VideoLike for VideoDto {
    fn video_id(&self) -> &str {
        &self.id
    }

    fn stash_scene_id(&self) -> Option<i64> {
        self.stash_scene_id
    }

    fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }
}

impl From<FindScenesQueryFindScenesScenes> for VideoDto {
    fn from(value: FindScenesQueryFindScenesScenes) -> Self {
        let file = value.files.get(0).expect("must have at least one file");

        VideoDto {
            id: value.id.clone(),
            stash_scene_id: Some(value.id.parse().expect("invalid scene id")),
            file_path: None,
            title: value
                .title
                .or(value.files.get(0).map(|m| m.basename.clone()))
                .unwrap_or_default(),
            performers: value.performers.into_iter().map(|p| p.name).collect(),
            file_name: file.basename.clone(),
            interactive: value.interactive,
            source: VideoSource::Stash,
            duration: file.duration,
            tags: value.tags.into_iter().map(|t| t.name).collect(),
            created_on: 0,
        }
    }
}

impl From<DbVideo> for VideoDto {
    fn from(value: DbVideo) -> Self {
        let title = value.video_title.unwrap_or_else(|| {
            Utf8Path::new(&value.file_path)
                .file_name()
                .map(From::from)
                .unwrap_or_default()
        });
        let tags = value
            .video_tags
            .map(|s| s.split(TAG_SEPARATOR).map(From::from).collect())
            .unwrap_or_default();

        VideoDto {
            id: value.id,
            stash_scene_id: value.stash_scene_id,
            title,
            performers: vec![],
            interactive: value.interactive,
            file_name: expect_file_name(&value.file_path),
            source: value.source,
            duration: value.duration,
            tags,
            file_path: Some(value.file_path),
            created_on: value.video_created_on,
        }
    }
}

#[derive(Serialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StashVideoDto {
    pub id: String,
    pub title: String,
    pub performers: Vec<String>,
    pub tags: Vec<String>,
    pub file_name: String,
    pub interactive: bool,
    pub source: VideoSource,
    pub duration: f64,
    pub stash_scene_id: Option<i64>,
    pub exists_in_database: bool,
    pub marker_count: usize,
    pub created_on: i64,
}

impl StashVideoDto {
    pub fn from(dto: VideoDto, exists_in_database: bool, marker_count: usize) -> Self {
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
            tags: dto.tags,
            marker_count,
            created_on: dto.created_on,
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
    pub marker_count: usize,
}

impl From<LocalVideoWithMarkers> for ListVideoDto {
    fn from(value: LocalVideoWithMarkers) -> Self {
        ListVideoDto {
            video: value.video.into(),
            marker_count: value.markers.len(),
        }
    }
}

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetailsDto {
    pub video: VideoDto,
    pub markers: Vec<MarkerDto>,
}

pub struct VideoDetailsDtoConverter {
    marker_converter: MarkerDtoConverter,
}

impl VideoDetailsDtoConverter {
    pub async fn new() -> Self {
        let marker_converter = MarkerDtoConverter::new().await;
        Self { marker_converter }
    }

    pub fn from_db(&self, value: LocalVideoWithMarkers) -> VideoDetailsDto {
        let db_video = value.video.clone();
        VideoDetailsDto {
            video: value.video.into(),
            markers: value
                .markers
                .into_iter()
                .map(|m| self.marker_converter.from_db(m, &db_video))
                .collect(),
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
    pub output_resolution: (u32, u32),
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

    pub fn size(&self) -> i64 {
        self.size.map(|s| s as i64).unwrap_or(Self::DEFAULT_SIZE)
    }

    pub fn page(&self) -> i64 {
        self.page.map(|p| p as i64).unwrap_or(Self::DEFAULT_PAGE)
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
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub title: Option<String>,
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

    fn file_path(&self) -> Option<&str>;
}
