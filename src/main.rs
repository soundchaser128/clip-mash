use std::sync::Arc;

use axum::{routing::get, Router};
use config::Config;

use crate::{cli::Cli, config::setup_config, ffmpeg::Ffmpeg, stash_api::Api};

mod cli;
mod config;
mod ffmpeg;
mod stash_api;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct State {
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
    let api = Api::new(&config.stash_url, &config.stash_url);
    let ffmpeg = Ffmpeg::new();
    let state = Arc::new(State {
        api,
        ffmpeg,
        config,
    });

    let app = Router::new()
        .with_state(state)
        .route("/api/hello", get(|| async { "Hello world!" }));

    let addr = "[::1]:5174";
    tracing::info!("running at {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
