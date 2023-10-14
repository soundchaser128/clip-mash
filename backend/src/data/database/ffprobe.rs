use sqlx::SqlitePool;
use tracing::info;

use crate::service::commands::ffprobe::FfProbe;
use crate::Result;

#[derive(Debug, Clone)]
pub struct FfProbeInfoDatabase {
    pool: SqlitePool,
}

pub struct VideoWithFilePath {
    pub id: String,
    pub file_path: String,
}

impl FfProbeInfoDatabase {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_info(&self, video_id: impl AsRef<str>) -> Result<FfProbe> {
        let video_id = video_id.as_ref();
        let info = sqlx::query_scalar!(
            "SELECT info FROM ffprobe_info WHERE video_id = $1",
            video_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(serde_json::from_str(&info)?)
    }

    pub async fn get_infos(&self, video_ids: &[&str]) -> Result<Vec<FfProbe>> {
        let mut infos = vec![];
        for video_id in video_ids {
            let info = self.get_info(&video_id).await?;
            infos.push(info);
        }
        Ok(infos)
    }

    pub async fn set_info(&self, video_id: &str, info: &FfProbe) -> Result<()> {
        info!("setting ffprobe info for video {video_id}");
        let info = serde_json::to_string(info)?;

        sqlx::query!(
            "INSERT INTO ffprobe_info (video_id, info) 
             VALUES ($1, $2)
             ON CONFLICT (video_id) DO UPDATE SET info = $2",
            video_id,
            info,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_videos_without_info(&self) -> Result<Vec<VideoWithFilePath>> {
        let videos =
            sqlx::query_as!(VideoWithFilePath,
            "SELECT id, file_path FROM videos WHERE id NOT IN (SELECT video_id FROM ffprobe_info)"
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(videos)
    }
}
