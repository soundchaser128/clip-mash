pub mod clip;
pub mod commands;
pub mod directories;
pub mod funscript;
pub mod generator;
pub mod local_video;
pub mod migrations;
pub mod music;
pub mod preview_image;
pub mod stash_config;

#[cfg(test)]
pub mod fixtures;

use clip_mash_types::{MarkerId, VideoId};

use crate::data::database::{DbMarker, DbVideo};
use crate::data::stash_api::find_scenes_query::FindScenesQueryFindScenesScenes;
use crate::data::stash_api::StashMarker;
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
}

impl From<DbVideo> for Video {
    fn from(value: DbVideo) -> Self {
        let file_name = expect_file_name(&value.file_path);

        Video {
            id: VideoId::LocalFile(value.id.clone()),
            interactive: value.interactive,
            title: file_name.clone(),
            info: VideoInfo::LocalFile { video: value },
            file_name,
            performers: vec![],
        }
    }
}

impl From<FindScenesQueryFindScenesScenes> for Video {
    fn from(value: FindScenesQueryFindScenesScenes) -> Self {
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
    pub loops: f64,
}

impl Marker {
    pub fn duration(&self) -> f64 {
        self.end_time - self.start_time
    }

    pub fn multiply(&self, factor: f64) -> Self {
        let new_duration = self.duration() * factor;
        let new_end_time = self.start_time + new_duration;

        Marker {
            id: self.id.clone(),
            start_time: self.start_time,
            end_time: new_end_time,
            index_within_video: self.index_within_video,
            video_id: self.video_id.clone(),
            title: self.title.clone(),
            info: self.info.clone(),
            loops: self.loops,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::service::fixtures::create_marker;

    #[test]
    fn test_marker_multiply() {
        let marker = create_marker("title", 14.0, 24.0, 0);
        let marker = marker.multiply(0.8);
        assert!(marker.start_time < marker.end_time);
        assert_eq!(marker.end_time, 22.0);
    }
}
