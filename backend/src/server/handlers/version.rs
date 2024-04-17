use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use tracing::warn;

use crate::server::error::AppError;
use crate::server::handlers::AppState;
use crate::service::new_version_checker::AppVersion;

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/version",
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
