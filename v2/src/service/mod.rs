pub mod clip;
pub mod download_ffmpeg;
pub mod funscript;
pub mod generator;
pub mod stash_config;

use serde::{Deserialize, Serialize};

use crate::data::{
    database::{DbMarker, DbVideo},
    stash_api::{
        find_markers_query::FindMarkersQueryFindSceneMarkers,
        find_scenes_query::FindScenesQueryFindScenesScenes, StashMarker,
    },
};

#[derive(Debug, Clone)]
pub enum VideoInfo {
    Stash {
        scene: FindScenesQueryFindScenesScenes,
    },
    LocalFile {
        video: DbVideo,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum VideoSource {
    Stash,
    LocalFile,
}

#[derive(Debug, Clone)]
pub struct Video {
    pub id: i64,
    pub title: String,
    pub interactive: bool,
    pub info: VideoInfo,
}

impl Video {
    pub fn source(&self) -> VideoSource {
        match self.info {
            VideoInfo::LocalFile { .. } => VideoSource::LocalFile,
            VideoInfo::Stash { .. } => VideoSource::Stash,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MarkerInfo {
    Stash { marker: StashMarker },
    LocalFile { marker: DbMarker },
}
#[derive(Debug, Clone, PartialEq)]
pub struct Marker {
    pub id: i64,
    pub start_time: f64,
    pub end_time: f64,
    pub index_within_video: usize,
    pub video_id: VideoId,
    pub title: String,
    pub info: MarkerInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MarkerId {
    LocalFile(i64),
    Stash(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
pub enum VideoId {
    LocalFile(String),
    Stash(String),
}
