use std::sync::Arc;

use axum::extract::multipart::Field;
use camino::{Utf8Path, Utf8PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::info;
use url::Url;

use crate::data::database::{CreateSong, Database, DbSong};
use crate::helpers::random::generate_id;
use crate::server::handlers::AppState;
use crate::service::commands::ffmpeg::FfmpegLocation;
use crate::service::commands::{ffprobe, YtDlp, YtDlpOptions};
use crate::service::directories::{Directories, FolderType};
use crate::service::music;
use crate::Result;

#[derive(Debug)]
pub struct SongInfo {
    pub path: Utf8PathBuf,
    pub duration: f64,
}

pub struct MusicDownloadService {
    db: Database,
    dirs: Directories,
    ffmpeg_location: FfmpegLocation,
}

impl From<Arc<AppState>> for MusicDownloadService {
    fn from(value: Arc<AppState>) -> Self {
        Self {
            db: value.database.clone(),
            dirs: value.directories.clone(),
            ffmpeg_location: value.ffmpeg_location.clone(),
        }
    }
}

impl MusicDownloadService {
    #[cfg(test)]
    pub fn new(database: Database, directories: Directories, ffmpeg: FfmpegLocation) -> Self {
        Self {
            db: database,
            dirs: directories,
            ffmpeg_location: ffmpeg,
        }
    }

    async fn get_download_directory(&self) -> Result<Utf8PathBuf> {
        let base_dir = self.dirs.music_dir();
        let song_id = generate_id();
        let output_dir = base_dir.join(song_id);

        if !output_dir.is_dir() {
            fs::create_dir_all(&output_dir).await?;
        }

        Ok(output_dir)
    }

    async fn download_to_file(&self, url: Url) -> Result<SongInfo> {
        let yt_dlp = YtDlp::new(self.dirs.clone());
        let options = YtDlpOptions {
            url,
            extract_audio: true,
            destination: FolderType::Music,
        };
        let result = yt_dlp.run(&options, &self.ffmpeg_location).await?;
        let ffprobe_result =
            ffprobe(result.downloaded_file.as_str(), &self.ffmpeg_location).await?;
        let duration = ffprobe_result.format.duration().unwrap_or_default();

        Ok(SongInfo {
            path: result.downloaded_file,
            duration,
        })
    }

    pub async fn download_song(&self, url: Url) -> Result<DbSong> {
        let existing_song = self.db.music.get_song_by_url(url.as_str()).await?;
        if let Some(mut song) = existing_song {
            if Utf8Path::new(&song.file_path).is_file() {
                Ok(song)
            } else {
                let downloaded_song = self.download_to_file(url.clone()).await?;
                self.db
                    .music
                    .update_song_file_path(
                        song.rowid.expect("must have rowid"),
                        downloaded_song.path.as_str(),
                    )
                    .await?;
                song.file_path = downloaded_song.path.to_string();
                Ok(song)
            }
        } else {
            let downloaded_song = self.download_to_file(url.clone()).await?;
            let beats = music::detect_beats(&downloaded_song.path, &self.ffmpeg_location).ok();
            let result = self
                .db
                .music
                .persist_song(CreateSong {
                    duration: downloaded_song.duration,
                    file_path: downloaded_song.path.to_string(),
                    url: url.to_string(),
                    beats,
                })
                .await?;
            Ok(result)
        }
    }

    pub async fn upload_song(&self, mut field: Field<'_>) -> Result<DbSong> {
        let file_name = field.file_name().expect("field must have a file name");
        let output_dir = self.get_download_directory().await?;
        let path = output_dir.join(file_name);
        info!("uploading song to {path}");
        let mut writer = fs::File::create(&path).await?;

        while let Some(chunk) = field.chunk().await? {
            writer.write_all(&chunk).await?;
        }

        let ffprobe_result = ffprobe(path.as_str(), &self.ffmpeg_location).await?;
        let beats = music::detect_beats(&path, &self.ffmpeg_location).ok();

        let result = self
            .db
            .music
            .persist_song(CreateSong {
                duration: ffprobe_result.format.duration().unwrap_or_default(),
                file_path: path.to_string(),
                url: format!("file:{path}"),
                beats,
            })
            .await?;
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use camino::Utf8Path;
    use sqlx::SqlitePool;
    use url::Url;

    use crate::data::database::Database;
    use crate::service::commands::ffmpeg::FfmpegLocation;
    use crate::service::directories::Directories;
    use crate::service::music::download::MusicDownloadService;

    #[ignore]
    #[sqlx::test]
    async fn test_download_song(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let directories = Directories::new().unwrap();
        let service =
            MusicDownloadService::new(database.clone(), directories, FfmpegLocation::System);
        let _ = color_eyre::install();
        let url: Url = "https://www.youtube.com/watch?v=DGaKVLFNWzs"
            .try_into()
            .unwrap();
        let info = service.download_song(url.clone()).await.unwrap();
        let path = Utf8Path::new(&info.file_path);
        assert!(path.is_file());

        let from_database = database
            .music
            .get_song_by_url(url.as_str())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(url.as_str(), from_database.url);

        fs::remove_file(&from_database.file_path).unwrap();

        let info = service.download_song(url.clone()).await.unwrap();
        assert_ne!(from_database.file_path, info.file_path);
        let path = Utf8Path::new(&info.file_path);
        assert!(path.is_file());

        let from_database = database
            .music
            .get_song_by_url(url.as_str())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(from_database.file_path, info.file_path);
    }
}
