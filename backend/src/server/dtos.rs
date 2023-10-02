use camino::Utf8Path;
use serde::Serialize;
use utoipa::ToSchema;

use crate::data::database::{DbMarkerWithVideo, DbVideo, LocalVideoWithMarkers, VideoSource};
use crate::data::stash_api::find_scenes_query::FindScenesQueryFindScenesScenes;
use crate::data::stash_api::StashMarker;
use crate::server::types::*;
use crate::service::video::TAG_SEPARATOR;
use crate::service::{Video, VideoInfo};
use crate::util::{add_api_key, expect_file_name};

impl From<StashMarker> for MarkerDto {
    fn from(value: StashMarker) -> Self {
        MarkerDto {
            id: value.id.parse().unwrap(),
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
            video_id: value.scene_id,
            source: VideoSource::Stash,
        }
    }
}

impl From<DbMarkerWithVideo> for MarkerDto {
    fn from(value: DbMarkerWithVideo) -> Self {
        MarkerDto {
            id: value.rowid.expect("marker must have a rowid"),
            start: value.start_time,
            end: value.end_time,
            file_name: Utf8Path::new(&value.file_path)
                .file_name()
                .map(|s| s.to_string()),
            performers: vec![],
            primary_tag: value.title,
            scene_interactive: value.interactive,
            scene_title: value.video_title,
            stream_url: format!("/api/local/video/{}/file", value.video_id),
            tags: vec![],
            screenshot_url: Some(format!(
                "/api/local/video/marker/{}/preview",
                value.rowid.unwrap()
            )),
            index_within_video: value.index_within_video as usize,
            video_id: value.video_id,
            source: value.source,
        }
    }
}

impl From<FindScenesQueryFindScenesScenes> for VideoDto {
    fn from(value: FindScenesQueryFindScenesScenes) -> Self {
        let file = value.files.get(0).expect("must have at least one file");

        VideoDto {
            id: value.id.clone(),
            stash_scene_id: Some(value.id.parse().expect("invalid scene id")),
            title: value
                .title
                .or(value.files.get(0).map(|m| m.basename.clone()))
                .unwrap_or_default(),
            performers: value.performers.into_iter().map(|p| p.name).collect(),
            file_name: file.basename.clone(),
            interactive: value.interactive,
            source: VideoSource::Stash,
            duration: file.duration,
            tags: Some(value.tags.into_iter().map(|t| t.name).collect()),
        }
    }
}

impl From<DbVideo> for VideoDto {
    fn from(value: DbVideo) -> Self {
        let title = value.video_title.unwrap_or_else(|| {
            Utf8Path::new(&value.file_path)
                .file_name()
                .map(From::from)
                .unwrap_or_default()
        });
        let tags = value
            .video_tags
            .map(|s| s.split(TAG_SEPARATOR).map(From::from).collect());

        VideoDto {
            id: value.id,
            stash_scene_id: value.stash_scene_id,
            title,
            performers: vec![],
            interactive: value.interactive,
            file_name: expect_file_name(&value.file_path),
            source: value.source,
            duration: value.duration,
            tags,
        }
    }
}

impl From<Video> for VideoDto {
    fn from(value: Video) -> Self {
        let duration = value.duration();
        let stash_scene_id = value.stash_scene_id();
        VideoDto {
            id: value.id,
            title: value.title,
            stash_scene_id,
            performers: value.performers,
            file_name: value.file_name,
            interactive: value.interactive,
            source: match value.info {
                VideoInfo::Stash { .. } => VideoSource::Stash,
                VideoInfo::LocalFile { video } => video.source,
            },
            duration,
            tags: value.tags,
        }
    }
}

impl From<LocalVideoWithMarkers> for ListVideoDto {
    fn from(value: LocalVideoWithMarkers) -> Self {
        let db_video = value.video.clone();
        ListVideoDto {
            video: value.video.into(),
            markers: value
                .markers
                .into_iter()
                .map(|m| MarkerDto::from_db(m, &db_video))
                .collect(),
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

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
#[aliases(
    ListVideoDtoPage = Page<ListVideoDto>,
    StashVideoDtoPage = Page<StashVideoDto>,
    MarkerDtoPage = Page<MarkerDto>,
)]
pub struct Page<T> {
    pub content: Vec<T>,
    pub total_items: usize,
    pub page_number: usize,
    pub page_size: usize,
    pub total_pages: usize,
}

impl<T> Page<T> {
    pub fn empty() -> Self {
        Page {
            content: vec![],
            total_items: 0,
            page_number: 0,
            page_size: 0,
            total_pages: 0,
        }
    }
}

impl<T: Serialize + ToSchema<'static>> Page<T> {
    pub fn new(content: Vec<T>, size: usize, page: PageParameters) -> Self {
        let page_number = page.page.unwrap_or(PageParameters::DEFAULT_PAGE as usize);
        let page_size = page.size.unwrap_or(PageParameters::DEFAULT_SIZE as usize);
        let total_pages = (size as f64 / page_size as f64).ceil() as usize;

        Page {
            content,
            total_items: size,
            page_number,
            page_size,
            total_pages,
        }
    }
}
