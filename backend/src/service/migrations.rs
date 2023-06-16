use tracing::info;

use crate::data::database::Database;
use crate::service::commands::ffprobe;
use crate::Result;

pub async fn run(database: &Database) -> Result<()> {
    info!("running migrations if necessary...");

    database.generate_all_beats().await?;
    set_video_durations(database).await?;

    Ok(())
}

async fn set_video_durations(database: &Database) -> Result<()> {
    let videos = database.get_videos().await?;
    for video in videos {
        // initial value from migration
        if video.duration == -1.0 {
            info!("determining duration for video {}", video.file_path);
            let ffprobe = ffprobe(&video.file_path).await?;
            let duration = ffprobe.duration();
            database.set_video_duration(&video.id, duration).await?;
        }
    }

    Ok(())
}
