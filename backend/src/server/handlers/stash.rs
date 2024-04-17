use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use tracing::info;
use utoipa::IntoParams;

use crate::data::database::Settings;
use crate::data::stash_api::StashApi;
use crate::server::error::AppError;
use crate::server::handlers::AppState;
use crate::service::stash_config::StashConfig;

#[utoipa::path(
    get,
    path = "/api/stash/config",
    responses(
        (status = 200, description = "The stash configuration if it exists", body = StashConfig),
    )
)]
#[axum::debug_handler]
pub async fn get_config(state: State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let config = state.stash_config_optional().await?;
    match config {
        Some(config) => Ok(Json(Some(config))),
        None => Ok(Json(None)),
    }
}

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ConfigQuery {
    url: String,
    api_key: Option<String>,
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
    let api = StashApi::new(url, api_key);
    let result = api.health().await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/api/stash/config",
    request_body = StashConfig,
    responses(
        (status = 204, description = "Stash configuration successfully set", body = ()),
    )
)]
#[axum::debug_handler]
pub async fn set_config(
    state: State<Arc<AppState>>,
    Json(config): Json<StashConfig>,
) -> Result<Json<&'static str>, AppError> {
    info!("setting config with URL {}", config.stash_url);
    state
        .database
        .settings
        .set(Settings { stash: config })
        .await?;

    Ok(Json("OK"))
}
