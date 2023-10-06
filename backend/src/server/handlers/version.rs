use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Version {
    pub version: &'static str,
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/version",
    responses(
        (status = 200, description = "Return the version of the application", body= Version)
    )
)]
pub async fn get_version() -> Json<Version> {
    let version = env!("CARGO_PKG_VERSION");
    Json(Version { version })
}
