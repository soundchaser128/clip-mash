use crate::{service::Marker, Result};
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
pub struct FindTagsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../graphql/schema.json",
    query_path = "../graphql/find_markers.graphql",
    response_derives = "Debug, Clone"
)]
pub struct FindMarkersQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../graphql/schema.json",
    query_path = "../graphql/find_performers.graphql",
    response_derives = "Debug"
)]
pub struct FindPerformersQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../graphql/schema.json",
    query_path = "../graphql/find_scenes.graphql",
    response_derives = "Debug, Clone"
)]
pub struct FindScenesQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../graphql/schema.json",
    query_path = "../graphql/health_check.graphql",
    response_derives = "Debug"
)]
struct HealthCheckQuery;

#[derive(Debug, Clone, Copy)]
pub enum FilterMode {
    Performers,
    Tags,
    Scenes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StashMarker {}

impl StashMarker {
    fn from_scene(s: FindScenesQueryFindScenesScenes, api_key: &str) -> Vec<Self> {
        todo!()
    }

    fn from_marker(m: FindMarkersQueryFindSceneMarkersSceneMarkers, api_key: &str) -> Self {
        todo!()
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_health() {}
}
