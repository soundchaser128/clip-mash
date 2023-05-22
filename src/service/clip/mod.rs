use self::pmv::{PmvClipLengths, PmvSongs};

use super::{beats::Beats, Clip, Marker};
use crate::{
    data::database::DbSong,
    server::dtos::ClipOptions,
    service::clip::{
        default::{DefaultClipCreator, DefaultClipOptions},
        pmv::{PmvClipCreator, PmvClipOptions},
        sort::{ClipSorter, RandomClipSorter, SceneOrderClipSorter},
    },
    util::create_seeded_rng,
};
use rand::rngs::StdRng;
use serde::Deserialize;
use std::fmt::Debug;
use tracing::info;

mod default;
mod pmv;
mod sort;

const MIN_DURATION: f64 = 2.0;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClipOrder {
    Random,
    SceneOrder,
    Pmv,
}

#[derive(Debug)]
pub struct CreateClipsOptions {
    pub order: ClipOrder,
    pub markers: Vec<Marker>,
    pub split_clips: bool,
    pub seed: Option<String>,
    pub clip_options: ClipOptions,
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

fn markers_to_clips(markers: Vec<Marker>) -> Vec<Clip> {
    markers
        .into_iter()
        .map(|marker| Clip {
            source: marker.video_id.source(),
            video_id: marker.video_id.clone(),
            marker_id: marker.id,
            range: (marker.start_time, marker.end_time),
            index_within_marker: 0,
            index_within_video: marker.index_within_video,
        })
        .collect()
}

fn normalize_beat_offsets(songs: &[DbSong]) -> Vec<f32> {
    let mut offsets = vec![];
    let mut current = 0.0;
    for song in songs {
        let beats: Beats =
            serde_json::from_str(song.beats.as_deref().expect("song must have beats"))
                .expect("must be valid json");

        for offset in beats.offsets {
            offsets.push(current + offset);
        }
        current += beats.length;
    }

    offsets
}

pub fn arrange_clips(mut options: CreateClipsOptions) -> (Vec<Clip>, Option<Vec<f32>>) {
    options.normalize_video_indices();
    let mut rng = create_seeded_rng(options.seed.as_deref());
    let mut order = options.order;

    // let beat_offsets = if options.songs.is_empty() {
    //     None
    // } else {
    //     Some(normalize_beat_offsets(&options.songs))
    // };

    // let clips = match (options.split_clips, options.max_duration) {
    //     (true, None) => marrkers_to_clips_default(options, &mut rng),
    //     (true, Some(duration)) => {
    //         order = ClipOrder::Pmv;
    //         markers_to_clips_pmv(options, duration, &mut rng)
    //     }
    //     (false, _) => markers_to_clips(options.markers),
    // };

    let clips = match options.clip_options {
        ClipOptions::Pmv { song_ids, clips } => {
            let songs = todo!();
            let options = PmvClipOptions {
                seed: options.seed,
                video_duration,
                clip_lengths: match clips {
                    PmvClipOptions::Randomized(_) => todo!(),
                    PmvClipOptions::Songs { beats_per_measure, cut_after_measure_count } => todo!(),
                }
            };
            let creator = PmvClipCreator;
            creator.create_clips(options.markers, options, &mut rng)
        },
        ClipOptions::Default(options) => {
            let options = DefaultClipOptions::from(options);
            let creator = DefaultClipCreator;
            creator.create_clips(options.markers, options, &mut rng)
        },
    };

    info!("generated {} clips", clips.len());
    let clips = match order {
        ClipOrder::Random => {
            let sorter = RandomClipSorter;
            sorter.sort_clips(clips, &mut rng)
        }
        ClipOrder::SceneOrder => {
            let sorter = SceneOrderClipSorter;
            sorter.sort_clips(clips, &mut rng)
        }
        ClipOrder::Pmv => clips,
    };

    (clips, beat_offsets)
}

pub trait ClipCreator {
    type Options;

    fn create_clips(
        &self,
        markers: Vec<Marker>,
        options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip>;
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use tracing_test::traced_test;

    use super::{ClipOrder, CreateClipsOptions};
    use crate::{
        service::{
            clip::{
                arrange_clips, pmv::PmvClipLengths, sort::ClipSorter, ClipCreator, PmvClipCreator,
                PmvClipOptions, SceneOrderClipSorter,
            },
            fixtures::{self, create_marker_video_id},
            Clip, MarkerId, VideoId, VideoSource,
        },
        util::create_seeded_rng,
    };

    #[traced_test]
    #[test]
    fn test_arrange_clips_basic() {
        let options = CreateClipsOptions {
            order: ClipOrder::SceneOrder,
            clip_duration: 30,
            markers: vec![
                create_marker_video_id(1, 1.0, 15.0, 0, "v2"),
                create_marker_video_id(2, 1.0, 17.0, 0, "v1"),
            ],
            split_clips: true,
            seed: None,
            max_duration: None,
            songs: vec![],
        };
        let (results, _) = arrange_clips(options);
        assert_eq!(4, results.len());
        assert_eq!((1.0, 11.0), results[0].range);
        assert_eq!((1.0, 8.5), results[1].range);
        assert_eq!((11.0, 17.0), results[2].range);
        assert_eq!((8.5, 15.0), results[3].range);
    }

    #[traced_test]
    #[test]
    fn test_arrange_clips_dont_split() {
        let options = CreateClipsOptions {
            order: ClipOrder::SceneOrder,
            clip_duration: 30,
            markers: vec![
                create_marker_video_id(1, 1.0, 15.0, 0, "v1"),
                create_marker_video_id(2, 1.0, 17.0, 0, "v2"),
            ],
            split_clips: false,
            seed: None,
            max_duration: None,
            songs: vec![],
        };
        let (results, _) = arrange_clips(options);
        assert_eq!(2, results.len());
        assert_eq!((1.0, 17.0), results[0].range);
        assert_eq!((1.0, 15.0), results[1].range);
    }

    #[traced_test]
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
            songs: vec![],
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
    fn test_arrange_clips_bug() {
        let video_duration = 673.515;
        let markers = fixtures::markers();
        let options = PmvClipOptions {
            seed: None,
            video_duration,
            clip_lengths: PmvClipLengths::Randomized {
                base_duration: 30.,
                divisors: vec![2.0, 3.0, 4.0],
            },
        };
        let clip_creator = PmvClipCreator;
        let mut rng = create_seeded_rng(None);
        let clips = clip_creator.create_clips(markers, options, &mut rng);
        let clip_duration: f64 = clips
            .iter()
            .map(|c| {
                let (start, end) = c.range;
                end - start
            })
            .sum();
        assert_approx_eq!(clip_duration, video_duration);
    }

    #[traced_test]
    #[test]
    fn sort_clips_scene_order() {
        let clips = vec![
            Clip {
                index_within_marker: 0,
                index_within_video: 0,
                marker_id: MarkerId::LocalFile(1),
                range: (0.0, 9.0),
                source: VideoSource::LocalFile,
                video_id: VideoId::LocalFile("video".into()),
            },
            Clip {
                index_within_marker: 0,
                index_within_video: 0,
                marker_id: MarkerId::LocalFile(2),
                range: (1.0, 12.0),
                source: VideoSource::LocalFile,
                video_id: VideoId::LocalFile("video".into()),
            },
        ];
        let mut rng = create_seeded_rng(None);
        let sorter = SceneOrderClipSorter;
        let sorted = sorter.sort_clips(clips, &mut rng);

        assert_eq!(sorted[0].range, (1.0, 12.0));
        assert_eq!(sorted[1].range, (0.0, 9.0));
    }
}
