use std::time::Duration;

use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use tracing::info;
use utoipa::ToSchema;

use crate::server::error::AppError;
use crate::service::handy::patterns::{
    CycleIncrementParameters, HandyController, HandyPattern, Range,
};

#[derive(Deserialize, ToSchema)]
pub struct StartHandyParameters {
    key: String,
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
    Json(StartHandyParameters { key }): Json<StartHandyParameters>,
) -> Result<(), AppError> {
    info!("starting handy motion");

    let parameters = CycleIncrementParameters {
        cycle_duration: Duration::from_secs(60),
        session_duration: Duration::from_secs(60 * 15),
        update_interval: Duration::from_millis(500),
        start_range: Range {
            min: 5.0,
            max: 15.0,
        },
        end_range: Range {
            min: 40.0,
            max: 70.0,
        },
        stroke_range: Range {
            min: 0.0,
            max: 80.0,
        },
    };

    let pattern = HandyPattern::CycleIncrement(parameters);
    let controller = HandyController::new(key);
    controller.start(pattern);

    Ok(())
}

#[axum::debug_handler]
pub async fn stop() -> Result<(), AppError> {
    todo!()
}

#[axum::debug_handler]
pub async fn status() -> Result<impl IntoResponse, AppError> {
    Ok(Json("nothing yet"))
}
