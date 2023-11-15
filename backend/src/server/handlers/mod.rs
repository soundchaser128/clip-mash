use crate::data::database::Database;
use crate::service::commands::ffmpeg::FfmpegLocation;
use crate::service::directories::Directories;
use crate::service::generator::CompilationGenerator;
use crate::service::new_version_checker::NewVersionChecker;

pub mod files;
pub mod library;
pub mod music;
pub mod progress;
pub mod project;
pub mod stash;
pub mod version;

pub struct AppState {
    pub generator: CompilationGenerator,
    pub database: Database,
    pub directories: Directories,
    pub ffmpeg_location: FfmpegLocation,
    pub new_version_checker: NewVersionChecker,
}
