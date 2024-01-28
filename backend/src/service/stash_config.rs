use color_eyre::eyre::eyre;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::info;
use utoipa::ToSchema;

use super::directories::Directories;
use crate::Result;

lazy_static! {
    static ref CONFIG: Mutex<Option<StashConfig>> = Default::default();
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StashConfig {
    pub stash_url: String,
    pub api_key: Option<String>,
}

impl StashConfig {
    fn load(directories: &Directories) -> Result<Self> {
        use std::fs;

        let config_file = directories.config_file_path();
        info!("trying to load config file from {}", config_file);

        let text = fs::read_to_string(&config_file)?;
        let config = serde_json::from_str(&text)?;
        Ok(config)
    }

    #[deprecated]
    pub async fn get() -> Result<StashConfig> {
        let config = CONFIG.lock().await;

        config.as_ref().cloned().ok_or_else(|| {
            eyre!("No configuration set up yet. Please enter your data in the web UI")
        })
    }

    #[deprecated]
    pub async fn get_or_empty() -> StashConfig {
        let config = CONFIG.lock().await;
        let config = config.as_ref().cloned();
        match config {
            Some(c) => c,
            None => StashConfig {
                api_key: Default::default(),
                stash_url: Default::default(),
            },
        }
    }
}

#[deprecated]
pub async fn init(directories: &Directories) {
    match StashConfig::load(directories) {
        Ok(config) => {
            let mut global = CONFIG.lock().await;
            global.replace(config);
        }
        Err(e) => {
            info!("no configuration found, or unable to load: {e}")
        }
    }
}

#[deprecated]
pub async fn set_config(config: StashConfig, directories: &Directories) -> Result<()> {
    use tokio::fs;

    let file_content = serde_json::to_string_pretty(&config)?;
    let mut global = CONFIG.lock().await;
    global.replace(config);

    let file = directories.config_file_path();
    fs::create_dir_all(file.parent().expect("config directory must have a parent")).await?;
    fs::write(&file, &file_content).await?;

    info!("wrote configuration to {}", file);

    Ok(())
}
