use crate::Result;
use camino::{Utf8Path, Utf8PathBuf};
use dialoguer::Input;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub stash_url: String,
    pub api_key: String,
}

fn config_file_folder() -> Option<Utf8PathBuf> {
    let dirs = ProjectDirs::from("xyz", "soundchaser128", "stash-compilation-maker")?;
    Utf8Path::from_path(dirs.cache_dir()).map(|p| p.to_owned())
}

pub fn setup_config() -> Result<Config> {
    let config_folder = config_file_folder().expect("no configuration file path found");
    let config_file = config_folder.join("config.json");
    tracing::info!("trying to load config file from {}", config_file);

    let config = if !config_file.is_file() {
        let mut url = Input::<String>::new()
            .with_prompt("Enter the URL of your Stash instance (e.g. http://localhost:9999)")
            .interact_text()?;

        if url.ends_with('/') {
            url.pop();
        }

        let api_key = Input::<String>::new()
            .with_prompt(format!(
                "Enter your Stash API key from {}/settings?tab=security",
                url
            ))
            .interact_text()?;

        let config = Config {
            api_key: api_key.trim().to_string(),
            stash_url: url.trim().to_string(),
        };
        let config_json = serde_json::to_string_pretty(&config)?;
        std::fs::create_dir_all(config_folder)?;
        std::fs::write(config_file, config_json)?;

        config
    } else {
        let text = std::fs::read_to_string(&config_file)?;
        serde_json::from_str(&text)?
    };

    Ok(config)
}
