use std::collections::BTreeSet;

use serde::Serialize;

use crate::stash::api::StashScene;

#[derive(Serialize, Debug)]
pub struct TagDto {
    pub name: String,
    pub id: String,
    pub count: i64,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PerformerDto {
    pub name: String,
    pub id: String,
    pub scene_count: i64,
    pub image_url: Option<String>,
    pub tags: Vec<String>,
    pub rating: Option<i64>,
    pub favorite: bool,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum VideoDto {
    Stash {
        id: String,
        title: String,
        image_url: String,
        performers: Vec<String>,
        marker_count: usize,
        tags: BTreeSet<String>,
        interactive: bool,
        studio: Option<String>,
        rating: Option<i64>,
    },
    LocalFile {
        id: String,
        file_name: String,
        interactive: bool,
    },
}

impl From<StashScene> for VideoDto {
    fn from(scene: StashScene) -> Self {
        VideoDto::Stash {
            id: scene.id,
            title: scene.title,
            image_url: "TODO".into(),
            performers: scene.performers,
            marker_count: scene.markers.len(),
            tags: scene.tags.into_iter().collect(),
            interactive: scene.interactive,
            studio: scene.studio,
            rating: scene.rating,
        }
    }
}
