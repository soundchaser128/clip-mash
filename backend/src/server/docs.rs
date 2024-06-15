use utoipa::OpenApi;

use super::handlers::files::{FileSystemEntry, ListFileEntriesResponse};
use super::handlers::library::{CreateMarkerRequest, VideoCleanupResponse};
use super::handlers::music::SongUpload;
use super::handlers::project::{CreateFunscriptBody, DescriptionData, ProjectCreateResponse};
use super::types::*;
use crate::data::database::{HandyConfig, MarkerCount, Settings, VideoSource, VideoUpdate};
use crate::server::handlers::handy::StartHandyParameters;
use crate::server::handlers::library::ListPerformerResponse;
use crate::server::handlers::{files, handy, library, music, progress, project, stash, system};
use crate::service::description_generator::DescriptionType;
use crate::service::directories::FolderType;
use crate::service::generator::PaddingType;
use crate::service::handy::patterns::{CycleIncrementParameters, HandyPattern, Range};
use crate::service::new_version_checker::AppVersion;
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
        library::migrate_preview_images,
        library::videos_need_encoding,
        library::list_marker_titles,
        library::list_performers,
        files::list_file_entries,
        files::get_file_stats,
        files::cleanup_folder,
        progress::get_progress_info,
        progress::delete_progress,
        project::create_video,
        project::download_video,
        project::fetch_clips,
        project::fetch_clips_interactive,
        project::get_beat_funscript,
        project::get_combined_funscript,
        project::get_new_id,
        project::list_finished_videos,
        project::generate_description,
        project::generate_random_seed,
        stash::get_stash_health,
        music::list_songs,
        music::get_beats,
        music::upload_music,
        music::download_music,
        system::get_version,
        system::get_config,
        system::set_config,
        system::restart,
        system::get_app_health,
        handy::start_handy,
        handy::stop_handy,
        handy::pause_handy,
        handy::handy_status,
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
            ClipLengthOptions,
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
            SongUpload,
            AppVersion,
            MarkerCount,
            FileSystemEntry,
            ListFileEntriesResponse,
            MarkerTitle,
            MarkerGroup,
            DescriptionType,
            DescriptionData,
            FolderType,
            PaddingType,
            Settings,
            CreateInteractiveClipsBody,
            InteractiveClipsQuery,
            ListPerformerResponse,
            StartHandyParameters,
            HandyConfig,
            HandyPattern,
            CycleIncrementParameters,
            Range,
        )
    ),
    tags(
        (name = "clip-mash", description = "API for creating video compilations")
    )
)]
pub struct ApiDoc;
