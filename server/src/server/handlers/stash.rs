use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::data::stash_api::StashApi;
use crate::server::error::AppError;

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
pub async fn get_stash_health(
    Query(ConfigQuery { url, api_key }): Query<ConfigQuery>,
) -> Result<impl IntoResponse, AppError> {
    let api = StashApi::new(url, api_key);
    let result = api.health().await?;
    Ok(Json(result))
}
