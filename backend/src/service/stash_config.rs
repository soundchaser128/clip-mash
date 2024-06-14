use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::info;
use utoipa::ToSchema;

use super::directories::Directories;
use crate::Result;

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct StashConfig {
    pub stash_url: String,
    pub api_key: Option<String>,
}

impl StashConfig {
    pub fn load(directories: &Directories) -> Result<Self> {
        use std::fs;

        let config_file = directories.config_file_path();
        info!("trying to load config file from {}", config_file);

        let text = fs::read_to_string(&config_file)?;
        let config = serde_json::from_str(&text)?;
        Ok(config)
    }

    pub fn is_empty(&self) -> bool {
        self.stash_url.is_empty()
    }
}
