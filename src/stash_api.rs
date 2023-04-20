use self::{
    find_markers_query::{
        CriterionModifier, FindFilterType, FindMarkersQueryFindSceneMarkersSceneMarkers,
        HierarchicalMultiCriterionInput, MultiCriterionInput, SceneMarkerFilterType,
    },
    find_performers_query::FindPerformersQueryFindPerformersPerformers as Performer,
    find_scenes_query::FindScenesQueryFindScenesScenes,
    find_tags_query::FindTagsQueryFindTagsTags as Tag,
    healt_check_query::SystemStatusEnum,
};
use crate::{
    config::Config,
    funscript::FunScript,
    http::{add_api_key, FilterMode},
    Result,
};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use serde::{Deserialize, Serialize};

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
    response_derives = "Debug, Clone, Serialize"
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
    response_derives = "Debug"
)]
pub struct FindScenesQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/health_check.graphql",
    response_derives = "Debug"
)]
pub struct HealtCheckQuery;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub scene: Scene,
    pub index_in_scene: usize,
    pub scene_interactive: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    pub id: String,
    pub scene_markers: Vec<SceneMarker>,
    pub scene_streams: Vec<Stream>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub url: String,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SceneMarker {
    pub start: u32,
}

impl Marker {
    pub fn from_marker(value: FindMarkersQueryFindSceneMarkersSceneMarkers, api_key: &str) -> Self {
        let end = value
            .scene
            .scene_markers
            .iter()
            .find(|m| m.seconds > value.seconds)
            .map(|m| m.seconds as u32);

        let index_in_scene = value
            .scene
            .scene_markers
            .iter()
            .position(|m| m.id == value.id)
            .expect("marker must exist within its own scene");

        Marker {
            id: value.id,
            start: value.seconds as u32,
            end,
            file_name: value.scene.files[0].basename.clone(),
            performers: value.scene.performers.into_iter().map(|p| p.name).collect(),
            primary_tag: value.primary_tag.name,
            scene_title: value.scene.title,
            screenshot_url: add_api_key(&value.screenshot, api_key),
            stream_url: add_api_key(&value.stream, api_key),
            index_in_scene,
            scene_interactive: value.scene.interactive,
            scene: Scene {
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

    fn from_scene(scene: FindScenesQueryFindScenesScenes, api_key: &str) -> Vec<Marker> {
        let mut markers = vec![];

        for (index_in_scene, marker) in scene.scene_markers.iter().enumerate() {
            let end = scene
                .scene_markers
                .iter()
                .find(|m| m.seconds > marker.seconds)
                .map(|m| m.seconds as u32);
            markers.push(Marker {
                id: marker.id.clone(),
                primary_tag: marker.primary_tag.name.clone(),
                stream_url: add_api_key(&marker.stream, api_key),
                screenshot_url: add_api_key(&marker.screenshot, api_key),
                start: marker.seconds as u32,
                end,
                index_in_scene,
                file_name: scene.files[0].basename.clone(),
                scene_title: scene.title.clone(),
                performers: scene.performers.iter().map(|p| p.name.clone()).collect(),
                scene_interactive: scene.interactive,
                scene: Scene {
                    id: scene.id.clone(),
                    scene_markers: scene
                        .scene_markers
                        .iter()
                        .map(|m| SceneMarker {
                            start: m.seconds as u32,
                        })
                        .collect(),
                    scene_streams: scene
                        .scene_streams
                        .iter()
                        .map(|s| Stream {
                            label: s.label.clone(),
                            url: s.url.clone(),
                        })
                        .collect(),
                },
            })
        }
        markers
    }
}

pub struct Api {
    api_url: String,
    api_key: String,
    client: Client,
}

impl Api {
    pub fn new(url: &str, api_key: &str) -> Self {
        Api {
            api_url: url.into(),
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    pub fn from_config(config: &Config) -> Self {
        Self::new(&config.stash_url, &config.api_key)
    }

    pub async fn load_config() -> Result<Self> {
        let config = Config::get().await?;
        Ok(Self::new(&config.stash_url, &config.api_key))
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

    pub async fn find_tags(&self) -> Result<Vec<Tag>> {
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
    ) -> Result<Vec<Marker>> {
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
                    .flat_map(|s| Marker::from_scene(s, &self.api_key))
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
        let markers = markers.find_scene_markers.scene_markers;
        Ok(markers
            .into_iter()
            .map(|m| Marker::from_marker(m, &self.api_key))
            .collect())
    }

    pub async fn find_performers(&self) -> Result<Vec<Performer>> {
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

    pub async fn health(&self) -> Result<SystemStatusEnum> {
        let variables = healt_check_query::Variables {};
        let request_body = HealtCheckQuery::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        let response: Response<healt_check_query::ResponseData> = response.json().await?;
        let status = response.data.unwrap().system_status.status;
        Ok(status)
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
}
