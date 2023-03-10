use std::{cmp::Reverse, sync::Arc};

use axum::{extract::State, Json};
use reqwest::Url;
use serde::Serialize;

use crate::{
    error::AppError,
    stash_api::{find_performers_query, find_tags_query},
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
    pub name: String,
    pub id: String,
    pub scene_count: i64,
    pub image_url: Option<String>,
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
    state: State<Arc<AppState>>
) -> Result<Json<Vec<Marker>>, AppError> {
    todo!()
}