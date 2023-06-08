use std::fmt::Debug;
use std::time::Instant;

use clip_mash_types::{
    Beats, Clip, ClipOptions, ClipOrder, ClipPickerOptions, PmvClipOptions, RoundRobinClipOptions,
    SongClipOptions,
};
use itertools::Itertools;
use rand::rngs::StdRng;
use tracing::{debug, info};

use super::Marker;
use crate::data::database::Database;
use crate::service::clip::equal_len::EqualLengthClipPicker;
use crate::service::clip::round_robin::RoundRobinClipPicker;
use crate::service::clip::sort::{ClipSorter, RandomClipSorter, SceneOrderClipSorter};
use crate::service::clip::weighted::WeightedRandomClipPicker;
use crate::util::create_seeded_rng;
use crate::Result;

mod equal_len;
mod picker;
mod pmv;
mod round_robin;
mod sort;
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

#[derive(Debug)]
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

fn merge_clips(input: Vec<Clip>) -> Vec<Clip> {
    info!("merging {} clips", input.len());
    let mut clips = vec![];
    for (key, group) in &input
        .into_iter()
        .group_by(|clip| (clip.video_id.clone(), clip.marker_id))
    {
        let group = group.collect_vec();
        let start = group[0].range.0;
        let end = group.last().unwrap().range.1;
        clips.push(Clip {
            source: group[0].source.clone(),
            video_id: key.0,
            marker_id: key.1,
            range: (start, end),
            index_within_marker: 0,
            index_within_video: group[0].index_within_video,
        })
    }

    debug!("merging resulted in clips {clips:#?}");
    info!("merge result: {} clips", clips.len());

    clips
}

pub struct ClipService {
    _db: Database,
}

impl ClipService {
    pub fn new(db: Database) -> Self {
        Self { _db: db }
    }

    pub async fn arrange_clips(&self, mut options: CreateClipsOptions) -> Result<ClipsResult> {
        let start = Instant::now();
        options.normalize_video_indices();

        let beat_offsets = if let ClipPickerOptions::RoundRobin(RoundRobinClipOptions {
            clip_lengths: PmvClipOptions::Songs(SongClipOptions { ref songs, .. }),
            ..
        }) = options.clip_options.clip_picker
        {
            Some(normalize_beat_offsets(songs))
        } else {
            None
        };

        let mut rng = create_seeded_rng(options.seed.as_deref());
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
        let clips = merge_clips(clips);

        let elapsed = start.elapsed();
        info!("generated {} clips in {:?}", clips.len(), elapsed);

        Ok(ClipsResult {
            clips,
            beat_offsets,
        })
    }
}

#[cfg(test)]
mod tests {
    use clip_mash_types::{
        Clip, ClipOptions, ClipPickerOptions, EqualLengthClipOptions, MarkerId, VideoSource,
    };
    use sqlx::SqlitePool;
    use tracing_test::traced_test;

    use super::{ClipOrder, CreateClipsOptions};
    use crate::data::database::Database;
    use crate::service::clip::sort::ClipSorter;
    use crate::service::clip::{merge_clips, ClipService, ClipsResult, SceneOrderClipSorter};
    use crate::service::fixtures::create_marker_video_id;
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
    fn test_arrange_clips_dont_split(pool: SqlitePool) {
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
        let service = ClipService::new(Database::with_pool(pool));
        let ClipsResult { clips: results, .. } = service.arrange_clips(options).await.unwrap();
        assert_eq!(2, results.len());
        assert_eq!((1.0, 17.0), results[0].range);
        assert_eq!((1.0, 15.0), results[1].range);
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

    #[traced_test]
    #[test]
    fn test_merge_two_clips() {
        let clips = vec![
            Clip {
                index_within_marker: 0,
                index_within_video: 0,
                marker_id: MarkerId::LocalFile(1),
                range: (0.0, 9.0),
                source: VideoSource::LocalFile,
                video_id: VideoId::LocalFile("1".into()),
            },
            Clip {
                index_within_marker: 1,
                index_within_video: 0,
                marker_id: MarkerId::LocalFile(1),
                range: (9.0, 12.0),
                source: VideoSource::LocalFile,
                video_id: VideoId::LocalFile("1".into()),
            },
        ];
        let merged = merge_clips(clips);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].range, (0.0, 12.0));
    }

    #[traced_test]
    #[test]
    fn test_merge_three_clips() {
        let clips = vec![
            Clip {
                index_within_marker: 0,
                index_within_video: 0,
                marker_id: MarkerId::LocalFile(1),
                range: (0.0, 9.0),
                source: VideoSource::LocalFile,
                video_id: VideoId::LocalFile("1".into()),
            },
            Clip {
                index_within_marker: 1,
                index_within_video: 0,
                marker_id: MarkerId::LocalFile(1),
                range: (9.0, 12.0),
                source: VideoSource::LocalFile,
                video_id: VideoId::LocalFile("1".into()),
            },
            Clip {
                index_within_marker: 2,
                index_within_video: 0,
                marker_id: MarkerId::LocalFile(1),
                range: (12.0, 18.0),
                source: VideoSource::LocalFile,
                video_id: VideoId::LocalFile("1".into()),
            },
            Clip {
                index_within_marker: 2,
                index_within_video: 0,
                marker_id: MarkerId::LocalFile(1),
                range: (12.0, 18.0),
                source: VideoSource::LocalFile,
                video_id: VideoId::LocalFile("2".into()),
            },
        ];
        let merged = merge_clips(clips);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].range, (0.0, 18.0));
        assert_eq!(merged[1].range, (12.0, 18.0));
    }
}
