use std::{cmp::Reverse, sync::Arc, time::Duration};

use axum::{
    body::StreamBody,
    extract::{Path, Query, State},
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Sse,
    },
    Json,
};
use futures::{
    stream::{self, Stream},
    FutureExt,
};
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;

use crate::{
    clip::{Clip, ClipOrder},
    config::{self, Config},
    error::AppError,
    ffmpeg,
    stash_api::{
        find_markers_query::{
            self, CriterionModifier, FindFilterType,
            FindMarkersQueryFindSceneMarkersSceneMarkers as GqlMarker,
            HierarchicalMultiCriterionInput, MultiCriterionInput, SceneMarkerFilterType,
        },
        find_performers_query, find_tags_query, Api,
    },
    AppState,
};

#[derive(Serialize, Debug)]
pub struct Tag {
    pub name: String,
    pub id: String,
    pub count: i64,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Performer {
    pub name: String,
    pub id: String,
    pub scene_count: i64,
    pub image_url: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Marker {
    pub id: String,
    pub primary_tag: String,
    pub stream_url: String,
    pub screenshot_url: String,
    pub start: u32,
    pub end: Option<u32>,
    pub scene_title: Option<String>,
    pub performers: Vec<String>,
    pub file_name: String,
}

impl From<GqlMarker> for Marker {
    fn from(value: GqlMarker) -> Self {
        todo!()
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkerResult {
    pub dtos: Vec<Marker>,
    pub gql: Vec<GqlMarker>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FilterMode {
    Performers,
    Tags,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkerOptions {
    pub selected_ids: String,
    pub mode: FilterMode,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Resolution {
    #[serde(rename = "720")]
    SevenTwenty,
    #[serde(rename = "1080")]
    TenEighty,
    #[serde(rename = "4K")]
    FourK,
}

impl Resolution {
    pub fn resolution(&self) -> (u32, u32) {
        match self {
            Resolution::SevenTwenty => (1280, 720),
            Resolution::TenEighty => (1920, 1080),
            Resolution::FourK => (3840, 2160),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SelectedMarker {
    pub id: String,
    pub duration: Option<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateVideoBody {
    pub select_mode: FilterMode,
    pub selected_ids: Vec<String>,
    pub clip_order: ClipOrder,
    pub clip_duration: u32,
    pub output_resolution: Resolution,
    pub output_fps: u32,
    pub selected_markers: Vec<SelectedMarker>,
    pub markers: Vec<GqlMarker>,
    pub id: String,
}

fn add_api_key(url: &str, api_key: &str) -> String {
    let mut url = Url::parse(url).expect("invalid url");
    url.query_pairs_mut().append_pair("apikey", api_key);
    url.to_string()
}

#[axum::debug_handler]
pub async fn fetch_tags() -> Result<Json<Vec<Tag>>, AppError> {
    let api = Api::load_config().await?;
    let tags = api.find_tags(find_tags_query::Variables {}).await?;
    let mut tags: Vec<_> = tags
        .into_iter()
        .map(|t| Tag {
            name: t.name,
            id: t.id,
            count: t.scene_marker_count.unwrap_or_default(),
        })
        .filter(|t| t.count > 0)
        .collect();
    tags.sort_by_key(|t| Reverse(t.count));

    tracing::debug!("returning tags {:?}", tags);

    Ok(Json(tags))
}

#[axum::debug_handler]
pub async fn fetch_performers() -> Result<Json<Vec<Performer>>, AppError> {
    let config = Config::get().await?;
    let api = Api::from_config(&config);
    let performers = api
        .find_performers(find_performers_query::Variables {})
        .await?;
    let mut performers: Vec<_> = performers
        .into_iter()
        .map(|p| Performer {
            id: p.id,
            scene_count: p.scene_count.unwrap_or_default(),
            name: p.name,
            image_url: p.image_path.map(|url| add_api_key(&url, &config.api_key)),
        })
        .filter(|p| p.scene_count > 0)
        .collect();
    performers.sort_by_key(|t| Reverse(t.scene_count));

    tracing::debug!("returning performers {:?}", performers);

    Ok(Json(performers))
}

#[axum::debug_handler]
pub async fn fetch_markers(
    state: State<Arc<AppState>>,
    Query(query): Query<MarkerOptions>,
) -> Result<Json<MarkerResult>, AppError> {
    let config = Config::get().await?;
    let api = Api::from_config(&config);
    tracing::info!("fetching markers for query {query:?}");

    let mut scene_filter = SceneMarkerFilterType {
        created_at: None,
        scene_created_at: None,
        scene_updated_at: None,
        updated_at: None,
        performers: None,
        scene_date: None,
        scene_tags: None,
        tag_id: None,
        tags: None,
    };

    let ids: Vec<_> = query.selected_ids.split(',').map(From::from).collect();

    match query.mode {
        FilterMode::Performers => {
            scene_filter.performers = Some(MultiCriterionInput {
                modifier: CriterionModifier::INCLUDES,
                value: Some(ids),
            });
        }
        FilterMode::Tags => {
            scene_filter.tags = Some(HierarchicalMultiCriterionInput {
                depth: None,
                modifier: CriterionModifier::INCLUDES,
                value: Some(ids),
            });
        }
    }

    let gql_markers = api
        .find_markers(find_markers_query::Variables {
            filter: Some(FindFilterType {
                per_page: Some(-1),
                page: None,
                q: None,
                sort: None,
                direction: None,
            }),
            scene_marker_filter: Some(scene_filter),
        })
        .await?;

    let api_key = &config.api_key;
    let dtos = gql_markers
        .clone()
        .into_iter()
        .map(|m| Marker::from(m))
        .collect();

    Ok(Json(MarkerResult {
        dtos,
        gql: gql_markers,
    }))
}

async fn create_video_inner(
    state: State<Arc<AppState>>,
    mut body: CreateVideoBody,
) -> Result<(), AppError> {
    body.markers
        .retain(|e| body.selected_markers.iter().any(|m| m.id == e.id));
    let clips = state.ffmpeg.gather_clips(&body).await?;
    state.ffmpeg.compile_clips(clips, &body).await?;

    Ok(())
}

#[axum::debug_handler]
pub async fn create_video(
    state: State<Arc<AppState>>,
    Json(body): Json<CreateVideoBody>,
) -> StatusCode {
    tracing::debug!("received json body: {:?}", body);
    tokio::spawn(async move {
        if let Err(e) = create_video_inner(state, body).await {
            tracing::error!("error: {e:?}");
        }
    });

    StatusCode::NO_CONTENT
}

#[axum::debug_handler]
pub async fn get_progress() -> Sse<impl Stream<Item = Result<Event, serde_json::Error>>> {
    let stream = futures::StreamExt::flat_map(stream::repeat_with(ffmpeg::get_progress), |f| {
        f.into_stream()
    });
    let stream = stream
        .map(|p| Event::default().json_data(p))
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[axum::debug_handler]
pub async fn download_video(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    use axum::{http::header, response::AppendHeaders};

    tracing::info!("downloading video {id}");
    let path = state.ffmpeg.video_dir.join(format!("{id}.mp4"));
    let file = tokio::fs::File::open(path).await?;
    let stream = ReaderStream::new(file);
    let content_disposition = format!("attachment; filename=\"{}.mp4\"", id);

    let headers = AppendHeaders([
        (header::CONTENT_TYPE, "video/mp4".to_string()),
        (header::CONTENT_DISPOSITION, content_disposition),
    ]);

    let body = StreamBody::new(stream);
    Ok((headers, body))
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
    tracing::info!("setting config with URL {}", config.stash_url);
    config::set_config(config).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[axum::debug_handler]
pub async fn fetch_clips(
    state: State<Arc<AppState>>,
    Json(body): Json<CreateVideoBody>,
) -> Json<Vec<Clip>> {
    // let clips: Vec<_> = body
    //     .markers
    //     .iter()
    //     .map(|marker| {
    //         let dto = body.selected_markers.iter().find(|m| m.id == marker.id).expect("no matching dto found");
    //         let range = state.ffmpeg.get_clip_offsets(
    //             marker,
    //             body.clip_duration,
    //             body.selected_markers
    //                 .iter()
    //                 .find(|n| n.id == marker.id)
    //                 .and_then(|m| m.duration),
    //         );

    //         Clip { marker: dto, range }
    //     })
    //     .collect();

    todo!()
}
