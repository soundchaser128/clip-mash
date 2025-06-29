use std::sync::Arc;

use axum::Json;
use axum::body::Body;
use axum::extract::multipart::Field;
use axum::extract::{Multipart, Path, Query, State};
use axum::response::IntoResponse;
use clip_mash::data::database::Database;
use clip_mash::service::commands::ffmpeg::FfmpegLocation;
use clip_mash::service::commands::ffprobe;
use color_eyre::Report;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tracing::info;
use url::Url;
use utoipa::{IntoParams, ToSchema};

use super::AppState;
use crate::server::error::AppError;
use clip_mash::data::database::music::{CreateSong, DbSong};
use clip_mash::service::music::{self, MusicDownloadService};
use clip_mash::types::*;
use clip_mash::util::expect_file_name;

#[derive(Deserialize, IntoParams)]
pub struct DownloadMusicQuery {
    pub url: String,
}

#[derive(Serialize, ToSchema)]
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
#[utoipa::path(
    post,
    path = "/api/song/download",
    params(DownloadMusicQuery),
    responses(
        (status = 200, description = "Download a song from a URL", body = SongDto)
    )
)]
pub async fn download_music(
    Query(DownloadMusicQuery { url }): Query<DownloadMusicQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SongDto>, AppError> {
    info!("downloading music at url {url}");
    let music_service = MusicDownloadService::new(
        state.database.clone(),
        state.directories.clone(),
        state.ffmpeg_location.clone(),
    );
    let url = Url::parse(&url)?;
    let song = music_service.download_song(url).await?;

    Ok(Json(song.into()))
}

#[axum::debug_handler]
pub async fn stream_song(
    Path(song_id): Path<i64>,
    State(state): State<Arc<AppState>>,
    request: axum::http::Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    use tower::util::ServiceExt;
    use tower_http::services::ServeFile;

    let song = state.database.music.get_song(song_id).await?;
    let result = ServeFile::new(song.file_path).oneshot(request).await;
    Ok(result)
}

#[derive(ToSchema)]
pub struct SongUpload {
    #[schema(value_type = String, format = Binary)]
    #[allow(unused)]
    file: String,
}

pub async fn upload_song(
    service: &MusicDownloadService,
    ffmpeg_location: &FfmpegLocation,
    database: &Database,
    mut field: Field<'_>,
) -> crate::Result<DbSong> {
    use tokio::fs;

    let file_name = field.file_name().expect("field must have a file name");
    let output_dir = service.get_download_directory().await?;
    let path = output_dir.join(file_name);
    info!("uploading song to {path}");
    let mut writer = fs::File::create(&path).await?;

    while let Some(chunk) = field.chunk().await? {
        writer.write_all(&chunk).await?;
    }

    let ffprobe_result = ffprobe(path.as_str(), &ffmpeg_location).await?;
    let beats = music::detect_beats(&path, &ffmpeg_location).ok();

    let result = database
        .music
        .persist_song(CreateSong {
            duration: ffprobe_result.format.duration().unwrap_or_default(),
            file_path: path.to_string(),
            url: format!("file:{path}"),
            beats,
        })
        .await?;
    Ok(result)
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/song/upload",
    request_body(content = SongUpload, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Uploads a song", body = SongDto),
    )
)]
/// Upload a song file
pub async fn upload_music(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<SongDto>, AppError> {
    let music_service = MusicDownloadService::new(
        state.database.clone(),
        state.directories.clone(),
        state.ffmpeg_location.clone(),
    );

    while let Some(field) = multipart.next_field().await.map_err(Report::from)? {
        if field.name() == Some("file") {
            let song = upload_song(
                &music_service,
                &state.ffmpeg_location,
                &state.database,
                field,
            )
            .await?;
            return Ok(Json(song.into()));
        }
    }

    Err(eyre!("missing form field `file`").into())
}

#[derive(Deserialize, IntoParams)]
pub struct ListSongsQuery {
    shuffle: Option<bool>,
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/song",
    params(ListSongsQuery),
    responses(
        (status = 200, description = "Lists all songs", body = Vec<SongDto>),
    )
)]
/// List all songs
pub async fn list_songs(
    Query(ListSongsQuery { shuffle }): Query<ListSongsQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SongDto>>, AppError> {
    use rand::seq::SliceRandom;

    let mut songs: Vec<SongDto> = state
        .database
        .music
        .list_songs()
        .await?
        .into_iter()
        .map(From::from)
        .collect();

    if let Some(true) = shuffle {
        songs.shuffle(&mut rand::rng());
    }

    Ok(Json(songs))
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/song/{id}/beats",
    params(
        ("id" = i64, Path, description = "The ID of the song to get beats for")
    ),
    responses(
        (status = 200, description = "Get beats for a song", body = Beats),
    )
)]
/// Get beats for a song, or detect them if they are not yet available.
pub async fn get_beats(
    Path(song_id): Path<i64>,
    state: State<Arc<AppState>>,
) -> Result<Json<Beats>, AppError> {
    let beats = match state.database.music.fetch_beats(song_id).await? {
        Some(beats) => beats,
        None => {
            let song = state.database.music.get_song(song_id).await?;
            let beats = music::detect_beats(&song.file_path, &state.ffmpeg_location)?;
            state
                .database
                .music
                .persist_beats(song.rowid.unwrap(), &beats)
                .await?;
            beats
        }
    };
    Ok(Json(beats))
}
