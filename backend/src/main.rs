use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::DefaultBodyLimit;
use axum::routing::{delete, get, post, put};
use axum::Router;
use color_eyre::Report;
use mimalloc::MiMalloc;
use tracing::{error, info, warn};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::data::alexandria::AlexandriaApi;
use crate::data::database::Database;
use crate::server::docs::ApiDoc;
use crate::server::handlers::AppState;
use crate::service::directories::Directories;
use crate::service::generator::CompilationGenerator;
use crate::service::new_version_checker::NewVersionChecker;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod data;
mod helpers;
mod server;
mod service;

pub use helpers::util;

pub type Result<T> = std::result::Result<T, Report>;

// 100 MB
const CONTENT_LENGTH_LIMIT: usize = 100 * 1000 * 1000;

fn find_unused_port() -> SocketAddr {
    let host = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1".into());
    let port = std::env::args()
        .nth(2)
        .and_then(|port| port.parse::<u16>().ok());

    // find first unused port
    let port = if cfg!(debug_assertions) {
        5174
    } else {
        match port {
            Some(port) => port,
            None => (1024..65535)
                .find(|port| std::net::TcpListener::bind(format!("{}:{}", host, port)).is_ok())
                .expect("failed to find unused port"),
        }
    };
    format!("{}:{}", host, port).parse().unwrap()
}

async fn run() -> Result<()> {
    use server::{handlers, static_files};
    use service::commands::ffmpeg;
    use service::migrations;

    let directories = Directories::new()?;
    let ffmpeg_location = ffmpeg::download_ffmpeg(&directories).await?;
    info!("using ffmpeg at {ffmpeg_location:?}");

    let database_file = if env::var("CLIP_MASH_SQLITE_IN_MEMORY").is_ok() {
        ":memory:".into()
    } else {
        directories.database_file().into_string()
    };

    info!("using database at {database_file:?}");

    let database = Database::new(&database_file).await?;
    let generator =
        CompilationGenerator::new(directories.clone(), &ffmpeg_location, database.clone()).await?;
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
        new_version_checker: NewVersionChecker::new(),
        alexandria: AlexandriaApi::new(reqwest::Client::new()),
    });

    let library_routes = Router::new()
        // list all videos (paginated, with search)
        .route("/video", get(handlers::library::list_videos))
        // add new videos either via stash, local or url
        .route("/video", post(handlers::library::add_new_videos))
        // returns whether a set of videos need to be re-encoded or not
        .route(
            "/video/need-encoding",
            post(handlers::library::videos_need_encoding),
        )
        // update video metadata
        .route("/video/:id", put(handlers::library::update_video))
        // sync a single video with stash
        .route(
            "/video/:id/stash/merge",
            post(handlers::library::merge_stash_video),
        )
        .route(
            "/cleanup/:folder_type",
            post(handlers::files::cleanup_folder),
        )
        // remove videos that don't exist on disk
        .route("/video/cleanup", post(handlers::library::cleanup_videos))
        // list videos on stash
        .route("/video/stash", get(handlers::library::list_stash_videos))
        // list alexandria videos
        .route(
            "/video/alexandria",
            get(handlers::library::list_alexandria_videos),
        )
        // get details on a single video
        .route("/video/:id", get(handlers::library::get_video))
        // delete a video
        .route("/video/:id", delete(handlers::library::delete_video))
        // detect markers in a video
        .route(
            "/video/:id/detect-markers",
            post(handlers::library::detect_markers),
        )
        // stream the video file
        .route("/video/:id/file", get(handlers::library::get_video_file))
        // get the generated preview image
        .route(
            "/video/:id/preview",
            get(handlers::library::get_video_preview),
        )
        // list all markers by video ID
        .route("/marker", get(handlers::library::list_markers))
        // list marker titles and counts, for autocompletion
        .route("/marker/title", get(handlers::library::list_marker_titles))
        // create new marker for video
        .route("/marker", post(handlers::library::create_new_marker))
        // update local marker
        .route("/marker/:id", put(handlers::library::update_marker))
        // delete local marker
        .route("/marker/:id", delete(handlers::library::delete_marker))
        // get the generated preview image for a marker
        .route(
            "/marker/:id/preview",
            get(handlers::library::get_marker_preview),
        )
        // split local marker
        .route("/marker/:id/split", post(handlers::library::split_marker))
        .route("/directory", get(handlers::files::list_file_entries))
        .route("/stats", get(handlers::files::get_file_stats))
        .route(
            "/migrate/preview",
            post(handlers::library::migrate_preview_images),
        );

    let project_routes = Router::new()
        .route("/clips", post(handlers::project::fetch_clips))
        .route("/id", get(handlers::project::get_new_id))
        .route("/create", post(handlers::project::create_video))
        .route(
            "/funscript/beat",
            post(handlers::project::get_beat_funscript),
        )
        .route(
            "/funscript/combined",
            post(handlers::project::get_combined_funscript),
        )
        .route("/finished", get(handlers::project::list_finished_videos))
        .route("/download", get(handlers::project::download_video))
        .route(
            "/description/:type",
            post(handlers::project::generate_description),
        );

    let stash_routes = Router::new().route("/health", get(handlers::stash::get_health));

    let system_routes = Router::new()
        .route("/restart", post(handlers::system::restart))
        .route("/sentry/error", post(handlers::system::sentry_error))
        .route("/version", get(handlers::system::get_version))
        .route("/health", get(handlers::system::get_health))
        .route("/configuration", get(handlers::system::get_config))
        .route("/configuration", post(handlers::system::set_config));

    let music_routes = Router::new()
        .route("/", get(handlers::music::list_songs))
        .route("/:id/stream", get(handlers::music::stream_song))
        .route("/download", post(handlers::music::download_music))
        .route("/upload", post(handlers::music::upload_music))
        .route("/:id/beats", get(handlers::music::get_beats));

    let progress_routes = Router::new()
        .route("/:id/stream", get(handlers::progress::get_progress_stream))
        .route("/:id/info", get(handlers::progress::get_progress_info))
        .route("/:id", delete(handlers::progress::delete_progress));

    let api_routes = Router::new()
        .nest("/project", project_routes)
        .nest("/library", library_routes)
        .nest("/stash", stash_routes)
        .nest("/system", system_routes)
        .nest("/song", music_routes)
        .nest("/progress", progress_routes);

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_routes)
        .fallback_service(static_files::service())
        .layer(DefaultBodyLimit::max(CONTENT_LENGTH_LIMIT))
        .layer(sentry_tower::NewSentryLayer::new_from_top())
        .layer(sentry_tower::SentryHttpLayer::with_transaction())
        .with_state(state);

    let addr = find_unused_port();
    info!("listening on {addr}");

    let is_debug_build = cfg!(debug_assertions);
    if !is_debug_build {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(500)).await;
            if webbrowser::open(&format!("http://{addr}")).is_err() {
                warn!(
                    "failed to open UI in browser, please navigate to http://localhost:{}",
                    addr.port()
                );
            }
        });
    }

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn main() -> Result<()> {
    use crate::helpers::{log, sentry};

    color_eyre::install()?;
    let _log_guard = log::setup_logger();
    let _sentry_guard = sentry::setup();

    if let Err(e) = log::cleanup_logs() {
        warn!("failed to cleanup logs: {}", e);
    }

    let version = env!("CARGO_PKG_VERSION");
    info!("starting clip-mash v{}", version);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            if let Err(e) = run().await {
                error!("failed to run: {e:?}");
            }
        });

    Ok(())
}
