use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

use crate::Result;

const BASE_URL: &str = "https://alexandria.soundchaser128.xyz";
pub const CONTENT_URL: &str = "https://content-next.soundchaser128.xyz";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AlexandriaVideoPage {
    pub content: Vec<AlexandriaVideo>,
    pub empty: bool,
    pub first: bool,
    pub last: bool,
    pub number: i64,
    pub number_of_elements: i64,
    pub size: i64,
    pub total_elements: i64,
    pub total_pages: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AlexandriaVideo {
    pub created_on: String,
    pub file_size: i64,
    pub friendly_id: String,
    pub height: i64,
    pub id: String,
    pub mime_type: String,
    pub tags: Vec<String>,
    pub title: String,
    pub url: String,
    pub width: i64,
}

pub struct AlexandriaApi {
    client: Client,
}

impl AlexandriaApi {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn fetch_videos(
        &self,
        query: Option<&str>,
        page: i64,
        per_page: i64,
    ) -> Result<AlexandriaVideoPage> {
        let page = page.to_string();
        let size = per_page.to_string();

        let mut url = Url::parse(BASE_URL).unwrap();
        url.set_path("/api/file");
        url.query_pairs_mut()
            .append_pair("page", &page)
            .append_pair("size", &size)
            .append_pair("fileType", "video");
        if let Some(query) = query {
            url.query_pairs_mut().append_pair("query", query);
        }

        let response = self.client.get(url).send().await?;
        let videos = response.json().await?;
        Ok(videos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_videos() {
        let client = Client::new();
        let api = AlexandriaApi::new(client);

        let videos = api.fetch_videos(None, 0, 24).await.unwrap();
        assert_eq!(videos.content.len(), 24);
    }
}
