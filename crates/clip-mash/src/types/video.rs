use camino::Utf8Path;
use serde::Serialize;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use utoipa::ToSchema;

use super::{MarkerDto, MarkerDtoConverter};
use crate::data::database::markers::VideoWithMarkers;
use crate::data::database::unix_timestamp_now;
use crate::data::database::videos::{DbVideo, VideoSource};
use crate::data::stash_api::StashApi;
use crate::data::stash_api::find_scenes_query::FindScenesQueryFindScenesScenes;
use crate::util::{add_api_key, expect_file_name};

pub trait VideoLike {
    fn video_id(&self) -> &str;

    fn stash_scene_id(&self) -> Option<i64>;

    fn file_path(&self) -> Option<&str>;
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

pub struct StashSceneWrapper<'a> {
    pub scene: FindScenesQueryFindScenesScenes,
    pub api_key: Option<&'a str>,
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

#[derive(Serialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoDto {
    pub id: String,
    pub title: String,
    pub file_name: String,
    pub file_path: Option<String>,
    pub interactive: bool,
    pub source: VideoSource,
    pub duration: f64,
    pub stash_scene_id: Option<i64>,
    pub tags: Vec<String>,
    pub created_on: i64,
}

impl VideoLike for VideoDto {
    fn video_id(&self) -> &str {
        &self.id
    }

    fn stash_scene_id(&self) -> Option<i64> {
        self.stash_scene_id
    }

    fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }
}

impl From<FindScenesQueryFindScenesScenes> for VideoDto {
    fn from(value: FindScenesQueryFindScenesScenes) -> Self {
        let file = value.files.get(0).expect("must have at least one file");
        let created_on = OffsetDateTime::parse(&value.created_at, &Rfc3339)
            .map(|time| time.unix_timestamp())
            .unwrap_or_else(|_| unix_timestamp_now());
        VideoDto {
            id: value.id.clone(),
            stash_scene_id: Some(value.id.parse().expect("invalid scene id")),
            file_path: None,
            title: value
                .title
                .or(value.files.get(0).map(|m| m.basename.clone()))
                .unwrap_or_default(),
            file_name: file.basename.clone(),
            interactive: value.interactive,
            source: VideoSource::Stash,
            duration: file.duration,
            tags: value.tags.into_iter().map(|t| t.name).collect(),
            created_on,
        }
    }
}

impl From<DbVideo> for VideoDto {
    fn from(value: DbVideo) -> Self {
        let tags = value.tags();
        let title = value.video_title.unwrap_or_else(|| {
            Utf8Path::new(&value.file_path)
                .file_name()
                .map(From::from)
                .unwrap_or_default()
        });

        VideoDto {
            id: value.id,
            stash_scene_id: value.stash_scene_id,
            title,
            interactive: value.interactive,
            file_name: expect_file_name(&value.file_path),
            source: value.source,
            duration: value.duration,
            tags,
            file_path: Some(value.file_path),
            created_on: value.video_created_on,
        }
    }
}

#[derive(Serialize, Debug, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StashVideoDto {
    pub id: String,
    pub title: String,
    pub performers: Vec<String>,
    pub tags: Vec<String>,
    pub file_name: String,
    pub interactive: bool,
    pub source: VideoSource,
    pub duration: f64,
    pub stash_scene_id: Option<i64>,
    pub exists_in_database: bool,
    pub marker_count: usize,
    pub created_on: i64,
}

impl StashVideoDto {
    pub fn from(dto: VideoDto, exists_in_database: bool, marker_count: usize) -> Self {
        Self {
            id: dto.id,
            title: dto.title,
            // TODO
            performers: vec![],
            file_name: dto.file_name,
            interactive: dto.interactive,
            source: dto.source,
            duration: dto.duration,
            stash_scene_id: dto.stash_scene_id,
            exists_in_database,
            tags: dto.tags,
            marker_count,
            created_on: dto.created_on,
        }
    }
}

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListVideoDto {
    pub video: VideoDto,
    pub marker_count: usize,
}

impl From<VideoWithMarkers> for ListVideoDto {
    fn from(value: VideoWithMarkers) -> Self {
        ListVideoDto {
            video: value.video.into(),
            marker_count: value.markers.len(),
        }
    }
}

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetailsDto {
    pub video: VideoDto,
    pub markers: Vec<MarkerDto>,
}

pub struct VideoDetailsDtoConverter {
    marker_converter: MarkerDtoConverter,
}

impl VideoDetailsDtoConverter {
    pub fn new(stash_api: StashApi) -> Self {
        let marker_converter = MarkerDtoConverter::new(stash_api);
        Self { marker_converter }
    }

    pub fn from_db(&self, value: VideoWithMarkers) -> VideoDetailsDto {
        let db_video = value.video.clone();
        VideoDetailsDto {
            video: value.video.into(),
            markers: value
                .markers
                .into_iter()
                .map(|m| self.marker_converter.from_db(m, &db_video))
                .collect(),
        }
    }
}
