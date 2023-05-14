use axum::{
    body::StreamBody,
    extract::{Query, State},
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Sse,
    },
    Json,
};
use futures::{
    stream::{self, Stream},
    FutureExt,
};
use serde::Deserialize;
use std::{collections::HashSet, sync::Arc, time::Duration};
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;
use tracing::{debug, error, info};

use crate::{
    data::stash_api::StashApi,
    server::{
        dtos::{ClipsResponse, CreateClipsBody, CreateVideoBody},
        error::AppError,
    },
    service::{
        clip::{self, ClipService},
        funscript::{FunScript, ScriptBuilder},
        generator,
        music::MusicService,
        stash_config::Config,
        Clip, VideoSource,
    },
};

use super::AppState;

#[axum::debug_handler]
pub async fn fetch_clips(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateClipsBody>,
) -> Result<Json<ClipsResponse>, AppError> {
    let config = Config::get().await?;
    let api = StashApi::from_config(&config);
    let service = ClipService::new(&state.database, &api);
    let video_ids: HashSet<_> = body.markers.iter().map(|m| m.video_id.clone()).collect();
    info!("found {} video IDs", video_ids.len());
    let options = service.convert_clip_options(body).await?;

    let clips = clip::arrange_clips(&options);
    info!("generated {} clips", clips.len());
    debug!("compiled clips {clips:#?}");
    let streams = clip::get_streams(video_ids, &config)?;
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
    };
    Ok(Json(response))
}

async fn create_video_inner(
    state: State<Arc<AppState>>,
    body: CreateVideoBody,
) -> Result<(), AppError> {
    let api = StashApi::load_config().await?;
    let service = ClipService::new(&state.database, &api);
    let options = service.convert_compilation_options(body).await?;

    let clips = state.generator.gather_clips(&options).await?;
    state.generator.compile_clips(&options, clips).await?;
    Ok(())
}

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
pub async fn get_progress() -> Sse<impl Stream<Item = Result<Event, serde_json::Error>>> {
    let stream = futures::StreamExt::flat_map(stream::repeat_with(generator::get_progress), |f| {
        f.into_stream()
    });
    let stream = stream
        .map(|p| Event::default().json_data(p))
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilenameQuery {
    file_name: String,
}

#[axum::debug_handler]
pub async fn download_video(
    state: State<Arc<AppState>>,
    Query(FilenameQuery { file_name }): Query<FilenameQuery>,
) -> Result<impl IntoResponse, AppError> {
    use axum::{http::header, response::AppendHeaders};

    info!("downloading video '{file_name}'");
    let path = state.generator.video_dir.join(&file_name);
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
pub async fn get_funscript(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateFunscriptBody>,
) -> Result<Json<FunScript>, AppError> {
    let api = StashApi::load_config().await?;
    let script_builder = ScriptBuilder::new(&api);
    let service = ClipService::new(&state.database, &api);
    let clips = service.convert_clips(body.clips).await?;
    let script = script_builder.combine_scripts(clips).await?;

    Ok(Json(script))
}

#[derive(Deserialize)]
pub struct DownloadMusicQuery {
    pub url: String,
}

#[axum::debug_handler]
pub async fn download_music(
    Query(DownloadMusicQuery { url }): Query<DownloadMusicQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<(), AppError> {
    info!("downloading music at url {url}");
    let music_service = MusicService::new(state.database.clone(), state.directories.clone());
    music_service.download_song(&url).await?;

    Ok(())
}
