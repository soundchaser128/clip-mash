use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::ToSchema;

use crate::Result;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum FolderType {
    TempVideo,
    CompilationVideo,
    DownloadedVideo,
    Music,
    Database,
    Config,
    PreviewImages,
}

impl FolderType {
    pub fn can_cleanup(&self) -> bool {
        matches!(self, FolderType::TempVideo | FolderType::CompilationVideo)
    }
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

const ENV_VAR: &str = "CLIP_MASH_BASE_DIR";

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
            FolderType::PreviewImages => self.preview_image_dir(),
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

    pub async fn stats(&self) -> Result<Vec<(FolderType, u64)>> {
        use self::FolderType::*;

        let mut map = HashMap::new();
        let folder_types = [
            TempVideo,
            CompilationVideo,
            DownloadedVideo,
            Music,
            PreviewImages,
        ];

        for ty in folder_types {
            let path = self.get(ty);
            let folder_size: u64 = tokio::task::spawn_blocking(move || {
                let mut size = 0;
                for entry in walkdir::WalkDir::new(&path) {
                    let entry = entry?;
                    if entry.file_type().is_file() {
                        let metadata = entry.metadata()?;
                        size += metadata.len();
                    }
                }

                Ok::<u64, color_eyre::Report>(size)
            })
            .await??;

            map.insert(ty, folder_size);
        }
        let mut tuples: Vec<_> = map.into_iter().collect();
        tuples.sort_by_key(|(_, size)| std::cmp::Reverse(*size));

        Ok(tuples)
    }

    pub async fn cleanup(&self, folder_type: FolderType) -> Result<()> {
        if !folder_type.can_cleanup() {
            info!("cannot cleanup folder type {:?}", folder_type);
            return Ok(());
        }

        let path = self.get(folder_type);
        info!("cleaning up folder {:?}", path);
        let mut stream = tokio::fs::read_dir(&path).await?;
        while let Some(entry) = stream.next_entry().await? {
            let path = Utf8PathBuf::from_path_buf(entry.path()).expect("must be utf-8 path");
            if path.is_file() && path.extension() == Some("mp4") {
                info!("cleaning up file {:?}", path);
                tokio::fs::remove_file(path).await?;
            }
        }

        Ok(())
    }
}
