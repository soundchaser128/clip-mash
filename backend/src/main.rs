use std::env;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::DefaultBodyLimit;
use axum::routing::{delete, get, post, put};
use axum::Router;
use color_eyre::Report;
use tracing::{info, warn};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::data::database::Database;
use crate::server::docs::ApiDoc;
use crate::server::handlers::AppState;
use crate::service::directories::Directories;
use crate::service::generator::CompilationGenerator;

mod data;
mod helpers;
mod server;
mod service;

pub use helpers::util;

pub type Result<T> = std::result::Result<T, Report>;

// 100 MB
const CONTENT_LENGTH_LIMIT: usize = 100 * 1000 * 1000;

fn setup_logger() {
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    let file_appender = tracing_appender::rolling::daily("./logs", "clip-mash.log");

    tracing_subscriber::fmt()
        .with_writer(file_appender.and(std::io::stdout))
        .with_ansi(true)
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    use server::{handlers, static_files};
    use service::commands::ffmpeg;
    use service::migrations;

    color_eyre::install()?;
    setup_logger();
    let version = env!("CARGO_PKG_VERSION");
    info!("starting clip-mash v{}", version);

    let directories = Directories::new()?;

    let ffmpeg_location = ffmpeg::download_ffmpeg(&directories).await?;
    info!("using ffmpeg at {ffmpeg_location:?}");

    service::stash_config::init(&directories).await;

    let database_file = directories.database_file();
    let database = Database::new(database_file.as_str()).await?;
    let generator =
        CompilationGenerator::new(directories.clone(), &ffmpeg_location, database.clone()).await?;
    migrations::run_async(
        database.clone(),
        directories.clone(),
        ffmpeg_location.clone(),
    );

    let state = Arc::new(AppState {
        generator,
        database,
        directories,
        ffmpeg_location,
    });

    let library_routes = Router::new()
        // list all videos (paginated, with search)
        .route("/video", get(handlers::library::list_videos))
        // add new videos either via stash, local or url
        .route("/video", post(handlers::library::add_new_videos))
        // list videos on stash
        .route("/video/stash", get(handlers::library::list_stash_videos))
        // get details on a single video
        .route("/video/:id", get(handlers::library::get_video))
        // detect markers in a video
        .route(
            "/video/:id/detect-markers",
            post(handlers::library::detect_markers),
        )
        // stream the video file
        .route("/video/:id/file", get(handlers::library::get_video_file))
        // get the generated preview image
        .route(
            "/video/:id/preview",
            get(handlers::library::get_video_preview),
        )
        // list all markers (paginated, with search)
        .route("/marker", get(handlers::library::list_markers))
        // create new marker for local video (stash tbd)
        .route("/marker", post(handlers::library::create_new_marker))
        // update local marker
        .route("/marker", put(handlers::library::update_marker))
        // delete local marker
        .route("/marker/:id", delete(handlers::library::delete_marker))
        // get the generated preview image for a marker
        .route(
            "/marker/:id/preview",
            get(handlers::library::get_marker_preview),
        )
        // split local marker
        .route("/marker/:id/split", post(handlers::library::split_marker));

    let project_routes = Router::new()
        .route("/clips", post(handlers::project::fetch_clips))
        .route("/id", get(handlers::project::get_new_id))
        .route("/create", post(handlers::project::create_video))
        .route(
            "/funscript/beat",
            post(handlers::project::get_beat_funscript),
        )
        .route(
            "/funscript/combined",
            post(handlers::project::get_combined_funscript),
        )
        .route(
            "/finished-videos",
            get(handlers::project::list_finished_videos),
        )
        .route("/download", get(handlers::project::download_video));

    let stash_routes = Router::new()
        .route("/config", get(handlers::stash::get_config))
        .route("/config", post(handlers::stash::set_config))
        .route("/health", get(handlers::stash::get_health));

    let api_routes = Router::new()
        .nest("/project", project_routes)
        .nest("/library", library_routes)
        .nest("/stash", stash_routes)
        .route("/version", get(handlers::version::get_version))
        .route(
            "/progress/:id/stream",
            get(handlers::progress::get_progress_stream),
        )
        .route(
            "/progress/:id/info",
            get(handlers::progress::get_progress_info),
        )
        .route("/song", get(handlers::music::list_songs))
        .route("/song/:id/stream", get(handlers::music::stream_song))
        .route("/song/download", post(handlers::music::download_music))
        .route("/song/upload", post(handlers::music::upload_music))
        .route("/song/:id/beats", get(handlers::music::get_beats));

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
        // via SwaggerUi instead we only make rapidoc to point to the existing doc.
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .nest("/api", api_routes)
        .fallback_service(static_files::service())
        .layer(DefaultBodyLimit::max(CONTENT_LENGTH_LIMIT))
        .with_state(state);

    let host = env::args().nth(1).unwrap_or_else(|| "[::1]".to_string());
    let addr = format!("{host}:5174");
    info!("running at {}", addr);

    let is_debug_build = cfg!(debug_assertions);
    if !is_debug_build {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(500)).await;
            if webbrowser::open("http://localhost:5174").is_err() {
                warn!("failed to open UI in browser, please navigate to http://localhost:5174");
            }
        });
    }

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
