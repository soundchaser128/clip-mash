use crate::{data::database::Database, service::generator::CompilationGenerator};

pub struct AppState {
    pub generator: CompilationGenerator,
    pub database: Database,
}

pub mod common {
    use std::{collections::HashSet, sync::Arc};

    use axum::{extract::State, Json};

    use crate::{
        data::stash_api::StashApi,
        server::{
            dtos::{ClipsResponse, CreateClipsBody},
            error::AppError,
        },
        service::{
            clip::{self, ClipService},
            stash_config::Config,
        },
    };

    use super::AppState;

    #[axum::debug_handler]
    pub async fn fetch_clips(
        State(state): State<Arc<AppState>>,
        Json(body): Json<CreateClipsBody>,
    ) -> Result<Json<ClipsResponse>, AppError> {
        let config = Config::get().await?;
        let api = StashApi::from_config(&config);
        let service = ClipService::new(&state.database, &api);
        let order = body.clip_order;
        let video_ids: HashSet<_> = body.markers.iter().map(|m| m.video_id.clone()).collect();
        let options = service.convert_clip_options(body).await?;
        let clips = clip::get_all_clips(&options);
        let clips = clip::compile_clips(clips, order);
        tracing::debug!("compiled clips {clips:#?}");
        let streams = clip::get_streams(video_ids, &config)?;
        // let mut scene_ids: Vec<_> = clips.iter().map(|c| c.video_id).collect();
        // scene_ids.sort();
        // scene_ids.dedup();

        // tracing::debug!("scene IDs: {:?}", scene_ids);
        // let scenes = api
        //     .find_scenes_by_ids(scene_ids)
        //     .await?
        //     .into_iter()
        //     .map(Scene::from)
        //     .collect();

        // Ok(Json(ClipsResponse {
        //     clips,
        //     streams,
        //     scenes,
        // }))
        Ok(Json(ClipsResponse { clips, streams }))
    }
}

pub mod stash {
    use std::cmp::Reverse;

    use axum::{extract::Query, response::IntoResponse, Json};
    use reqwest::StatusCode;
    use serde::Deserialize;

    use crate::{
        data::stash_api::{FilterMode, StashApi},
        server::{
            dtos::{MarkerDto, PerformerDto, TagDto, VideoDto},
            error::AppError,
        },
        service::stash_config::Config,
        util::add_api_key,
    };

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MarkerOptions {
        pub selected_ids: String,
        pub mode: FilterMode,
        pub include_all: bool,
    }

    #[axum::debug_handler]
    pub async fn fetch_tags() -> Result<Json<Vec<TagDto>>, AppError> {
        let api = StashApi::load_config().await?;
        let tags = api.find_tags().await?;
        let mut tags: Vec<_> = tags
            .into_iter()
            .map(|t| TagDto {
                name: t.name,
                id: t.id,
                marker_count: t.scene_marker_count.unwrap_or_default(),
            })
            .filter(|t| t.marker_count > 0)
            .collect();
        tags.sort_by_key(|t| Reverse(t.marker_count));

        tracing::debug!("returning tags {:?}", tags);

        Ok(Json(tags))
    }

    #[axum::debug_handler]
    pub async fn fetch_performers() -> Result<Json<Vec<PerformerDto>>, AppError> {
        let config = Config::get().await?;
        let api = StashApi::from_config(&config);
        let performers = api.find_performers().await?;
        let mut performers: Vec<_> = performers
            .into_iter()
            .map(|p| PerformerDto {
                id: p.id,
                scene_count: p.scene_count.unwrap_or_default(),
                name: p.name,
                image_url: p.image_path.map(|url| add_api_key(&url, &config.api_key)),
                tags: p.tags.into_iter().map(|t| t.name).collect(),
                rating: p.rating100,
                favorite: p.favorite,
            })
            .filter(|p| p.scene_count > 0)
            .collect();
        performers.sort_by_key(|t| Reverse(t.scene_count));

        tracing::debug!("returning performers {:?}", performers);

        Ok(Json(performers))
    }

    #[axum::debug_handler]
    pub async fn fetch_markers(
        query: Query<MarkerOptions>,
    ) -> Result<Json<Vec<MarkerDto>>, AppError> {
        let config = Config::get().await?;
        let api = StashApi::from_config(&config);
        tracing::info!("fetching markers for query {query:?}");
        let ids: Vec<_> = query.selected_ids.split(',').map(From::from).collect();

        let markers = api.find_markers(ids, query.mode, query.include_all).await?;
        let markers = markers.into_iter().map(From::from).collect();
        Ok(Json(markers))
    }

    #[axum::debug_handler]
    pub async fn fetch_scenes() -> Result<Json<Vec<VideoDto>>, AppError> {
        let config = Config::get().await?;
        let api = StashApi::from_config(&config);
        let videos = api.find_scenes().await?;
        let videos = videos.into_iter().map(From::from).collect();
        Ok(Json(videos))
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ConfigQuery {
        url: String,
        api_key: String,
    }

    #[axum::debug_handler]
    pub async fn get_health(
        Query(ConfigQuery { url, api_key }): Query<ConfigQuery>,
    ) -> Result<Json<String>, AppError> {
        let api = StashApi::new(&url, &api_key);
        let result = api.health().await?;
        Ok(Json(result))
    }

    #[axum::debug_handler]
    pub async fn get_config() -> impl IntoResponse {
        match Config::get().await {
            Ok(config) => Json(Some(config)),
            Err(_) => Json(None),
        }
    }

    #[axum::debug_handler]
    pub async fn set_config(Json(config): Json<Config>) -> Result<StatusCode, AppError> {
        use crate::service::stash_config;

        tracing::info!("setting config with URL {}", config.stash_url);
        stash_config::set_config(config).await?;

        Ok(StatusCode::NO_CONTENT)
    }
}

pub mod local {
    use std::sync::Arc;

    use axum::{
        body::Body,
        extract::{Path, Query, State},
        response::IntoResponse,
        Json,
    };
    use camino::Utf8PathBuf;
    use reqwest::StatusCode;
    use serde::Deserialize;
    use tower::ServiceExt;

    use crate::{
        data::database::CreateMarker,
        server::{error::AppError, handlers::AppState},
    };

    #[axum::debug_handler]
    pub async fn get_video(
        Path(id): Path<String>,
        state: State<Arc<AppState>>,
        request: axum::http::Request<Body>,
    ) -> Result<impl IntoResponse, AppError> {
        use tower_http::services::ServeFile;

        let video = state.database.get_video(&id).await?;
        if let Some(video) = video {
            let result = ServeFile::new(video.file_path).oneshot(request).await;
            Ok(result)
        } else {
            Err(AppError::StatusCode(StatusCode::NOT_FOUND))
        }
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ListVideoQuery {
        path: String,
        recurse: bool,
    }

    // #[axum::debug_handler]
    // pub async fn list_videos(
    //     Query(ListVideoQuery { path, recurse }): Query<ListVideoQuery>,
    //     state: State<Arc<AppState>>,
    // ) -> Result<Json<Vec<LocalVideoWithMarkers>>, AppError> {
    //     use crate::local::find::list_videos;

    //     let videos = list_videos(Utf8PathBuf::from(path), recurse, &state.database).await?;
    //     Ok(Json(videos))
    // }

    // #[axum::debug_handler]
    // pub async fn persist_marker(
    //     state: State<Arc<AppState>>,
    //     Json(marker): Json<CreateMarker>,
    // ) -> Result<Json<Marker>, AppError> {
    //     tracing::info!("saving marker {marker:?} to the database");
    //     let marker = state.database.persist_marker(marker).await?;

    //     todo!()
    // }

    #[axum::debug_handler]
    pub async fn delete_marker(
        Path(id): Path<i64>,
        state: State<Arc<AppState>>,
    ) -> Result<(), AppError> {
        tracing::info!("deleting marker {id}");
        state.database.delete_marker(id).await?;

        Ok(())
    }
}
