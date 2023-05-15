use crate::{
    data::database::{CreateSong, Database, DbSong},
    service::ffprobe::ffprobe,
    Result,
};
use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::bail;
use nanoid::nanoid;
use tokio::fs;
use tracing::info;
use youtube_dl::YoutubeDl;

use super::directories::Directories;

const YT_DLP_EXECUTABLE: &str = if cfg!(target_os = "windows") {
    "yt-dlp.exe"
} else {
    "yt-dlp"
};

#[derive(Debug)]
pub struct SongInfo {
    pub path: Utf8PathBuf,
    pub duration: f64,
}

pub struct MusicService {
    db: Database,
    dirs: Directories,
}

impl MusicService {
    pub fn new(database: Database, directories: Directories) -> Self {
        Self {
            db: database,
            dirs: directories,
        }
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

    async fn download_to_file(&self, url: &str) -> Result<SongInfo> {
        let yt_dlp = self.ensure_yt_dlp().await?;

        let base_dir = self.dirs.music_dir();
        let song_id = nanoid!(8);
        let output_dir = base_dir.join(song_id);

        if !output_dir.is_dir() {
            fs::create_dir_all(base_dir).await?;
        }

        YoutubeDl::new(url)
            .youtube_dl_path(yt_dlp)
            .extract_audio(true)
            .download(true)
            .output_directory(output_dir.as_str())
            .run_async()
            .await?;

        let mut iterator = fs::read_dir(output_dir).await?;
        let entry = iterator.next_entry().await?;
        if let Some(entry) = entry {
            let path = Utf8PathBuf::from_path_buf(entry.path()).expect("path must be utf-8");
            info!("downloaded music to {path}");
            let info = ffprobe(&path).await?;
            let duration = info.format.duration().unwrap_or_default();

            Ok(SongInfo { path, duration })
        } else {
            bail!("could not find downloaded music file")
        }
    }

    pub async fn download_song(&self, url: &str) -> Result<DbSong> {
        let existing_song = self.db.get_song_by_url(url).await?;
        if let Some(mut song) = existing_song {
            if Utf8Path::new(&song.file_path).is_file() {
                Ok(song)
            } else {
                let downloaded_song = self.download_to_file(url).await?;
                self.db
                    .update_song_file_path(
                        song.rowid.expect("must have rowid"),
                        downloaded_song.path.as_str(),
                    )
                    .await?;
                song.file_path = downloaded_song.path.to_string();
                Ok(song)
            }
        } else {
            let downloaded_song = self.download_to_file(url).await?;
            let result = self
                .db
                .persist_song(CreateSong {
                    duration: downloaded_song.duration,
                    file_path: downloaded_song.path.to_string(),
                    url: url.to_string(),
                })
                .await?;
            Ok(result)
        }
    }
}

#[cfg(test)]
mod test {
    use camino::Utf8Path;
    use sqlx::SqlitePool;
    use std::fs;

    use crate::{
        data::database::Database,
        service::{directories::Directories, music::MusicService},
    };

    #[sqlx::test]
    async fn test_download_song(pool: SqlitePool) {
        let database = Database::with_pool(pool);
        let directories = Directories::new().unwrap();
        let service = MusicService::new(database.clone(), directories);
        let _ = color_eyre::install();
        let url = "https://www.youtube.com/watch?v=DGaKVLFNWzs";
        let info = service.download_song(url).await.unwrap();
        let path = Utf8Path::new(&info.file_path);
        assert!(path.is_file());

        let from_database = database.get_song_by_url(url).await.unwrap().unwrap();
        assert_eq!(url, from_database.url);

        fs::remove_file(&from_database.file_path).unwrap();

        let info = service.download_song(url).await.unwrap();
        assert_ne!(from_database.file_path, info.file_path);
        let path = Utf8Path::new(&info.file_path);
        assert!(path.is_file());

        let from_database = database.get_song_by_url(url).await.unwrap().unwrap();
        assert_eq!(from_database.file_path, info.file_path);
    }
}
