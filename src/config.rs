use crate::Result;
use camino::{Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub stash_url: String,
    pub api_key: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        use std::fs;
        
        let config_folder = config_file_folder().expect("no configuration file path found");
        let config_file = config_folder.join("config.json");
        tracing::info!("trying to load config file from {}", config_file);
    
        let text = fs::read_to_string(&config_file)?;
        let config = serde_json::from_str(&text)?;
        Ok(config)
    }
}

fn config_file_folder() -> Option<Utf8PathBuf> {
    let dirs = ProjectDirs::from("xyz", "soundchaser128", "stash-compilation-maker")?;
    Utf8Path::from_path(dirs.config_dir()).map(|p| p.to_owned())
}
