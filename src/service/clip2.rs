use std::collections::HashMap;

use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use tracing::info;

use crate::util::create_seeded_rng;

use super::{Clip, Marker};

const MIN_DURATION: f64 = 2.0;

pub trait ClipCreator {
    type Options;

    fn create_clips(&self, markers: Vec<Marker>, options: Self::Options) -> Vec<Clip>;
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

    fn create_clips(&self, markers: Vec<Marker>, options: Self::Options) -> Vec<Clip> {
        let duration = options.clip_duration as f64;
        let clip_lengths = [
            (duration / 1.5).max(MIN_DURATION),
            (duration / 2.0).max(MIN_DURATION),
            (duration / 3.0).max(MIN_DURATION),
            (duration / 4.0).max(MIN_DURATION),
        ];

        let mut rng = create_seeded_rng(options.seed.as_deref());
        let max_duration = options.video_duration;
        let mut total_duration = 0.0;
        let mut clips = vec![];
        let mut marker_idx = 0;

        let mut start_times: HashMap<i64, (f64, usize)> =
            markers.iter().map(|m| (m.id.inner(), (0.0, 0))).collect();

        while total_duration <= max_duration {
            let marker = &markers[marker_idx % markers.len()];
            let clip_duration = clip_lengths
                .choose(&mut rng)
                .expect("must find one element");

            let (start, index) = start_times[&marker.id.inner()];
            let end = (start + clip_duration).min(marker.end_time);
            let duration = end - start;
            if duration >= MIN_DURATION {
                info!("adding clip ({start}, {end})");
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

pub struct DefaultClipOptions {}

pub struct DefaultClipCreator;

impl ClipCreator for DefaultClipCreator {
    type Options = DefaultClipOptions;

    fn create_clips(&self, markers: Vec<Marker>, options: Self::Options) -> Vec<Clip> {
        todo!()
    }
}

pub trait ClipSorter {
    fn sort_clips(clips: Vec<Clip>, rng: &mut StdRng) -> Vec<Clip>;
}

pub struct RandomClipSorter;

impl ClipSorter for RandomClipSorter {
    fn sort_clips(mut clips: Vec<Clip>, rng: &mut StdRng) -> Vec<Clip> {
        clips.shuffle(rng);
        clips
    }
}

pub struct SceneOrderClipSorter;

impl ClipSorter for SceneOrderClipSorter {
    fn sort_clips(clips: Vec<Clip>, rng: &mut StdRng) -> Vec<Clip> {
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

    use crate::service::{
        clip2::{ClipCreator, PmvClipCreator, PmvClipOptions},
        fixtures,
    };

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
        let clips = clip_creator.create_clips(markers, options);
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
