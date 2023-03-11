use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use config::Config;

use crate::{config::setup_config, ffmpeg::Ffmpeg, stash_api::Api};

mod config;
mod error;
mod ffmpeg;
mod http;
mod stash_api;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct AppState {
    pub config: Config,
    pub api: Api,
    pub ffmpeg: Ffmpeg,
}

#[tokio::main]
async fn main() -> Result<()> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let config = setup_config()?;
    let api = Api::new(&config.stash_url, &config.api_key);
    let ffmpeg = Ffmpeg::new();
    let state = Arc::new(AppState {
        api,
        ffmpeg,
        config,
    });

    let app = Router::new()
        .route("/api/tags", get(http::fetch_tags))
        .route("/api/performers", get(http::fetch_performers))
        .route("/api/markers", get(http::fetch_markers))
        .route("/api/create", post(http::create_video))
        .route("/api/progress", get(http::get_progress))
        .with_state(state);

    let addr = "[::1]:5174";
    tracing::info!("running at {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
