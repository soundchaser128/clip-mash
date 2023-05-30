use std::fmt::Debug;
use std::time::Instant;

use clip_mash_types::{
    Clip, ClipOptions, ClipOrder, PmvClipOptions, RandomizedClipOptions, SongClipOptions,
};
use rand::rngs::StdRng;
use tracing::info;

use self::pmv::{PmvClipLengths, PmvSongs};
use super::Marker;
use crate::data::database::{Database, DbSong};
use crate::service::clip::default::{DefaultClipCreator, DefaultClipOptions};
use crate::service::clip::pmv::{PmvClipCreator, PmvClipCreatorOptions};
use crate::service::clip::sort::{ClipSorter, RandomClipSorter, SceneOrderClipSorter};
use crate::service::music::parse_beats;
use crate::util::create_seeded_rng;
use crate::Result;

mod default;
mod pmv;
mod sort;

#[derive(Debug)]
pub struct CreateClipsOptions {
    pub order: ClipOrder,
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

pub struct ClipService {
    db: Database,
}

impl ClipService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn arrange_clips(
        &self,
        mut options: CreateClipsOptions,
    ) -> Result<(Vec<Clip>, Option<Vec<f32>>)> {
        let start = Instant::now();
        options.normalize_video_indices();
        let mut rng = create_seeded_rng(options.seed.as_deref());
        let order = options.order;
        let mut beat_offsets = None;

        let clips = match options.clip_options {
            ClipOptions::Pmv { song_ids, clips } => {
                let songs = self.db.get_songs(&song_ids).await?;
                beat_offsets = Some(normalize_beat_offsets(&songs));

                let video_duration: f64 = songs.iter().map(|s| s.duration).sum();
                let pmv_options = PmvClipCreatorOptions {
                    seed: options.seed,
                    video_duration,
                    clip_lengths: match clips {
                        PmvClipOptions::Randomized(RandomizedClipOptions {
                            base_duration,
                            divisors,
                        }) => PmvClipLengths::Randomized {
                            base_duration,
                            divisors,
                        },
                        PmvClipOptions::Songs(SongClipOptions {
                            beats_per_measure,
                            cut_after_measures,
                        }) => {
                            let beats = parse_beats(&songs);
                            PmvClipLengths::Songs(PmvSongs::new(
                                beats,
                                beats_per_measure,
                                match cut_after_measures {
                                    clip_mash_types::MeasureCount::Fixed { count } => {
                                        pmv::MeasureCount::Fixed(count)
                                    }
                                    clip_mash_types::MeasureCount::Random { min, max } => {
                                        pmv::MeasureCount::Randomized { min, max }
                                    }
                                },
                            ))
                        }
                    },
                };
                let creator = PmvClipCreator;
                creator.create_clips(options.markers, pmv_options, &mut rng)
            }
            ClipOptions::Default(o) => {
                let default_options = DefaultClipOptions {
                    clip_duration: o.base_duration as u32,
                    seed: options.seed,
                    divisors: o.divisors,
                };
                let creator = DefaultClipCreator;
                creator.create_clips(options.markers, default_options, &mut rng)
            }
            ClipOptions::NoSplit => markers_to_clips(options.markers),
        };

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

        let elapsed = start.elapsed();
        info!("generated {} clips in {:?}", clips.len(), elapsed);

        Ok((clips, beat_offsets))
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
    use clip_mash_types::{Clip, ClipOptions, MarkerId, RandomizedClipOptions, VideoSource};
    use sqlx::SqlitePool;
    use tracing_test::traced_test;

    use super::{ClipOrder, CreateClipsOptions};
    use crate::data::database::Database;
    use crate::service::clip::pmv::{PmvClipCreatorOptions, PmvClipLengths};
    use crate::service::clip::sort::ClipSorter;
    use crate::service::clip::{ClipCreator, ClipService, PmvClipCreator, SceneOrderClipSorter};
    use crate::service::fixtures::{self, create_marker_video_id};
    use crate::service::VideoId;
    use crate::util::create_seeded_rng;

    #[traced_test]
    #[sqlx::test]
    fn test_arrange_clips_basic(pool: SqlitePool) {
        let options = CreateClipsOptions {
            order: ClipOrder::SceneOrder,
            markers: vec![
                create_marker_video_id(1, 1.0, 15.0, 0, "v2"),
                create_marker_video_id(2, 1.0, 17.0, 0, "v1"),
            ],
            seed: None,
            clip_options: ClipOptions::Default(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
        };
        let service = ClipService::new(Database::with_pool(pool));
        let (results, _) = service.arrange_clips(options).await.unwrap();
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
            order: ClipOrder::SceneOrder,
            markers: vec![
                create_marker_video_id(1, 1.0, 15.0, 0, "v1"),
                create_marker_video_id(2, 1.0, 17.0, 0, "v2"),
            ],
            seed: None,
            clip_options: ClipOptions::NoSplit,
        };
        let service = ClipService::new(Database::with_pool(pool));
        let (results, _) = service.arrange_clips(options).await.unwrap();
        assert_eq!(2, results.len());
        assert_eq!((1.0, 17.0), results[0].range);
        assert_eq!((1.0, 15.0), results[1].range);
    }

    #[traced_test]
    #[test]
    fn test_normalize_video_indices() {
        let mut options = CreateClipsOptions {
            order: ClipOrder::SceneOrder,
            markers: vec![
                create_marker_video_id(1, 140.0, 190.0, 5, "v2".into()),
                create_marker_video_id(2, 1.0, 17.0, 0, "v1".into()),
                create_marker_video_id(3, 80.0, 120.0, 3, "v2".into()),
                create_marker_video_id(4, 1.0, 15.0, 0, "v3".into()),
                create_marker_video_id(5, 20.0, 60.0, 3, "v1".into()),
            ],
            seed: None,
            clip_options: ClipOptions::Default(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
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
        let options = PmvClipCreatorOptions {
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
