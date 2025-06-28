use crate::Result;
use clip_mash::data::database::Database;
use clip_mash::data::stash_api::StashApi;
use clip_mash::service::commands::ffmpeg::FfmpegLocation;
use clip_mash::service::directories::Directories;
use clip_mash::service::new_version_checker::NewVersionChecker;
use clip_mash::service::stash_config::StashConfig;

pub mod files;
pub mod handy;
pub mod library;
pub mod music;
pub mod progress;
pub mod project;
pub mod stash;
pub mod system;

pub struct AppState {
    pub database: Database,
    pub directories: Directories,
    pub ffmpeg_location: FfmpegLocation,
    pub new_version_checker: NewVersionChecker,
}

impl AppState {
    /// Fetch the stash configuration from the database. If the configuration is not set,
    /// a default (empty) configuration is returned.
    pub async fn stash_config(&self) -> Result<StashConfig> {
        let settings = self.database.settings.fetch().await?;
        Ok(settings.stash)
    }

    pub async fn stash_api(&self) -> Result<StashApi> {
        let stash_config = self.stash_config().await?;
        Ok(StashApi::with_config(stash_config))
    }
}
