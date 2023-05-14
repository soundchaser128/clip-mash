use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;

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

    pub fn config_file_path(&self) -> Utf8PathBuf {
        self.config_dir().join("config.json")
    }

    pub fn music_dir(&self) -> Utf8PathBuf {
        self.cache_dir().join("music")
    }

    pub fn video_dir(&self) -> Utf8PathBuf {
        self.cache_dir().join("videos")
    }
}
