use std::{cmp::Reverse, sync::Arc};

use axum::{
    extract::{Query, State},
    Json,
};
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    ffmpeg::formatted_scene,
    stash_api::{
        find_markers_query::{
            self, CriterionModifier, FindFilterType, HierarchicalMultiCriterionInput,
            MultiCriterionInput, SceneMarkerFilterType,
        },
        find_performers_query, find_tags_query,
    },
    AppState,
};

#[derive(Serialize, Debug)]
pub struct Tag {
    pub name: String,
    pub id: String,
    pub count: i64,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Performer {
    pub name: String,
    pub id: String,
    pub scene_count: i64,
    pub image_url: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Marker {
    pub id: String,
    pub primary_tag: String,
    pub stream_url: String,
    pub screenshot_url: String,
    pub start: u32,
    pub end: Option<u32>,
    pub scene_title: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FilterMode {
    Performers,
    Tags,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkerOptions {
    pub selected_ids: String,
    pub mode: FilterMode,
}

fn add_api_key(url: &str, api_key: &str) -> String {
    let mut url = Url::parse(url).expect("invalid url");
    url.query_pairs_mut().append_pair("apikey", api_key);
    url.to_string()
}

#[axum::debug_handler]
pub async fn fetch_tags(state: State<Arc<AppState>>) -> Result<Json<Vec<Tag>>, AppError> {
    let tags = state.api.find_tags(find_tags_query::Variables {}).await?;
    let mut tags: Vec<_> = tags
        .into_iter()
        .map(|t| Tag {
            name: t.name,
            id: t.id,
            count: t.scene_marker_count.unwrap_or_default(),
        })
        .filter(|t| t.count > 0)
        .collect();
    tags.sort_by_key(|t| Reverse(t.count));

    tracing::debug!("returning tags {:?}", tags);

    Ok(Json(tags))
}

#[axum::debug_handler]
pub async fn fetch_performers(
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<Performer>>, AppError> {
    let performers = state
        .api
        .find_performers(find_performers_query::Variables {})
        .await?;
    let mut performers: Vec<_> = performers
        .into_iter()
        .map(|p| Performer {
            id: p.id,
            scene_count: p.scene_count.unwrap_or_default(),
            name: p.name,
            image_url: p
                .image_path
                .map(|url| add_api_key(&url, &state.config.api_key)),
        })
        .filter(|p| p.scene_count > 0)
        .collect();
    performers.sort_by_key(|t| Reverse(t.scene_count));

    tracing::debug!("returning performers {:?}", performers);

    Ok(Json(performers))
}

#[axum::debug_handler]
pub async fn fetch_markers(
    state: State<Arc<AppState>>,
    Query(query): Query<MarkerOptions>,
) -> Result<Json<Vec<Marker>>, AppError> {
    tracing::info!("fetching markers for query {query:?}");

    let mut scene_filter = SceneMarkerFilterType {
        created_at: None,
        scene_created_at: None,
        scene_updated_at: None,
        updated_at: None,
        performers: None,
        scene_date: None,
        scene_tags: None,
        tag_id: None,
        tags: None,
    };

    let ids: Vec<_> = query.selected_ids.split(",").map(From::from).collect();

    match query.mode {
        FilterMode::Performers => {
            scene_filter.performers = Some(MultiCriterionInput {
                modifier: CriterionModifier::INCLUDES,
                value: Some(ids),
            });
        }
        FilterMode::Tags => {
            scene_filter.tags = Some(HierarchicalMultiCriterionInput {
                depth: None,
                modifier: CriterionModifier::INCLUDES,
                value: Some(ids),
            });
        }
    }

    let markers = state
        .api
        .find_markers(find_markers_query::Variables {
            filter: Some(FindFilterType {
                per_page: Some(-1),
                page: None,
                q: None,
                sort: None,
                direction: None,
            }),
            scene_marker_filter: Some(scene_filter),
        })
        .await?;

    let api_key = &state.config.api_key;
    let markers = markers
        .into_iter()
        .map(|m| {
            let title = formatted_scene(&m);
            let (_, end) = state.ffmpeg.get_time_range(&m);
            Marker {
                id: m.id,
                primary_tag: m.primary_tag.name,
                stream_url: add_api_key(&m.stream, api_key),
                screenshot_url: add_api_key(&m.screenshot, api_key),
                start: m.seconds as u32,
                end,
                scene_title: title,
            }
        })
        .collect();

    Ok(Json(markers))
}
