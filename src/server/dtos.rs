use std::collections::HashMap;

use camino::Utf8Path;
use clip_mash_types::*;
use serde::{Deserialize, Serialize};

use crate::{
    data::{
        database::{DbMarker, DbVideo, LocalVideoWithMarkers},
        stash_api::{find_scenes_query::FindScenesQueryFindScenesScenes, StashMarker},
    },
    service::Video,
    util::{add_api_key, expect_file_name},
};

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

impl From<LocalVideoWithMarkers> for ListVideoDto {
    fn from(value: LocalVideoWithMarkers) -> Self {
        ListVideoDto {
            video: value.video.into(),
            markers: value.markers.into_iter().map(From::from).collect(),
        }
    }
}
