use reqwest::Client;

use crate::data::database::Database;
use crate::service::commands::ffmpeg::FfmpegLocation;
use crate::service::directories::Directories;
use crate::service::generator::CompilationGenerator;

pub mod files;
pub mod library;
pub mod music;
pub mod progress;
pub mod project;
pub mod stash;
pub mod version;
pub mod updater;

pub struct AppState {
    pub generator: CompilationGenerator,
    pub database: Database,
    pub directories: Directories,
    pub ffmpeg_location: FfmpegLocation,
    pub reqwest: Client,
}
