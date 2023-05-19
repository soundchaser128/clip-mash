use std::collections::{HashMap, HashSet};

use crate::data::database::DbSong;
use crate::{data::database::Database, server::dtos::CreateClipsBody};
use crate::{
    data::stash_api::StashApi,
    server::dtos::{CreateVideoBody, SelectedMarker},
    Result,
};
use color_eyre::eyre::bail;
use reqwest::Url;
use serde::Deserialize;

use super::{
    generator::CompilationOptions, stash_config::Config, Clip, Marker, MarkerId, MarkerInfo, Video,
    VideoId,
};

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClipOrder {
    Random,
    SceneOrder,
}

#[derive(Debug)]
pub struct CreateClipsOptions {
    pub order: ClipOrder,
    pub clip_duration: u32,
    pub markers: Vec<Marker>,
    pub split_clips: bool,
    pub seed: Option<String>,
    pub max_duration: Option<f64>,
}

impl CreateClipsOptions {
    pub fn normalize_video_indices(&mut self) {
        use itertools::Itertools;

        self.markers.sort_by_key(|m| m.video_id.clone());
        for (_, group) in &self.markers.iter_mut().group_by(|m| m.video_id.clone()) {
            let mut group = group.collect_vec();
            group.sort_by_key(|m| m.index_within_video);
            for (index, marker) in group.iter_mut().enumerate() {
                marker.index_within_video = index;
            }
        }
    }
}

pub fn get_streams(
    video_ids: HashSet<VideoId>,
    config: &Config,
) -> Result<HashMap<String, String>> {
    let mut urls = HashMap::new();

    for id in video_ids {
        match id {
            VideoId::LocalFile(_) => {
                let url = format!("/api/local/video/{id}");
                urls.insert(id.to_string(), url);
            }
            VideoId::Stash(_) => {
                let mut url = Url::parse(&config.stash_url)?;
                url.set_path(&format!("/scene/{id}/stream"));
                url.query_pairs_mut().append_pair("apikey", &config.api_key);
                urls.insert(id.to_string(), url.to_string());
            }
        }
    }

    Ok(urls)
}

pub struct ClipService<'a> {
    db: &'a Database,
    stash_api: &'a StashApi,
}

impl<'a> ClipService<'a> {
    pub fn new(db: &'a Database, stash_api: &'a StashApi) -> Self {
        ClipService { db, stash_api }
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
            clip_duration: body.clip_duration,
            split_clips: body.split_clips,
            markers: self.convert_selected_markers(body.markers).await?,
            seed: body.seed,
            max_duration: if body.song_ids.is_empty() || !body.trim_video_for_songs {
                None
            } else {
                Some(self.db.sum_song_durations(&body.song_ids).await?)
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::service::{
        clip2::arrange_clips,
        fixtures::{self, create_marker, create_marker_video_id},
        MarkerId,
    };
    use tracing_test::traced_test;

    use super::{ClipOrder, CreateClipsOptions};

    #[test]
    fn test_compile_clips() {
        let options = CreateClipsOptions {
            order: ClipOrder::SceneOrder,
            clip_duration: 30,
            markers: vec![create_marker(1.0, 15.0, 0), create_marker(1.0, 17.0, 0)],
            split_clips: true,
            seed: None,
            max_duration: None,
        };
        let results = arrange_clips(options);
        assert_eq!(4, results.len());
    }

    #[test]
    fn test_normalize_video_indices() {
        let mut options = CreateClipsOptions {
            order: ClipOrder::SceneOrder,
            clip_duration: 30,
            markers: vec![
                create_marker_video_id(1, 140.0, 190.0, 5, "v2".into()),
                create_marker_video_id(2, 1.0, 17.0, 0, "v1".into()),
                create_marker_video_id(3, 80.0, 120.0, 3, "v2".into()),
                create_marker_video_id(4, 1.0, 15.0, 0, "v3".into()),
                create_marker_video_id(5, 20.0, 60.0, 3, "v1".into()),
            ],
            split_clips: true,
            seed: None,
            max_duration: None,
        };

        options.normalize_video_indices();

        let marker = options
            .markers
            .iter()
            .find(|m| m.id == MarkerId::LocalFile(1))
            .unwrap();
        assert_eq!(marker.index_within_video, 1);

        let marker = options
            .markers
            .iter()
            .find(|m| m.id == MarkerId::LocalFile(2))
            .unwrap();
        assert_eq!(marker.index_within_video, 0);

        let marker = options
            .markers
            .iter()
            .find(|m| m.id == MarkerId::LocalFile(3))
            .unwrap();
        assert_eq!(marker.index_within_video, 0);

        let marker = options
            .markers
            .iter()
            .find(|m| m.id == MarkerId::LocalFile(4))
            .unwrap();
        assert_eq!(marker.index_within_video, 0);

        let marker = options
            .markers
            .iter()
            .find(|m| m.id == MarkerId::LocalFile(5))
            .unwrap();
        assert_eq!(marker.index_within_video, 1);
    }

    #[traced_test]
    #[test]
    fn test_bug() {
        let max_duration = 673.515;
        let options = CreateClipsOptions {
            order: ClipOrder::SceneOrder,
            clip_duration: 30,
            markers: fixtures::markers(),
            split_clips: true,
            seed: None,
            max_duration: Some(max_duration),
        };
        let marker_duration: f64 = options.markers.iter().map(|m| m.duration()).sum();
        let clips = arrange_clips(options);
        let clip_duration: f64 = clips
            .iter()
            .map(|c| {
                let (start, end) = c.range;
                end - start
            })
            .sum();

        println!("marker duration = {marker_duration}, clip_duration = {clip_duration}, max_duration = {max_duration}");
    }
}
