use std::collections::HashSet;

use clip_mash_types::{Clip, WeightedRandomClipOptions};
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::rngs::StdRng;
use tracing::{debug, info};

use super::ClipPicker;
use crate::service::clip::length_picker::ClipLengthPicker;
use crate::service::clip::state::{MarkerStart, MarkerState};
use crate::service::Marker;

pub struct WeightedRandomClipPicker;

fn validate_options(
    markers: &[Marker],
    options: &WeightedRandomClipOptions,
    weight_labels: &HashSet<&str>,
) {
    for (title, weight) in &options.weights {
        assert!(
            *weight > 0.0,
            "weight for title {} must be greater than 0",
            title
        );
        let marker_count = markers.iter().filter(|m| &m.title == title).count();
        assert!(marker_count > 0, "no markers found for title {}", title);
    }

    let weights_exist = markers
        .iter()
        .all(|m| weight_labels.contains(m.title.as_str()));
    assert!(weights_exist, "all markers must have a weight");
}

impl ClipPicker for WeightedRandomClipPicker {
    type Options = WeightedRandomClipOptions;

    fn pick_clips(
        &mut self,
        mut markers: Vec<Marker>,
        mut options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip> {
        info!("using WeightedRandomClipPicker to make clips: {options:#?}");
        debug!("using markers: {markers:#?}");
        options.weights.retain(|(_, weight)| *weight > 0.0);
        let weight_labels: HashSet<_> = options
            .weights
            .iter()
            .map(|(label, _)| label.as_str())
            .collect();
        markers.retain(|m| weight_labels.contains(m.title.as_str()));

        validate_options(&markers, &options, &weight_labels);
        let choices = options.weights;

        let distribution = WeightedIndex::new(choices.iter().map(|item| item.1))
            .expect("could not build distribution");
        let mut total_duration = 0.0;
        let mut marker_state = MarkerState::new(markers);
        let mut clips = vec![];
        let mut clip_lengths: ClipLengthPicker = options.clip_lengths.into();

        while total_duration <= options.length {
            debug!("marker state: {marker_state:#?}, total duration: {total_duration}, target duration: {}", options.length);
            if marker_state.markers.is_empty() {
                info!("no more markers to pick from, stopping");
                break;
            }
            let marker_tag = &choices[distribution.sample(rng)].0;
            if let Some(marker) = marker_state.find_marker_by_title(&marker_tag, rng) {
                let clip_duration = clip_lengths.pick_duration(rng);
                if clip_duration.is_none() {
                    break;
                }
                if let Some(MarkerStart {
                    start_time: start,
                    index,
                    ..
                }) = marker_state.get(&marker.id)
                {
                    let clip_duration = clip_duration.unwrap();
                    let end = (start + clip_duration).min(marker.end_time);
                    let duration = end - start;

                    clips.push(Clip {
                        index_within_marker: *index,
                        index_within_video: marker.index_within_video,
                        marker_id: marker.id,
                        range: (*start, end),
                        source: marker.video_id.source(),
                        video_id: marker.video_id.clone(),
                    });
                    info!(
                        "adding clip for video {} with duration {duration} and title {}",
                        marker.video_id, marker.title
                    );

                    marker_state.update(&marker.id, end, index + 1);
                    total_duration += duration;
                }
            } else {
                debug!(
                    "no marker found for title {marker_tag}, skipping, remaining markers: {:?}",
                    marker_state.markers
                );
            }
        }
        let clips_duration: f64 = clips.iter().map(|c| c.duration()).sum();
        if clips_duration > options.length {
            let slack = (clips_duration - options.length) / clips.len() as f64;
            info!("clip duration {clips_duration} longer than permitted maximum duration {}, making each clip {slack} shorter", options.length);
            for clip in &mut clips {
                clip.range.1 = clip.range.1 - slack;
            }
        }

        clips
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use clip_mash_types::{
        PmvClipOptions, RandomizedClipOptions, RoundRobinClipOptions, WeightedRandomClipOptions,
    };
    use float_cmp::assert_approx_eq;
    use tracing_test::traced_test;

    use super::validate_options;
    use crate::service::clip::round_robin::RoundRobinClipPicker;
    use crate::service::clip::weighted::WeightedRandomClipPicker;
    use crate::service::clip::ClipPicker;
    use crate::service::fixtures;
    use crate::util::create_seeded_rng;

    #[traced_test]
    #[test]
    fn test_weighted_random_clips() {
        let target_duration = 100.0;
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
            length: target_duration,
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
        let clip_duration: f64 = clips.iter().map(|c| c.duration()).sum();
        assert_approx_eq!(f64, clip_duration, target_duration, epsilon = 0.1);
    }

    #[traced_test]
    #[test]
    fn test_arrange_clips_length_bug() {
        let video_duration = 673.515;
        let markers = fixtures::markers();
        let marker_titles: HashSet<_> = markers.iter().map(|m| m.title.clone()).collect();

        let options = RoundRobinClipOptions {
            length: video_duration,
            clip_lengths: PmvClipOptions::Randomized(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
        };

        let mut rng = create_seeded_rng(None);
        let mut picker = RoundRobinClipPicker;
        let clips = picker.pick_clips(markers.clone(), options, &mut rng);
        let clip_duration: f64 = clips.iter().map(|c| c.duration()).sum();
        assert_approx_eq!(f64, clip_duration, video_duration, epsilon = 0.1);

        let weights = marker_titles
            .into_iter()
            .map(|title| (title, 1.0))
            .collect();
        let mut picker = WeightedRandomClipPicker;
        let options = WeightedRandomClipOptions {
            clip_lengths: PmvClipOptions::Randomized(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
            weights,
            length: video_duration,
        };

        let clips = picker.pick_clips(markers, options, &mut rng);
        let clip_duration: f64 = clips.iter().map(|c| c.duration()).sum();

        assert_approx_eq!(f64, clip_duration, video_duration, epsilon = 0.1);
    }

    #[traced_test]
    #[test]
    fn test_weighted_distribution() {
        let options = WeightedRandomClipOptions {
            weights: vec![
                ("Cowgirl".into(), 0.0),
                ("Doggy Style".into(), 1.0),
                ("Handjiob".into(), 1.0),
                ("Mating Press".into(), 1.0),
                ("Missionary".into(), 0.0),
                ("Sex".into(), 0.0),
                ("Sideways".into(), 1.0),
            ],
            length: 956.839832,
            clip_lengths: PmvClipOptions::Randomized(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
        };
        let markers = fixtures::other_markers();
        let mut rng = create_seeded_rng(None);
        let mut picker = WeightedRandomClipPicker;
        let clips = picker.pick_clips(markers.clone(), options, &mut rng);
        for clip in clips {
            let marker = markers
                .iter()
                .find(|marker| marker.id == clip.marker_id)
                .unwrap();
            assert_ne!(marker.title, "Cowgirl");
            assert_ne!(marker.title, "Missionary");
            assert_ne!(marker.title, "Sex");
        }
    }

    #[traced_test]
    #[test]
    fn test_weighted_marker_infinite_loop_bug() {
        let options = WeightedRandomClipOptions {
            weights: vec![
                ("Cowgirl".into(), 1.0),
                ("Doggy Style".into(), 1.0),
                ("Handjiob".into(), 1.0),
                ("Mating Press".into(), 1.0),
                ("Missionary".into(), 0.0),
                ("Sex".into(), 1.0),
                ("Sideways".into(), 1.0),
            ],
            length: 956.839832,
            clip_lengths: PmvClipOptions::Randomized(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
        };
        let markers = fixtures::other_markers();
        let mut rng = create_seeded_rng(None);
        let mut picker = WeightedRandomClipPicker;
        let clips = picker.pick_clips(markers, options, &mut rng);
        let clip_duration: f64 = clips.iter().map(|c| c.duration()).sum();
        assert!(clip_duration >= 0.0);
    }

    #[test]
    fn test_validate_options_valid() {
        let markers = vec![
            fixtures::create_marker("A", 0.0, 30.0, 0),
            fixtures::create_marker("B", 0.0, 30.0, 1),
            fixtures::create_marker("C", 0.0, 30.0, 2),
        ];
        let options = WeightedRandomClipOptions {
            weights: vec![
                ("A".to_string(), 1.0),
                ("B".to_string(), 2.0),
                ("C".to_string(), 3.0),
            ],
            clip_lengths: PmvClipOptions::Randomized(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
            length: 30.0,
        };
        let weight_labels = vec!["A", "B", "C"].into_iter().collect();

        validate_options(&markers, &options, &weight_labels);
    }

    #[test]
    #[should_panic(expected = "weight for title A must be greater than 0")]
    fn test_validate_options_zero_weight() {
        let markers = vec![fixtures::create_marker("A", 0.0, 30.0, 0)];
        let options = WeightedRandomClipOptions {
            weights: vec![("A".to_string(), 0.0)],
            clip_lengths: PmvClipOptions::Randomized(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
            length: 30.0,
        };
        let weight_labels = vec!["A"].into_iter().collect();

        validate_options(&markers, &options, &weight_labels);
    }

    #[test]
    #[should_panic(expected = "no markers found for title B")]
    fn test_validate_options_missing_marker() {
        let markers = vec![fixtures::create_marker("A", 0.0, 30.0, 0)];
        let options = WeightedRandomClipOptions {
            weights: vec![("B".to_string(), 1.0)],
            clip_lengths: PmvClipOptions::Randomized(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
            length: 30.0,
        };
        let weight_labels = vec!["B"].into_iter().collect();

        validate_options(&markers, &options, &weight_labels);
    }

    #[test]
    #[should_panic(expected = "all markers must have a weight")]
    fn test_validate_options_missing_weight() {
        let markers = vec![fixtures::create_marker("A", 0.0, 30.0, 0)];
        let options = WeightedRandomClipOptions {
            weights: vec![],
            clip_lengths: PmvClipOptions::Randomized(RandomizedClipOptions {
                base_duration: 30.0,
                divisors: vec![2.0, 3.0, 4.0],
            }),
            length: 30.0,
        };
        let weight_labels = vec![].into_iter().collect();

        validate_options(&markers, &options, &weight_labels);
    }
}
