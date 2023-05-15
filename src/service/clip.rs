use std::collections::{HashMap, HashSet};

use crate::data::database::DbSong;
use crate::{data::database::Database, server::dtos::CreateClipsBody};
use crate::{
    data::stash_api::StashApi,
    server::dtos::{CreateVideoBody, SelectedMarker},
    util::{self},
    Result,
};
use color_eyre::eyre::bail;
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use reqwest::Url;
use serde::Deserialize;
use tracing::{debug, info};

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

#[derive(Debug, PartialEq)]
pub struct MarkerWithClips {
    pub marker: Marker,
    pub clips: Vec<Clip>,
}

fn get_clips(
    marker: &Marker,
    max_marker_duration: Option<f64>,
    options: &CreateClipsOptions,
    rng: &mut StdRng,
) -> MarkerWithClips {
    const MIN_DURATION: f64 = 2.0;

    let duration = options.clip_duration as f64;
    let clip_lengths = [
        (duration / 2.0).max(MIN_DURATION),
        (duration / 3.0).max(MIN_DURATION),
        (duration / 4.0).max(MIN_DURATION),
    ];

    let start = marker.start_time;
    let end = max_marker_duration
        .map(|n| marker.start_time + n)
        .unwrap_or(marker.end_time);

    let mut index = 0;
    let mut offset = start;
    let mut clips = vec![];
    while offset < end {
        let duration = clip_lengths.choose(rng).unwrap();
        let start = offset;
        let end = (offset + duration).min(end);
        let duration = end - start;
        if duration > MIN_DURATION {
            clips.push(Clip {
                source: marker.video_id.source(),
                video_id: marker.video_id.clone(),
                marker_id: marker.id,
                range: (start, end),
                index_within_marker: index,
                index_within_video: marker.index_within_video,
            });
            index += 1;
        }
        offset += duration;
    }

    MarkerWithClips {
        marker: marker.clone(),
        clips,
    }
}

#[derive(Debug)]
pub struct CreateClipsOptions {
    pub order: ClipOrder,
    pub clip_duration: u32,
    pub markers: Vec<Marker>,
    pub split_clips: bool,
    pub sort_mode: ClipSortMode,
    pub seed: Option<String>,
    pub max_duration: Option<f64>,
}

fn get_all_clips(options: &CreateClipsOptions) -> Vec<MarkerWithClips> {
    let mut rng = util::create_seeded_rng(options.seed.as_deref());
    debug!("creating clips for options {options:?}");
    let marker_duration: f64 = options.markers.iter().map(|m| m.duration()).sum();
    info!("total duration of all markers: {marker_duration} seconds");
    info!("maximum duration: {:?}", options.max_duration);

    let max_duration_per_marker = options
        .max_duration
        .map(|seconds| seconds.min(marker_duration))
        .map(|seconds| seconds / options.markers.len() as f64);
    info!("duration per marker {max_duration_per_marker:?}");

    options
        .markers
        .iter()
        .map(|marker| {
            if options.split_clips {
                get_clips(marker, max_duration_per_marker, options, &mut rng)
            } else {
                MarkerWithClips {
                    marker: marker.clone(),
                    clips: vec![Clip {
                        source: marker.video_id.source(),
                        video_id: marker.video_id.clone(),
                        marker_id: marker.id,
                        range: (marker.start_time, marker.end_time),
                        index_within_marker: 0,
                        index_within_video: marker.index_within_video,
                    }],
                }
            }
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClipSortMode {
    VideoIndex,
    MarkerIndex,
}

impl ClipSortMode {
    pub fn key(&self, clip: &Clip, random: usize) -> impl Ord {
        match self {
            ClipSortMode::VideoIndex => (clip.index_within_video, clip.index_within_marker, random),
            ClipSortMode::MarkerIndex => (clip.index_within_marker, random, random),
        }
    }
}

fn compile_clips(clips: Vec<MarkerWithClips>, options: &CreateClipsOptions) -> Vec<Clip> {
    let mut rng = util::create_seeded_rng(options.seed.as_deref());

    match options.order {
        ClipOrder::SceneOrder => {
            let mut clips: Vec<_> = clips
                .into_iter()
                .flat_map(|m| m.clips)
                .map(|c| (c, rng.gen::<usize>()))
                .collect();

            clips.sort_by_key(|(clip, random)| options.sort_mode.key(clip, *random));
            clips.into_iter().map(|(clip, _)| clip).collect()
        }
        ClipOrder::Random => {
            let mut clips: Vec<_> = clips.into_iter().flat_map(|c| c.clips).collect();
            clips.shuffle(&mut rng);
            clips
        }
    }
}

pub fn arrange_clips(options: &CreateClipsOptions) -> Vec<Clip> {
    let clips = get_all_clips(options);
    compile_clips(clips, options)
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
            sort_mode: body.sort_mode,
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
    use crate::{
        data::database::DbMarker,
        service::{
            clip::{arrange_clips, ClipSortMode},
            Marker, MarkerId, MarkerInfo, VideoId,
        },
    };

    use fake::{faker::filesystem::en::FilePath, Fake, Faker};
    use nanoid::nanoid;

    use super::{get_all_clips, ClipOrder, CreateClipsOptions};

    fn create_marker(start_time: f64, end_time: f64, index: usize) -> Marker {
        Marker {
            // id: Faker.fake(),
            id: MarkerId::LocalFile(1),
            start_time,
            end_time,
            index_within_video: index,
            video_id: VideoId::LocalFile(nanoid!(8)),
            title: Faker.fake(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    end_time,
                    start_time,
                    rowid: None,
                    title: Faker.fake(),
                    video_id: Faker.fake(),
                    file_path: FilePath().fake(),
                    index_within_video: index as i64,
                },
            },
        }
    }

    #[test]
    fn test_get_clips() {
        let options = CreateClipsOptions {
            order: ClipOrder::SceneOrder,
            clip_duration: 30,
            markers: vec![create_marker(1.0, 15.0, 0), create_marker(1.0, 17.0, 0)],
            split_clips: true,
            sort_mode: ClipSortMode::VideoIndex,
            seed: None,
            max_duration: None,
        };
        let mut results1 = get_all_clips(&options);
        assert_eq!(2, results1.len());

        let results2 = get_all_clips(&options);
        assert_eq!(results1, results2);

        let clips = results1.remove(0);
        assert_eq!(2, clips.clips.len());
        assert_eq!(clips.clips[0].range.0, 1.0);
        assert_eq!(clips.clips[1].range.1, 15.0);

        let clips = results1.remove(0);
        assert_eq!(2, clips.clips.len());
        assert_eq!(clips.clips[0].range.0, 1.0);
        assert_eq!(clips.clips[1].range.1, 17.0);
    }

    #[test]
    fn test_compile_clips() {
        let options = CreateClipsOptions {
            order: ClipOrder::SceneOrder,
            clip_duration: 30,
            markers: vec![create_marker(1.0, 15.0, 0), create_marker(1.0, 17.0, 0)],
            split_clips: true,
            sort_mode: ClipSortMode::VideoIndex,
            seed: None,
            max_duration: None,
        };
        let results = arrange_clips(&options);
        assert_eq!(4, results.len());
    }

    #[test]
    fn test_compile_clips_with_time_limit() {
        let options = CreateClipsOptions {
            order: ClipOrder::SceneOrder,
            clip_duration: 15,
            markers: vec![
                create_marker(1.0, 15.0, 0),
                create_marker(1.0, 17.0, 0),
                create_marker(20.0, 34.0, 1),
                create_marker(17.0, 40.0, 1),
            ],
            split_clips: true,
            sort_mode: ClipSortMode::VideoIndex,
            seed: None,
            max_duration: Some(30.0),
        };

        let clips = arrange_clips(&options);
        let total_duration: f64 = clips.iter().map(|c| c.range.1 - c.range.0).sum();
        assert_eq!(30.0, total_duration);
    }
}
