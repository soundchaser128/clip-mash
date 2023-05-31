use std::sync::Arc;

use camino::Utf8PathBuf;
use tokio::fs;
use url::Url;

use crate::server::handlers::common::FolderType;
use crate::server::handlers::AppState;
use crate::service::directories::Directories;
use crate::Result;

const YT_DLP_EXECUTABLE: &str = if cfg!(target_os = "windows") {
    "yt-dlp.exe"
} else {
    "yt-dlp"
};

#[derive(Debug)]
pub struct Options {
    pub url: Url,
    pub extract_music: bool,
    pub destination: FolderType,
}

pub struct YtDlp {
    dirs: Directories,
}

impl From<Arc<AppState>> for YtDlp {
    fn from(value: Arc<AppState>) -> Self {
        Self {
            dirs: value.directories.clone(),
        }
    }
}

impl YtDlp {
    async fn ensure_yt_dlp(&self) -> Result<Utf8PathBuf> {
        let path = self.dirs.cache_dir();
        if !path.is_dir() {
            fs::create_dir_all(path).await?;
        }

        let executable = path.join(YT_DLP_EXECUTABLE);
        if !executable.is_file() {
            youtube_dl::download_yt_dlp(path).await?;
        }
        Ok(executable)
    }

    pub async fn run(&self, options: &Options) -> Result<Utf8PathBuf> {
        let path = self.ensure_yt_dlp().await?;
        todo!()
    }
}
