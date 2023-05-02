use crate::Result;
use graphql_client::{GraphQLQuery, Response};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

use self::{
    find_markers_query::{
        CriterionModifier, FindFilterType, FindMarkersQueryFindSceneMarkersSceneMarkers,
        HierarchicalMultiCriterionInput, MultiCriterionInput, SceneMarkerFilterType,
    },
    find_scenes_query::{
        FindScenesQueryFindScenesScenes, FindScenesQueryFindScenesScenesSceneMarkers,
    },
    find_tags_query::FindTagsQueryFindTagsTags,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../graphql/schema.json",
    query_path = "../graphql/find_tags.graphql",
    response_derives = "Debug"
)]
struct FindTagsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../graphql/schema.json",
    query_path = "../graphql/find_markers.graphql",
    response_derives = "Debug"
)]
struct FindMarkersQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../graphql/schema.json",
    query_path = "../graphql/find_performers.graphql",
    response_derives = "Debug"
)]
struct FindPerformersQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../graphql/schema.json",
    query_path = "../graphql/find_scenes.graphql",
    response_derives = "Debug"
)]
struct FindScenesQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../graphql/schema.json",
    query_path = "../graphql/health_check.graphql",
    response_derives = "Debug"
)]
struct HealthCheckQuery;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SceneMarker {
    pub start: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub url: String,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedScene {
    pub id: String,
    pub scene_markers: Vec<SceneMarker>,
    pub scene_streams: Vec<Stream>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StashStandaloneMarker {
    pub id: String,
    pub primary_tag: String,
    pub tags: Vec<String>,
    pub stream_url: String,
    pub screenshot_url: String,
    pub start: f64,
    pub end: Option<f64>,
    pub scene_title: Option<String>,
    pub performers: Vec<String>,
    pub file_name: String,
    pub scene: EmbeddedScene,
    pub index_in_scene: usize,
    pub scene_interactive: bool,
}

impl StashStandaloneMarker {
    fn from_marker(
        value: FindMarkersQueryFindSceneMarkersSceneMarkers,
        api_key: &str,
    ) -> StashStandaloneMarker {
        let end = value
            .scene
            .scene_markers
            .iter()
            .find(|m| m.seconds > value.seconds)
            .map(|m| m.seconds);

        let index_in_scene = value
            .scene
            .scene_markers
            .iter()
            .position(|m| m.id == value.id)
            .expect("marker must exist within its own scene");

        StashStandaloneMarker {
            id: value.id,
            start: value.seconds,
            end,
            tags: value.tags.into_iter().map(|t| t.name).collect(),
            file_name: value.scene.files[0].basename.clone(),
            performers: value.scene.performers.into_iter().map(|p| p.name).collect(),
            primary_tag: value.primary_tag.name,
            scene_title: value.scene.title,
            screenshot_url: add_api_key(&value.screenshot, api_key),
            stream_url: add_api_key(&value.stream, api_key),
            index_in_scene,
            scene_interactive: value.scene.interactive,
            scene: EmbeddedScene {
                id: value.scene.id,
                scene_markers: value
                    .scene
                    .scene_markers
                    .into_iter()
                    .map(|m| SceneMarker {
                        start: m.seconds as u32,
                    })
                    .collect(),
                scene_streams: value
                    .scene
                    .scene_streams
                    .into_iter()
                    .map(|s| Stream {
                        label: s.label,
                        url: s.url,
                    })
                    .collect(),
            },
        }
    }
}

fn add_api_key(url: &str, api_key: &str) -> String {
    let mut url = Url::parse(url).expect("invalid url");
    url.query_pairs_mut().append_pair("apikey", api_key);
    url.to_string()
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScenePaths {
    pub screenshot: Option<String>,
    pub preview: Option<String>,
    pub sprite: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StashEmbeddedMarker {
    pub id: String,
    pub primary_tag: String,
    pub tags: Vec<String>,
    pub stream_url: String,
    pub screenshot_url: String,
    pub start: f64,
    pub end: Option<f64>,
}

impl From<FindScenesQueryFindScenesScenesSceneMarkers> for StashEmbeddedMarker {
    fn from(m: FindScenesQueryFindScenesScenesSceneMarkers) -> Self {
        StashEmbeddedMarker {
            id: m.id,
            primary_tag: m.primary_tag.name,
            tags: m.tags.into_iter().map(|m| m.name).collect(),
            stream_url: m.stream,
            screenshot_url: m.screenshot,
            start: m.seconds,
            scene_title: None,
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StashScene {
    pub id: String,
    pub title: String,
    pub rating: Option<i64>,
    pub interactive: bool,
    pub performers: Vec<String>,
    pub streams: Vec<Stream>,
    pub markers: Vec<StashEmbeddedMarker>,
    pub paths: ScenePaths,
    pub tags: Vec<String>,
    pub studio: Option<String>,
}

impl From<FindScenesQueryFindScenesScenes> for StashScene {
    fn from(s: FindScenesQueryFindScenesScenes) -> Self {
        StashScene {
            id: s.id,
            title: s
                .title
                .or(s.files.into_iter().map(|f| f.basename).next())
                .unwrap_or_default(),
            rating: s.rating100,
            interactive: s.interactive,
            performers: s.performers.into_iter().map(|p| p.name).collect(),
            streams: s
                .scene_streams
                .into_iter()
                .map(|s| Stream {
                    label: s.label,
                    url: s.url,
                })
                .collect(),
            paths: ScenePaths {
                screenshot: s.paths.screenshot,
                preview: s.paths.preview,
                sprite: s.paths.sprite,
            },
            markers: s.scene_markers.into_iter().map(From::from).collect(),
            tags: s.tags.into_iter().map(|t| t.name).collect(),
            studio: s.studio.map(|s| s.name),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StashTag {
    pub id: String,
    pub name: String,
    pub marker_count: i64,
}

impl From<FindTagsQueryFindTagsTags> for StashTag {
    fn from(value: FindTagsQueryFindTagsTags) -> Self {
        StashTag {
            id: value.id,
            name: value.name,
            marker_count: value.scene_marker_count.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FilterMode {
    Performers,
    Tags,
    Scenes,
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

    pub async fn find_scenes(&self) -> Result<Vec<StashScene>> {
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
        let scenes = response
            .data
            .unwrap()
            .find_scenes
            .scenes
            .into_iter()
            .map(From::from)
            .collect();

        Ok(scenes)
    }

    pub async fn find_scenes_by_ids(&self, ids: Vec<i64>) -> Result<Vec<StashScene>> {
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
            Some(scenes) => Ok(scenes
                .find_scenes
                .scenes
                .into_iter()
                .map(From::from)
                .collect()),
            None => Ok(vec![]),
        }
    }

    pub async fn find_tags(&self) -> Result<Vec<StashTag>> {
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
        let tags = response
            .data
            .unwrap()
            .find_tags
            .tags
            .into_iter()
            .map(From::from)
            .collect();

        Ok(tags)
    }

    pub async fn find_markers(
        &self,
        ids: Vec<String>,
        mode: FilterMode,
        include_all: bool,
    ) -> Result<Vec<StashStandaloneMarker>> {
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

                return Ok(scenes.into_iter().flat_map(|s| s.markers).collect());
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
        let markers = markers.find_scene_markers.scene_markers;
        Ok(markers
            .into_iter()
            .map(|m| StashStandaloneMarker::from_marker(m, &self.api_key))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_health() {}
}
