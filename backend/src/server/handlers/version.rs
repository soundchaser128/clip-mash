use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::server::error::AppError;
use crate::server::handlers::AppState;
use crate::service::new_version_checker::AppVersion;

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/version",
    responses(
        (status = 200, description = "Return the version of the application", body= AppVersion)
    )
)]
pub async fn get_version(State(state): State<Arc<AppState>>) -> Result<Json<AppVersion>, AppError> {
    let data = state.new_version_checker.check_for_updates().await?;
    Ok(Json(data))
}
