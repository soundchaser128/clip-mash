use std::{sync::Arc, time::Duration};

use axum::{
    routing::{delete, get, post},
    Router,
};
use color_eyre::Report;

use crate::{
    data::database::Database, server::handlers::AppState, service::generator::CompilationGenerator,
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

    service::stash_config::init().await;

    let ffmpeg = CompilationGenerator::new().await?;
    let database = Database::new().await?;
    let state = Arc::new(AppState {
        generator: ffmpeg,
        database,
    });

    let app = Router::new()
        .route("/api/stash/health", get(handlers::stash::get_health))
        .route("/api/stash/tags", get(handlers::stash::fetch_tags))
        .route(
            "/api/stash/performers",
            get(handlers::stash::fetch_performers),
        )
        .route("/api/stash/scenes", get(handlers::stash::fetch_scenes))
        .route("/api/stash/markers", get(handlers::stash::fetch_markers))
        .route("/api/stash/config", get(handlers::stash::get_config))
        .route("/api/stash/config", post(handlers::stash::set_config))
        .route("/api/local/video", post(handlers::local::list_videos))
        .route("/api/local/video/:id", get(handlers::local::get_video))
        .route(
            "/api/local/video/marker",
            post(handlers::local::persist_marker),
        )
        .route(
            "/api/local/video/marker/:id",
            delete(handlers::local::delete_marker),
        )
        .route("/api/clips", post(handlers::common::fetch_clips))
        .route("/api/create", post(handlers::common::create_video))
        .route("/api/progress", get(handlers::common::get_progress))
        .route("/api/download", get(handlers::common::download_video))
        .route("/api/funscript", post(handlers::common::get_funscript))
        .fallback_service(static_files::service())
        .with_state(state);

    let host = env::args().nth(1).unwrap_or_else(|| "[::1]".to_string());
    let addr = format!("{host}:5174");
    tracing::info!("running at {}", addr);

    let is_debug_build = cfg!(debug_assertions);
    if !is_debug_build {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(500)).await;
            if webbrowser::open("http://localhost:5174").is_err() {
                tracing::warn!(
                    "failed to open UI in browser, please navigate to http://localhost:5147"
                );
            }
        });
    }

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
