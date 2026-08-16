use std::env;
use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use tracing::{error, info, warn};

use crate::server::error::AppError;
use crate::server::handlers::AppState;
use clip_mash::data::database::Settings;
use clip_mash::service::new_version_checker::AppVersion;

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/system/version",
    responses(
        (status = 200, description = "Return the version of the application", body = AppVersion)
    )
)]
pub async fn get_version(State(state): State<Arc<AppState>>) -> Result<Json<AppVersion>, AppError> {
    let result = state.new_version_checker.check_for_updates().await;
    match result {
        Ok(version) => Ok(Json(version)),
        Err(err) => {
            warn!("error while checking for updates: {}", err);
            Ok(Json(AppVersion {
                newest_version: "0.0.0".into(),
                current_version: env!("CARGO_PKG_VERSION").to_string(),
                needs_update: false,
            }))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/system/configuration",
    responses(
        (status = 200, description = "Returns the application configuration", body = Settings),
    )
)]
#[axum::debug_handler]
pub async fn get_config(state: State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let config = state.database.settings.fetch().await?;
    Ok(Json(config))
}

#[utoipa::path(
    post,
    path = "/api/system/configuration",
    request_body = Settings,
    responses(
        (status = 204, description = "Application configuration successfully set", body = ()),
    )
)]
#[axum::debug_handler]
pub async fn set_config(
    state: State<Arc<AppState>>,
    Json(config): Json<Settings>,
) -> Result<Json<&'static str>, AppError> {
    info!("setting config {:#?}", config);
    state.database.settings.set(config).await?;

    Ok(Json("OK"))
}

#[axum::debug_handler]
pub async fn sentry_error() {
    panic!("Sentry backend error test")
}

#[utoipa::path(
    post,
    path = "/api/system/restart",
    responses(
        (status = 204, description = "Restart the server", body = ()),
    )
)]
#[axum::debug_handler]
pub async fn restart() {
    use std::env;
    use std::process::Command;

    info!("Restarting server");
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    Command::new(program)
        .args(&args[1..])
        .spawn()
        .expect("Failed to restart the process");

    std::process::exit(0);
}

#[utoipa::path(
    get,
    path = "/api/system/health",
    responses(
        (status = 200, description = "The application is healthy", body = ()),
        (status = 503, description = "The application is not healthy", body = ()),
    )
)]
#[axum::debug_handler]
pub async fn get_app_health(state: State<Arc<AppState>>) -> impl IntoResponse {
    match state.database.settings.fetch_optional().await {
        Err(e) => {
            error!("Failed to fetch settings: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "status": "error",
                    "message": e.to_string()
                })),
            )
        }
        _ => (
            StatusCode::OK,
            Json(json!({
                "status": "ok"
            })),
        ),
    }
}
