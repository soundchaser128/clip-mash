use std::{collections::HashMap, marker};

use serde::{Deserialize, Serialize};

use crate::{
    data::{stash_api::{
        find_scenes_query::FindScenesQueryFindScenesScenes, FilterMode, StashMarker,
    }, database::{LocalVideoWithMarkers, DbMarker}},
    service::{
        clip::{ClipOrder, CreateClipsOptions},
        Clip, Marker, MarkerId, MarkerInfo, VideoId, generator::VideoResolution,
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
    // TODO
}

impl From<StashMarker> for MarkerDto {
    fn from(value: StashMarker) -> Self {
        MarkerDto {
            id: MarkerId::Stash(value.id),
        }
    }
}

impl From<DbMarker> for MarkerDto {
    fn from(value: DbMarker) -> Self {
        todo!()
    }
}

#[derive(Serialize, Debug)]
pub struct VideoDto {
    // TODO
}

impl From<FindScenesQueryFindScenesScenes> for VideoDto {
    fn from(value: FindScenesQueryFindScenesScenes) -> Self {
        VideoDto {
            
        }
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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListVideoDto {

}

impl From<LocalVideoWithMarkers> for ListVideoDto {
    fn from(value: LocalVideoWithMarkers) -> Self {
        todo!()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateVideoBody {
    pub file_name: String,
    pub clips: Vec<Clip>,
    pub markers: Vec<SelectedMarker>,
    pub output_resolution: VideoResolution,
    pub output_fps: u32,
}