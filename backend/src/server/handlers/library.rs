use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use reqwest::StatusCode;
use serde::Deserialize;
use tower::ServiceExt;
use tracing::{info, warn};
use utoipa::IntoParams;

use crate::server::dtos::Page;
use crate::server::error::AppError;
use crate::server::handlers::AppState;
use crate::server::types::{CreateMarker, ListVideoDto, MarkerDto, PageParameters, UpdateMarker};
use crate::service::preview_image::PreviewGenerator;
use crate::service::scene_detection;

#[derive(Deserialize, IntoParams)]
pub struct VideoSearchQuery {
    pub query: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/local/video",
    params(VideoSearchQuery),
    responses(
        (status = 200, description = "Lists all videos with given query", body = ListVideoDtoPage),
    )
)]
#[axum::debug_handler]
pub async fn list_videos(
    Query(page): Query<PageParameters>,
    Query(VideoSearchQuery { query }): Query<VideoSearchQuery>,
    state: State<Arc<AppState>>,
) -> Result<Json<Page<ListVideoDto>>, AppError> {
    info!("handling list_videos request with page {page:?} and query {query:?}");
    let (videos, size) = state
        .database
        .videos
        .list_videos(query.as_deref(), &page)
        .await?;
    Ok(Json(Page::new(videos, size, page)))
}

pub async fn add_new_videos() -> impl IntoResponse {
    todo!()
}

#[axum::debug_handler]
pub async fn get_video(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
) -> Result<Json<ListVideoDto>, AppError> {
    let video = state.database.videos.get_video_with_markers(&id).await?;
    if let Some(video) = video {
        let dto = video.into();
        Ok(Json(dto))
    } else {
        Err(AppError::StatusCode(StatusCode::NOT_FOUND))
    }
}

#[derive(Deserialize)]
pub struct DetectMarkersQuery {
    pub threshold: Option<f64>,
}

#[utoipa::path(
    get,
    path = "/api/local/video/{id}/markers",
    params(
        ("id" = String, Path, description = "The ID of the video to detect markers for"),
        ("threshold" = Option<f64>, Query, description = "The threshold for the marker detection (from 0.0 to 1.0)")
    ),
    responses(
        (status = 200, description = "All newly created markers", body = Vec<MarkerDto>),
    )
)]
#[axum::debug_handler]
pub async fn detect_markers(
    Path(id): Path<String>,
    Query(DetectMarkersQuery { threshold }): Query<DetectMarkersQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<MarkerDto>>, AppError> {
    let created_markers =
        scene_detection::find_and_persist_markers(&id, threshold.unwrap_or(0.4), state.clone())
            .await?;
    Ok(Json(created_markers))
}

#[axum::debug_handler]
pub async fn get_video_file(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    request: axum::http::Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    use tower_http::services::ServeFile;

    let video = state.database.videos.get_video(&id).await?;
    if let Some(video) = video {
        let result = ServeFile::new(video.file_path).oneshot(request).await;
        Ok(result)
    } else {
        Err(AppError::StatusCode(StatusCode::NOT_FOUND))
    }
}

#[axum::debug_handler]
pub async fn get_video_preview(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    request: axum::http::Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    use tower_http::services::ServeFile;

    let video = state.database.videos.get_video(&id).await?;
    if let Some(preview_image) = video.and_then(|v| v.video_preview_image) {
        let mut result = ServeFile::new(preview_image)
            .oneshot(request)
            .await
            .unwrap();
        result.headers_mut().insert(
            "Cache-Control",
            "public, max-age=31536000, immutable".parse().unwrap(),
        );

        Ok(result)
    } else {
        Err(AppError::StatusCode(StatusCode::NOT_FOUND))
    }
}

#[axum::debug_handler]
pub async fn get_marker_preview(
    Path(id): Path<i64>,
    state: State<Arc<AppState>>,
    request: axum::http::Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    use tower_http::services::ServeFile;

    let marker = state.database.markers.get_marker(id).await?;
    if let Some(preview_image) = marker.marker_preview_image {
        let result = ServeFile::new(preview_image).oneshot(request).await;
        Ok(result)
    } else {
        Err(AppError::StatusCode(StatusCode::NOT_FOUND))
    }
}

#[derive(Deserialize)]
pub struct ListMarkersQuery {
    pub ids: String,
}

#[axum::debug_handler]
pub async fn list_markers(state: State<Arc<AppState>>) -> Result<Json<Vec<MarkerDto>>, AppError> {
    let markers = state.database.markers.get_all_markers().await?;
    let markers = markers.into_iter().map(From::from).collect();
    Ok(Json(markers))
}

fn validate_marker(marker: &CreateMarker) -> HashMap<&'static str, &'static str> {
    let mut errors = HashMap::new();
    if marker.title.trim().is_empty() {
        errors.insert("title", "Title must not be empty");
    }
    if marker.end <= marker.start {
        errors.insert("end", "Marker end must be after start");
    }
    errors
}

#[utoipa::path(
    post,
    path = "/api/local/video/marker",
    request_body = CreateMarker,
    responses(
        (status = 200, description = "The newly created marker", body = MarkerDto),
    )
)]
#[axum::debug_handler]
pub async fn create_new_marker(
    state: State<Arc<AppState>>,
    Json(mut marker): Json<CreateMarker>,
) -> Result<Json<MarkerDto>, AppError> {
    let validation = validate_marker(&marker);
    if !validation.is_empty() {
        Err(AppError::Validation(validation))
    } else {
        info!("saving marker {marker:?} to the database");

        if let Some(video) = state.database.videos.get_video(&marker.video_id).await? {
            let preview_generator: PreviewGenerator = state.0.clone().into();
            let preview_image = preview_generator
                .generate_preview(&video.id, &video.file_path, video.duration / 2.0)
                .await?;
            marker.preview_image_path = Some(preview_image.to_string());
            let marker = state.database.markers.create_new_marker(marker).await?;

            Ok(Json(marker.into()))
        } else {
            warn!("No video found for marker {marker:?}, returning 404");
            Err(AppError::StatusCode(StatusCode::NOT_FOUND))
        }
    }
}

#[axum::debug_handler]
pub async fn update_marker(
    state: State<Arc<AppState>>,
    Json(marker): Json<UpdateMarker>,
) -> Result<Json<MarkerDto>, AppError> {
    info!("updating marker with {marker:?}");

    let marker = state.database.markers.update_marker(marker).await?;
    Ok(Json(marker.into()))
}

#[axum::debug_handler]
pub async fn delete_marker(
    Path(id): Path<i64>,
    state: State<Arc<AppState>>,
) -> Result<(), AppError> {
    info!("deleting marker {id}");
    state.database.markers.delete_marker(id).await?;

    Ok(())
}

#[derive(Deserialize)]
pub struct SplitMarkerQuery {
    pub time: f64,
}

#[axum::debug_handler]
pub async fn split_marker(
    Path(id): Path<i64>,
    Query(SplitMarkerQuery { time }): Query<SplitMarkerQuery>,
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<MarkerDto>>, AppError> {
    info!("splitting marker {id} att time {time}");
    let video_id = state.database.markers.split_marker(id, time).await?;
    let mut new_markers = state
        .database
        .videos
        .get_video_with_markers(&video_id)
        .await?
        .expect("markers must exist")
        .markers;

    let preview_generator: PreviewGenerator = state.0.clone().into();
    for marker in &mut new_markers {
        if marker.marker_preview_image.is_none() {
            let path = preview_generator
                .generate_preview(&marker.video_id, &marker.file_path, marker.start_time)
                .await?;
            marker.marker_preview_image = Some(path.to_string());
            state
                .database
                .markers
                .set_marker_preview_image(marker.rowid.unwrap(), path.as_str())
                .await?;
        }
    }

    Ok(Json(new_markers.into_iter().map(From::from).collect()))
}
