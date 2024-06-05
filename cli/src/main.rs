use clap::{Parser, Subcommand};
use color_eyre::Result;
use commands::video::ListVideoOptions;

mod commands;

#[derive(Debug, Subcommand)]
enum TopLevelCommand {
    /// Manage videos
    #[clap(subcommand)]
    Video(VideoCommand),

    /// Manage markers
    #[clap(subcommand)]
    Marker(MarkerCommand),
}

#[derive(Debug, Subcommand)]
enum VideoCommand {
    /// List videos
    List {
        #[clap(short, long)]
        page: Option<i32>,

        #[clap(short, long)]
        size: Option<i32>,

        #[clap(short, long)]
        query: Option<String>,
    },
    /// Upload a new video
    Upload {},
}

#[derive(Debug, Subcommand)]
enum MarkerCommand {
    /// List markers
    List {},
    /// Add a new marker
    Add {},
}

#[derive(Parser, Debug)]
struct Args {
    /// The URL of the ClipMash API
    #[clap(short, long)]
    url: Option<String>,

    #[clap(subcommand)]
    command: TopLevelCommand,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        TopLevelCommand::Video(VideoCommand::List { page, size, query }) => {
            commands::video::list(ListVideoOptions {
                url: args.url,
                page,
                size,
                query,
            })
            .await
        }
        TopLevelCommand::Video(VideoCommand::Upload {}) => commands::video::upload().await,
        TopLevelCommand::Marker(MarkerCommand::List {}) => commands::marker::list().await,
        TopLevelCommand::Marker(MarkerCommand::Add {}) => commands::marker::add().await,
    }
}
