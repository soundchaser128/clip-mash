use std::{collections::HashMap, marker};

use serde::{Deserialize, Serialize};

use crate::{
    data::stash_api::{
        find_scenes_query::FindScenesQueryFindScenesScenes, FilterMode, StashMarker,
    },
    service::{
        clip::{ClipOrder, CreateClipsOptions},
        Clip, Marker, MarkerId, MarkerInfo, VideoId,
    },
};

#[derive(Serialize, Debug)]
pub struct TagDto {
    pub name: String,
    pub id: String,
    pub marker_count: i64,
}

#[derive(Serialize, Debug)]
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
pub struct MarkerDto {
    pub id: MarkerId,
}

impl From<StashMarker> for MarkerDto {
    fn from(value: StashMarker) -> Self {
        MarkerDto {
            id: MarkerId::Stash(value.id),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct VideoDto {}

impl From<FindScenesQueryFindScenesScenes> for VideoDto {
    fn from(value: FindScenesQueryFindScenesScenes) -> Self {
        todo!()
    }
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
#[serde(rename_all = "camelCase")]
pub struct CreateClipsBody {
    pub clip_order: ClipOrder,
    pub clip_duration: u32,
    pub select_mode: FilterMode,
    pub split_clips: bool,
    pub markers: Vec<SelectedMarker>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClipsResponse {
    pub clips: Vec<Clip>,
    pub streams: HashMap<VideoId, String>,
}
