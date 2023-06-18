use camino::Utf8Path;
use tracing::{info, warn};

use super::directories::Directories;
use super::preview_image::PreviewGenerator;
use crate::data::database::Database;
use crate::service::commands::ffprobe;
use crate::Result;

pub async fn run(database: Database, directories: Directories) -> Result<()> {
    let migrator = Migrator::new(database, directories);
    migrator.run().await
}

pub struct Migrator {
    database: Database,
    directories: Directories,
}

impl Migrator {
    pub fn new(database: Database, directories: Directories) -> Self {
        Migrator {
            database,
            directories,
        }
    }

    async fn set_video_durations(&self) -> Result<()> {
        let videos = self.database.get_videos().await?;
        for video in videos {
            // initial value from migration
            if video.duration == -1.0 {
                info!("determining duration for video {}", video.file_path);
                if !Utf8Path::new(&video.file_path).exists() {
                    info!("video {} does not exist, skipping", video.file_path);
                } else if let Ok(ffprobe) = ffprobe(&video.file_path, &self.directories).await {
                    let duration = ffprobe.duration().unwrap_or_default();
                    self.database
                        .set_video_duration(&video.id, duration)
                        .await?;
                } else {
                    warn!("failed to determine duration for video {}", video.file_path);
                }
            }
        }

        Ok(())
    }

    async fn generate_video_preview_images(&self) -> Result<()> {
        let preview_generator = PreviewGenerator::new(self.directories.clone());
        let videos = self.database.get_videos().await?;
        for video in videos {
            if video.video_preview_image.is_none() {
                let preview_image = preview_generator
                    .generate_preview(&video.id, &video.file_path, video.duration / 2.0)
                    .await;
                match preview_image {
                    Ok(path) => {
                        self.database
                            .set_video_preview_image(&video.id, path.as_str())
                            .await?
                    }
                    Err(err) => warn!(
                        "failed to generate preview image for video {}: {:?}",
                        video.file_path, err
                    ),
                }
            }
        }

        Ok(())
    }

    async fn generate_marker_preview_images(&self) -> Result<()> {
        let preview_generator = PreviewGenerator::new(self.directories.clone());
        let markers = self.database.get_markers().await?;
        for marker in markers {
            if marker.marker_preview_image.is_none() {
                let preview_image = preview_generator
                    .generate_preview(&marker.video_id, &marker.file_path, marker.start_time)
                    .await;
                match preview_image {
                    Ok(path) => {
                        self.database
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

    pub async fn run(&self) -> Result<()> {
        info!("running migrations if necessary...");

        self.database
            .generate_all_beats(self.directories.clone())
            .await?;
        self.set_video_durations().await?;
        self.generate_video_preview_images().await?;
        self.generate_marker_preview_images().await?;

        Ok(())
    }
}
