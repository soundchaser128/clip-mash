use std::fs;
use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;
use serde::Deserialize;
use tracing::info;

use crate::Result;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum FolderType {
    Videos,
    Music,
    Database,
    Config,
}

#[derive(Clone)]
pub struct Directories {
    dirs: Arc<ProjectDirs>,
}

impl Directories {
    pub fn new() -> Result<Self> {
        let dirs = ProjectDirs::from("xyz", "soundchaser128", "stash-compilation-maker")
            .expect("could not determine config path");

        for directory in &[dirs.config_dir(), dirs.cache_dir(), dirs.data_dir()] {
            fs::create_dir_all(directory)?;
        }
        let dirs = Directories {
            dirs: Arc::new(dirs),
        };

        for directory in &[dirs.preview_image_dir(), dirs.music_dir(), dirs.video_dir()] {
            fs::create_dir_all(directory)?;
        }

        dirs.info();

        Ok(dirs)
    }

    pub fn get(&self, ty: FolderType) -> Utf8PathBuf {
        match ty {
            FolderType::Videos => self.video_dir(),
            FolderType::Music => self.music_dir(),
            FolderType::Database => self.database_file(),
            FolderType::Config => self.config_dir().to_owned(),
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

    pub fn preview_image_dir(&self) -> Utf8PathBuf {
        self.cache_dir().join("preview-images")
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

    #[allow(unused)]
    pub async fn cleanup_videos(&self) -> Result<()> {
        use tokio::fs;

        let mut stream = fs::read_dir(self.video_dir()).await?;
        while let Some(file) = stream.next_entry().await? {
            let path = Utf8PathBuf::from_path_buf(file.path()).expect("path must be utf-8");
            if let Some("mp4") = path.extension() {
                fs::remove_file(path).await?;
            }
        }

        Ok(())
    }
}
