use itertools::Itertools;
use tracing::info;

use crate::Result;
use crate::data::database::Database;

#[derive(Clone)]
pub struct EncodingOptimizationService {
    database: Database,
}

impl EncodingOptimizationService {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn needs_re_encode(&self, video_ids: &[&str]) -> Result<bool> {
        if video_ids.len() == 1 {
            info!("only one video, no need to re-encode");
            return Ok(false);
        }

        info!("checking encoding parameters for videos {:?}", video_ids);
        let video_infos = self.database.ffprobe.get_infos(video_ids).await?;

        let parameters: Vec<_> = video_infos
            .into_iter()
            .map(|v| v.video_parameters())
            .collect();
        info!("video parameters: {:#?}", parameters);
        let can_concatenate = parameters.into_iter().all_equal();
        info!("can concatenate: {}", can_concatenate);
        Ok(!can_concatenate)
    }
}
