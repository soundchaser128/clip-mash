use std::collections::HashMap;

use clip_mash_types::{
    Clip, EqualLengthClipOptions, RoundRobinClipOptions, WeightedRandomClipOptions,
};
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::rngs::StdRng;
use rand::seq::{IteratorRandom, SliceRandom};
use tracing::{debug, info};

use super::pmv::PmvClipLengths;
use crate::service::Marker;

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

pub struct RoundRobinClipPicker;

impl ClipPicker for RoundRobinClipPicker {
    type Options = RoundRobinClipOptions;

    fn pick_clips(
        &mut self,
        markers: Vec<Marker>,
        options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip> {
        info!("using RoundRobinClipPicker to make clips from markers {markers:#?} with options {options:#?}");

        let max_duration = options
            .length
            .unwrap_or_else(|| markers.iter().map(|m| m.duration()).sum());
        info!("maximum video duration: {max_duration}");
        let mut total_duration = 0.0;
        let mut clips = vec![];
        let mut marker_idx = 0;
        let mut clip_lengths: PmvClipLengths = options.clip_lengths.into();
        let has_music = matches!(clip_lengths, PmvClipLengths::Songs(_));

        let mut start_times: HashMap<i64, (f64, usize)> = markers
            .iter()
            .map(|m| (m.id.inner(), (m.start_time, 0)))
            .collect();

        while (total_duration - max_duration).abs() > 0.01 {
            let marker = &markers[marker_idx % markers.len()];
            let clip_duration = clip_lengths.pick_duration(rng);
            if clip_duration.is_none() {
                break;
            }
            let (start, index) = start_times[&marker.id.inner()];
            let clip_duration = clip_duration.unwrap();

            let end = (start + clip_duration).min(marker.end_time);
            let duration = end - start;
            if has_music || duration >= MIN_DURATION {
                info!(
                    "adding clip for video {} from {start} - {end}, total_length = {total_duration}",
                    marker.video_id
                );
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
            info!("clip duration {clips_duration} longer than permitted maximum duration {max_duration}, making each clip {slack} shorter");
            for clip in &mut clips {
                clip.range.1 -= slack;
            }
        }

        clips
    }
}

pub struct WeightedRandomClipPicker;

impl ClipPicker for WeightedRandomClipPicker {
    type Options = WeightedRandomClipOptions;

    fn pick_clips(
        &mut self,
        markers: Vec<Marker>,
        options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip> {
        info!("using WeightedRandomClipPicker to make clips: {options:#?}");
        let choices = options.weights;
        let distribution = WeightedIndex::new(choices.iter().map(|item| item.1))
            .expect("could not build distribution");
        let mut total_duration = 0.0;
        let mut start_times: HashMap<i64, (f64, usize)> = markers
            .iter()
            .map(|m| (m.id.inner(), (m.start_time, 0)))
            .collect();
        let mut clips = vec![];
        let mut clip_lengths: PmvClipLengths = options.clip_lengths.into();

        while (total_duration - options.length).abs() > 0.01 {
            let marker_tag = &choices[distribution.sample(rng)].0;
            let next_marker = markers
                .iter()
                .filter(|m| &m.title == marker_tag)
                .choose(rng);
            if let Some(marker) = next_marker {
                let (start, index) = start_times[&marker.id.inner()];
                let clip_duration = clip_lengths.pick_duration(rng);
                if clip_duration.is_none() {
                    break;
                }
                // if (start - marker.end_time).abs() < 0.01 {
                //     start_times.remove(&marker.id.inner());
                //     break;
                // }

                let clip_duration = clip_duration.unwrap();
                let end = (start + clip_duration).min(marker.end_time);
                let duration = end - start;
                clips.push(Clip {
                    index_within_marker: index,
                    index_within_video: marker.index_within_video,
                    marker_id: marker.id,
                    range: (start, end),
                    source: marker.video_id.source(),
                    video_id: marker.video_id.clone(),
                });
                info!(
                    "adding clip for tag {} with duration {}",
                    marker.title, duration
                );

                start_times.insert(marker.id.inner(), (end, index + 1));
                total_duration += duration;
            }
        }

        clips
    }
}

pub struct EqualLengthClipPicker;

impl ClipPicker for EqualLengthClipPicker {
    type Options = EqualLengthClipOptions;

    fn pick_clips(
        &mut self,
        markers: Vec<Marker>,
        options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip> {
        assert!(options.divisors.len() > 0, "divisors must not be empty");
        info!("using EqualLengthClipPicker to make clips: {options:?}");

        let duration = options.clip_duration;
        let clip_lengths: Vec<f64> = options
            .divisors
            .into_iter()
            .map(|d| (duration / d).max(MIN_DURATION))
            .collect();
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use assert_approx_eq::assert_approx_eq;
    use clip_mash_types::{
        PmvClipOptions, RandomizedClipOptions, RoundRobinClipOptions, WeightedRandomClipOptions,
    };
    use tracing_test::traced_test;

    use super::RoundRobinClipPicker;
    use crate::service::clip::picker::{ClipPicker, WeightedRandomClipPicker};
    use crate::service::fixtures;
    use crate::util::create_seeded_rng;

    #[traced_test]
    #[test]
    fn test_weighted_random_clips() {
        let weights = vec![
            ("Cowgirl".into(), 1.0 / 3.0),
            ("Blowjob".into(), 1.0 / 3.0),
            ("Doggy Style".into(), 1.0 / 3.0),
        ];

        let mut picker = WeightedRandomClipPicker;
        let options = WeightedRandomClipOptions {
            clip_lengths: PmvClipOptions::Randomized(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
            length: 200.0,
            weights: weights.clone(),
        };

        let markers = vec![
            fixtures::create_marker("Blowjob", 0.0, 30.0, 0),
            fixtures::create_marker("Blowjob", 0.0, 30.0, 1),
            fixtures::create_marker("Cowgirl", 0.0, 30.0, 0),
            fixtures::create_marker("Cowgirl", 0.0, 30.0, 1),
            fixtures::create_marker("Doggy Style", 0.0, 30.0, 0),
            fixtures::create_marker("Doggy Style", 0.0, 30.0, 1),
        ];
        let mut rng = create_seeded_rng(None);

        let clips = picker.pick_clips(markers.clone(), options, &mut rng);
        let clips: Vec<_> = clips
            .into_iter()
            .map(|clip| {
                let marker_id = clip.marker_id;
                let marker = markers.iter().find(|m| m.id == marker_id).unwrap();
                (clip, marker)
            })
            .collect();
        let tags: Vec<_> = weights.iter().map(|m| &m.0).collect();
        let mut counts: HashMap<&String, usize> = HashMap::new();
        for (_, marker) in clips {
            assert!(tags.contains(&&marker.title));
            let count = counts.entry(&marker.title).or_default();
            *count += 1;
        }

        dbg!(counts);
    }

    #[traced_test]
    #[test]
    fn test_arrange_clips_length_bug() {
        let video_duration = 673.515;
        let markers = fixtures::markers();
        let options = RoundRobinClipOptions {
            length: Some(video_duration),
            clip_lengths: clip_mash_types::PmvClipOptions::Randomized(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
        };

        let mut rng = create_seeded_rng(None);
        let mut picker = RoundRobinClipPicker;
        let clips = picker.pick_clips(markers, options, &mut rng);
        let clip_duration: f64 = clips
            .iter()
            .map(|c| {
                let (start, end) = c.range;
                end - start
            })
            .sum();
        assert_eq!(66, clips.len());
        assert_approx_eq!(clip_duration, video_duration);
    }

    #[traced_test]
    #[test]
    fn test_arrange_clips_loop_bug() {
        let options = RoundRobinClipOptions {
            length: None,
            clip_lengths: PmvClipOptions::Randomized(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
        };
        let markers = fixtures::other_markers();
        let mut rng = create_seeded_rng(None);
        let mut picker = RoundRobinClipPicker;
        let clips = picker.pick_clips(markers, options, &mut rng);
        dbg!(clips);
    }
}
