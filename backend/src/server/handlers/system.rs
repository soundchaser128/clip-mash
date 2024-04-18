use std::env;
use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tracing::{error, info, warn};

use crate::server::error::AppError;
use crate::server::handlers::AppState;
use crate::service::new_version_checker::AppVersion;

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

#[axum::debug_handler]
pub async fn sentry_error() {
    panic!("Sentry backend error test")
}

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

#[axum::debug_handler]
pub async fn get_health(state: State<Arc<AppState>>) -> impl IntoResponse {
    if let Err(e) = state.database.settings.fetch_optional().await {
        error!("Failed to fetch settings: {}", e);
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "status": "error",
                "message": e.to_string()
            })),
        )
    } else {
        (
            StatusCode::OK,
            Json(json!({
                "status": "ok"
            })),
        )
    }
}
