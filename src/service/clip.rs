use super::{Clip, Marker};
use crate::util::create_seeded_rng;
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use serde::Deserialize;
use std::{collections::HashMap, fmt::Debug};
use tracing::{debug, info};

const MIN_DURATION: f64 = 2.0;

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

pub fn arrange_clips(mut options: CreateClipsOptions) -> Vec<Clip> {
    options.normalize_video_indices();
    let mut rng = create_seeded_rng(options.seed.as_deref());

    let clips = match options.max_duration {
        Some(duration) => {
            let creator = PmvClipCreator {};
            creator.create_clips(
                options.markers,
                PmvClipOptions {
                    clip_duration: options.clip_duration,
                    seed: options.seed,
                    video_duration: duration,
                },
                &mut rng,
            )
        }
        None => {
            let creator = DefaultClipCreator {};
            creator.create_clips(
                options.markers,
                DefaultClipOptions {
                    clip_duration: options.clip_duration,
                    seed: options.seed,
                },
                &mut rng,
            )
        }
    };
    info!("generated {} clips", clips.len());

    let sorter: Box<dyn ClipSorter> = match options.order {
        ClipOrder::Random => Box::new(RandomClipSorter),
        ClipOrder::SceneOrder => Box::new(SceneOrderClipSorter),
    };

    sorter.sort_clips(clips, &mut rng)
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

#[derive(Debug)]
pub struct PmvClipOptions {
    pub seed: Option<String>,
    pub clip_duration: u32,
    pub video_duration: f64,
}

pub struct PmvClipCreator;

impl ClipCreator for PmvClipCreator {
    type Options = PmvClipOptions;

    fn create_clips(
        &self,
        markers: Vec<Marker>,
        options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip> {
        info!(
            "using PmvClipCreator to create clips, options: {:?}",
            options
        );
        let duration = options.clip_duration as f64;
        let clip_lengths = [
            (duration / 1.5).max(MIN_DURATION),
            (duration / 2.0).max(MIN_DURATION),
            (duration / 3.0).max(MIN_DURATION),
            (duration / 4.0).max(MIN_DURATION),
        ];

        let max_duration = options.video_duration;
        let mut total_duration = 0.0;
        let mut clips = vec![];
        let mut marker_idx = 0;

        let mut start_times: HashMap<i64, (f64, usize)> =
            markers.iter().map(|m| (m.id.inner(), (0.0, 0))).collect();

        while total_duration <= max_duration {
            let marker = &markers[marker_idx % markers.len()];
            let clip_duration = clip_lengths.choose(rng).expect("must find one element");

            let (start, index) = start_times[&marker.id.inner()];
            let end = (start + clip_duration).min(marker.end_time);
            let duration = end - start;
            if duration >= MIN_DURATION {
                debug!("adding clip {start} - {end}");
                clips.push(Clip {
                    index_within_marker: index,
                    index_within_video: marker.index_within_video,
                    marker_id: marker.id,
                    range: (start, end),
                    source: marker.video_id.source(),
                    video_id: marker.video_id.clone(),
                });
            }

            total_duration += duration;
            marker_idx += 1;
            start_times.insert(marker.id.inner(), (end, index + 1));
        }

        let clips_duration: f64 = clips.iter().map(|c| c.duration()).sum();
        if clips_duration > max_duration {
            let slack = (clips_duration - max_duration) / clips.len() as f64;
            for clip in &mut clips {
                clip.range.1 = clip.range.1 - slack;
            }
        }

        clips
    }
}

#[derive(Debug)]
pub struct DefaultClipOptions {
    pub clip_duration: u32,
    pub seed: Option<String>,
}

pub struct DefaultClipCreator;

impl ClipCreator for DefaultClipCreator {
    type Options = DefaultClipOptions;

    fn create_clips(
        &self,
        markers: Vec<Marker>,
        options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip> {
        info!(
            "using DefaultClipCreator to create clips, options: {:?}",
            options
        );
        let duration = options.clip_duration as f64;
        let clip_lengths = [
            (duration / 1.5).max(MIN_DURATION),
            (duration / 2.0).max(MIN_DURATION),
            (duration / 3.0).max(MIN_DURATION),
            (duration / 4.0).max(MIN_DURATION),
        ];
        let mut clips = vec![];
        for marker in markers {
            let start = marker.start_time;
            let end = marker.end_time;

            debug!("clip start = {start}, end = {end}");

            let mut index = 0;
            let mut offset = start;
            while offset < end {
                let duration = clip_lengths.choose(rng).unwrap();
                let start = offset;
                let end = (offset + duration).min(end);
                let duration = end - start;
                if duration > MIN_DURATION {
                    debug!("adding clip {} - {}", start, end);
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
        }

        clips
    }
}

pub trait ClipSorter {
    fn sort_clips(&self, clips: Vec<Clip>, rng: &mut StdRng) -> Vec<Clip>;
}

#[derive(Debug)]
pub struct RandomClipSorter;

impl ClipSorter for RandomClipSorter {
    fn sort_clips(&self, mut clips: Vec<Clip>, rng: &mut StdRng) -> Vec<Clip> {
        info!("sorting clips with RandomClipSorter");
        clips.shuffle(rng);
        clips
    }
}

#[derive(Debug)]
pub struct SceneOrderClipSorter;

impl ClipSorter for SceneOrderClipSorter {
    fn sort_clips(&self, clips: Vec<Clip>, rng: &mut StdRng) -> Vec<Clip> {
        info!("sorting clips with SceneOrderClipSorter");
        let mut clips: Vec<_> = clips.into_iter().map(|c| (c, rng.gen::<usize>())).collect();

        clips.sort_by_key(|(clip, random)| {
            (clip.index_within_video, clip.index_within_marker, *random)
        });
        clips.into_iter().map(|(clip, _)| clip).collect()
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use tracing_test::traced_test;

    use super::{ClipOrder, CreateClipsOptions};
    use crate::{
        service::{
            clip::{arrange_clips, ClipCreator, PmvClipCreator, PmvClipOptions},
            fixtures::{self, create_marker, create_marker_video_id},
            MarkerId,
        },
        util::create_seeded_rng,
    };

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
    fn test_bug_clips2() {
        let video_duration = 673.515;
        let markers = fixtures::markers();
        let options = PmvClipOptions {
            clip_duration: 30,
            seed: None,
            video_duration,
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
        assert_approx_eq!(clip_duration, video_duration)
    }
}
