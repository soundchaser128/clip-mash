use utoipa::OpenApi;

use super::dtos::ListVideoDtoPage;
use super::handlers::project::CreateFunscriptBody;
use super::types::*;
use crate::data::database::VideoSource;
use crate::server::handlers::{library, progress, project, stash};
use crate::service::stash_config::Config;
use crate::service::video::AddVideosRequest;

#[derive(OpenApi)]
#[openapi(
    paths(
        library::add_new_videos,
        library::create_new_marker,
        library::delete_marker,
        library::detect_markers,
        library::get_video,
        library::list_markers,
        library::list_videos,
        library::split_marker,
        library::update_marker,
        progress::get_progress_info,
        project::create_video,
        project::download_video,
        project::fetch_clips,
        project::get_beat_funscript,
        project::get_combined_funscript,
        project::get_new_id,
        stash::get_config,
    ),
    components(
        schemas(
            ListVideoDtoPage,
            ListVideoDto,
            MarkerDto,
            VideoDto,
            MarkerId,
            VideoId,
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
            VideoResolution,
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
            Config,
            SortDirection,
            SongDto,
        )
    ),
    tags(
        (name = "clip-mash", description = "API for creating video compilations")
    )
)]
pub struct ApiDoc;
