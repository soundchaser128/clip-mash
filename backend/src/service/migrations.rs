use std::time::Instant;

use camino::Utf8Path;
use tracing::{info, warn};

use super::commands::ffmpeg::FfmpegLocation;
use super::directories::Directories;
use super::preview_image::PreviewGenerator;
use crate::data::database::{AllVideosFilter, Database, VideoUpdate};
use crate::service::commands::ffprobe;
use crate::Result;

pub fn run_async(database: Database, directories: Directories, ffmpeg_location: FfmpegLocation) {
    tokio::spawn(async move {
        let migrator = Migrator::new(database, directories, ffmpeg_location);
        if let Err(e) = migrator.run().await {
            tracing::error!("failed to run migrations: {e:?}")
        }
    });
}

pub struct Migrator {
    database: Database,
    directories: Directories,
    ffmpeg_location: FfmpegLocation,
}

impl Migrator {
    pub fn new(
        database: Database,
        directories: Directories,
        ffmpeg_location: FfmpegLocation,
    ) -> Self {
        Migrator {
            database,
            directories,
            ffmpeg_location,
        }
    }

    async fn set_video_durations(&self) -> Result<()> {
        info!("setting video durations if necessary");
        let videos = self
            .database
            .videos
            .get_videos(AllVideosFilter::NoVideoDuration)
            .await?;
        for video in videos {
            info!("determining duration for video {}", video.file_path);
            if !Utf8Path::new(&video.file_path).exists() {
                info!("video {} does not exist, skipping", video.file_path);
            } else if let Ok(ffprobe) = ffprobe(&video.file_path, &self.ffmpeg_location).await {
                let duration = ffprobe.duration().unwrap_or_default();
                self.database
                    .videos
                    .set_video_duration(&video.id, duration)
                    .await?;
            } else {
                warn!("failed to determine duration for video {}", video.file_path);
            }
        }

        Ok(())
    }

    async fn generate_video_preview_images(&self) -> Result<()> {
        info!("generating video preview images if necessary");
        let preview_generator =
            PreviewGenerator::new(self.directories.clone(), self.ffmpeg_location.clone());
        let videos = self
            .database
            .videos
            .get_videos(AllVideosFilter::NoPreviewImage)
            .await?;
        for video in videos {
            let preview_image = preview_generator
                .generate_preview(&video.id, &video.file_path, video.duration / 2.0)
                .await;
            match preview_image {
                Ok(path) => {
                    self.database
                        .videos
                        .set_video_preview_image(&video.id, path.as_str())
                        .await?
                }
                Err(err) => warn!(
                    "failed to generate preview image for video {}: {:?}",
                    video.file_path, err
                ),
            }
        }

        Ok(())
    }

    async fn generate_marker_preview_images(&self) -> Result<()> {
        info!("generating marker preview images if necessary");
        let preview_generator =
            PreviewGenerator::new(self.directories.clone(), self.ffmpeg_location.clone());
        let markers = self
            .database
            .markers
            .get_markers_without_preview_images()
            .await?;
        for marker in markers {
            if marker.marker_preview_image.is_none() {
                let preview_image = preview_generator
                    .generate_preview(&marker.video_id, &marker.file_path, marker.start_time)
                    .await;
                match preview_image {
                    Ok(path) => {
                        self.database
                            .markers
                            .set_marker_preview_image(marker.rowid.unwrap(), path.as_str())
                            .await?;
                    }
                    Err(err) => warn!(
                        "failed to generate preview image for marker {}: {:?}",
                        marker.file_path, err
                    ),
                }
            }
        }

        Ok(())
    }

    async fn initialize_video_titles(&self) -> Result<()> {
        info!("initializing video titles if necessary");
        let videos = self
            .database
            .videos
            .get_videos(AllVideosFilter::NoTitle)
            .await?;
        for video in videos {
            let title = Utf8Path::new(&video.file_path)
                .file_stem()
                .unwrap_or_default()
                .to_string();
            self.database
                .videos
                .update_video(&video.id, VideoUpdate { title: Some(title), tags: None })
                .await?;
        }

        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        info!("running migrations");
        let start = Instant::now();

        futures::try_join!(
            self.database
                .music
                .generate_all_beats(self.ffmpeg_location.clone()),
            self.set_video_durations(),
            self.generate_video_preview_images(),
            self.generate_marker_preview_images(),
            self.database.progress.cleanup_progress(),
            self.initialize_video_titles(),
        )?;

        let elapsed = start.elapsed();
        info!("running migrations took {elapsed:?}");

        Ok(())
    }
}
