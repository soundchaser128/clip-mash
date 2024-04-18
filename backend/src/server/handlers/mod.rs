use crate::data::database::Database;
use crate::data::stash_api::StashApi;
use crate::service::commands::ffmpeg::FfmpegLocation;
use crate::service::directories::Directories;
use crate::service::generator::CompilationGenerator;
use crate::service::new_version_checker::NewVersionChecker;
use crate::service::stash_config::StashConfig;
use crate::Result;

pub mod files;
pub mod library;
pub mod music;
pub mod progress;
pub mod project;
pub mod stash;
pub mod system;

pub struct AppState {
    pub generator: CompilationGenerator,
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

    pub async fn stash_config_optional(&self) -> Result<Option<StashConfig>> {
        let settings = self.database.settings.fetch_optional().await?;
        Ok(settings.map(|s| s.stash))
    }

    pub async fn stash_api(&self) -> Result<StashApi> {
        let stash_config = self.stash_config().await?;
        Ok(StashApi::with_config(stash_config))
    }
}
