use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;
use tracing::info;

#[derive(Clone)]
pub struct Directories {
    dirs: Arc<ProjectDirs>,
}

impl Directories {
    pub fn new() -> Self {
        let dirs = ProjectDirs::from("xyz", "soundchaser128", "stash-compilation-maker")
            .expect("could not determine config path");
        Directories {
            dirs: Arc::new(dirs),
        }
    }

    pub fn config_dir(&self) -> &Utf8Path {
        Utf8Path::from_path(self.dirs.config_dir()).expect("path must be utf-8")
    }

    pub fn cache_dir(&self) -> &Utf8Path {
        Utf8Path::from_path(self.dirs.cache_dir()).expect("path must be utf-8")
    }

    pub fn data_dir(&self) -> &Utf8Path {
        Utf8Path::from_path(self.dirs.data_dir()).expect("path must be utf-8")
    }

    pub fn config_file_path(&self) -> Utf8PathBuf {
        self.config_dir().join("config.json")
    }

    pub fn music_dir(&self) -> Utf8PathBuf {
        self.cache_dir().join("music")
    }

    pub fn video_dir(&self) -> Utf8PathBuf {
        self.cache_dir().join("videos")
    }

    pub fn database_file(&self) -> Utf8PathBuf {
        self.data_dir().join("videos.sqlite3")
    }

    pub fn info(&self) {
        info!("config directory: {}", self.config_dir());
        info!("cache directory: {}", self.cache_dir());
        info!("music directory: {}", self.music_dir());
        info!("video directory: {}", self.video_dir());
        info!("database file: {}", self.database_file());
    }
}
