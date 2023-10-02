use std::collections::HashMap;

use crate::data::database::VideoSource;
use crate::data::stash_api::StashApi;
use crate::server::types::{Clip, VideoLike};

pub struct StreamUrlService {
    stash_api: StashApi,
}

impl StreamUrlService {
    pub async fn new() -> Self {
        let stash_api = StashApi::load_config().await;
        StreamUrlService { stash_api }
    }

    pub fn get_clip_streams<V: VideoLike>(
        &self,
        clips: &[Clip],
        videos: &[V],
    ) -> HashMap<String, String> {
        let mut streams = HashMap::new();
        for clip in clips {
            match clip.source {
                VideoSource::Folder | VideoSource::Download => {
                    if !streams.contains_key(&clip.video_id) {
                        let url = format!("/api/library/video/{}/file", clip.video_id);
                        streams.insert(clip.video_id.clone(), url);
                    }
                }
                VideoSource::Stash => {
                    if !streams.contains_key(&clip.video_id) {
                        let stash_id = videos
                            .iter()
                            .find(|v| v.video_id() == clip.video_id)
                            .and_then(|v| v.stash_scene_id());
                        if let Some(stash_id) = stash_id {
                            let url = self.stash_api.get_stream_url(stash_id);
                            streams.insert(clip.video_id.clone(), url);
                        }
                    }
                }
            }
        }
        streams
    }
}
