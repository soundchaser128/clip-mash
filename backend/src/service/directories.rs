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
    TempVideo,
    CompilationVideo,
    DownloadedVideo,
    Music,
    Database,
    Config,
}

trait DirectorySupplier {
    fn cache_dir(&self) -> &Utf8Path;
    fn config_dir(&self) -> &Utf8Path;
    fn data_dir(&self) -> &Utf8Path;
}

impl DirectorySupplier for ProjectDirs {
    fn cache_dir(&self) -> &Utf8Path {
        Utf8Path::from_path(self.cache_dir()).expect("path must be utf-8")
    }

    fn config_dir(&self) -> &Utf8Path {
        Utf8Path::from_path(self.config_dir()).expect("path must be utf-8")
    }

    fn data_dir(&self) -> &Utf8Path {
        Utf8Path::from_path(self.data_dir()).expect("path must be utf-8")
    }
}

struct EnvDirectorySupplier {
    cache_dir: Utf8PathBuf,
    config_dir: Utf8PathBuf,
    data_dir: Utf8PathBuf,
}

impl EnvDirectorySupplier {
    pub fn new(base_dir: String) -> Self {
        let base_dir = Utf8PathBuf::from(base_dir);

        Self {
            cache_dir: base_dir.join("cache"),
            config_dir: base_dir.join("config"),
            data_dir: base_dir.join("data"),
        }
    }
}

impl DirectorySupplier for EnvDirectorySupplier {
    fn cache_dir(&self) -> &Utf8Path {
        &self.cache_dir
    }

    fn config_dir(&self) -> &Utf8Path {
        &self.config_dir
    }

    fn data_dir(&self) -> &Utf8Path {
        &self.data_dir
    }
}

#[derive(Clone)]
pub struct Directories {
    dirs: Arc<Box<dyn DirectorySupplier + 'static + Send + Sync>>,
}

const ENV_VAR: &'static str = "CLIP_MASH_BASE_DIR";

impl Directories {
    pub fn new() -> Result<Self> {
        let dirs: Box<dyn DirectorySupplier + Send + Sync> = match std::env::var(ENV_VAR) {
            Ok(value) => {
                info!(
                    "using base directory from environment variable {}: {}",
                    ENV_VAR, value
                );
                Box::new(EnvDirectorySupplier::new(value))
            }
            Err(_) => Box::new(
                ProjectDirs::from("xyz", "soundchaser128", "stash-compilation-maker")
                    .expect("could not determine config path"),
            ),
        };

        for directory in &[dirs.config_dir(), dirs.cache_dir(), dirs.data_dir()] {
            fs::create_dir_all(directory)?;
        }

        let dirs = Directories {
            dirs: Arc::new(dirs),
        };

        for directory in &[
            dirs.preview_image_dir(),
            dirs.music_dir(),
            dirs.compilation_video_dir(),
            dirs.temp_video_dir(),
            dirs.downloaded_video_dir(),
        ] {
            fs::create_dir_all(directory)?;
        }

        dirs.info();

        Ok(dirs)
    }

    pub fn get(&self, ty: FolderType) -> Utf8PathBuf {
        match ty {
            FolderType::TempVideo => self.temp_video_dir(),
            FolderType::CompilationVideo => self.compilation_video_dir(),
            FolderType::Music => self.music_dir(),
            FolderType::Database => self.database_file(),
            FolderType::Config => self.config_dir().to_owned(),
            FolderType::DownloadedVideo => self.downloaded_video_dir(),
        }
    }

    pub fn config_dir(&self) -> &Utf8Path {
        self.dirs.config_dir()
    }

    pub fn cache_dir(&self) -> &Utf8Path {
        self.dirs.cache_dir()
    }

    pub fn data_dir(&self) -> &Utf8Path {
        self.dirs.data_dir()
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

    pub fn temp_video_dir(&self) -> Utf8PathBuf {
        self.cache_dir().join("videos").join("clips")
    }

    pub fn compilation_video_dir(&self) -> Utf8PathBuf {
        self.cache_dir().join("videos").join("finished")
    }

    pub fn database_file(&self) -> Utf8PathBuf {
        self.data_dir().join("videos.sqlite3")
    }

    pub fn downloaded_video_dir(&self) -> Utf8PathBuf {
        self.cache_dir().join("videos").join("downloaded")
    }

    pub fn info(&self) {
        info!("config directory: {}", self.config_dir());
        info!("cache directory: {}", self.cache_dir());
        info!("music directory: {}", self.music_dir());
        info!(
            "compilation video directory: {}",
            self.compilation_video_dir()
        );
        info!("temporary video directory: {}", self.temp_video_dir());
        info!("database file: {}", self.database_file());
    }

    #[allow(unused)]
    pub async fn cleanup_videos(&self) -> Result<()> {
        use tokio::fs;

        let mut stream = fs::read_dir(self.temp_video_dir()).await?;
        while let Some(file) = stream.next_entry().await? {
            let path = Utf8PathBuf::from_path_buf(file.path()).expect("path must be utf-8");
            if let Some("mp4") = path.extension() {
                fs::remove_file(path).await?;
            }
        }

        Ok(())
    }
}
