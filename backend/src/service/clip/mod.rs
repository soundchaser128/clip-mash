use std::fmt::Debug;
use std::time::Instant;

use clip_mash_types::{Clip, ClipOptions, ClipOrder, ClipPickerOptions};
use rand::rngs::StdRng;
use tracing::info;

use super::Marker;
use crate::data::database::{Database, DbSong};
use crate::service::clip::picker::{
    ClipPicker, EqualLengthClipPicker, RoundRobinClipPicker, WeightedRandomClipPicker,
};
use crate::service::clip::sort::{ClipSorter, RandomClipSorter, SceneOrderClipSorter};
use crate::service::music::parse_beats;
use crate::util::create_seeded_rng;
use crate::Result;

mod default;
mod picker;
mod pmv;
mod sort;

#[derive(Debug)]
pub struct CreateClipsOptions {
    pub markers: Vec<Marker>,
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
    for beats in parse_beats(songs) {
        for offset in beats.offsets {
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

pub struct ClipService {
    db: Database,
}

impl ClipService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn arrange_clips(&self, mut options: CreateClipsOptions) -> Result<ClipsResult> {
        let start = Instant::now();
        options.normalize_video_indices();
        let mut rng = create_seeded_rng(options.seed.as_deref());
        let beat_offsets = None;
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

        Ok(ClipsResult {
            clips,
            beat_offsets,
        })
    }
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
    use clip_mash_types::{
        Clip, ClipOptions, ClipPickerOptions, EqualLengthClipOptions, MarkerId,
        RandomizedClipOptions, RoundRobinClipOptions, VideoSource,
    };
    use sqlx::SqlitePool;
    use tracing_test::traced_test;

    use super::{ClipOrder, CreateClipsOptions};
    use crate::data::database::Database;
    use crate::service::clip::sort::ClipSorter;
    use crate::service::clip::{ClipService, ClipsResult, SceneOrderClipSorter};
    use crate::service::fixtures::{self, create_marker_video_id};
    use crate::service::VideoId;
    use crate::util::create_seeded_rng;

    #[traced_test]
    #[sqlx::test]
    fn test_arrange_clips_basic(pool: SqlitePool) {
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
        let service = ClipService::new(Database::with_pool(pool));
        let ClipsResult { clips: results, .. } = service.arrange_clips(options).await.unwrap();
        assert_eq!(4, results.len());
        assert_eq!((1.0, 11.0), results[0].range);
        assert_eq!((1.0, 8.5), results[1].range);
        assert_eq!((11.0, 17.0), results[2].range);
        assert_eq!((8.5, 15.0), results[3].range);
    }

    #[traced_test]
    #[sqlx::test]
    fn test_arrange_clips_dont_split(_pool: SqlitePool) {
        todo!()
        // let options = CreateClipsOptions {
        //     order: ClipOrder::SceneOrder,
        //     markers: vec![
        //         create_marker_video_id(1, 1.0, 15.0, 0, "v1"),
        //         create_marker_video_id(2, 1.0, 17.0, 0, "v2"),
        //     ],
        //     seed: None,
        //     clip_options: ClipOptions::NoSplit,
        // };
        // let service = ClipService::new(Database::with_pool(pool));
        // let (results, _) = service.arrange_clips(options).await.unwrap();
        // assert_eq!(2, results.len());
        // assert_eq!((1.0, 17.0), results[0].range);
        // assert_eq!((1.0, 15.0), results[1].range);
    }

    #[traced_test]
    #[test]
    fn test_normalize_video_indices() {
        let mut options = CreateClipsOptions {
            markers: vec![
                create_marker_video_id(1, 140.0, 190.0, 5, "v2".into()),
                create_marker_video_id(2, 1.0, 17.0, 0, "v1".into()),
                create_marker_video_id(3, 80.0, 120.0, 3, "v2".into()),
                create_marker_video_id(4, 1.0, 15.0, 0, "v3".into()),
                create_marker_video_id(5, 20.0, 60.0, 3, "v1".into()),
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
    #[sqlx::test]
    async fn test_arrange_clips_bug(pool: SqlitePool) {
        let video_duration = 673.515;
        let markers = fixtures::markers();
        let options = CreateClipsOptions {
            markers,
            seed: None,
            clip_options: ClipOptions {
                order: ClipOrder::SceneOrder,
                clip_picker: ClipPickerOptions::RoundRobin(RoundRobinClipOptions {
                    length: video_duration,
                    clip_lengths: clip_mash_types::PmvClipOptions::Randomized(
                        RandomizedClipOptions {
                            base_duration: 30.0,
                            divisors: vec![2.0, 3.0, 4.0],
                        },
                    ),
                }),
            },
        };

        let service = ClipService::new(Database::with_pool(pool));
        let ClipsResult { clips, .. } = service.arrange_clips(options).await.unwrap();
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
