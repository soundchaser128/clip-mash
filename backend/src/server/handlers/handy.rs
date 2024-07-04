use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::ToSchema;

use crate::server::error::AppError;
use crate::server::handlers::AppState;
use crate::service::handy::client::{HandyClient, IHandyClient};
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
/// Start the handy with the given pattern and key.
pub async fn start_handy(
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
/// Stop the handy's movement
pub async fn stop_handy() -> Result<(), AppError> {
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
/// Pause the handy's movement
pub async fn pause_handy() -> Result<(), AppError> {
    patterns::pause().await;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/api/handy",
    responses(
        (status = 200, description = "Get the current status of the handy", body = Option<ControllerStatus>),
    )
)]
#[axum::debug_handler]
/// Get the current status of the handy
pub async fn handy_status() -> Result<impl IntoResponse, AppError> {
    let status = patterns::status().await;
    Ok(Json(status))
}

#[derive(Serialize, ToSchema)]
pub struct HandyConnectedResponse {
    pub connected: bool,
}

#[utoipa::path(
    get,
    path = "/api/handy/connected",
    responses(
        (status = 200, description = "Get the connection status of the handy", body = HandyConnectedResponse),
    )
)]
#[axum::debug_handler]
/// Get the connection status of the handy
pub async fn handy_connected(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let settings = state.database.settings.fetch().await?;
    if let Some(handy) = settings.handy {
        let client = HandyClient::new(handy.key);
        let is_connected = client.is_connected().await?;
        info!("handy connected: {is_connected}");
        Ok(Json(HandyConnectedResponse {
            connected: is_connected,
        }))
    } else {
        Ok(Json(HandyConnectedResponse { connected: false }))
    }
}
