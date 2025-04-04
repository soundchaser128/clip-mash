use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::info;

use crate::Result;

#[derive(Debug, PartialEq, Eq)]
pub struct VideoParameters {
    pub fps: String,
    pub width: i64,
    pub height: i64,
    pub codec: String,
}

#[derive(Deserialize, Serialize)]
pub struct FfProbe {
    pub streams: Vec<Stream>,
    pub format: Format,
}

impl FfProbe {
    pub fn duration(&self) -> Option<f64> {
        self.format
            .duration
            .as_deref()
            .and_then(|n| n.parse::<f64>().ok())
    }

    pub fn video_parameters(self) -> VideoParameters {
        let video_stream = self
            .streams
            .iter()
            .find(|s| s.codec_type.as_deref() == Some("video"));

        VideoParameters {
            fps: video_stream
                .map(|s| s.avg_frame_rate.clone())
                .unwrap_or_else(|| "N/A".into()),
            width: video_stream.and_then(|s| s.width).unwrap_or_default(),
            height: video_stream.and_then(|s| s.height).unwrap_or_default(),
            codec: video_stream
                .and_then(|s| s.codec_name.clone())
                .unwrap_or_else(|| "N/A".into()),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Stream {
    pub index: i64,
    pub codec_name: Option<String>,
    pub codec_long_name: Option<String>,
    pub codec_type: Option<String>,
    pub codec_time_base: Option<String>,

    pub sample_fmt: Option<String>,
    pub sample_rate: Option<String>,
    pub channels: Option<i64>,
    pub bits_per_sample: Option<i64>,
    pub r_frame_rate: String,
    pub avg_frame_rate: String,
    pub time_base: String,
    pub start_pts: Option<i64>,
    pub start_time: Option<String>,
    pub duration_ts: Option<i64>,
    pub duration: Option<String>,
    pub bit_rate: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub id: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Format {
    pub filename: String,
    pub nb_streams: i64,
    pub nb_programs: i64,
    pub format_name: String,
    pub format_long_name: String,
    pub start_time: Option<String>,
    pub duration: Option<String>,
    pub size: Option<String>,
    pub bit_rate: Option<String>,
    pub tags: Option<FormatTags>,
}

impl Format {
    pub fn duration(&self) -> Option<f64> {
        self.duration
            .as_ref()
            .and_then(|duration| duration.parse::<f64>().ok())
    }
}

#[derive(Deserialize, Serialize)]
pub struct FormatTags {
    #[serde(rename = "WMFSDKNeeded")]
    pub wmfsdkneeded: Option<String>,
    #[serde(rename = "DeviceConformanceTemplate")]
    pub device_conformance_template: Option<String>,
    #[serde(rename = "WMFSDKVersion")]
    pub wmfsdkversion: Option<String>,
    #[serde(rename = "IsVBR")]
    pub is_vbr: Option<String>,
    pub major_brand: Option<String>,
    pub minor_version: Option<String>,
    pub compatible_brands: Option<String>,
    pub creation_time: Option<String>,
    pub encoder: Option<String>,
}

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
        let info: String = sqlx::query_scalar!(
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
