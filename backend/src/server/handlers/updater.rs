use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use reqwest::StatusCode;
use serde::Deserialize;
use tracing::error;
use utoipa::IntoParams;

use crate::server::error::AppError;
use crate::server::handlers::AppState;
use crate::service::updater::{self, Updater};

#[derive(Debug, Deserialize, IntoParams)]
pub struct SelfUpdateQuery {
    pub tag: Option<String>,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/self/update",
    params(SelfUpdateQuery),
    responses(
        (status = 200, description = "Successfully updated itself", body = ()),
    )
)]
pub async fn self_update(Query(query): Query<SelfUpdateQuery>) -> impl IntoResponse {
    if let Err(e) = updater::self_update(query.tag.as_deref()).await {
        error!("failed to self-update: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::OK
    }
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/self/update",
    responses(
        (status = 200, description = "Currently installed version and whether it is the newest", body = AppVersion),
    )
)]
pub async fn check_for_updates(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let updater = Updater::from(state);
    let app_version = updater.check_for_updates().await?;

    Ok(Json(app_version))
}
