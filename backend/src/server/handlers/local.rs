use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use clip_mash_types::{ListVideoDto, MarkerDto, PageParameters, VideoDto};
use reqwest::StatusCode;
use serde::Deserialize;
use tower::ServiceExt;
use tracing::{info, warn};
use url::Url;

use crate::data::database::CreateMarker;
use crate::server::dtos::Page;
use crate::server::error::AppError;
use crate::server::handlers::AppState;
use crate::service::local_video::VideoService;
use crate::service::preview_image::PreviewGenerator;

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

#[axum::debug_handler]
pub async fn get_video_preview(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    request: axum::http::Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    use tower_http::services::ServeFile;

    let video = state.database.get_video(&id).await?;
    if let Some(preview_image) = video.and_then(|v| v.video_preview_image) {
        let result = ServeFile::new(preview_image).oneshot(request).await;
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

    let marker = state.database.get_marker(id).await?;
    if let Some(preview_image) = marker.marker_preview_image {
        let result = ServeFile::new(preview_image).oneshot(request).await;
        Ok(result)
    } else {
        Err(AppError::StatusCode(StatusCode::NOT_FOUND))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddNewVideosBody {
    path: String,
    recurse: bool,
}

pub async fn add_new_videos(
    state: State<Arc<AppState>>,
    Json(body): Json<AddNewVideosBody>,
) -> Result<impl IntoResponse, AppError> {
    let video_service: VideoService = state.0.clone().into();
    video_service
        .add_new_videos(body.path, body.recurse)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct VideoSearchQuery {
    pub query: Option<String>,
}

#[axum::debug_handler]
pub async fn list_videos(
    Query(page): Query<PageParameters>,
    Query(VideoSearchQuery { query }): Query<VideoSearchQuery>,
    state: State<Arc<AppState>>,
) -> Result<Json<Page<ListVideoDto>>, AppError> {
    info!("handling list_videos request with page {page:?} and query {query:?}");
    let (videos, size) = state.database.list_videos(query.as_deref(), page).await?;
    Ok(Json(Page::new(videos, size, page)))
}

#[derive(Deserialize)]
pub struct ListMarkersQuery {
    pub ids: String,
}

#[axum::debug_handler]
pub async fn list_markers(
    Query(ListMarkersQuery { ids }): Query<ListMarkersQuery>,
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<MarkerDto>>, AppError> {
    let ids: Vec<_> = ids.split(',').map(|s| s.trim()).collect();
    let markers = state.database.get_markers_for_video_ids(&ids).await?;
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

#[axum::debug_handler]
pub async fn persist_marker(
    state: State<Arc<AppState>>,
    Json(mut marker): Json<CreateMarker>,
) -> Result<Json<MarkerDto>, AppError> {
    let validation = validate_marker(&marker);
    if !validation.is_empty() {
        Err(AppError::Validation(validation))
    } else {
        info!("saving marker {marker:?} to the database");

        if let Some(video) = state.database.get_video(&marker.video_id).await? {
            let preview_generator: PreviewGenerator = state.0.clone().into();
            let preview_image = preview_generator
                .generate_preview(&video.id, &video.file_path, video.duration / 2.0)
                .await?;
            marker.preview_image_path = Some(preview_image.to_string());
            let marker = state.database.persist_marker(marker).await?;

            Ok(Json(marker.into()))
        } else {
            warn!("No video found for marker {marker:?}, returning 404");
            Err(AppError::StatusCode(StatusCode::NOT_FOUND))
        }
    }
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

#[derive(Deserialize)]
pub struct DownloadVideoQuery {
    pub url: Url,
}

#[axum::debug_handler]
pub async fn download_video(
    Query(DownloadVideoQuery { url }): Query<DownloadVideoQuery>,
    state: State<Arc<AppState>>,
) -> Result<Json<VideoDto>, AppError> {
    let service = VideoService::from(state.0);
    let (video_id, path) = service.download_video(url).await?;
    let db_video = service.persist_downloaded_video(video_id, path).await?;
    Ok(Json(db_video.into()))
}
