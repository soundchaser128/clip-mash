use axum::{
    routing::{delete, get, post},
    Router,
};
use color_eyre::Report;
use compilation::generate::CompilationGenerator;
use local::db::Database;
use std::{sync::Arc, time::Duration};

mod compilation;
mod download_ffmpeg;
mod error;
mod local;
mod server;
mod stash;
mod util;

pub type Result<T> = std::result::Result<T, Report>;

pub struct AppState {
    pub generator: CompilationGenerator,
    pub database: Database,
}

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

    stash::config::init().await;

    let ffmpeg = CompilationGenerator::new().await?;
    let database = Database::new().await?;
    let state = Arc::new(AppState {
        generator: ffmpeg,
        database,
    });

    let app = Router::new()
        .route("/api/health", get(handlers::get_health))
        .route("/api/tags", get(handlers::fetch_tags))
        .route("/api/performers", get(handlers::fetch_performers))
        .route("/api/scenes", get(handlers::fetch_scenes))
        .route("/api/markers", get(handlers::fetch_markers))
        .route("/api/clips", post(handlers::fetch_clips))
        .route("/api/create", post(handlers::create_video))
        .route("/api/progress", get(handlers::get_progress))
        .route("/api/download", get(handlers::download_video))
        .route("/api/funscript", post(handlers::get_funscript))
        .route("/api/config", get(handlers::get_config))
        .route("/api/config", post(handlers::set_config))
        .route("/api/video", post(handlers::list_videos))
        .route("/api/video/:id", get(handlers::get_video))
        .route("/api/video/marker", post(handlers::persist_marker))
        .route("/api/video/marker/:id", delete(handlers::delete_marker))
        .fallback_service(static_files::service())
        .with_state(state);

    let host = env::args().nth(1).unwrap_or_else(|| "[::1]".to_string());
    let addr = format!("{host}:5174");
    tracing::info!("running at {}", addr);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(500)).await;

        if webbrowser::open("http://localhost:5174").is_err() {
            tracing::warn!(
                "failed to open UI in browser, please navigate to http://localhost:5147"
            );
        }
    });

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
