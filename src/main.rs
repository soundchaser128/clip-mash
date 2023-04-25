use crate::ffmpeg::Ffmpeg;
use axum::{
    routing::{get, post},
    Router,
};
use color_eyre::Report;
use std::{sync::Arc, time::Duration};

mod clip;
mod config;
mod download_ffmpeg;
mod error;
mod ffmpeg;
mod funscript;
mod http;
mod stash_api;
mod static_files;
mod util;

pub type Result<T> = std::result::Result<T, Report>;

pub struct AppState {
    pub ffmpeg: Ffmpeg,
}

#[tokio::main]
async fn main() -> Result<()> {
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

    config::init().await;

    let ffmpeg = Ffmpeg::new().await?;
    let state = Arc::new(AppState { ffmpeg });

    let app = Router::new()
        .route("/api/health", get(http::get_health))
        .route("/api/tags", get(http::fetch_tags))
        .route("/api/performers", get(http::fetch_performers))
        .route("/api/scenes", get(http::fetch_scenes))
        .route("/api/markers", get(http::fetch_markers))
        .route("/api/clips", post(http::fetch_clips))
        .route("/api/create", post(http::create_video))
        .route("/api/progress", get(http::get_progress))
        .route("/api/download", get(http::download_video))
        .route("/api/funscript", post(http::get_funscript))
        .route("/api/config", get(http::get_config))
        .route("/api/config", post(http::set_config))
        .route("/api/upload", post(http::upload_videos))
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
