use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use axum::body::{Body, StreamBody};
use axum::extract::{Multipart, Path, Query, State};
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Sse};
use axum::Json;
use camino::Utf8PathBuf;
use color_eyre::eyre::eyre;
use color_eyre::Report;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;
use tracing::{debug, error, info};
use url::Url;
use utoipa::IntoParams;

use super::AppState;
use crate::data::database::DbSong;
use crate::data::service::DataService;
use crate::data::stash_api::StashApi;
use crate::server::error::AppError;
use crate::server::handlers::get_streams;
use crate::server::types::*;
use crate::service::clip::{ClipService, ClipsResult};
use crate::service::directories::FolderType;
use crate::service::funscript::{self, FunScript, ScriptBuilder};
use crate::service::music::{self, MusicDownloadService};
use crate::service::stash_config::Config;
use crate::util::{expect_file_name, generate_id};

#[utoipa::path(
    post,
    path = "/api/clips",
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
    let config = Config::get_or_empty().await;
    let service = DataService::new(state.database.clone()).await;
    let video_ids: HashSet<_> = body.markers.iter().map(|m| m.video_id.clone()).collect();
    info!("found {} video IDs", video_ids.len());
    let options = service.convert_clip_options(body).await?;
    debug!("clip options: {options:?}");

    let clip_service = ClipService::new();
    let ClipsResult {
        beat_offsets,
        clips,
    } = clip_service.arrange_clips(options);
    let streams = get_streams(video_ids, &config)?;
    let mut video_ids: Vec<_> = clips.iter().map(|c| c.video_id.clone()).collect();
    video_ids.sort();
    video_ids.dedup();

    let videos = service
        .fetch_videos(&video_ids)
        .await?
        .into_iter()
        .map(From::from)
        .collect();

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
    let service = DataService::new(state.database.clone()).await;
    let options = service.convert_compilation_options(body).await?;

    let clips = state.generator.gather_clips(&options).await?;
    state.generator.compile_clips(&options, clips).await?;
    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/create",
    request_body = CreateVideoBody,
    responses(
        (status = 200, description = "The file name of the video to be created (returns immediately)", body = String),
    )
)]
#[axum::debug_handler]
pub async fn create_video(
    state: State<Arc<AppState>>,
    Json(mut body): Json<CreateVideoBody>,
) -> String {
    use sanitise_file_name::sanitise;

    body.file_name = sanitise(&body.file_name);
    let file_name = body.file_name.clone();
    debug!("received json body: {:?}", body);

    tokio::spawn(async move {
        if let Err(e) = create_video_inner(state, body).await {
            error!("error: {e:?}");
        }
    });

    file_name
}

#[axum::debug_handler]
pub async fn get_progress_stream(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, AppError>>> {
    use async_stream::try_stream;

    let stream = try_stream! {
        let state = state.clone();
        while let Some(progress) = state.database.get_progress(id.clone()).await? {
            yield Event::default().json_data(progress).unwrap();
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[utoipa::path(
    post,
    path = "/api/progress/info",
    responses(
        (status = 200, description = "The current progress of video creation, or null if it is finished", body = Progress),
    )
)]
#[axum::debug_handler]
pub async fn get_progress_info(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let progress = state.database.get_progress(&id).await?;
    Ok(Json(progress))
}

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct FilenameQuery {
    file_name: String,
}

#[utoipa::path(
    get,
    path = "/api/download",
    params(FilenameQuery),
    responses(
        (status = 200, description = "Download the finished video", body = Vec<u8>),
    )
)]
#[axum::debug_handler]
pub async fn download_video(
    state: State<Arc<AppState>>,
    Query(FilenameQuery { file_name }): Query<FilenameQuery>,
) -> Result<impl IntoResponse, AppError> {
    use axum::http::header;
    use axum::response::AppendHeaders;

    info!("downloading video '{file_name}'");
    let path = state.directories.compilation_video_dir().join(&file_name);
    let file = tokio::fs::File::open(path).await?;
    let stream = ReaderStream::new(file);
    let content_disposition = format!("attachment; filename=\"{}\"", file_name);

    let headers = AppendHeaders([
        (header::CONTENT_TYPE, "video/mp4".to_string()),
        (header::CONTENT_DISPOSITION, content_disposition),
    ]);

    let body = StreamBody::new(stream);
    Ok((headers, body))
}

#[derive(Deserialize)]
pub struct CreateFunscriptBody {
    pub clips: Vec<Clip>,
    pub source: VideoSource,
}

#[axum::debug_handler]
pub async fn get_combined_funscript(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateFunscriptBody>,
) -> Result<Json<FunScript>, AppError> {
    let api = StashApi::load_config().await?;
    let script_builder = ScriptBuilder::new(&api);
    let service = DataService::new(state.database.clone()).await;
    let clips = service.convert_clips(body.clips).await?;
    let script = script_builder.combine_scripts(clips).await?;

    Ok(Json(script))
}

#[axum::debug_handler]
pub async fn get_beat_funscript(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateBeatFunscriptBody>,
) -> Result<Json<FunScript>, AppError> {
    let songs = state.database.get_songs(&body.song_ids).await?;
    let beats: Vec<Beats> = songs
        .into_iter()
        .filter_map(|s| s.beats.and_then(|b| serde_json::from_str(&b).ok()))
        .collect();
    let script = funscript::create_beat_script(beats, body.stroke_type);

    Ok(Json(script))
}

#[derive(Deserialize)]
pub struct DownloadMusicQuery {
    pub url: Url,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SongDto {
    pub song_id: i64,
    pub duration: f64,
    pub file_name: String,
    pub url: String,
    pub beats: Vec<f32>,
}

impl From<DbSong> for SongDto {
    fn from(value: DbSong) -> Self {
        let beats: Option<Beats> = value.beats.and_then(|str| serde_json::from_str(&str).ok());

        SongDto {
            song_id: value.rowid.expect("must have rowid set"),
            duration: value.duration,
            file_name: expect_file_name(&value.file_path),
            url: value.url,
            beats: beats.map(|b| b.offsets).unwrap_or_default(),
        }
    }
}

#[axum::debug_handler]
pub async fn download_music(
    Query(DownloadMusicQuery { url }): Query<DownloadMusicQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SongDto>, AppError> {
    info!("downloading music at url {url}");
    let music_service = MusicDownloadService::from(state);
    let song = music_service.download_song(url).await?;

    Ok(Json(song.into()))
}

#[axum::debug_handler]
pub async fn stream_song(
    Path(song_id): Path<i64>,
    State(state): State<Arc<AppState>>,
    request: axum::http::Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    use tower::ServiceExt;
    use tower_http::services::ServeFile;

    let song = state.database.get_song(song_id).await?;
    let result = ServeFile::new(song.file_path).oneshot(request).await;
    Ok(result)
}

#[axum::debug_handler]
pub async fn upload_music(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<SongDto>, AppError> {
    let music_service = MusicDownloadService::from(state);

    while let Some(field) = multipart.next_field().await.map_err(Report::from)? {
        if field.name() == Some("file") {
            let song = music_service.upload_song(field).await?;
            return Ok(Json(song.into()));
        }
    }

    Err(eyre!("missing form field `file`").into())
}

#[axum::debug_handler]
pub async fn list_songs(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SongDto>>, AppError> {
    let songs = state
        .database
        .list_songs()
        .await?
        .into_iter()
        .map(From::from)
        .collect();

    Ok(Json(songs))
}

#[derive(Debug, Deserialize)]
pub struct FolderTypeQuery {
    pub folder: FolderType,
}

pub async fn open_folder(
    Query(FolderTypeQuery { folder }): Query<FolderTypeQuery>,
    state: State<Arc<AppState>>,
) -> Result<(), AppError> {
    info!("opening folder {folder:?}");
    let path = state.directories.get(folder);

    opener::open(path).map_err(Report::from)?;

    Ok(())
}

#[axum::debug_handler]
pub async fn get_beats(
    Path(song_id): Path<i64>,
    state: State<Arc<AppState>>,
) -> Result<Json<Beats>, AppError> {
    let beats = match state.database.fetch_beats(song_id).await? {
        Some(beats) => beats,
        None => {
            let song = state.database.get_song(song_id).await?;
            let beats = music::detect_beats(&song.file_path, &state.ffmpeg_location)?;
            state
                .database
                .persist_beats(song.rowid.unwrap(), &beats)
                .await?;
            beats
        }
    };
    Ok(Json(beats))
}

#[axum::debug_handler]
pub async fn get_new_id() -> Json<NewId> {
    let id = generate_id();
    Json(NewId { id })
}

#[derive(Serialize)]
pub struct Version {
    pub version: &'static str,
}

#[axum::debug_handler]
pub async fn get_version() -> Json<Version> {
    let version = env!("CARGO_PKG_VERSION");
    Json(Version { version })
}

#[axum::debug_handler]
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
