use std::cmp::Reverse;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use reqwest::StatusCode;
use serde::Deserialize;
use tracing::{debug, info};
use utoipa::IntoParams;

use crate::data::stash_api::{FilterMode, StashApi};
use crate::server::dtos::StashSceneWrapper;
use crate::server::error::AppError;
use crate::server::handlers::AppState;
use crate::server::types::*;
use crate::service::stash_config::Config;
use crate::util::add_api_key;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkerOptions {
    pub selected_ids: String,
    pub mode: FilterMode,
    pub include_all: bool,
}

#[axum::debug_handler]
pub async fn fetch_tags() -> Result<Json<Vec<TagDto>>, AppError> {
    let api = StashApi::load_config().await?;
    let tags = api.find_tags().await?;
    let mut tags: Vec<_> = tags
        .into_iter()
        .map(|t| TagDto {
            name: t.name,
            id: t.id,
            marker_count: t.scene_marker_count.unwrap_or_default(),
        })
        .filter(|t| t.marker_count > 0)
        .collect();
    tags.sort_by_key(|t| Reverse(t.marker_count));

    debug!("returning tags {:?}", tags);

    Ok(Json(tags))
}

#[axum::debug_handler]
pub async fn fetch_performers() -> Result<Json<Vec<PerformerDto>>, AppError> {
    let config = Config::get().await?;
    let api = StashApi::from_config(&config);
    let performers = api.find_performers().await?;
    let mut performers: Vec<_> = performers
        .into_iter()
        .map(|p| PerformerDto {
            id: p.id,
            scene_count: p.scene_count.unwrap_or_default(),
            name: p.name,
            image_url: p.image_path.map(|url| add_api_key(&url, &config.api_key)),
            tags: p.tags.into_iter().map(|t| t.name).collect(),
            rating: p.rating100,
            favorite: p.favorite,
        })
        .filter(|p| p.scene_count > 0)
        .collect();
    performers.sort_by_key(|t| Reverse(t.scene_count));

    debug!("returning performers {:?}", performers);

    Ok(Json(performers))
}

#[axum::debug_handler]
pub async fn fetch_markers(query: Query<MarkerOptions>) -> Result<Json<Vec<MarkerDto>>, AppError> {
    let config = Config::get().await?;
    let api = StashApi::from_config(&config);
    info!("fetching markers for query {query:?}");
    let ids: Vec<_> = query.selected_ids.split(',').map(From::from).collect();

    let markers = api.find_markers(ids, query.mode, query.include_all).await?;
    let markers = markers.into_iter().map(From::from).collect();
    Ok(Json(markers))
}

#[axum::debug_handler]
pub async fn fetch_scenes() -> Result<Json<Vec<StashScene>>, AppError> {
    let config = Config::get().await?;
    let api = StashApi::from_config(&config);
    let videos = api.find_scenes().await?;
    let videos = videos
        .into_iter()
        .map(|scene| {
            StashScene::from(StashSceneWrapper {
                scene,
                api_key: &config.api_key,
            })
        })
        .collect();
    Ok(Json(videos))
}

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ConfigQuery {
    url: String,
    api_key: String,
}

#[utoipa::path(
    get,
    path = "/api/stash/health",
    params(ConfigQuery),
    responses(
        (status = 200, description = "Stash server is reachable and API key is valid", body = String),
        (status = 500, description = "Stash server is not reachable or API key is invalid", body = String),
    )
)]
#[axum::debug_handler]
pub async fn get_health(
    Query(ConfigQuery { url, api_key }): Query<ConfigQuery>,
) -> Result<impl IntoResponse, AppError> {
    let api = StashApi::new(&url, &api_key);
    let result = api.health().await?;
    Ok(Json(result))
}

#[axum::debug_handler]
pub async fn get_config() -> impl IntoResponse {
    match Config::get().await {
        Ok(config) => Json(Some(config)),
        Err(_) => Json(None),
    }
}

#[axum::debug_handler]
pub async fn set_config(
    state: State<Arc<AppState>>,
    Json(config): Json<Config>,
) -> Result<StatusCode, AppError> {
    use crate::service::stash_config;

    info!("setting config with URL {}", config.stash_url);
    stash_config::set_config(config, &state.directories).await?;

    Ok(StatusCode::NO_CONTENT)
}
