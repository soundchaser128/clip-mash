use utoipa::OpenApi;

use super::dtos::ListVideoDtoPage;
use super::handlers::local::AddNewVideosBody;
use super::types::*;
use crate::server::handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::stash::get_health,
        handlers::local::add_new_videos,
        handlers::local::list_videos,
        handlers::local::detect_markers,
        handlers::local::create_new_marker,
        handlers::common::fetch_clips,
        handlers::common::create_video,
        handlers::common::get_progress_info,
        handlers::common::download_video,
    ),
    components(
        schemas(
            AddNewVideosBody,
            ListVideoDtoPage,
            ListVideoDto,
            MarkerDto,
            VideoDto,
            MarkerId,
            VideoId,
            VideoSource,
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
        )
    ),
    tags(
        (name = "clip-mash", description = "API for creating video compilations")
    )
)]
pub struct ApiDoc;
