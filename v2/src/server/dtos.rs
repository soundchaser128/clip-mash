use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    data::{
        database::{DbMarker, DbVideo, LocalVideoWithMarkers},
        stash_api::{find_scenes_query::FindScenesQueryFindScenesScenes, FilterMode, StashMarker},
    },
    service::{
        clip::ClipOrder,
        generator::{find_stash_stream_url, find_stream_url, VideoResolution},
        Clip, MarkerId, VideoId,
    },
};

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
    pub primary_tag: String,
    pub stream_url: String,
    pub start: f64,
    pub end: f64,
    pub scene_title: Option<String>,
    pub performers: Vec<String>,
    pub file_name: Option<String>,
    pub scene_interactive: bool,
    pub tags: Vec<String>,
}

impl From<StashMarker> for MarkerDto {
    fn from(value: StashMarker) -> Self {
        MarkerDto {
            id: MarkerId::Stash(value.id),
            primary_tag: value.primary_tag,
            stream_url: find_stash_stream_url(&value).into(),
            start: value.start,
            end: value.end,
            scene_title: value.scene_title,
            performers: value.performers,
            file_name: value.file_name,
            scene_interactive: value.scene_interactive,
            tags: value.tags,
        }
    }
}

impl From<DbMarker> for MarkerDto {
    fn from(value: DbMarker) -> Self {
        MarkerDto {
            id: MarkerId::LocalFile(value.rowid.expect("must have an ID")),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct VideoDto {
    // TODO
}

impl From<FindScenesQueryFindScenesScenes> for VideoDto {
    fn from(_value: FindScenesQueryFindScenesScenes) -> Self {
        VideoDto {}
    }
}

impl From<DbVideo> for VideoDto {
    fn from(_value: DbVideo) -> Self {
        VideoDto {}
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
    pub video: VideoDto,
    pub markers: Vec<MarkerDto>,
}

impl From<LocalVideoWithMarkers> for ListVideoDto {
    fn from(value: LocalVideoWithMarkers) -> Self {
        ListVideoDto {
            video: value.video.into(),
            markers: value.markers.into_iter().map(From::from).collect(),
        }
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
