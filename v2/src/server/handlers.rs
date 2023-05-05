use crate::{data::database::Database, service::generator::CompilationGenerator};

pub struct AppState {
    pub generator: CompilationGenerator,
    pub database: Database,
}

pub mod stash {
    use std::cmp::Reverse;

    use axum::Json;
    use sqlx::query::Query;

    use crate::{
        data::stash_api::StashApi,
        server::{
            dtos::{PerformerDto, TagDto},
            error::AppError,
        },
        service::stash_config::Config,
        util::add_api_key,
    };

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
        let api = StashApi::from_config(config);
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
        Query(query): Query<MarkerOptions>,
    ) -> Result<Json<MarkerResult>, AppError> {
        let config = Config::get().await?;
        let api = StashApi::from_config(&config);
        tracing::info!("fetching markers for query {query:?}");
        let ids: Vec<_> = query.selected_ids.split(',').map(From::from).collect();

        let markers = api.find_markers(ids, query.mode, query.include_all).await?;
        Ok(Json(MarkerResult { dtos: markers }))
    }

    #[axum::debug_handler]
    pub async fn fetch_scenes() -> Result<Json<Vec<StashScene>>, AppError> {
        let config = Config::get().await?;
        let api = StashApi::from_config(&config);
        let api_key = &config.api_key;
        let scenes = api.find_scenes().await?;
        Ok(Json(scenes))
    }
}

pub mod local {}
