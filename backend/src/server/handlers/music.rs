use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Multipart, Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use color_eyre::eyre::eyre;
use color_eyre::Report;
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;
use utoipa::{IntoParams, ToSchema};

use super::AppState;
use crate::data::database::music::DbSong;
use crate::server::error::AppError;
use crate::server::types::*;
use crate::service::music::{self, MusicDownloadService};
use crate::util::expect_file_name;

#[derive(Deserialize, IntoParams)]
pub struct DownloadMusicQuery {
    pub url: String,
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
    let music_service = MusicDownloadService::from(state);
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
    let music_service = MusicDownloadService::from(state);

    while let Some(field) = multipart.next_field().await.map_err(Report::from)? {
        if field.name() == Some("file") {
            let song = music_service.upload_song(field).await?;
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
        songs.shuffle(&mut rand::thread_rng());
    }

    Ok(Json(songs))
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/song/{id}/beats",
    params(
        ("id" = String, Path, description = "The ID of the song to get beats for")
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
