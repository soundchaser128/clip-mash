use utoipa::OpenApi;

use crate::server::handlers;
/*

    let stash_routes = Router::new()
        .route("/health", get(handlers::stash::get_health))
        .route("/tags", get(handlers::stash::fetch_tags))
        .route("/performers", get(handlers::stash::fetch_performers))
        .route("/scenes", get(handlers::stash::fetch_scenes))
        .route("/markers", get(handlers::stash::fetch_markers))
        .route("/config", get(handlers::stash::get_config))
        .route("/config", post(handlers::stash::set_config));

    let local_routes = Router::new()
        .route("/video", get(handlers::local::list_videos))
        .route("/video", post(handlers::local::add_new_videos))
        .route("/video/:id", get(handlers::local::get_video))
        .route("/video/:id/markers", post(handlers::local::detect_markers))
        .route("/video/:id/file", get(handlers::local::get_video_file))
        .route(
            "/video/:id/preview",
            get(handlers::local::get_video_preview),
        )
        .route("/video/marker", get(handlers::local::list_markers))
        .route("/video/marker", post(handlers::local::create_new_marker))
        .route("/video/marker", put(handlers::local::update_marker))
        .route("/video/marker/:id", delete(handlers::local::delete_marker))
        .route(
            "/video/marker/:id/split",
            post(handlers::local::split_marker),
        )
        .route(
            "/video/marker/:id/preview",
            get(handlers::local::get_marker_preview),
        )
        .route("/video/download", post(handlers::local::download_video));

    let api_routes = Router::new()
        .nest("/local", local_routes)
        .nest("/stash", stash_routes)
        .route("/version", get(handlers::common::get_version))
        .route("/clips", post(handlers::common::fetch_clips))
        .route("/create", post(handlers::common::create_video))
        .route(
            "/progress/stream",
            get(handlers::common::get_progress_stream),
        )
        .route("/progress/info", get(handlers::common::get_progress_info))
        .route("/download", get(handlers::common::download_video))
        .route(
            "/funscript/combined",
            post(handlers::common::get_combined_funscript),
        )
        .route(
            "/funscript/beat",
            post(handlers::common::get_beat_funscript),
        )
        .route("/song", get(handlers::common::list_songs))
        .route("/song/:id/stream", get(handlers::common::stream_song))
        .route("/song/download", post(handlers::common::download_music))
        .route("/song/upload", post(handlers::common::upload_music))
        .route("/song/:id/beats", get(handlers::common::get_beats))
        .route("/directory/open", get(handlers::common::open_folder))
        .route("/id", get(handlers::common::get_new_id));
*/

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::stash::get_health,
        // todo::list_todos,
        // todo::search_todos,
        // todo::create_todo,
        // todo::mark_done,
        // todo::delete_todo,
    ),
    components(
        // schemas(todo::Todo, todo::TodoError)
    ),
    tags(
        (name = "clip-mash", description = "API for creating video compilations")
    )
)]
pub struct ApiDoc;
