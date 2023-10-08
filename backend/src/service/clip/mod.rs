use std::fmt::Debug;
use std::time::Instant;

use itertools::Itertools;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use tracing::info;

use super::Marker;
use crate::server::types::{Beats, Clip, ClipOptions, ClipOrder, ClipPickerOptions};
use crate::service::clip::equal_len::EqualLengthClipPicker;
use crate::service::clip::round_robin::RoundRobinClipPicker;
use crate::service::clip::sort::{ClipSorter, RandomClipSorter, SceneOrderClipSorter};
use crate::service::clip::weighted::WeightedRandomClipPicker;
use crate::util::create_seeded_rng;

mod equal_len;
mod length_picker;
mod round_robin;
mod sort;
mod state;
mod weighted;

const MIN_DURATION: f64 = 1.5;

pub trait ClipPicker {
    type Options;

    fn pick_clips(
        &mut self,
        markers: Vec<Marker>,
        options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClipsOptions {
    pub markers: Vec<Marker>,
    pub seed: Option<String>,
    pub clip_options: ClipOptions,
}

impl CreateClipsOptions {
    pub fn normalize_video_indices(&mut self) {
        self.markers.sort_by_key(|m| m.video_id.clone());
        for (_, group) in &self.markers.iter_mut().group_by(|m| m.video_id.clone()) {
            let mut group = group.collect_vec();
            group.sort_by_key(|m| m.index_within_video);
            for (index, marker) in group.iter_mut().enumerate() {
                marker.index_within_video = index;
            }
        }
    }

    pub fn apply_marker_loops(self) -> Self {
        let markers: Vec<_> = self
            .markers
            .into_iter()
            .flat_map(|marker| {
                let loops = marker.loops;
                vec![marker; loops]
            })
            .collect();
        Self { markers, ..self }
    }
}

fn markers_to_clips(markers: Vec<Marker>) -> Vec<Clip> {
    markers
        .into_iter()
        .map(|marker| Clip {
            source: marker.source,
            video_id: marker.video_id.clone(),
            marker_id: marker.id,
            range: (marker.start_time, marker.end_time),
            index_within_marker: 0,
            index_within_video: marker.index_within_video,
        })
        .collect()
}

fn normalize_beat_offsets(songs: &[Beats]) -> Vec<f32> {
    let mut offsets = vec![];
    let mut current = 0.0;
    for beats in songs {
        for offset in &beats.offsets {
            offsets.push(current + offset);
        }
        current += beats.length;
    }

    offsets
}

pub struct ClipsResult {
    pub clips: Vec<Clip>,
    pub beat_offsets: Option<Vec<f32>>,
}

pub struct ClipService {}

impl ClipService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn arrange_clips(&self, mut options: CreateClipsOptions) -> ClipsResult {
        let start = Instant::now();
        options.normalize_video_indices();
        let mut options = options.apply_marker_loops();

        let beat_offsets = options
            .clip_options
            .clip_picker
            .songs()
            .map(normalize_beat_offsets);

        if options.clip_options.clip_picker.has_music() {
            info!("options have music, not sorting clips");
            options.clip_options.order = ClipOrder::NoOp;
        }

        let mut rng = create_seeded_rng(options.seed.as_deref());
        options.markers.shuffle(&mut rng);
        let clips = match options.clip_options.clip_picker {
            ClipPickerOptions::RoundRobin(picker_options) => {
                let mut picker = RoundRobinClipPicker;
                picker.pick_clips(options.markers, picker_options, &mut rng)
            }
            ClipPickerOptions::WeightedRandom(picker_options) => {
                let mut picker = WeightedRandomClipPicker;
                picker.pick_clips(options.markers, picker_options, &mut rng)
            }
            ClipPickerOptions::EqualLength(picker_options) => {
                let mut picker = EqualLengthClipPicker;
                picker.pick_clips(options.markers, picker_options, &mut rng)
            }
            ClipPickerOptions::NoSplit => markers_to_clips(options.markers),
        };

        let clips = match options.clip_options.order {
            ClipOrder::Random => {
                let sorter = RandomClipSorter;
                sorter.sort_clips(clips, &mut rng)
            }
            ClipOrder::SceneOrder => {
                let sorter = SceneOrderClipSorter;
                sorter.sort_clips(clips, &mut rng)
            }
            ClipOrder::NoOp => clips,
        };

        let elapsed = start.elapsed();
        info!("generated {} clips in {:?}", clips.len(), elapsed);

        ClipsResult {
            clips,
            beat_offsets,
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use tracing_test::traced_test;

    use super::{ClipOrder, CreateClipsOptions};
    use crate::data::database::VideoSource;
    use crate::server::types::{
        Clip, ClipOptions, ClipPickerOptions, EqualLengthClipOptions, PmvClipOptions,
        RandomizedClipOptions, RoundRobinClipOptions,
    };
    use crate::service::clip::sort::ClipSorter;
    use crate::service::clip::{ClipService, ClipsResult, SceneOrderClipSorter};
    use crate::service::fixtures::{create_marker_video_id, create_marker_with_loops};
    use crate::util::create_seeded_rng;

    #[traced_test]
    #[test]
    fn test_arrange_clips_basic() {
        let options = CreateClipsOptions {
            markers: vec![
                create_marker_video_id(1, 1.0, 15.0, 0, "v2"),
                create_marker_video_id(2, 1.0, 17.0, 0, "v1"),
            ],
            seed: None,
            clip_options: ClipOptions {
                clip_picker: ClipPickerOptions::EqualLength(EqualLengthClipOptions {
                    clip_duration: 30.0,
                    divisors: vec![2.0, 3.0, 4.0],
                }),
                order: ClipOrder::SceneOrder,
            },
        };
        let service = ClipService::new();
        let ClipsResult { clips: results, .. } = service.arrange_clips(options);
        assert_eq!(3, results.len());
        assert_eq!((1.0, 11.0), results[0].range);
        assert_eq!((1.0, 16.0), results[1].range);
        assert_eq!((11.0, 15.0), results[2].range);
    }

    #[traced_test]
    #[test]
    fn test_arrange_clips_dont_split() {
        let options = CreateClipsOptions {
            markers: vec![
                create_marker_video_id(1, 1.0, 15.0, 0, "v1"),
                create_marker_video_id(2, 1.0, 17.0, 0, "v2"),
            ],
            seed: None,
            clip_options: ClipOptions {
                clip_picker: ClipPickerOptions::NoSplit,
                order: ClipOrder::SceneOrder,
            },
        };
        let service = ClipService::new();
        let ClipsResult { clips: results, .. } = service.arrange_clips(options);
        assert_eq!(2, results.len());
        assert_eq!((1.0, 15.0), results[0].range);
        assert_eq!((1.0, 17.0), results[1].range);
    }

    #[traced_test]
    #[test]
    fn test_normalize_video_indices() {
        let mut options = CreateClipsOptions {
            markers: vec![
                create_marker_video_id(1, 140.0, 190.0, 5, "v2"),
                create_marker_video_id(2, 1.0, 17.0, 0, "v1"),
                create_marker_video_id(3, 80.0, 120.0, 3, "v2"),
                create_marker_video_id(4, 1.0, 15.0, 0, "v3"),
                create_marker_video_id(5, 20.0, 60.0, 3, "v1"),
            ],
            seed: None,
            clip_options: ClipOptions {
                clip_picker: ClipPickerOptions::EqualLength(EqualLengthClipOptions {
                    clip_duration: 30.0,
                    divisors: vec![2.0, 3.0, 4.0],
                }),
                order: ClipOrder::SceneOrder,
            },
        };

        options.normalize_video_indices();

        let marker = options.markers.iter().find(|m| m.id == 1).unwrap();
        assert_eq!(marker.index_within_video, 1);

        let marker = options.markers.iter().find(|m| m.id == 2).unwrap();
        assert_eq!(marker.index_within_video, 0);

        let marker = options.markers.iter().find(|m| m.id == 3).unwrap();
        assert_eq!(marker.index_within_video, 0);

        let marker = options.markers.iter().find(|m| m.id == 4).unwrap();
        assert_eq!(marker.index_within_video, 0);

        let marker = options.markers.iter().find(|m| m.id == 5).unwrap();
        assert_eq!(marker.index_within_video, 1);
    }

    #[traced_test]
    #[test]
    fn sort_clips_scene_order() {
        let clips = vec![
            Clip {
                index_within_marker: 0,
                index_within_video: 0,
                marker_id: 1,
                range: (0.0, 9.0),
                source: VideoSource::Folder,
                video_id: "video".into(),
            },
            Clip {
                index_within_marker: 0,
                index_within_video: 0,
                marker_id: 2,
                range: (1.0, 12.0),
                source: VideoSource::Folder,
                video_id: "video".into(),
            },
        ];
        let mut rng = create_seeded_rng(None);
        let sorter = SceneOrderClipSorter;
        let sorted = sorter.sort_clips(clips, &mut rng);

        assert_eq!(sorted[0].range, (1.0, 12.0));
        assert_eq!(sorted[1].range, (0.0, 9.0));
    }

    #[test]
    #[traced_test]
    fn test_loop_markers() {
        let options = CreateClipsOptions {
            markers: vec![
                create_marker_with_loops(1, 1.0, 15.0, 0, "v1", 2),
                create_marker_with_loops(2, 1.0, 17.0, 0, "v2", 3),
            ],
            seed: None,
            clip_options: ClipOptions {
                clip_picker: ClipPickerOptions::RoundRobin(RoundRobinClipOptions {
                    clip_lengths: PmvClipOptions::Randomized(RandomizedClipOptions {
                        base_duration: 10.0,
                        divisors: vec![2.0, 3.0, 4.0],
                    }),
                    length: 30.0,
                }),
                order: ClipOrder::SceneOrder,
            },
        };
        let service = ClipService::new();
        let ClipsResult { clips: results, .. } = service.arrange_clips(options);
        let total_duration: f64 = results.iter().map(|c| c.duration()).sum();
        assert_approx_eq!(f64, 30.0, total_duration, epsilon = 0.01);
        assert_eq!(results.len(), 8);
    }

    #[test]
    #[traced_test]
    fn test_apply_marker_loops() {
        let m1 = create_marker_with_loops(1, 1.0, 15.0, 0, "v1", 2);
        let m2 = create_marker_with_loops(2, 3.5, 17.0, 0, "v2", 3);
        let options = CreateClipsOptions {
            markers: vec![m1.clone(), m2.clone()],
            seed: None,
            clip_options: ClipOptions {
                clip_picker: ClipPickerOptions::RoundRobin(RoundRobinClipOptions {
                    clip_lengths: PmvClipOptions::Randomized(RandomizedClipOptions {
                        base_duration: 10.0,
                        divisors: vec![2.0, 3.0, 4.0],
                    }),
                    length: 30.0,
                }),
                order: ClipOrder::SceneOrder,
            },
        };
        let options = options.apply_marker_loops();
        assert_eq!(options.markers.len(), 5);
        assert_eq!(options.markers[0].id, m1.id);
        assert_eq!(options.markers[1].id, m1.id);
        assert_eq!(options.markers[2].id, m2.id);
        assert_eq!(options.markers[3].id, m2.id);
        assert_eq!(options.markers[4].id, m2.id);
    }

    #[test]
    #[traced_test]
    fn test_infinite_loop_marker_loops_with_music() {
        let string = std::fs::read_to_string("testfiles/infinite-loop.json").unwrap();
        let options: CreateClipsOptions = serde_json::from_str(&string).unwrap();
        let service = ClipService::new();
        let result = service.arrange_clips(options).clips;
        let expected_length = 1084.0275;
        let total_duration: f64 = result.iter().map(|c| c.duration()).sum();
        assert_approx_eq!(f64, expected_length, total_duration, epsilon = 0.01);
    }
}
