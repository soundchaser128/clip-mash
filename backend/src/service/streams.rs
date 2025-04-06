use std::collections::HashMap;

use crate::data::database::videos::VideoSource;
use crate::data::database::Database;
use crate::data::stash_api::StashApi;
use crate::server::types::{Clip, VideoLike};
use crate::Result;

#[derive(Debug, Clone, Copy)]
pub enum LocalVideoSource {
    Url,
    File,
}

#[derive(Clone)]
pub struct StreamUrlService {
    stash_api: StashApi,
    database: Database,
}

impl StreamUrlService {
    pub async fn new(database: Database) -> Self {
        let config = database.settings.fetch().await.unwrap_or_default();
        let stash_api = StashApi::with_config(config.stash);
        StreamUrlService {
            stash_api,
            database,
        }
    }

    fn get_stream_url<V: VideoLike>(
        &self,
        source: VideoSource,
        video_id: &str,
        local_video_source: LocalVideoSource,
        videos: &[V],
    ) -> Option<String> {
        match source {
            VideoSource::Folder | VideoSource::Download => match local_video_source {
                LocalVideoSource::Url => Some(format!("/api/library/video/{}/file", video_id)),
                LocalVideoSource::File => {
                    let video_file_path = videos
                        .iter()
                        .find(|v| v.video_id() == video_id)
                        .and_then(|v| v.file_path());
                    video_file_path.map(From::from)
                }
            },

            VideoSource::Stash => videos
                .iter()
                .find(|v| v.video_id() == video_id)
                .and_then(|v| v.stash_scene_id())
                .map(|stash_id| self.stash_api.get_stream_url(stash_id)),
        }
    }

    pub async fn get_video_streams(
        &self,
        video_ids: &[&str],
        local_video_source: LocalVideoSource,
    ) -> Result<HashMap<String, String>> {
        let mut streams = HashMap::new();
        let videos = self.database.videos.get_videos_by_ids(video_ids).await?;
        for video in &videos {
            let url = self.get_stream_url(video.source, &video.id, local_video_source, &videos);
            if let Some(url) = url {
                if !streams.contains_key(&video.id) {
                    streams.insert(video.id.clone(), url);
                }
            }
        }
        Ok(streams)
    }

    pub fn get_clip_streams<V: VideoLike>(
        &self,
        clips: &[Clip],
        videos: &[V],
        local_video_source: LocalVideoSource,
    ) -> HashMap<String, String> {
        let mut streams = HashMap::new();
        for clip in clips {
            let url = self.get_stream_url(clip.source, &clip.video_id, local_video_source, videos);
            if let Some(url) = url {
                if !streams.contains_key(&clip.video_id) {
                    streams.insert(clip.video_id.clone(), url);
                }
            }
        }
        streams
    }
}
