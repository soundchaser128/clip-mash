use std::sync::Arc;

use axum::{routing::get, Json, Router};
use config::Config;

use crate::{cli::Cli, config::setup_config, ffmpeg::Ffmpeg, stash_api::Api};

mod cli;
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
        .with_state(state);

    let addr = "0.0.0.0:5174";
    tracing::info!("running at {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
