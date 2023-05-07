pub mod clip;
pub mod download_ffmpeg;
pub mod funscript;
pub mod generator;
pub mod local_video;
pub mod stash_config;

use std::fmt;

use camino::Utf8Path;
use serde::{Deserialize, Serialize};

use crate::data::{
    database::{DbMarker, DbVideo},
    stash_api::{find_scenes_query::FindScenesQueryFindScenesScenes, StashMarker},
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoSource {
    Stash,
    LocalFile,
}

#[derive(Debug, Clone)]
pub struct Video {
    pub id: VideoId,
    pub title: String,
    pub interactive: bool,
    pub info: VideoInfo,
}

impl From<DbVideo> for Video {
    fn from(value: DbVideo) -> Self {
        Video {
            id: VideoId::LocalFile(value.id.clone()),
            interactive: value.interactive,
            title: Utf8Path::new(&value.file_path)
                .file_name()
                .expect("must have a file name")
                .to_string(),
            info: VideoInfo::LocalFile { video: value },
        }
    }
}

impl From<FindScenesQueryFindScenesScenes> for Video {
    fn from(value: FindScenesQueryFindScenesScenes) -> Self {
        Video {
            id: VideoId::Stash(value.id.clone()),
            interactive: value.interactive,
            title: value
                .title
                .clone()
                .or(value.files.iter().map(|f| f.basename.clone()).next())
                .unwrap_or_default(),
            info: VideoInfo::Stash { scene: value },
        }
    }
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

impl MarkerInfo {
    pub fn video_id(&self) -> VideoId {
        match self {
            Self::Stash { marker } => VideoId::Stash(marker.scene_id.clone()),
            Self::LocalFile { marker } => VideoId::LocalFile(marker.video_id.clone()),
        }
    }

    pub fn title(&self) -> &str {
        match self {
            Self::Stash { marker } => &marker.primary_tag,
            Self::LocalFile { marker } => &marker.title,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Marker {
    pub id: MarkerId,
    pub start_time: f64,
    pub end_time: f64,
    pub index_within_video: usize,
    pub video_id: VideoId,
    pub title: String,
    pub info: MarkerInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "type", content = "id")]
pub enum MarkerId {
    LocalFile(i64),
    Stash(i64),
}

impl fmt::Display for MarkerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarkerId::LocalFile(id) => write!(f, "{}", id),
            MarkerId::Stash(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
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
            &id
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
