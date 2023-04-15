use crate::Result;
use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::eyre;
use directories::ProjectDirs;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

lazy_static! {
    static ref CONFIG: Mutex<Option<Config>> = Default::default();
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub stash_url: String,
    pub api_key: String,
}

impl Config {
    fn load() -> Result<Self> {
        use std::fs;

        let config_file = config_file_path().expect("no configuration file path found");
        tracing::info!("trying to load config file from {}", config_file);

        let text = fs::read_to_string(&config_file)?;
        let config = serde_json::from_str(&text)?;
        Ok(config)
    }

    pub async fn get() -> Result<Config> {
        let config = CONFIG.lock().await;

        config.as_ref().map(|c| c.clone()).ok_or_else(|| {
            eyre!("No configuration set up yet. Please enter your data in the web UI")
        })
    }
}

fn config_file_path() -> Option<Utf8PathBuf> {
    let dirs = ProjectDirs::from("xyz", "soundchaser128", "stash-compilation-maker")?;
    Utf8Path::from_path(dirs.config_dir()).map(|p| p.join("config.json"))
}

pub async fn init() {
    match Config::load() {
        Ok(config) => {
            let mut global = CONFIG.lock().await;
            global.replace(config);
        }
        Err(e) => {
            tracing::info!("no configuration found, or unable to load: {e}")
        }
    }
}

pub async fn set_config(config: Config) -> Result<()> {
    use tokio::fs;

    let file_content = serde_json::to_string_pretty(&config)?;
    let mut global = CONFIG.lock().await;
    global.replace(config);

    let file = config_file_path().expect("no configuration file path found");
    fs::create_dir_all(file.parent().expect("config directory must have a parent")).await?;
    fs::write(&file, &file_content).await?;

    tracing::info!("wrote configuration to {}", file);

    Ok(())
}
