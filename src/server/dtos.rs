use std::collections::HashMap;

use camino::Utf8Path;
use serde::{Deserialize, Serialize};

use crate::{
    data::{
        database::{DbMarker, DbVideo, LocalVideoWithMarkers},
        stash_api::{find_scenes_query::FindScenesQueryFindScenesScenes, StashMarker},
    },
    service::{
        clip::{ClipOrder, ClipSortMode},
        generator::VideoResolution,
        Clip, MarkerId, Video, VideoId,
    },
    util::{add_api_key, expect_file_name},
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

impl From<StashMarker> for MarkerDto {
    fn from(value: StashMarker) -> Self {
        MarkerDto {
            id: MarkerId::Stash(value.id.parse().unwrap()),
            stream_url: value.stream_url,
            primary_tag: value.primary_tag,
            start: value.start,
            end: value.end,
            scene_title: value.scene_title,
            performers: value.performers,
            file_name: value.file_name,
            scene_interactive: value.scene_interactive,
            tags: value.tags,
            screenshot_url: Some(value.screenshot_url),
            index_within_video: value.index_within_video,
            video_id: VideoId::Stash(value.scene_id),
        }
    }
}

impl From<DbMarker> for MarkerDto {
    fn from(value: DbMarker) -> Self {
        MarkerDto {
            id: MarkerId::LocalFile(value.rowid.expect("must have an ID")),
            start: value.start_time,
            end: value.end_time,
            file_name: Utf8Path::new(&value.file_path)
                .file_name()
                .map(|s| s.to_string()),
            performers: vec![],
            primary_tag: value.title,
            scene_interactive: false,
            scene_title: None,
            stream_url: format!("TODO"),
            tags: vec![],
            screenshot_url: None,
            index_within_video: value.index_within_video as usize,
            video_id: VideoId::LocalFile(value.video_id),
        }
    }
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

impl From<FindScenesQueryFindScenesScenes> for VideoDto {
    fn from(value: FindScenesQueryFindScenesScenes) -> Self {
        VideoDto {
            id: VideoId::Stash(value.id),
            title: value
                .title
                .or(value.files.get(0).map(|m| m.basename.clone()))
                .unwrap_or_default(),
            performers: value.performers.into_iter().map(|p| p.name).collect(),
            file_name: value
                .files
                .get(0)
                .map(|f| f.basename.clone())
                .unwrap_or_default(),
            interactive: value.interactive,
        }
    }
}

impl From<DbVideo> for VideoDto {
    fn from(value: DbVideo) -> Self {
        VideoDto {
            id: VideoId::LocalFile(value.id),
            title: Utf8Path::new(&value.file_path)
                .file_name()
                .map(From::from)
                .unwrap_or_default(),
            performers: vec![],
            interactive: value.interactive,
            file_name: expect_file_name(&value.file_path),
        }
    }
}

impl From<Video> for VideoDto {
    fn from(value: Video) -> Self {
        VideoDto {
            id: value.id,
            title: value.title,
            performers: value.performers,
            file_name: value.file_name,
            interactive: value.interactive,
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
    pub split_clips: bool,
    pub markers: Vec<SelectedMarker>,
    pub sort_mode: Vec<ClipSortMode>,
    pub seed: Option<String>,
    pub song_ids: Vec<i64>,
    pub trim_video_for_songs: bool,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClipsResponse {
    pub clips: Vec<Clip>,
    pub streams: HashMap<String, String>,
    pub videos: Vec<VideoDto>,
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
