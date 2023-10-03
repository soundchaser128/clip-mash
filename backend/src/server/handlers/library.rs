use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tower::ServiceExt;
use tracing::{info, warn};
use utoipa::{IntoParams, ToSchema};

use crate::data::stash_api::StashApi;
use crate::server::error::AppError;
use crate::server::handlers::AppState;
use crate::server::types::{
    CreateMarker, ListVideoDto, MarkerDto, Page, PageParameters, StashVideoDto, UpdateMarker,
    VideoDto,
};
use crate::service::preview_image::PreviewGenerator;
use crate::service::scene_detection;
use crate::service::stash_config::StashConfig;
use crate::service::video::{AddVideosRequest, VideoService};

#[derive(Deserialize, IntoParams)]
pub struct VideoSearchQuery {
    pub query: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/library/video",
    params(VideoSearchQuery, PageParameters),
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

#[utoipa::path(
    get,
    path = "/api/library/video/stash",
    params(VideoSearchQuery, PageParameters),
    responses(
        (status = 200, description = "Lists all videos in Stash with given query", body = StashVideoDtoPage),
    )
)]
#[axum::debug_handler]
pub async fn list_stash_videos(
    Query(page): Query<PageParameters>,
    Query(VideoSearchQuery { query }): Query<VideoSearchQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Page<StashVideoDto>>, AppError> {
    info!("listing stash videos with page {page:?} and query {query:?}");
    let stash_api = StashApi::load_config_or_fail().await;
    if let Err(_) = stash_api {
        info!("no stash config found, returning empty page");
        return Ok(Json(Page::empty()));
    }
    let stash_api = stash_api.unwrap();
    let (stash_videos, count) = stash_api.find_scenes(&page, query).await?;
    info!("found {} stash videos", stash_videos.len());
    let ids: Vec<i64> = stash_videos
        .iter()
        .map(|v| v.id.parse().expect("stash id must be numeric"))
        .collect();
    let scene_ids_in_database = state.database.videos.get_stash_scene_ids(&ids).await?;
    let videos: Vec<_> = stash_videos
        .into_iter()
        .map(|v| {
            let video_dto = VideoDto::from(v);
            let id = video_dto.stash_scene_id.unwrap();
            let exists = scene_ids_in_database.contains(&id);
            StashVideoDto::from(video_dto, exists)
        })
        .collect();

    Ok(Json(Page::new(videos, count, page)))
}

#[utoipa::path(
    post,
    path = "/api/library/video",
    request_body = AddVideosRequest,
    responses(
        (status = 200, description = "Add new videos", body = Vec<VideoDto>),
    )
)]
#[axum::debug_handler]
pub async fn add_new_videos(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AddVideosRequest>,
) -> Result<impl IntoResponse, AppError> {
    let video_service = VideoService::new(state).await?;
    let api_key = StashConfig::get().await.map(|c| c.api_key).ok();
    let new_videos: Vec<_> = video_service
        .add_videos(request, api_key.as_deref())
        .await?
        .into_iter()
        .map(|v| VideoDto::from(v))
        .collect();

    Ok(Json(new_videos))
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VideoCleanupResponse {
    pub deleted_count: u32,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/library/video/cleanup",
    responses(
        (status = 200, description = "Delete unused videos", body = VideoCleanupResponse),
    )
)]
pub async fn cleanup_videos(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let video_service = VideoService::new(state).await?;
    let deleted_count = video_service.cleanup_videos().await?;
    Ok(Json(VideoCleanupResponse { deleted_count }))
}

#[utoipa::path(
    get,
    path = "/api/library/video/{id}",
    params(
        ("id" = String, Path, description = "The ID of the video to fetch")
    ),
    responses(
        (status = 200, description = "Get details for a video", body = ListVideoDto),
    )
)]
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
    post,
    path = "/api/library/video/{id}/detect-markers",
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

#[utoipa::path(
    get,
    path = "/api/library/marker",
    params(VideoSearchQuery, PageParameters),
    responses(
        (status = 200, description = "List markers", body = MarkerDtoPage),
    )
)]
#[axum::debug_handler]
pub async fn list_markers(
    Query(page): Query<PageParameters>,
    Query(VideoSearchQuery { query }): Query<VideoSearchQuery>,
    state: State<Arc<AppState>>,
) -> Result<Json<Page<MarkerDto>>, AppError> {
    let (markers, count) = state
        .database
        .markers
        .list_markers(query.as_deref(), &page)
        .await?;
    let markers = markers.into_iter().map(From::from).collect();
    Ok(Json(Page::new(markers, count as usize, page)))
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
    path = "/api/library/marker",
    request_body = CreateMarker,
    responses(
        (status = 200, description = "Create a new marker", body = MarkerDto),
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

            Ok(Json(MarkerDto::from_db(marker, &video)))
        } else {
            warn!("No video found for marker {marker:?}, returning 404");
            Err(AppError::StatusCode(StatusCode::NOT_FOUND))
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/library/marker",
    request_body = UpdateMarker,
    responses(
        (status = 200, description = "Updates a marker", body = MarkerDto),
    )
)]
#[axum::debug_handler]
pub async fn update_marker(
    state: State<Arc<AppState>>,
    Json(marker): Json<UpdateMarker>,
) -> Result<Json<MarkerDto>, AppError> {
    info!("updating marker with {marker:?}");

    let marker = state.database.markers.update_marker(marker).await?;
    let video = state
        .database
        .videos
        .get_video(&marker.video_id)
        .await?
        .expect("video for marker must exist");
    Ok(Json(MarkerDto::from_db(marker, &video)))
}

#[utoipa::path(
    delete,
    path = "/api/library/marker/{id}",
    params(
        ("id" = i64, Path, description = "The ID of the marker to delete"),
    ),
    responses(
        (status = 200, description = "Deletes the given marker", body = ()),
    )
)]
#[axum::debug_handler]
pub async fn delete_marker(
    Path(id): Path<i64>,
    state: State<Arc<AppState>>,
) -> Result<Json<&'static str>, AppError> {
    // TODO delete preview image as well
    info!("deleting marker {id}");
    state.database.markers.delete_marker(id).await?;

    Ok(Json("OK"))
}

#[derive(Deserialize)]
pub struct SplitMarkerQuery {
    pub time: f64,
}

#[utoipa::path(
    post,
    path = "/api/library/marker/{id}/split",
    params(
        ("id" = i64, Path, description = "The ID of the marker to split"),
        ("time" = f64, Query, description = "The time to split the marker at")
    ),
    responses(
        (status = 200, description = "Split a marker at the specified timestamp", body = Vec<MarkerDto>),
    )
)]
#[axum::debug_handler]
pub async fn split_marker(
    Path(id): Path<i64>,
    Query(SplitMarkerQuery { time }): Query<SplitMarkerQuery>,
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<MarkerDto>>, AppError> {
    info!("splitting marker {id} att time {time}");
    let video_id = state.database.markers.split_marker(id, time).await?;
    let data = state
        .database
        .videos
        .get_video_with_markers(&video_id)
        .await?
        .expect("markers must exist");

    let mut new_markers = data.markers;
    let preview_generator: PreviewGenerator = state.0.clone().into();
    let file_path = data.video.file_path.clone();
    for marker in &mut new_markers {
        if marker.marker_preview_image.is_none() {
            let path = preview_generator
                .generate_preview(&marker.video_id, &file_path, marker.start_time)
                .await?;
            marker.marker_preview_image = Some(path.to_string());
            state
                .database
                .markers
                .set_marker_preview_image(marker.rowid.unwrap(), path.as_str())
                .await?;
        }
    }

    Ok(Json(
        new_markers
            .into_iter()
            .map(|m| MarkerDto::from_db(m, &data.video))
            .collect(),
    ))
}
