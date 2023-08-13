use utoipa::OpenApi;

use super::dtos::ListVideoDtoPage;
use super::handlers::local::AddNewVideosBody;
use super::types::{ListVideoDto, MarkerDto, MarkerId, VideoDto, VideoId, VideoSource};
use crate::server::handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::stash::get_health,
        handlers::local::add_new_videos,
        handlers::local::list_videos,
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
        )
    ),
    tags(
        (name = "clip-mash", description = "API for creating video compilations")
    )
)]
pub struct ApiDoc;
