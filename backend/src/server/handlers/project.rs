use std::sync::Arc;

use axum::body::StreamBody;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use camino::Utf8PathBuf;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;
use tracing::{debug, error, info};
use utoipa::{IntoParams, ToSchema};

use super::AppState;
use crate::server::error::AppError;
use crate::server::types::*;
use crate::service::clip::{ClipService, ClipsResult};
use crate::service::description_generator::DescriptionType;
use crate::service::funscript::{self, FunScript, ScriptBuilder};
use crate::service::options_converter::OptionsConverterService;
use crate::service::streams::{LocalVideoSource, StreamUrlService};
use crate::util::generate_id;

#[utoipa::path(
    post,
    path = "/api/project/clips",
    request_body = CreateClipsBody,
    responses(
        (status = 200, description = "The newly created marker", body = ClipsResponse),
    )
)]
#[axum::debug_handler]
pub async fn fetch_clips(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateClipsBody>,
) -> Result<Json<ClipsResponse>, AppError> {
    let service = OptionsConverterService::new(state.database.clone());
    let options = service.convert_clip_options(body).await?;
    debug!("clip options: {options:?}");

    let clip_service = ClipService::new();
    let ClipsResult {
        beat_offsets,
        clips,
    } = clip_service.arrange_clips(options);

    let mut video_ids: Vec<_> = clips.iter().map(|c| c.video_id.as_str()).collect();
    video_ids.sort();
    video_ids.dedup();

    let videos: Vec<_> = state
        .database
        .videos
        .get_videos_by_ids(&video_ids)
        .await?
        .into_iter()
        .map(From::from)
        .collect();

    let stream_service = StreamUrlService::new(state.database.clone()).await;
    let streams = stream_service.get_clip_streams(&clips, &videos, LocalVideoSource::Url);

    let response = ClipsResponse {
        clips,
        streams,
        videos,
        beat_offsets,
    };
    Ok(Json(response))
}

async fn create_video_inner(
    state: State<Arc<AppState>>,
    body: CreateVideoBody,
) -> Result<(), AppError> {
    let service = OptionsConverterService::new(state.database.clone());
    let options = service.convert_compilation_options(body).await?;

    let clips = state.generator.gather_clips(&options).await?;
    state.generator.compile_clips(&options, clips).await?;
    Ok(())
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjectCreateResponse {
    pub final_file_name: String,
}

#[utoipa::path(
    post,
    path = "/api/project/create",
    request_body = CreateVideoBody,
    responses(
        (status = 200, description = "The file name of the video to be created (returns immediately)", body = ProjectCreateResponse),
    )
)]
#[axum::debug_handler]
pub async fn create_video(
    state: State<Arc<AppState>>,
    Json(mut body): Json<CreateVideoBody>,
) -> Json<ProjectCreateResponse> {
    use sanitise_file_name::sanitise;

    body.file_name = sanitise(&body.file_name);
    let file_name = body.file_name.clone();
    debug!("received json body: {:?}", body);

    tokio::spawn(async move {
        if let Err(e) = create_video_inner(state, body).await {
            error!("error: {e:?}");
        }
    });

    Json(ProjectCreateResponse {
        final_file_name: file_name,
    })
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/project/finished",
    responses(
        (status = 200, description = "List all finished videos", body = Vec<String>),
    )
)]
pub async fn list_finished_videos(
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, AppError> {
    use tokio::fs;

    let root = state.directories.compilation_video_dir();
    let mut read_dir = fs::read_dir(root).await?;
    let mut file_names = Vec::new();
    while let Some(entry) = read_dir.next_entry().await? {
        let path = Utf8PathBuf::from_path_buf(entry.path()).expect("must be utf-8 path");
        if let Some(name) = path.file_name() {
            file_names.push(name.to_string());
        }
    }

    Ok(Json(file_names))
}

#[utoipa::path(
    get,
    path = "/api/project/id",
    responses(
        (status = 200, description = "Generate a new ID", body = NewId),
    )
)]
#[axum::debug_handler]
pub async fn get_new_id() -> Json<NewId> {
    let id = generate_id();
    Json(NewId { id })
}

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct FilenameQuery {
    video_id: String,
}

#[utoipa::path(
    get,
    path = "/api/project/download",
    params(FilenameQuery),
    responses(
        (status = 200, description = "Download the finished video", body = Vec<u8>),
    )
)]
#[axum::debug_handler]
pub async fn download_video(
    state: State<Arc<AppState>>,
    Query(FilenameQuery { video_id }): Query<FilenameQuery>,
) -> Result<impl IntoResponse, AppError> {
    use axum::http::header;
    use axum::response::AppendHeaders;
    use tokio::fs;

    info!("downloading video '{video_id}'");
    let mut iter = fs::read_dir(state.directories.compilation_video_dir()).await?;
    let mut path = None;
    while let Some(entry) = iter.next_entry().await? {
        let file_path = Utf8PathBuf::from_path_buf(entry.path()).expect("must be utf-8 path");
        if let Some(name) = file_path.file_name() {
            if name.contains(&video_id) {
                path = Some(file_path);
                break;
            }
        }
    }
    if path.is_none() {
        return Err(eyre!("no video found for video ID '{video_id}'").into());
    }
    let path = path.unwrap();
    let file = fs::File::open(&path).await?;

    let stream = ReaderStream::new(file);
    let file_name = path.file_name().unwrap();
    let content_disposition = format!("attachment; filename=\"{}\"", file_name);

    let headers = AppendHeaders([
        (header::CONTENT_TYPE, "video/mp4".to_string()),
        (header::CONTENT_DISPOSITION, content_disposition),
    ]);

    let body = StreamBody::new(stream);
    Ok((headers, body))
}

#[derive(Deserialize, ToSchema)]
pub struct CreateFunscriptBody {
    pub clips: Vec<Clip>,
}

#[utoipa::path(
    post,
    path = "/api/project/funscript/combined",
    request_body = CreateFunscriptBody,
    responses(
        (status = 200, description = "Create a funscript by combining the funscripts from the videos", body = serde_json::Value),
    )
)]
#[axum::debug_handler]
pub async fn get_combined_funscript(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateFunscriptBody>,
) -> Result<Json<FunScript>, AppError> {
    let script_builder = ScriptBuilder::new().await;
    let service = OptionsConverterService::new(state.database.clone());
    let clips = service.convert_clips(body.clips).await?;
    let script = script_builder.create_combined_funscript(clips).await?;

    Ok(Json(script))
}

#[utoipa::path(
    post,
    path = "/api/project/funscript/beat",
    request_body = CreateBeatFunscriptBody,
    responses(
        (status = 200, description = "Create a funscript from the beats of the music", body = serde_json::Value),
    )
)]
#[axum::debug_handler]
pub async fn get_beat_funscript(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateBeatFunscriptBody>,
) -> Result<Json<FunScript>, AppError> {
    let songs = state.database.music.get_songs(&body.song_ids).await?;
    let beats: Vec<Beats> = songs
        .into_iter()
        .filter_map(|s| s.beats.and_then(|b| serde_json::from_str(&b).ok()))
        .collect();
    let script = funscript::create_beat_funscript(beats, body.stroke_type);

    Ok(Json(script))
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionData {
    pub body: String,
    pub content_type: String,
}

#[utoipa::path(
    post,
    path = "/api/project/description/{type}",
    params(
        ("type" = DescriptionType, Path, description = "The type of the description to generate")
    ),
    request_body = CreateVideoBody,
    responses(
        (status = 200, description = "Generate a description for the video", body = DescriptionData),
    )
)]
#[axum::debug_handler]
pub async fn generate_description(
    Path(description_type): Path<DescriptionType>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateVideoBody>,
) -> Result<impl IntoResponse, AppError> {
    use crate::service::description_generator::render_description;

    let service = OptionsConverterService::new(state.database.clone());
    let options = service.convert_compilation_options(body).await?;
    let description = render_description(&options, description_type)?;

    Ok(Json(DescriptionData {
        body: description,
        content_type: description_type.content_type().to_string(),
    }))
}
