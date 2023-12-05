#![allow(non_camel_case_types)]

use color_eyre::eyre::bail;
use graphql_client::{GraphQLQuery, Response};
use ordered_float::OrderedFloat;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::debug;

use self::create_marker::SceneMarkerCreateInput;
use self::create_tag::TagCreateInput;
use self::find_scenes_query::{
    FindScenesQueryFindScenesScenes, FindScenesQueryFindScenesScenesSceneMarkers,
    FindScenesQueryFindScenesScenesSceneStreams,
};
use super::database::{unix_timestamp_now, DbMarker};
use crate::server::types::PageParameters;
use crate::service::funscript::FunScript;
use crate::service::stash_config::StashConfig;
use crate::util::add_api_key;
use crate::Result;

type Time = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/find_scenes.graphql",
    response_derives = "Debug, Clone"
)]
pub struct FindScenesQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/health_check.graphql",
    response_derives = "Debug"
)]
struct HealthCheckQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/create_marker.graphql",
    response_derives = "Debug"
)]
struct CreateMarker;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/create_tag.graphql",
    response_derives = "Debug"
)]
struct CreateTag;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/get_tag.graphql",
    response_derives = "Debug"
)]
struct GetTag;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FilterMode {
    Performers,
    Tags,
    Scenes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneStream {
    pub url: String,
    pub label: Option<String>,
}

impl From<FindScenesQueryFindScenesScenesSceneStreams> for SceneStream {
    fn from(stream: FindScenesQueryFindScenesScenesSceneStreams) -> Self {
        SceneStream {
            url: stream.url,
            label: stream.label,
        }
    }
}

trait HasStart {
    fn start(&self) -> f64;
}

impl HasStart for &FindScenesQueryFindScenesScenesSceneMarkers {
    fn start(&self) -> f64 {
        self.seconds
    }
}

fn compute_end<M>(start: f64, markers: impl IntoIterator<Item = M>, duration: f64) -> f64
where
    M: HasStart,
{
    markers
        .into_iter()
        .find(|m| m.start() > start)
        .map(|m| m.start())
        .unwrap_or(duration)
}

pub trait MarkerLike {
    fn start(&self) -> f64;

    fn end(&self) -> f64;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StashMarker {
    pub id: String,
    pub primary_tag: String,
    pub start: f64,
    pub end: f64,
    pub streams: Vec<SceneStream>,
    pub scene_id: String,
    pub scene_title: Option<String>,
    pub performers: Vec<String>,
    pub file_name: Option<String>,
    pub scene_interactive: bool,
    pub tags: Vec<String>,
    pub screenshot_url: String,
    pub stream_url: String,
    pub index_within_video: usize,
    pub created_on: i64,
}

impl MarkerLike for StashMarker {
    fn start(&self) -> f64 {
        self.start
    }

    fn end(&self) -> f64 {
        self.end
    }
}

impl StashMarker {
    pub fn from_scene(scene: FindScenesQueryFindScenesScenes, api_key: &str) -> Vec<Self> {
        let duration = scene
            .files
            .iter()
            .max_by_key(|f| OrderedFloat(f.duration))
            .map(|f| f.duration)
            .unwrap_or_default();

        let markers = scene.scene_markers.clone();

        scene
            .scene_markers
            .into_iter()
            .enumerate()
            .map(|(idx, m)| StashMarker {
                id: m.id,
                primary_tag: m.primary_tag.name,
                scene_id: scene.id.clone(),
                scene_interactive: scene.interactive,
                scene_title: scene.title.clone(),
                tags: m.tags.into_iter().map(|t| t.name).collect(),
                performers: scene
                    .performers
                    .clone()
                    .into_iter()
                    .map(|p| p.name)
                    .collect(),
                file_name: scene.files.get(0).map(|f| f.basename.clone()),
                start: m.seconds,
                end: compute_end(m.seconds, &markers, duration),
                streams: scene
                    .scene_streams
                    .clone()
                    .into_iter()
                    .map(From::from)
                    .collect(),
                screenshot_url: add_api_key(&m.screenshot, api_key),
                stream_url: add_api_key(&m.stream, api_key),
                index_within_video: idx,
                created_on: unix_timestamp_now(),
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct StashApi {
    api_url: String,
    api_key: Option<String>,
    client: Client,
}

impl StashApi {
    pub fn new(api_url: &str, api_key: &str) -> Self {
        StashApi {
            api_url: api_url.into(),
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    pub async fn load_config() -> Self {
        let config = StashConfig::get_or_empty().await;
        StashApi::new(&config.stash_url, &config.api_key)
    }

    pub async fn load_config_or_fail() -> Result<Self> {
        if let Ok(config) = StashConfig::get().await {
            Ok(StashApi::new(&config.stash_url, &config.api_key))
        } else {
            bail!("no stash config found")
        }
    }

    pub async fn health(&self) -> Result<String> {
        let variables = health_check_query::Variables {};
        let request_body = HealthCheckQuery::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        let response: Response<health_check_query::ResponseData> = response.json().await?;
        let status = response.data.unwrap().system_status.status;
        Ok(serde_json::to_string(&status)?)
    }

    pub async fn find_scene(&self, scene_id: i64) -> Result<FindScenesQueryFindScenesScenes> {
        let variables = find_scenes_query::Variables {
            scene_ids: Some(vec![scene_id]),
            query: None,
            page: 0,
            page_size: 1,
            has_markers: None,
        };
        let request_body = FindScenesQuery::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        let response: Response<find_scenes_query::ResponseData> = response.json().await?;

        match response.data {
            Some(scenes) => Ok(scenes.find_scenes.scenes.into_iter().next().unwrap()),
            None => bail!("no scene found"),
        }
    }

    pub async fn find_scenes(
        &self,
        page: &PageParameters,
        query: Option<String>,
        with_markers: Option<bool>,
    ) -> Result<(Vec<FindScenesQueryFindScenesScenes>, usize)> {
        let variables = find_scenes_query::Variables {
            page_size: page.size(),
            page: page.page(),
            query,
            scene_ids: None,
            has_markers: match with_markers {
                Some(true) => Some("true".into()),
                _ => None,
            },
        };
        let request_body = FindScenesQuery::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        let response: Response<find_scenes_query::ResponseData> = response.json().await?;
        debug!("stash response: {:#?}", response);
        let (scenes, count) = response
            .data
            .map(|r| (r.find_scenes.scenes, r.find_scenes.count as usize))
            .unwrap_or_default();

        Ok((scenes, count))
    }

    pub async fn find_scenes_by_ids(
        &self,
        ids: Vec<i64>,
    ) -> Result<Vec<FindScenesQueryFindScenesScenes>> {
        let variables = find_scenes_query::Variables {
            scene_ids: Some(ids),
            query: None,
            page: 0,
            page_size: -1,
            has_markers: None,
        };
        let request_body = FindScenesQuery::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        let response: Response<find_scenes_query::ResponseData> = response.json().await?;

        match response.data {
            Some(scenes) => Ok(scenes.find_scenes.scenes),
            None => Ok(vec![]),
        }
    }

    pub async fn add_tag(&self, tag: String) -> Result<String> {
        let variables = create_tag::Variables {
            input: TagCreateInput {
                name: tag,
                description: None,
                aliases: None,
                child_ids: None,
                ignore_auto_tag: None,
                image: None,
                parent_ids: None,
            },
        };

        let request_body = CreateTag::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        let response: Response<create_tag::ResponseData> = response.json().await?;
        debug!("stash response: {:#?}", response);
        Ok(response.data.unwrap().tag_create.unwrap().id)
    }

    pub async fn get_tag_id(&self, tag: String) -> Result<Option<String>> {
        let variables = get_tag::Variables { tag };

        let request_body = GetTag::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        let response: Response<get_tag::ResponseData> = response.json().await?;
        debug!("stash response: {:#?}", response);

        Ok(response
            .data
            .map(|r| r.find_tags.tags)
            .and_then(|mut r| if r.is_empty() { None } else { r.pop() })
            .map(|r| r.id))
    }

    pub async fn add_marker(&self, marker: DbMarker, scene_id: String) -> Result<i64> {
        let tag_id = self.get_tag_id(marker.title.clone()).await?;
        let tag_id = match tag_id {
            Some(id) => id,
            None => self.add_tag(marker.title.clone()).await?,
        };

        let variables = create_marker::Variables {
            marker: SceneMarkerCreateInput {
                scene_id,
                primary_tag_id: tag_id,
                title: marker.title,
                seconds: marker.start_time,
                tag_ids: None,
            },
        };

        let request_body = CreateMarker::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        let response: Response<create_marker::ResponseData> = response.json().await?;
        debug!("stash response: {:#?}", response);
        let new_marker_id = response.data.unwrap().scene_marker_create.unwrap().id;

        Ok(new_marker_id.parse()?)
    }

    pub async fn get_funscript(&self, scene_id: i64) -> Result<FunScript> {
        let url = format!("{}/scene/{}/funscript", self.api_url, scene_id);
        let response = self
            .client
            .get(url)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    pub fn get_screenshot_url(&self, id: i64) -> String {
        let url = format!("{}/scene/{}/screenshot", self.api_url, id);
        add_api_key(&url, &self.api_key)
    }

    pub fn get_stream_url(&self, id: i64) -> String {
        let url = format!("{}/scene/{}/stream", self.api_url, id);
        add_api_key(&url, &self.api_key)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_health() {}
}
