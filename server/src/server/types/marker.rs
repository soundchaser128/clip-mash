use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::data::database::markers::{DbMarker, DbMarkerWithVideo};
use crate::data::database::videos::{DbVideo, VideoSource};
use crate::data::stash_api::StashApi;
use crate::util::expect_file_name;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MarkerTitle {
    pub title: String,
    pub count: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MarkerGroup {
    pub markers: Vec<MarkerTitle>,
    pub name: String,
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
    pub created_on: i64,
}

pub struct MarkerDtoConverter {
    stash_api: StashApi,
}

impl MarkerDtoConverter {
    pub fn new(stash_api: StashApi) -> Self {
        Self { stash_api }
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
            tags: video.tags(),
            screenshot_url: self.screenshot_url(marker.rowid.unwrap()),
            index_within_video: marker.index_within_video as usize,
            source: video.source,
            created_on: marker.marker_created_on,
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
            created_on: value.marker_created_on,
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
    #[allow(unused)]
    pub selected: Option<bool>,
    pub title: String,
    pub loops: usize,
    pub source: VideoSource,
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
    #[allow(unused)]
    pub video_interactive: bool,
    pub created_on: Option<i64>,
    pub marker_stash_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMarker {
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub title: Option<String>,
    pub stash_marker_id: Option<i64>,
}
