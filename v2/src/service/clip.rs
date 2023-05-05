use std::collections::{HashMap, HashSet};

use crate::util;
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use serde::Deserialize;

use super::{Clip, Marker, Video, VideoId};

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

pub fn get_clips(
    marker: &Marker,
    options: &CreateClipsOptions,
    rng: &mut StdRng,
) -> MarkerWithClips {
    let clip_lengths = [
        (options.clip_duration / 2).max(2) as f64,
        (options.clip_duration / 3).max(2) as f64,
        (options.clip_duration / 4).max(2) as f64,
    ];

    let start = marker.start_time;
    let end = marker.end_time;

    let mut index = 0;
    let mut offset = start;
    let mut clips = vec![];
    while offset < end {
        let duration = clip_lengths.choose(rng).unwrap();
        let start = offset;
        let end = (offset + duration).min(marker.end_time);
        clips.push(Clip {
            source: options.video.source(),
            video_id: options.video.id,
            marker_id: marker.id,
            range: (start, end),
            index_within_marker: index,
            index_within_video: marker.index_within_video,
        });
        offset += duration;
        index += 1;
    }

    MarkerWithClips {
        marker: marker.clone(),
        clips,
    }
}

#[derive(Debug)]
pub struct CreateClipsOptions {
    pub clip_duration: u32,
    pub markers: Vec<Marker>,
    pub video: Video,
    pub split_clips: bool,
    pub max_duration: Option<u32>,
}

pub fn get_all_clips(options: &CreateClipsOptions) -> Vec<MarkerWithClips> {
    let mut rng = util::create_seeded_rng();
    tracing::debug!("creating clips for options {options:?}");

    options
        .markers
        .iter()
        .map(|marker| {
            if options.split_clips {
                get_clips(marker, &options, &mut rng)
            } else {
                MarkerWithClips {
                    marker: marker.clone(),
                    clips: vec![Clip {
                        source: options.video.source(),
                        video_id: options.video.id,
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

pub fn compile_clips(clips: Vec<MarkerWithClips>, order: ClipOrder) -> Vec<Clip> {
    let mut rng = util::create_seeded_rng();

    match order {
        ClipOrder::SceneOrder => {
            let mut clips: Vec<_> = clips
                .into_iter()
                .flat_map(|m| m.clips)
                .map(|c| (c, rng.gen::<u32>()))
                .collect();
            // TODO parameter to control order by
            clips.sort_by_key(|(clip, random)| (clip.index_within_video, *random));
            clips.into_iter().map(|(clip, _)| clip).collect()
        }
        ClipOrder::Random => {
            let mut clips: Vec<_> = clips.into_iter().flat_map(|c| c.clips).collect();
            clips.shuffle(&mut rng);
            clips
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::database::{DbMarker, DbVideo},
        service::{Marker, MarkerInfo, Video, VideoInfo},
    };
    use fake::faker::lorem::en::*;
    use fake::{faker::filesystem::en::FilePath, Fake, Faker};

    use super::{compile_clips, get_all_clips, ClipOrder, CreateClipsOptions};

    fn create_marker(start_time: f64, end_time: f64, index: usize) -> Marker {
        Marker {
            id: Faker.fake(),
            start_time,
            end_time,
            index_within_video: index,
            video_id: Faker.fake(),
            title: Faker.fake(),
            info: MarkerInfo::LocalFile {
                marker: DbMarker {
                    end_time,
                    start_time,
                    rowid: None,
                    title: Faker.fake(),
                    video_id: Faker.fake(),
                    file_path: FilePath().fake(),
                },
            },
        }
    }

    fn create_video() -> Video {
        Video {
            id: Faker.fake(),
            title: Sentence(4..12).fake(),
            interactive: Faker.fake(),
            info: VideoInfo::LocalFile {
                video: DbVideo {
                    file_path: FilePath().fake(),
                    id: Faker.fake(),
                    interactive: Faker.fake(),
                },
            },
        }
    }

    #[test]
    fn test_get_clips() {
        let options = CreateClipsOptions {
            clip_duration: 30,
            markers: vec![create_marker(1.0, 15.0, 0), create_marker(1.0, 17.0, 0)],
            max_duration: None,
            split_clips: true,
            video: create_video(),
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
            clip_duration: 30,
            markers: vec![create_marker(1.0, 15.0, 0), create_marker(1.0, 17.0, 0)],
            max_duration: None,
            split_clips: true,
            video: create_video(),
        };
        let results = get_all_clips(&options);
        let results = compile_clips(results, ClipOrder::SceneOrder);
        assert_eq!(4, results.len());
    }
}

pub fn get_streams(video_ids: HashSet<VideoId>) -> HashMap<VideoId, String> {
    todo!()
}
