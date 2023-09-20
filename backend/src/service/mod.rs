pub mod clip;
pub mod commands;
pub mod directories;
pub mod funscript;
pub mod generator;
pub mod migrations;
pub mod music;
pub mod preview_image;
pub mod scene_detection;
pub mod stash_config;
pub mod video;

#[cfg(test)]
pub mod fixtures;

use serde::{Deserialize, Serialize};

use crate::data::database::{DbMarker, DbVideo};
use crate::data::stash_api::find_scenes_query::FindScenesQueryFindScenesScenes;
use crate::data::stash_api::StashMarker;
use crate::server::types::{MarkerId, VideoId};
use crate::util::expect_file_name;

#[derive(Debug, Clone)]
pub enum VideoInfo {
    Stash {
        scene: Box<FindScenesQueryFindScenesScenes>,
    },
    LocalFile {
        video: DbVideo,
    },
}

#[derive(Debug, Clone)]
pub struct Video {
    pub id: VideoId,
    pub title: String,
    pub tags: Option<Vec<String>>,
    pub interactive: bool,
    pub file_name: String,
    pub performers: Vec<String>,
    pub info: VideoInfo,
}

impl Video {
    pub fn duration(&self) -> f64 {
        match &self.info {
            VideoInfo::Stash { scene } => scene.files[0].duration,
            VideoInfo::LocalFile { video } => video.duration,
        }
    }

    pub fn stash_scene_id(&self) -> Option<i64> {
        match &self.info {
            VideoInfo::Stash { scene } => Some(scene.id.parse().unwrap()),
            VideoInfo::LocalFile { video } => video.stash_scene_id,
        }
    }
}

impl From<DbVideo> for Video {
    fn from(value: DbVideo) -> Self {
        let file_name = expect_file_name(&value.file_path);
        let tags = value.tags();

        Video {
            id: VideoId::LocalFile(value.id.clone()),
            tags,
            interactive: value.interactive,
            title: value
                .video_title
                .clone()
                .unwrap_or_else(|| expect_file_name(&value.file_path)),
            info: VideoInfo::LocalFile { video: value },
            file_name,
            performers: vec![],
        }
    }
}

impl From<FindScenesQueryFindScenesScenes> for Video {
    fn from(value: FindScenesQueryFindScenesScenes) -> Self {
        let tags = value.tags.clone().into_iter().map(|t| t.name).collect();
        Video {
            id: VideoId::Stash(value.id.clone()),
            interactive: value.interactive,
            file_name: value
                .files
                .get(0)
                .map(|f| f.basename.clone())
                .unwrap_or_default(),
            performers: value
                .performers
                .clone()
                .into_iter()
                .map(|p| p.name)
                .collect(),
            title: value
                .title
                .clone()
                .or(value.files.iter().map(|f| f.basename.clone()).next())
                .unwrap_or_default(),
            info: VideoInfo::Stash {
                scene: Box::new(value),
            },
            tags: Some(tags),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marker {
    pub id: MarkerId,
    pub start_time: f64,
    pub end_time: f64,
    pub index_within_video: usize,
    pub video_id: VideoId,
    pub title: String,
    pub info: MarkerInfo,
    pub loops: usize,
}

impl Marker {
    pub fn duration(&self) -> f64 {
        self.end_time - self.start_time
    }
}
