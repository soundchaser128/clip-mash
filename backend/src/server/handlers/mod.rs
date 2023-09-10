use std::collections::{HashMap, HashSet};

use reqwest::Url;

use crate::data::database::Database;
use crate::server::types::VideoId;
use crate::service::commands::ffmpeg::FfmpegLocation;
use crate::service::directories::Directories;
use crate::service::generator::CompilationGenerator;
use crate::service::stash_config::Config;

pub mod library;
pub mod music;
pub mod progress;
pub mod project;
pub mod version;

pub struct AppState {
    pub generator: CompilationGenerator,
    pub database: Database,
    pub directories: Directories,
    pub ffmpeg_location: FfmpegLocation,
}

pub fn get_streams(
    video_ids: HashSet<VideoId>,
    config: &Config,
) -> crate::Result<HashMap<String, String>> {
    let mut urls = HashMap::new();

    for id in video_ids {
        match id {
            VideoId::LocalFile(_) => {
                let url = format!("/api/local/video/{id}/file");
                urls.insert(id.to_string(), url);
            }
            VideoId::Stash(_) => {
                let mut url = Url::parse(&config.stash_url)?;
                url.set_path(&format!("/scene/{id}/stream"));
                url.query_pairs_mut().append_pair("apikey", &config.api_key);
                urls.insert(id.to_string(), url.to_string());
            }
        }
    }

    Ok(urls)
}
