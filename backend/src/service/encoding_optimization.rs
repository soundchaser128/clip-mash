use std::collections::HashMap;

use itertools::Itertools;
use tracing::info;

use super::commands::ffmpeg::FfmpegLocation;
use super::commands::ffprobe::{ffprobe, FfProbe};
use super::streams::{LocalVideoSource, StreamUrlService};
use crate::data::database::Database;
use crate::Result;

#[derive(Clone)]
pub struct EncodingOptimizationService {
    ffmpeg_location: FfmpegLocation,
    streams_service: StreamUrlService,
}

impl EncodingOptimizationService {
    pub async fn new(ffmpeg_location: FfmpegLocation, database: Database) -> Self {
        let streams_service = StreamUrlService::new(database).await;
        EncodingOptimizationService {
            ffmpeg_location,
            streams_service,
        }
    }

    async fn get_video_infos<'a>(
        &self,
        streams: &'a HashMap<String, String>,
    ) -> HashMap<&'a str, Option<FfProbe>> {
        let mut result = HashMap::new();
        for (video_id, url) in streams {
            let ffprobe = ffprobe(url, &self.ffmpeg_location).await;
            result.insert(video_id.as_str(), ffprobe.ok());
        }

        result
    }

    pub async fn needs_re_encode(&self, video_ids: &[&str]) -> Result<bool> {
        let streams = self
            .streams_service
            .get_video_streams(video_ids, LocalVideoSource::File)
            .await?;
        let video_infos = self.get_video_infos(&streams).await;

        let any_ffprobe_errors = video_infos.values().any(|v| v.is_none());
        if any_ffprobe_errors {
            Ok(true)
        } else {
            let parameters: Vec<_> = video_infos
                .into_values()
                .map(|v| v.unwrap().video_parameters())
                .collect();
            info!("video parameters: {:#?}", parameters);
            let can_concatenate = parameters.into_iter().all_equal();
            Ok(!can_concatenate)
        }
    }
}
