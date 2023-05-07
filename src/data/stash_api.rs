use crate::{
    service::{funscript::FunScript, stash_config::Config},
    util::add_api_key,
    Result,
};
use color_eyre::eyre::bail;
use graphql_client::{GraphQLQuery, Response};
use ordered_float::OrderedFloat;
use reqwest::Client;
use serde::Deserialize;

use self::{
    find_markers_query::{
        CriterionModifier, FindFilterType, FindMarkersQueryFindSceneMarkersSceneMarkers,
        FindMarkersQueryFindSceneMarkersSceneMarkersSceneSceneMarkers,
        FindMarkersQueryFindSceneMarkersSceneMarkersSceneSceneStreams,
        HierarchicalMultiCriterionInput, MultiCriterionInput, SceneMarkerFilterType,
    },
    find_performers_query::FindPerformersQueryFindPerformersPerformers,
    find_scenes_query::{
        FindScenesQueryFindScenesScenes, FindScenesQueryFindScenesScenesSceneMarkers,
        FindScenesQueryFindScenesScenesSceneStreams,
    },
    find_tags_query::FindTagsQueryFindTagsTags,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/find_tags.graphql",
    response_derives = "Debug"
)]
pub struct FindTagsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/find_markers.graphql",
    response_derives = "Debug, Clone"
)]
pub struct FindMarkersQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/find_performers.graphql",
    response_derives = "Debug"
)]
pub struct FindPerformersQuery;

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

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FilterMode {
    Performers,
    Tags,
    Scenes,
}

#[derive(Debug, Clone, PartialEq)]
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

impl From<FindMarkersQueryFindSceneMarkersSceneMarkersSceneSceneStreams> for SceneStream {
    fn from(stream: FindMarkersQueryFindSceneMarkersSceneMarkersSceneSceneStreams) -> Self {
        SceneStream {
            url: stream.url,
            label: stream.label,
        }
    }
}

trait MarkerInfo {
    fn start(&self) -> f64;
}

impl MarkerInfo for &FindScenesQueryFindScenesScenesSceneMarkers {
    fn start(&self) -> f64 {
        self.seconds
    }
}

impl MarkerInfo for &FindMarkersQueryFindSceneMarkersSceneMarkersSceneSceneMarkers {
    fn start(&self) -> f64 {
        self.seconds
    }
}

fn compute_end<M>(start: f64, markers: impl IntoIterator<Item = M>, duration: f64) -> f64
where
    M: MarkerInfo,
{
    markers
        .into_iter()
        .find(|m| m.start() > start)
        .map(|m| m.start())
        .unwrap_or(duration)
}

#[derive(Debug, Clone, PartialEq)]
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
}

impl StashMarker {
    fn from_scene(scene: FindScenesQueryFindScenesScenes, api_key: &str) -> Vec<Self> {
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
            })
            .collect()
    }

    fn from_marker(m: FindMarkersQueryFindSceneMarkersSceneMarkers, api_key: &str) -> Self {
        let duration = m
            .scene
            .files
            .iter()
            .max_by_key(|f| OrderedFloat(f.duration))
            .map(|f| f.duration)
            .unwrap_or_default();
        let index = m
            .scene
            .scene_markers
            .iter()
            .position(|n| m.id == n.id)
            .expect("marker must exist within its own scene");

        StashMarker {
            id: m.id,
            primary_tag: m.primary_tag.name,
            scene_id: m.scene.id,
            streams: m.scene.scene_streams.into_iter().map(From::from).collect(),
            start: m.seconds,
            end: compute_end(m.seconds, &m.scene.scene_markers, duration),
            file_name: m.scene.files.into_iter().map(|f| f.basename).next(),
            performers: m.scene.performers.into_iter().map(|p| p.name).collect(),
            scene_interactive: m.scene.interactive,
            scene_title: m.scene.title,
            tags: m.tags.into_iter().map(|m| m.name).collect(),
            screenshot_url: add_api_key(&m.screenshot, api_key),
            stream_url: add_api_key(&m.stream, api_key),
            index_within_video: index,
        }
    }
}

pub struct StashApi {
    api_url: String,
    api_key: String,
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

    pub async fn load_config() -> Result<Self> {
        let config = Config::get().await?;
        Ok(StashApi::new(&config.stash_url, &config.api_key))
    }

    pub fn from_config(config: &Config) -> Self {
        StashApi::new(&config.stash_url, &config.api_key)
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

    pub async fn find_scenes(&self) -> Result<Vec<FindScenesQueryFindScenesScenes>> {
        let variables = find_scenes_query::Variables { scene_ids: None };
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
        let scenes = response.data.unwrap().find_scenes.scenes;

        Ok(scenes)
    }

    pub async fn get_marker(&self, video_id: &str, marker_id: i64) -> Result<StashMarker> {
        let mut scenes = self.find_scenes_by_ids(vec![video_id.parse()?]).await?;
        if scenes.len() != 1 {
            bail!("found {} scenes for ID {video_id}", scenes.len());
        }
        let markers = StashMarker::from_scene(scenes.remove(0), &self.api_key);
        if markers.is_empty() {
            bail!("no marker found for video ID {video_id} and marker ID {marker_id}")
        } else {
            let string_id = marker_id.to_string();
            if let Some(marker) = markers.into_iter().find(|m| m.id == string_id) {
                Ok(marker)
            } else {
                bail!("no marker with ID {marker_id} found in scene {video_id}")
            }
        }
    }

    pub async fn find_scenes_by_ids(
        &self,
        ids: Vec<i64>,
    ) -> Result<Vec<FindScenesQueryFindScenesScenes>> {
        let variables = find_scenes_query::Variables {
            scene_ids: Some(ids),
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

    pub async fn find_tags(&self) -> Result<Vec<FindTagsQueryFindTagsTags>> {
        let variables = find_tags_query::Variables {};
        let request_body = FindTagsQuery::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        tracing::debug!("url = '{url}', api key = '{}'", self.api_key);

        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        let response: Response<find_tags_query::ResponseData> = response.json().await?;
        let tags = response.data.unwrap().find_tags.tags;

        Ok(tags)
    }

    pub async fn find_markers(
        &self,
        ids: Vec<String>,
        mode: FilterMode,
        include_all: bool,
    ) -> Result<Vec<StashMarker>> {
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

        match mode {
            FilterMode::Performers => {
                scene_filter.performers = Some(MultiCriterionInput {
                    modifier: if include_all {
                        CriterionModifier::INCLUDES_ALL
                    } else {
                        CriterionModifier::INCLUDES
                    },
                    value: Some(ids),
                });
            }
            FilterMode::Tags => {
                scene_filter.tags = Some(HierarchicalMultiCriterionInput {
                    depth: None,
                    modifier: if include_all {
                        CriterionModifier::INCLUDES_ALL
                    } else {
                        CriterionModifier::INCLUDES
                    },
                    value: Some(ids),
                });
            }
            FilterMode::Scenes => {
                let ids = ids
                    .into_iter()
                    .map(|s| s.parse().expect("id must be a valid integer"))
                    .collect();
                let scenes = self.find_scenes_by_ids(ids).await?;

                return Ok(scenes
                    .into_iter()
                    .flat_map(|s| StashMarker::from_scene(s, &self.api_key))
                    .collect());
            }
        }
        let variables = find_markers_query::Variables {
            filter: Some(FindFilterType {
                per_page: Some(-1),
                page: None,
                q: None,
                sort: None,
                direction: None,
            }),
            scene_marker_filter: Some(scene_filter),
        };

        let request_body = FindMarkersQuery::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;

        let response: Response<find_markers_query::ResponseData> = response.json().await?;
        let markers = response.data.unwrap();
        let markers = markers
            .find_scene_markers
            .scene_markers
            .into_iter()
            .map(|m| StashMarker::from_marker(m, &self.api_key))
            .collect();
        Ok(markers)
    }

    pub async fn get_funscript(&self, scene_id: &str) -> Result<FunScript> {
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

    pub async fn find_performers(
        &self,
    ) -> Result<Vec<FindPerformersQueryFindPerformersPerformers>> {
        let variables = find_performers_query::Variables {};
        let request_body = FindPerformersQuery::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;

        let response: Response<find_performers_query::ResponseData> = response.json().await?;
        let performers = response.data.unwrap();
        Ok(performers.find_performers.performers)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_health() {}
}
