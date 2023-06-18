use camino::Utf8Path;
use tracing::{info, warn};

use super::preview_image::generate_preview_image;
use crate::data::database::Database;
use crate::service::commands::ffprobe;
use crate::Result;

pub async fn run(database: &Database) -> Result<()> {
    info!("running migrations if necessary...");

    database.generate_all_beats().await?;
    set_video_durations(database).await?;
    generate_video_preview_images(database).await?;
    generate_marker_preview_images(database).await?;

    Ok(())
}

async fn set_video_durations(database: &Database) -> Result<()> {
    let videos = database.get_videos().await?;
    for video in videos {
        // initial value from migration
        if video.duration == -1.0 {
            info!("determining duration for video {}", video.file_path);
            if !Utf8Path::new(&video.file_path).exists() {
                info!("video {} does not exist, skipping", video.file_path);
            } else if let Ok(ffprobe) = ffprobe(&video.file_path).await {
                let duration = ffprobe.duration().unwrap_or_default();
                database.set_video_duration(&video.id, duration).await?;
            } else {
                warn!("failed to determine duration for video {}", video.file_path);
            }
        }
    }

    Ok(())
}

async fn generate_video_preview_images(database: &Database) -> Result<()> {
    let videos = database.get_videos().await?;
    for video in videos {
        if video.video_preview_image.is_none() {
            let preview_image =
                generate_preview_image(&video.file_path, video.duration / 2.0).await;
            match preview_image {
                Ok(path) => {
                    database
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

async fn generate_marker_preview_images(database: &Database) -> Result<()> {
    let markers = database.get_markers().await?;
    for marker in markers {
        if marker.marker_preview_image.is_none() {
            let preview_image = generate_preview_image(&marker.file_path, marker.start_time).await;
            match preview_image {
                Ok(path) => {
                    database
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
