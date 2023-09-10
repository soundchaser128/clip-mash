use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Version {
    pub version: &'static str,
}

#[axum::debug_handler]
pub async fn get_version() -> Json<Version> {
    let version = env!("CARGO_PKG_VERSION");
    Json(Version { version })
}
