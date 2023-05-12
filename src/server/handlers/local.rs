use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use camino::Utf8PathBuf;
use reqwest::StatusCode;
use serde::Deserialize;
use tower::ServiceExt;
use tracing::info;

use crate::{
    data::database::CreateMarker,
    server::{
        dtos::{ListVideoDto, MarkerDto},
        error::AppError,
        handlers::AppState,
    },
};

#[axum::debug_handler]
pub async fn get_video(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    request: axum::http::Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    use tower_http::services::ServeFile;

    let video = state.database.get_video(&id).await?;
    if let Some(video) = video {
        let result = ServeFile::new(video.file_path).oneshot(request).await;
        Ok(result)
    } else {
        Err(AppError::StatusCode(StatusCode::NOT_FOUND))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListVideoQuery {
    path: String,
    recurse: bool,
}

#[axum::debug_handler]
pub async fn list_videos(
    Query(ListVideoQuery { path, recurse }): Query<ListVideoQuery>,
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<ListVideoDto>>, AppError> {
    use crate::service::local_video;

    let videos =
        local_video::list_videos(Utf8PathBuf::from(path), recurse, &state.database).await?;
    Ok(Json(videos.into_iter().map(From::from).collect()))
}

#[axum::debug_handler]
pub async fn persist_marker(
    state: State<Arc<AppState>>,
    Json(marker): Json<CreateMarker>,
) -> Result<Json<MarkerDto>, AppError> {
    info!("saving marker {marker:?} to the database");
    let marker = state.database.persist_marker(marker).await?;

    Ok(Json(marker.into()))
}

#[axum::debug_handler]
pub async fn delete_marker(
    Path(id): Path<i64>,
    state: State<Arc<AppState>>,
) -> Result<(), AppError> {
    info!("deleting marker {id}");
    state.database.delete_marker(id).await?;

    Ok(())
}
