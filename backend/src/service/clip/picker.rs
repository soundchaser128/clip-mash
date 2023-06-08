use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use clip_mash_types::{
    Clip, EqualLengthClipOptions, MarkerId, PmvClipOptions, RoundRobinClipOptions,
    WeightedRandomClipOptions,
};
use float_cmp::approx_eq;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::rngs::StdRng;
use rand::seq::{IteratorRandom, SliceRandom};
use tracing::{debug, info};

use super::pmv::PmvClipLengths;
use crate::service::Marker;

const MIN_DURATION: f64 = 1.5;

#[derive(Debug)]
struct MarkerStart {
    start_time: f64,
    end_time: f64,
    index: usize,
}

#[derive(Debug)]
struct MarkerState {
    data: HashMap<i64, MarkerStart>,
    markers: Vec<Marker>,
}

impl MarkerState {
    pub fn new(data: Vec<Marker>) -> Self {
        Self {
            data: data
                .iter()
                .map(|m| {
                    (
                        m.id.inner(),
                        MarkerStart {
                            start_time: m.start_time,
                            end_time: m.end_time,
                            index: 0,
                        },
                    )
                })
                .collect(),
            markers: data,
        }
    }

    pub fn get(&self, id: &MarkerId) -> Option<&MarkerStart> {
        self.data.get(&id.inner())
    }

    pub fn update(&mut self, id: &MarkerId, start_time: f64, index: usize) {
        let entry = self.data.entry(id.inner()).and_modify(|e| {
            e.start_time = start_time;
            e.index = index;
        });

        if let Entry::Occupied(e) = entry {
            if approx_eq!(f64, e.get().end_time, start_time, epsilon = 0.001) {
                e.remove();
                let index = self.markers.iter().position(|m| m.id == *id).unwrap();
                self.markers.remove(index);
            }
        }
    }

    fn find_marker_by_index(&self, index: usize) -> Option<Marker> {
        if self.markers.is_empty() {
            None
        } else {
            self.markers.get(index % self.markers.len()).cloned()
        }
    }

    fn find_marker_by_title(&self, title: &str, rng: &mut StdRng) -> Option<Marker> {
        self.markers
            .iter()
            .filter(|m| &m.title == title)
            .choose(rng)
            .cloned()
    }
}

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

        let max_duration = options.length;
        let mut total_duration = 0.0;
        let mut clips = vec![];
        let mut marker_idx = 0;
        let has_music = matches!(options.clip_lengths, PmvClipOptions::Songs(_));
        let mut clip_lengths: PmvClipLengths = options.clip_lengths.into();
        let mut marker_state = MarkerState::new(markers);

        while total_duration <= options.length {
            debug!("marker state: {marker_state:#?}, total duration: {total_duration}, target duration: {}", options.length);
            if marker_state.markers.is_empty() {
                info!("no more markers to pick from, stopping");
                break;
            }

            if let Some(marker) = marker_state.find_marker_by_index(marker_idx) {
                if let Some(MarkerStart {
                    start_time: start,
                    index,
                    ..
                }) = marker_state.get(&marker.id)
                {
                    let clip_duration = clip_lengths.pick_duration(rng);
                    if clip_duration.is_none() {
                        break;
                    }
                    let clip_duration = clip_duration.unwrap();
                    let end = (start + clip_duration).min(marker.end_time);
                    let duration = end - start;
                    if has_music || duration >= MIN_DURATION {
                        info!(
                            "adding clip for video {} with duration {duration} and title {}",
                            marker.video_id, marker.title
                        );
                        clips.push(Clip {
                            index_within_marker: *index,
                            index_within_video: marker.index_within_video,
                            marker_id: marker.id,
                            range: (*start, end),
                            source: marker.video_id.source(),
                            video_id: marker.video_id.clone(),
                        });
                    }

                    total_duration += duration;
                    marker_idx += 1;
                    marker_state.update(&marker.id, end, index + 1);
                }
            }
        }

        let clips_duration: f64 = clips.iter().map(|c| c.duration()).sum();
        if clips_duration > max_duration {
            let slack = (clips_duration - max_duration) / clips.len() as f64;
            info!("clip duration {clips_duration} longer than permitted maximum duration {max_duration}, making each clip {slack} shorter");
            for clip in &mut clips {
                clip.range.1 = clip.range.1 - slack;
            }
        }

        clips
    }
}

pub struct WeightedRandomClipPicker;

impl WeightedRandomClipPicker {
    fn validate_options(&self, markers: &[Marker], options: &WeightedRandomClipOptions) {
        for (title, weight) in &options.weights {
            assert!(
                *weight > 0.0,
                "weight for title {} must be greater than 0",
                title
            );
            let marker_count = markers.iter().filter(|m| &m.title == title).count();
            assert!(marker_count > 0, "no markers found for title {}", title);

            let weights_exist = markers
                .iter()
                .all(|m| options.weights.iter().any(|(t, _)| t == &m.title));
            assert!(weights_exist);
        }
    }
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
        let weight_labels: HashSet<_> = options.weights.iter().map(|(label, _)| label).collect();
        markers.retain(|m| weight_labels.contains(&m.title));

        self.validate_options(&markers, &options);
        let choices = options.weights;

        let distribution = WeightedIndex::new(choices.iter().map(|item| item.1))
            .expect("could not build distribution");
        let mut total_duration = 0.0;
        let mut marker_state = MarkerState::new(markers);
        let mut clips = vec![];
        let mut clip_lengths: PmvClipLengths = options.clip_lengths.into();

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

            let mut index = 0;
            let mut offset = start;
            while offset < end {
                let duration = clip_lengths.choose(rng).unwrap();
                let start = offset;
                let end = (offset + duration).min(end);
                let duration = end - start;
                if duration > MIN_DURATION {
                    info!(
                        "adding clip for video {} with duration {duration} and title {}",
                        marker.video_id, marker.title
                    );
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
    use std::collections::HashSet;

    use clip_mash_types::{
        PmvClipOptions, RandomizedClipOptions, RoundRobinClipOptions, WeightedRandomClipOptions,
    };
    use float_cmp::assert_approx_eq;
    use tracing_test::traced_test;

    use super::RoundRobinClipPicker;
    use crate::service::clip::picker::{ClipPicker, WeightedRandomClipPicker};
    use crate::service::fixtures::{self, other_markers};
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
        let length = options.length;
        let markers = other_markers();
        let mut rng = create_seeded_rng(None);
        let mut picker = WeightedRandomClipPicker;
        let clips = picker.pick_clips(markers, options, &mut rng);
        let clip_duration: f64 = clips.iter().map(|c| c.duration()).sum();
        assert!(clip_duration >= 0.0);
    }
}
