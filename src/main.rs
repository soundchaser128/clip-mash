use crate::{api::Api, cli::Cli, config::setup_config, ffmpeg::Ffmpeg};

mod api;
mod cli;
mod config;
mod ffmpeg;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let config = setup_config()?;
    let client = Api::new(&config.stash_url, &config.api_key);
    let cli = Cli::new(&client);
    cli.print_info();
    let options = cli.ask_questions().await?;

    let ffmpeg = Ffmpeg::new();
    let clips = ffmpeg.gather_clips(&options).await?;
    let result_file = ffmpeg.compile_clips(clips, &options).await?;
    println!("wrote result to {}", result_file);

    Ok(())
}
