use utoipa::OpenApi;

use super::handlers::library::{CreateMarkerRequest, VideoCleanupResponse};
use super::handlers::project::{CreateFunscriptBody, ProjectCreateResponse};
use super::types::*;
use crate::data::database::{VideoSource, VideoUpdate};
use crate::server::handlers::{library, progress, project, stash};
use crate::service::stash_config::StashConfig;
use crate::service::video::AddVideosRequest;

#[derive(OpenApi)]
#[openapi(
    paths(
        library::add_new_videos,
        library::update_video,
        library::create_new_marker,
        library::delete_marker,
        library::delete_video,
        library::detect_markers,
        library::get_video,
        library::list_markers,
        library::list_videos,
        library::split_marker,
        library::update_marker,
        library::list_stash_videos,
        library::cleanup_videos,
        library::merge_stash_video,
        library::videos_need_encoding,
        progress::get_progress_info,
        project::create_video,
        project::download_video,
        project::fetch_clips,
        project::get_beat_funscript,
        project::get_combined_funscript,
        project::get_new_id,
        project::list_finished_videos,
        stash::get_config,
        stash::get_health,
        stash::set_config,
    ),
    components(
        schemas(
            ListVideoDtoPage,
            StashVideoDtoPage,
            MarkerDtoPage,
            ListVideoDto,
            MarkerDto,
            VideoDto,
            CreateClipsBody,
            ClipsResponse,
            ClipOrder,
            ClipOptions,
            SelectedMarker,
            Clip,
            ClipPickerOptions,
            RoundRobinClipOptions,
            WeightedRandomClipOptions,
            EqualLengthClipOptions,
            RoundRobinClipOptions,
            WeightedRandomClipOptions,
            EqualLengthClipOptions,
            PmvClipOptions,
            RandomizedClipOptions,
            SongClipOptions,
            MeasureCount,
            Beats,
            CreateVideoBody,
            EncodingEffort,
            VideoCodec,
            VideoQuality,
            Progress,
            CreateMarker,
            VideoSource,
            CreateBeatFunscriptBody,
            StrokeType,
            CreateFunscriptBody,
            UpdateMarker,
            AddVideosRequest,
            NewId,
            StashConfig,
            SortDirection,
            SongDto,
            StashVideoDto,
            ProjectCreateResponse,
            VideoCleanupResponse,
            VideoUpdate,
            VideoDetailsDto,
            CreateMarkerRequest,
        )
    ),
    tags(
        (name = "clip-mash", description = "API for creating video compilations")
    )
)]
pub struct ApiDoc;
