use utoipa::OpenApi;

use super::dtos::ListVideoDtoPage;
use super::handlers::local::AddNewVideosBody;
use super::types::*;
use crate::server::handlers::{stash, common, local};

#[derive(OpenApi)]
#[openapi(
    paths(
        stash::get_health,
        local::add_new_videos,
        local::list_videos,
        local::detect_markers,
        local::create_new_marker,
        common::fetch_clips,
        common::create_video,
        common::get_progress_info,
        common::download_video,
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
