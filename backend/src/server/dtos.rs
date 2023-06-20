use camino::Utf8Path;
use clip_mash_types::*;

use crate::data::database::{DbMarker, DbVideo, LocalVideoSource, LocalVideoWithMarkers};
use crate::data::stash_api::find_scenes_query::FindScenesQueryFindScenesScenes;
use crate::data::stash_api::StashMarker;
use crate::service::{Video, VideoInfo};
use crate::util::{add_api_key, expect_file_name};

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
            stream_url: format!("/api/local/video/{}", value.video_id),
            tags: vec![],
            screenshot_url: Some(format!(
                "/api/local/video/marker/{}/preview",
                value.rowid.unwrap()
            )),
            index_within_video: value.index_within_video as usize,
            video_id: VideoId::LocalFile(value.video_id),
        }
    }
}

impl From<FindScenesQueryFindScenesScenes> for VideoDto {
    fn from(value: FindScenesQueryFindScenesScenes) -> Self {
        let file = value.files.get(0).expect("must have at least one file");

        VideoDto {
            id: VideoId::Stash(value.id),
            title: value
                .title
                .or(value.files.get(0).map(|m| m.basename.clone()))
                .unwrap_or_default(),
            performers: value.performers.into_iter().map(|p| p.name).collect(),
            file_name: file.basename.clone(),
            interactive: value.interactive,
            source: VideoSource::Stash,
            duration: file.duration,
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
            source: match value.source {
                LocalVideoSource::Folder => VideoSource::LocalFile,
                LocalVideoSource::Download => VideoSource::DownloadedLocalFile,
            },
            duration: value.duration,
        }
    }
}

impl From<Video> for VideoDto {
    fn from(value: Video) -> Self {
        let duration = value.duration();
        VideoDto {
            id: value.id,
            title: value.title,
            performers: value.performers,
            file_name: value.file_name,
            interactive: value.interactive,
            source: match value.info {
                VideoInfo::Stash { .. } => VideoSource::Stash,
                VideoInfo::LocalFile { video } => match video.source {
                    LocalVideoSource::Folder => VideoSource::LocalFile,
                    LocalVideoSource::Download => VideoSource::DownloadedLocalFile,
                },
            },
            duration,
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

pub struct StashSceneWrapper<'a> {
    pub scene: FindScenesQueryFindScenesScenes,
    pub api_key: &'a str,
}

impl<'a> From<StashSceneWrapper<'a>> for StashScene {
    fn from(value: StashSceneWrapper<'a>) -> Self {
        let StashSceneWrapper { scene, api_key } = value;
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
