use std::collections::HashMap;

use serde::{Deserialize, Serialize};


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

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoDto {
    pub id: VideoId,
    pub title: String,
    pub performers: Vec<String>,
    pub file_name: String,
    pub interactive: bool,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SelectedMarker {
    pub id: MarkerId,
    pub video_id: VideoId,
    pub selected_range: (f64, f64),
    pub index_within_video: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct RandomizedClipOptions {
    pub base_duration: f64,
    pub divisors: Vec<f64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum PmvClipOptions {
    Randomized(RandomizedClipOptions),
    Songs {
        beats_per_measure: usize,
        // TODO allow randomizing
        cut_after_measure_count: usize,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ClipOptions {
    Pmv {
        song_ids: Vec<i64>,
        clips: PmvClipOptions,
    },
    Default(RandomizedClipOptions),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateClipsBody {
    pub clip_order: ClipOrder,
    pub split_clips: bool,
    pub markers: Vec<SelectedMarker>,
    pub seed: Option<String>,
    pub clips: ClipOptions,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClipsResponse {
    pub clips: Vec<Clip>,
    pub streams: HashMap<String, String>,
    pub videos: Vec<VideoDto>,
    pub beat_offsets: Option<Vec<f32>>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListVideoDto {
    pub video: VideoDto,
    pub markers: Vec<MarkerDto>,
}

#[derive(Deserialize, Debug)]
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

impl StashScene {
    pub fn from(scene: FindScenesQueryFindScenesScenes, api_key: &str) -> Self {
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
);