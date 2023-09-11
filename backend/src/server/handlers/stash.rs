use axum::response::IntoResponse;
use axum::Json;

use crate::service::stash_config::Config;

#[utoipa::path(
    get,
    path = "/api/stash/config",
    responses(
        (status = 200, description = "The stash configuration if it exists", body = Config),
    )
)]
#[axum::debug_handler]
pub async fn get_config() -> impl IntoResponse {
    match Config::get().await {
        Ok(config) => Json(Some(config)),
        Err(_) => Json(None),
    }
}
