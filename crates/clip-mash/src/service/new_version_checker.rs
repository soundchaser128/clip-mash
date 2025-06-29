use semver::Version;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::Mutex;
use tracing::{debug, info};
use utoipa::ToSchema;

use crate::Result;

const GITHUB_USER: &str = "soundchaser128";
const GITHUB_REPO_NAME: &str = "clip-mash";

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppVersion {
    pub newest_version: String,
    pub current_version: String,
    pub needs_update: bool,
}

pub struct NewVersionChecker {
    client: reqwest::Client,
    cached_version: Mutex<Option<AppVersion>>,
}

impl Default for NewVersionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl NewVersionChecker {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            cached_version: Mutex::new(None),
        }
    }

    async fn fetch_release(&self) -> Result<Value> {
        let url = format!(
            "https://api.github.com/repos/{GITHUB_USER}/{GITHUB_REPO_NAME}/releases/latest"
        );
        info!("sending request to {url}");
        let response = self
            .client
            .get(&url)
            .header("User-Agent", "clip-mash")
            .send()
            .await?
            .error_for_status()?;
        let release = response.json::<Value>().await?;
        Ok(release)
    }

    pub async fn check_for_updates(&self) -> Result<AppVersion> {
        let mut cached_version = self.cached_version.lock().await;
        if let Some(version) = &*cached_version {
            debug!("returning cached application version");
            return Ok(version.clone());
        }

        let release = self.fetch_release().await?;
        let name = release["tag_name"].as_str().unwrap();
        info!("latest release is {name}");
        // compare it to the current version
        let version = &name[1..];
        let version = Version::parse(version)?;
        let current_version = Version::parse(env!("CARGO_PKG_VERSION"))?;
        info!("current version is {current_version}, latest version is {version}");

        let version = AppVersion {
            newest_version: version.to_string(),
            current_version: current_version.to_string(),
            needs_update: version > current_version,
        };

        cached_version.replace(version.clone());

        Ok(version)
    }
}
