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

fn get_debug_hostname() -> &'static str {
    use std::env::consts::OS;

    match OS {
        "windows" => "0.0.0.0",
        "macos" => "[::1]",
        _ => "localhost",
    }
}

fn get_port() -> u16 {
    use rand::Rng;

    let port = std::env::args()
        .nth(2)
        .and_then(|port| port.parse::<u16>().ok());
    match port {
        Some(port) => port,
        None => {
            if cfg!(debug_assertions) {
                5174
            } else {
                let random_port = rand::thread_rng().gen_range(1024..65535);
                info!("using random port {random_port}");
                random_port
            }
        }
    }
}

fn get_host() -> String {
    if cfg!(debug_assertions) {
        get_debug_hostname().into()
    } else {
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| "127.0.0.1".into())
    }
}

fn get_address() -> SocketAddr {
    let host = get_host();
    let port = get_port();
    let addr = format!("{}:{}", host, port);
    info!("listening on {addr}");

    addr.parse()
        .expect(&format!("Unable to parse address '{addr}'"))
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
    let version = database.sqlite_version().await?;
    info!("using sqlite version {version}");
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
    });

    let library_routes = Router::new()
        .route("/video", get(handlers::library::list_videos))
        .route("/video", post(handlers::library::add_new_videos))
        .route(
            "/video/need-encoding",
            post(handlers::library::videos_need_encoding),
        )
        .route("/video/:id", put(handlers::library::update_video))
        .route(
            "/video/:id/stash/merge",
            post(handlers::library::merge_stash_video),
        )
        .route(
            "/cleanup/:folder_type",
            post(handlers::files::cleanup_folder),
        )
        .route("/video/cleanup", post(handlers::library::cleanup_videos))
        .route("/video/stash", get(handlers::library::list_stash_videos))
        .route("/video/tags", get(handlers::library::list_video_tags))
        .route("/video/:id", get(handlers::library::get_video))
        .route("/video/:id", delete(handlers::library::delete_video))
        .route(
            "/video/:id/detect-markers",
            post(handlers::library::detect_markers),
        )
        .route("/video/:id/file", get(handlers::library::get_video_file))
        .route(
            "/video/:id/preview",
            get(handlers::library::get_video_preview),
        )
        .route("/marker", get(handlers::library::list_markers))
        .route("/marker/title", get(handlers::library::list_marker_titles))
        .route("/marker", post(handlers::library::create_new_marker))
        .route("/marker/:id", put(handlers::library::update_marker))
        .route("/marker/:id", delete(handlers::library::delete_marker))
        .route(
            "/marker/:id/preview",
            get(handlers::library::get_marker_preview),
        )
        .route("/marker/:id/split", post(handlers::library::split_marker))
        .route("/performers", get(handlers::library::list_performers))
        .route("/directory", get(handlers::files::list_file_entries))
        .route("/stats", get(handlers::files::get_file_stats))
        .route(
            "/migrate/preview",
            post(handlers::library::migrate_preview_images),
        );

    let project_routes = Router::new()
        .route("/clips", post(handlers::project::fetch_clips))
        .route(
            "/clips/interactive",
            post(handlers::project::fetch_clips_interactive),
        )
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
        )
        .route("/random-seed", get(handlers::project::generate_random_seed));

    let stash_routes = Router::new().route("/health", get(handlers::stash::get_stash_health));

    let system_routes = Router::new()
        .route("/restart", post(handlers::system::restart))
        .route("/sentry/error", post(handlers::system::sentry_error))
        .route("/version", get(handlers::system::get_version))
        .route("/health", get(handlers::system::get_app_health))
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

    let handy_routes = Router::new()
        .route("/start", post(handlers::handy::start_handy))
        .route("/stop", post(handlers::handy::stop_handy))
        .route("/pause", post(handlers::handy::pause_handy))
        .route("/connected", get(handlers::handy::handy_connected))
        .route("/", get(handlers::handy::handy_status));

    let api_routes = Router::new()
        .nest("/project", project_routes)
        .nest("/library", library_routes)
        .nest("/stash", stash_routes)
        .nest("/system", system_routes)
        .nest("/song", music_routes)
        .nest("/progress", progress_routes)
        .nest("/handy", handy_routes);

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_routes)
        .fallback_service(static_files::service())
        .layer(DefaultBodyLimit::max(CONTENT_LENGTH_LIMIT))
        .layer(sentry_tower::NewSentryLayer::new_from_top())
        .layer(sentry_tower::SentryHttpLayer::with_transaction())
        .with_state(state);

    let addr = get_address();

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
