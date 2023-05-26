use std::collections::{HashMap, HashSet};

use clip_mash_types::*;
use color_eyre::eyre::bail;

use super::{
    database::{Database, DbSong},
    stash_api::StashApi,
};
use crate::{
    service::{
        clip::CreateClipsOptions, generator::CompilationOptions, stash_config::Config, Marker,
        MarkerInfo, Video,
    },
    Result,
};

pub struct DataService {
    db: Database,
    stash_api: StashApi,
}

impl DataService {
    pub async fn new(db: Database) -> Self {
        let config = Config::get_or_empty().await;
        let api = StashApi::from_config(&config);
        Self { db, stash_api: api }
    }

    pub async fn fetch_marker_details(
        &self,
        id: &MarkerId,
        video_id: &VideoId,
    ) -> Result<MarkerInfo> {
        match id {
            MarkerId::LocalFile(id) => {
                let marker = self.db.get_marker(*id).await?;
                Ok(MarkerInfo::LocalFile { marker })
            }
            MarkerId::Stash(marker_id) => {
                let marker = self
                    .stash_api
                    .get_marker(video_id.as_stash_id(), *marker_id)
                    .await?;
                Ok(MarkerInfo::Stash { marker })
            }
        }
    }

    pub async fn fetch_video(&self, id: &VideoId) -> Result<Video> {
        match id {
            VideoId::LocalFile(id) => {
                let video = self.db.get_video(id).await?;
                if let Some(video) = video {
                    Ok(video.into())
                } else {
                    bail!("no video found for id {id}")
                }
            }
            VideoId::Stash(id) => {
                let id = id.parse()?;
                let mut scenes = self.stash_api.find_scenes_by_ids(vec![id]).await?;
                if scenes.len() != 1 {
                    bail!("found more or fewer than one result for id {id}")
                }
                Ok(scenes.remove(0).into())
            }
        }
    }

    pub async fn fetch_videos(&self, ids: &[VideoId]) -> Result<Vec<Video>> {
        let mut videos = vec![];
        for id in ids {
            videos.push(self.fetch_video(id).await?);
        }

        Ok(videos)
    }

    pub async fn convert_clips(&self, clips: Vec<Clip>) -> Result<Vec<(Video, Clip)>> {
        let all_video_ids: HashSet<_> = clips.iter().map(|c| &c.video_id).collect();
        let mut videos = HashMap::new();
        for id in all_video_ids {
            let video = self.fetch_video(id).await?;
            videos.insert(id, video);
        }

        let mut results = vec![];
        for clip in &clips {
            let video = videos.get(&clip.video_id).unwrap().clone();
            results.push((video, clip.clone()));
        }
        Ok(results)
    }

    async fn convert_selected_markers(&self, markers: Vec<SelectedMarker>) -> Result<Vec<Marker>> {
        let mut results = vec![];

        for selected_marker in markers {
            let (start_time, end_time) = selected_marker.selected_range;
            let marker_details: MarkerInfo = self
                .fetch_marker_details(&selected_marker.id, &selected_marker.video_id)
                .await?;
            let video_id = marker_details.video_id().clone();
            let title = marker_details.title().to_string();
            results.push(Marker {
                start_time,
                end_time,
                id: selected_marker.id,
                info: marker_details,
                video_id,
                index_within_video: selected_marker.index_within_video,
                title,
            })
        }

        Ok(results)
    }

    pub async fn convert_compilation_options(
        &self,
        body: CreateVideoBody,
    ) -> Result<CompilationOptions> {
        let songs = self.resolve_songs(&body.song_ids).await?;

        Ok(CompilationOptions {
            clips: body.clips,
            markers: self.convert_selected_markers(body.selected_markers).await?,
            output_resolution: body.output_resolution,
            output_fps: body.output_fps,
            file_name: body.file_name,
            songs,
            music_volume: body.music_volume.unwrap_or(0.0),
        })
    }

    async fn resolve_songs(&self, song_ids: &[i64]) -> Result<Vec<DbSong>> {
        self.db.get_songs(song_ids).await
    }

    pub async fn convert_clip_options(&self, body: CreateClipsBody) -> Result<CreateClipsOptions> {
        Ok(CreateClipsOptions {
            order: body.clip_order,
            markers: self.convert_selected_markers(body.markers).await?,
            seed: body.seed,
            clip_options: body.clips,
        })
    }
}
