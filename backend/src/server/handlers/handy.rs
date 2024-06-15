use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use tracing::info;
use utoipa::ToSchema;

use crate::server::error::AppError;
use crate::service::handy::patterns::{self, HandyController, HandyPattern};

#[derive(Deserialize, ToSchema)]
pub struct StartHandyParameters {
    key: String,
    pattern: HandyPattern,
}

#[utoipa::path(
    post,
    path = "/api/handy/start",
    request_body = StartHandyParameters,
    responses(
        (status = 200, description = "Started motion successfully", body = ()),
    )
)]
#[axum::debug_handler]
/// Start the handy
pub async fn start(
    Json(StartHandyParameters { key, pattern }): Json<StartHandyParameters>,
) -> Result<(), AppError> {
    info!("starting handy motion with pattern {pattern:?}");
    let controller = HandyController::new(key);
    controller.start(pattern).await?;

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/handy/stop",
    responses(
        (status = 200, description = "Stopped motion successfully", body = ()),
    )
)]
#[axum::debug_handler]
pub async fn stop() -> Result<(), AppError> {
    patterns::stop().await;

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/handy/pause",
    responses(
        (status = 200, description = "Paused motion successfully", body = ()),
    )
)]
#[axum::debug_handler]
pub async fn pause() -> Result<(), AppError> {
    patterns::pause().await;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/api/handy",
    responses(
        (status = 200, description = "Get the current status of the handy", body = ()),
    )
)]
#[axum::debug_handler]
pub async fn status() -> Result<impl IntoResponse, AppError> {
    let status = patterns::status().await;
    Ok(Json(status))
}
