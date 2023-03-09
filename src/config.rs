use std::{collections::HashMap, error::Error};

use crate::Result;
use camino::{Utf8Path, Utf8PathBuf};
use dialoguer::Input;
use directories::ProjectDirs;
use tinyjson::JsonValue;

#[derive(Debug, Clone)]
pub struct Config {
    pub stash_url: String,
    pub api_key: String,
}

fn error(e: &str) -> Box<dyn Error> {
    e.into()
}

fn string(value: &JsonValue) -> Result<String> {
    match value {
        JsonValue::String(str) => Ok(str.into()),
        _ => Err(format!("not a string: {:?}", value).into()),
    }
}

impl TryFrom<JsonValue> for Config {
    type Error = Box<dyn Error>;

    fn try_from(value: JsonValue) -> Result<Self> {
        let object: &HashMap<_, _> = value.get().ok_or(error("invalid json value"))?;
        let stash_url = object
            .get("stash_url")
            .ok_or(error("missing `stash_url`"))?;
        let api_key = object.get("api_key").ok_or(error("missing `api_key`"))?;
        Ok(Config {
            stash_url: string(stash_url)?,
            api_key: string(api_key)?,
        })
    }
}

impl From<Config> for JsonValue {
    fn from(value: Config) -> Self {
        let mut map = HashMap::new();
        map.insert("stash_url".into(), value.stash_url.into());
        map.insert("api_key".into(), value.api_key.into());
        map.into()
    }
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
        let config_json: JsonValue = config.clone().into();
        let config_json = config_json.format()?;

        std::fs::create_dir_all(config_folder)?;
        std::fs::write(config_file, config_json)?;

        config
    } else {
        let text = std::fs::read_to_string(&config_file)?;
        let json: JsonValue = text.parse()?;
        let config: Config = json.try_into()?;

        config
    };

    Ok(config)
}
