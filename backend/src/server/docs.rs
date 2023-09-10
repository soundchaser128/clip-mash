use utoipa::OpenApi;

use super::dtos::ListVideoDtoPage;
use super::types::*;
use crate::server::handlers::{library, progress, project};

#[derive(OpenApi)]
#[openapi(
    paths(
        library::list_videos,
        library::detect_markers,
        library::create_new_marker,
        project::fetch_clips,
        project::create_video,
        progress::get_progress_info,
        project::download_video,
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
        )
    ),
    tags(
        (name = "clip-mash", description = "API for creating video compilations")
    )
)]
pub struct ApiDoc;
