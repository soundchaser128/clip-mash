use std::env;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::DefaultBodyLimit;
use axum::routing::{delete, get, post, put};
use axum::Router;
use color_eyre::Report;
use tracing::{info, warn};

use crate::data::database::Database;
use crate::server::handlers::AppState;
use crate::service::directories::Directories;
use crate::service::generator::CompilationGenerator;

mod data;
mod progress;
mod server;
mod service;
mod util;

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

    aide::gen::on_error(|error| {
        tracing::error!("failed to generate OpenAPI spec: {:?}", error);
    });

    aide::gen::extract_schemas(true);

    service::stash_config::init(&directories).await;

    let generator = CompilationGenerator::new(directories.clone(), &ffmpeg_location).await?;
    let database_file = directories.database_file();
    let database = Database::new(database_file.as_str()).await?;
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

    let stash_routes = Router::new()
        .route("/health", get(handlers::stash::get_health))
        .route("/tags", get(handlers::stash::fetch_tags))
        .route("/performers", get(handlers::stash::fetch_performers))
        .route("/scenes", get(handlers::stash::fetch_scenes))
        .route("/markers", get(handlers::stash::fetch_markers))
        .route("/config", get(handlers::stash::get_config))
        .route("/config", post(handlers::stash::set_config));

    let local_routes = Router::new()
        .route("/video", get(handlers::local::list_videos))
        .route("/video", post(handlers::local::add_new_videos))
        .route("/video/:id", get(handlers::local::get_video))
        .route("/video/:id/markers", post(handlers::local::detect_markers))
        .route("/video/:id/file", get(handlers::local::get_video_file))
        .route(
            "/video/:id/preview",
            get(handlers::local::get_video_preview),
        )
        .route("/video/marker", get(handlers::local::list_markers))
        .route("/video/marker", post(handlers::local::create_new_marker))
        .route("/video/marker", put(handlers::local::update_marker))
        .route("/video/marker/:id", delete(handlers::local::delete_marker))
        .route(
            "/video/marker/:id/split",
            post(handlers::local::split_marker),
        )
        .route(
            "/video/marker/:id/preview",
            get(handlers::local::get_marker_preview),
        )
        .route("/video/download", post(handlers::local::download_video));

    let api_routes = Router::new()
        .nest("/local", local_routes)
        .nest("/stash", stash_routes)
        .route("/version", get(handlers::common::get_version))
        .route("/clips", post(handlers::common::fetch_clips))
        .route("/create", post(handlers::common::create_video))
        .route(
            "/progress/stream",
            get(handlers::common::get_progress_stream),
        )
        .route("/progress/info", get(handlers::common::get_progress_info))
        .route("/download", get(handlers::common::download_video))
        .route(
            "/funscript/combined",
            post(handlers::common::get_combined_funscript),
        )
        .route(
            "/funscript/beat",
            post(handlers::common::get_beat_funscript),
        )
        .route("/song", get(handlers::common::list_songs))
        .route("/song/:id/stream", get(handlers::common::stream_song))
        .route("/song/download", post(handlers::common::download_music))
        .route("/song/upload", post(handlers::common::upload_music))
        .route("/song/:id/beats", get(handlers::common::get_beats))
        .route("/directory/open", get(handlers::common::open_folder))
        .route("/id", get(handlers::common::get_new_id));

    let app = Router::new()
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
