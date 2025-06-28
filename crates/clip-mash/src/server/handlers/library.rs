use std::collections::HashMap;
use std::sync::Arc;

use axum::Json;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use camino::Utf8Path;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};
use utoipa::{IntoParams, ToSchema};

use crate::data::database::markers::{ListMarkersFilter, MarkerCount};
use crate::data::database::videos::{TagCount, VideoSearchQuery, VideoSource, VideoUpdate};
use crate::data::stash_api::StashApi;
use crate::server::error::AppError;
use crate::server::handlers::AppState;
use crate::server::types::{
    CreateMarker, ListVideoDto, MarkerDto, MarkerDtoConverter, Page, PageParameters, StashVideoDto,
    UpdateMarker, VideoDetailsDto, VideoDetailsDtoConverter, VideoDto,
};
use crate::service::encoding_optimization::EncodingOptimizationService;
use crate::service::migrations::Migrator;
use crate::service::preview_image::PreviewGenerator;
use crate::service::scene_detection;
use crate::service::video::{AddVideosRequest, VideoService};

#[utoipa::path(
    get,
    path = "/api/library/video",
    params(VideoSearchQuery, PageParameters),
    responses(
        (status = 200, description = "Lists all videos with given query", body = Page<ListVideoDto>),
    )
)]
#[axum::debug_handler]
/// Lists videos (paginated, with search)
pub async fn list_videos(
    Query(page): Query<PageParameters>,
    Query(mut query): Query<VideoSearchQuery>,
    state: State<Arc<AppState>>,
) -> Result<Json<Page<ListVideoDto>>, AppError> {
    query.query = query.query.map(|q| format!("\"{}\"", q.trim()));
    info!("handling list_videos request with page {page:?} and query {query:?}");
    let (videos, size) = state.database.videos.list_videos(query, &page).await?;
    Ok(Json(Page::new(videos, size, page)))
}

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct StashVideoQuery {
    pub query: Option<String>,
    pub with_markers: Option<bool>,
}

#[utoipa::path(
    get,
    path = "/api/library/video/stash",
    params(StashVideoQuery, PageParameters),
    responses(
        (status = 200, description = "Lists all videos in Stash with given query", body = Page<StashVideoDto>),
    )
)]
#[axum::debug_handler]
/// Lists videos on the configured Stash instance
pub async fn list_stash_videos(
    Query(page): Query<PageParameters>,
    Query(StashVideoQuery {
        query,
        with_markers,
    }): Query<StashVideoQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Page<StashVideoDto>>, AppError> {
    info!("listing stash videos with page {page:?} and query {query:?}");
    let settings = state.database.settings.fetch_optional().await?;

    if settings.is_none() {
        info!("no stash config found, returning empty page");
        return Ok(Json(Page::empty()));
    }
    let stash_api = StashApi::with_config(settings.unwrap().stash);

    let (stash_videos, count) = stash_api.find_scenes(&page, query, with_markers).await?;
    info!("found {} stash videos", stash_videos.len());
    let ids: Vec<i64> = stash_videos
        .iter()
        .map(|v| v.id.parse().expect("stash id must be numeric"))
        .collect();
    let scene_ids_in_database = state.database.videos.get_stash_scene_ids(&ids).await?;
    let videos: Vec<_> = stash_videos
        .into_iter()
        .map(|v| {
            let marker_count = v.scene_markers.len();
            let video_dto = VideoDto::from(v);
            let id = video_dto.stash_scene_id.unwrap();
            let exists = scene_ids_in_database.contains(&id);
            StashVideoDto::from(video_dto, exists, marker_count)
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
/// Adds new videos either via stash, local files or URL (to download)
pub async fn add_new_videos(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AddVideosRequest>,
) -> Result<impl IntoResponse, AppError> {
    let video_service = VideoService::new(state).await?;
    let new_videos: Vec<_> = video_service
        .add_videos(request)
        .await?
        .into_iter()
        .map(VideoDto::from)
        .collect();

    Ok(Json(new_videos))
}

#[derive(Serialize, ToSchema)]
pub struct ListPerformerResponse {
    pub title: String,
    pub count: usize,
}

#[derive(Deserialize, IntoParams)]
pub struct ListPerformersQuery {
    pub prefix: Option<String>,
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/library/performers",
    params(ListPerformersQuery),
    responses(
        (status = 200, description = "List all performers", body = Vec<ListPerformerResponse>),
    )
)]
/// Lists all performers from videos and their number of markers
pub async fn list_performers(
    State(state): State<Arc<AppState>>,
    Query(ListPerformersQuery { prefix }): Query<ListPerformersQuery>,
) -> Result<impl IntoResponse, AppError> {
    let performers = state
        .database
        .performers
        .find_by_prefix(prefix.as_deref().unwrap_or(""))
        .await?;
    let performers: Vec<_> = performers
        .into_iter()
        .map(|p| ListPerformerResponse {
            title: p.name,
            count: p.marker_count as usize,
        })
        .collect();
    Ok(Json(performers))
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/library/video/need-encoding",
    request_body = Vec<String>,
    responses(
        (status = 200, description = "Check if the videos can be combined without encoding", body = bool),
    )
)]
/// Returns whether a set of videos need to be re-encoded or not
pub async fn videos_need_encoding(
    State(state): State<Arc<AppState>>,
    Json(mut video_ids): Json<Vec<String>>,
) -> Result<impl IntoResponse, AppError> {
    if video_ids.is_empty() {
        let all_ids = state.database.videos.get_video_ids_with_markers().await?;
        video_ids.extend(all_ids);
    }

    let service = EncodingOptimizationService::new(state.database.clone());
    let ids: Vec<_> = video_ids.iter().map(|s| s.as_str()).collect();

    let needs_encoding = service.needs_re_encode(&ids).await?;
    Ok(Json(needs_encoding))
}

#[utoipa::path(
    put,
    path = "/api/library/video/{id}",
    params(
        ("id" = String, Path, description = "The ID of the video to update")
    ),
    request_body = VideoUpdate,
    responses(
        (status = 200, description = "Update video metadata", body = ()),
    )
)]
/// Updates video metadata
pub async fn update_video(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<VideoUpdate>,
) -> Result<impl IntoResponse, AppError> {
    state.database.videos.update_video(&id, body).await?;

    Ok(Json("OK"))
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
/// Removes videos that don't exist on disk
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
        (status = 200, description = "Get details for a video", body = VideoDetailsDto),
    )
)]
#[axum::debug_handler]
/// Gets details on a single video
pub async fn get_video(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
) -> Result<Json<VideoDetailsDto>, AppError> {
    let video = state.database.videos.get_video_with_markers(&id).await?;
    if let Some(video) = video {
        let stash_api = state.stash_api().await?;
        let converter = VideoDetailsDtoConverter::new(stash_api);
        let dto = converter.from_db(video);
        Ok(Json(dto))
    } else {
        Err(AppError::StatusCode(StatusCode::NOT_FOUND))
    }
}

#[utoipa::path(
    delete,
    path = "/api/library/video/{id}",
    params(
        ("id" = String, Path, description = "The ID of the video to delete")
    ),
    responses(
        (status = 200, description = "Delete a single video from the database", body = ()),
    )
)]
#[axum::debug_handler]
/// Deletes a video
pub async fn delete_video(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    use tokio::fs;

    let video = state.database.videos.get_video(&id).await?;
    if let Some(video) = video {
        if video.source == VideoSource::Download {
            let path = Utf8Path::new(&video.file_path);
            if let Err(e) = fs::remove_file(&path).await {
                warn!("failed to delete downloaded video at {path}: {e:?}");
            }

            info!("deleted downloaded video at {path}");
        }
    }

    state.database.videos.delete_video(&id).await?;
    Ok(Json("OK"))
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
/// Tries to detect markers in a video by detecting scene changes.
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
/// Serves the video file for a given video ID
pub async fn get_video_file(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    request: axum::http::Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    use tower::util::ServiceExt;
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
/// Serves the preview image for a given video ID
pub async fn get_video_preview(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    request: axum::http::Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    use tower::util::ServiceExt;
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
/// Gets the generated preview image for a marker
pub async fn get_marker_preview(
    Path(id): Path<i64>,
    state: State<Arc<AppState>>,
    request: axum::http::Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    use tower::util::ServiceExt;
    use tower_http::services::ServeFile;

    debug!("getting preview image for marker {id}");
    let marker = state.database.markers.get_marker(id).await?;
    if let Some(preview_image) = marker.marker_preview_image {
        debug!("preview image found at {preview_image}");
        let result = ServeFile::new(preview_image).oneshot(request).await;
        Ok(result)
    } else {
        Err(AppError::StatusCode(StatusCode::NOT_FOUND))
    }
}

#[derive(Deserialize, IntoParams, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListMarkersQuery {
    pub video_ids: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/library/marker",
    params(ListMarkersQuery),
    responses(
        (status = 200, description = "List markers", body = Vec<MarkerDto>),
    )
)]
#[axum::debug_handler]
/// Lists all markers for a set of video IDs.
pub async fn list_markers(
    state: State<Arc<AppState>>,
    Query(body): Query<ListMarkersQuery>,
) -> Result<Json<Vec<MarkerDto>>, AppError> {
    info!("listing markers for {body:?}");
    let video_ids: Option<Vec<_>> = body
        .video_ids
        .map(|ids| ids.split(',').map(String::from).collect());
    if video_ids.is_none() || video_ids.as_ref().unwrap().is_empty() {
        info!("no video IDs provided, returning empty list");
        return Ok(Json(vec![]));
    }
    let markers = state
        .database
        .markers
        .list_markers(Some(ListMarkersFilter::VideoIds(video_ids.unwrap())), None)
        .await?;
    let stash_api = state.stash_api().await?;
    let converter = MarkerDtoConverter::new(stash_api);

    let markers = markers
        .into_iter()
        .map(|m| converter.from_db_with_video(m))
        .collect();
    Ok(Json(markers))
}

#[derive(Deserialize, IntoParams)]
pub struct ListMarkerTitlesQuery {
    pub count: Option<i64>,
    pub prefix: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/library/marker/title",
    params(ListMarkerTitlesQuery),
    responses(
        (status = 200, description = "List marker titles", body = Vec<MarkerCount>),
    )
)]
#[axum::debug_handler]
/// Lists marker titles and nunber of occurrences
pub async fn list_marker_titles(
    Query(ListMarkerTitlesQuery { count, prefix }): Query<ListMarkerTitlesQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<MarkerCount>>, AppError> {
    let count = count.unwrap_or(20);
    let results = state
        .database
        .markers
        .get_marker_titles(count, prefix.as_deref())
        .await?;
    Ok(Json(results))
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

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateMarkerRequest {
    marker: CreateMarker,
    create_in_stash: bool,
}

#[utoipa::path(
    post,
    path = "/api/library/marker",
    request_body = CreateMarkerRequest,
    responses(
        (status = 200, description = "Create a new marker", body = MarkerDto),
    )
)]
#[axum::debug_handler]
/// Creates a new marker for a video.
pub async fn create_new_marker(
    state: State<Arc<AppState>>,
    Json(body): Json<CreateMarkerRequest>,
) -> Result<Json<MarkerDto>, AppError> {
    let CreateMarkerRequest {
        mut marker,
        create_in_stash,
    } = body;

    let validation = validate_marker(&marker);
    if !validation.is_empty() {
        Err(AppError::Validation(validation))
    } else {
        info!("saving marker {marker:?} to the database");

        if let Some(video) = state.database.videos.get_video(&marker.video_id).await? {
            let preview_generator: PreviewGenerator = state.0.clone().into();
            let preview_image = preview_generator
                .generate_preview(&video.id, &video.file_path, marker.start)
                .await?;
            marker.preview_image_path = Some(preview_image.to_string());
            let marker = state.database.markers.create_new_marker(marker).await?;
            let stash_api = state.stash_api().await?;
            let converter = MarkerDtoConverter::new(stash_api);

            if create_in_stash && video.source == VideoSource::Stash {
                let scene_id = video.stash_scene_id.unwrap();
                let stash_api = state.stash_api().await?;
                let stash_id = stash_api
                    .add_marker(marker.clone(), scene_id.to_string())
                    .await?;
                state
                    .database
                    .markers
                    .update_marker(
                        marker.rowid.unwrap(),
                        UpdateMarker {
                            stash_marker_id: Some(stash_id),
                            ..Default::default()
                        },
                    )
                    .await?;
            }

            Ok(Json(converter.from_db(marker, &video)))
        } else {
            warn!("No video found for marker {marker:?}, returning 404");
            Err(AppError::StatusCode(StatusCode::NOT_FOUND))
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/library/marker/{id}",
    params(
        ("id" = i64, Path, description = "The ID of the marker to update")
    ),
    request_body = UpdateMarker,
    responses(
        (status = 200, description = "Updates a marker", body = MarkerDto),
    )
)]
#[axum::debug_handler]
/// Update a marker, additionally updates the marker in Stash if applicable and desired.
pub async fn update_marker(
    state: State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(marker): Json<UpdateMarker>,
) -> Result<Json<MarkerDto>, AppError> {
    info!("updating marker with {marker:?}");

    let marker = state.database.markers.update_marker(id, marker).await?;
    let video = state
        .database
        .videos
        .get_video(&marker.video_id)
        .await?
        .expect("video for marker must exist");
    let stash_api = state.stash_api().await?;
    let converter = MarkerDtoConverter::new(stash_api);
    Ok(Json(converter.from_db(marker, &video)))
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
/// Deletes a marker.
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
/// Splits a marker into two at the specified time.
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
                .set_marker_preview_image(marker.rowid.unwrap(), Some(path.as_str()))
                .await?;
        }
    }

    let stash_api = state.stash_api().await?;
    let converter = MarkerDtoConverter::new(stash_api);

    Ok(Json(
        new_markers
            .into_iter()
            .map(|m| converter.from_db(m, &data.video))
            .collect(),
    ))
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/library/video/{id}/stash/merge",
    params(
        ("id" = String, Path, description = "The ID of video to merge"),
    ),
    responses(
        (status = 200, description = "Merge the video data from stash into the local video", body = ListVideoDto),
    )
)]
/// Synchronizes a single video with stash
pub async fn merge_stash_video(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let video_service = VideoService::new(state).await?;
    let new_video = video_service.merge_stash_scene(&id).await?;
    info!("new video after merging: {new_video:?}");

    Ok(Json(new_video))
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/library/migrate/preview",
    responses(
        (status = 200, description = "Successfully migrated preview images to WebP", body = ()),
    )
)]
pub async fn migrate_preview_images(State(state): State<Arc<AppState>>) -> Result<(), AppError> {
    let migrator = Migrator::new(
        state.database.clone(),
        state.directories.clone(),
        state.ffmpeg_location.clone(),
    );

    migrator.migrate_preview_images().await?;

    Ok(())
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/library/video/tags",
    responses(
        (status = 200, description = "List tags for videos", body = Vec<TagCount>),
    )
)]
pub async fn list_video_tags(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TagCount>>, AppError> {
    let tags = state.database.videos.list_tags(100).await?;
    Ok(Json(tags))
}
