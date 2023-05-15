use std::{sync::Arc, time::Duration};

use axum::{
    routing::{delete, get, post},
    Router,
};
use color_eyre::Report;
use tracing::{info, warn};

use crate::{
    data::database::Database,
    server::handlers::AppState,
    service::{directories::Directories, generator::CompilationGenerator},
};

mod data;
mod server;
mod service;
mod util;

pub type Result<T> = std::result::Result<T, Report>;

#[tokio::main]
async fn main() -> Result<()> {
    use server::{handlers, static_files};
    use std::env;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    color_eyre::install()?;

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    let directories = Directories::new()?;
    directories.info();

    service::stash_config::init(&directories).await;

    let ffmpeg = CompilationGenerator::new(directories.clone()).await?;
    let database_file = directories.database_file();
    let database = Database::new(database_file.as_str()).await?;
    let state = Arc::new(AppState {
        generator: ffmpeg,
        database,
        directories,
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
        .route("/video", post(handlers::local::list_videos))
        .route("/video/:id", get(handlers::local::get_video))
        .route("/video/marker", post(handlers::local::persist_marker))
        .route("/video/marker/:id", delete(handlers::local::delete_marker));

    let api_routes = Router::new()
        .nest("/local", local_routes)
        .nest("/stash", stash_routes)
        .route("/clips", post(handlers::common::fetch_clips))
        .route("/create", post(handlers::common::create_video))
        .route("/progress", get(handlers::common::get_progress))
        .route("/download", get(handlers::common::download_video))
        .route("/funscript", post(handlers::common::get_funscript))
        .route("/music", get(handlers::common::list_songs))
        .route("/music", post(handlers::common::download_music));

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(static_files::service())
        .with_state(state);

    let host = env::args().nth(1).unwrap_or_else(|| "[::1]".to_string());
    let addr = format!("{host}:5174");
    info!("running at {}", addr);

    let is_debug_build = cfg!(debug_assertions);
    if !is_debug_build {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(500)).await;
            if webbrowser::open("http://localhost:5174").is_err() {
                warn!("failed to open UI in browser, please navigate to http://localhost:5147");
            }
        });
    }

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
