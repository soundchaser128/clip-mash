use std::sync::Arc;

use camino::Utf8PathBuf;
use color_eyre::eyre::bail;
use tokio::fs;
use tracing::info;
use url::Url;
use youtube_dl::YoutubeDl;

use super::ffmpeg::FfmpegLocation;
use crate::server::handlers::AppState;
use crate::service::directories::{Directories, FolderType};
use crate::util::generate_id;
use crate::Result;

const YT_DLP_EXECUTABLE: &str = if cfg!(target_os = "windows") {
    "yt-dlp.exe"
} else {
    "yt-dlp"
};

#[derive(Debug)]
pub struct YtDlpOptions {
    pub url: Url,
    pub extract_audio: bool,
    pub destination: FolderType,
}

pub struct DownloadResult {
    pub downloaded_file: Utf8PathBuf,
    pub generated_id: String,
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
    pub fn new(dirs: Directories) -> Self {
        Self { dirs }
    }

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

    pub async fn run(
        &self,
        options: &YtDlpOptions,
        ffmpeg_location: &FfmpegLocation,
    ) -> Result<DownloadResult> {
        let yt_dlp_path = self.ensure_yt_dlp().await?;
        let base_dir = self.dirs.get(options.destination);
        let id = generate_id();
        let dir = base_dir.join(&id);

        let mut youtube_dl = YoutubeDl::new(options.url.as_str());
        youtube_dl.youtube_dl_path(yt_dlp_path);

        if options.extract_audio {
            youtube_dl.extract_audio(true);
        }

        info!("using ffmpeg {:?}", ffmpeg_location);
        if let FfmpegLocation::Local(path) = ffmpeg_location {
            youtube_dl
                .extra_arg("--ffmpeg-location")
                .extra_arg(path.as_str());
        }

        youtube_dl.download_to_async(&dir).await?;

        let mut iterator = fs::read_dir(dir).await?;
        let entry = iterator.next_entry().await?;

        if let Some(entry) = entry {
            let path = Utf8PathBuf::from_path_buf(entry.path()).expect("path must be utf-8");
            info!("yt-dlp finished, path {path}");

            Ok(DownloadResult {
                downloaded_file: path,
                generated_id: id,
            })
        } else {
            bail!("could not find downloaded music file")
        }
    }
}
