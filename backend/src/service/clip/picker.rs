use std::collections::hash_map::Entry;
use std::collections::HashMap;

use clip_mash_types::MarkerId;
use float_cmp::approx_eq;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;

use crate::service::Marker;

#[derive(Debug)]
pub struct MarkerStart {
    pub start_time: f64,
    pub end_time: f64,
    pub index: usize,
}

#[derive(Debug)]
pub struct MarkerState {
    pub data: HashMap<i64, MarkerStart>,
    pub markers: Vec<Marker>,
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

    pub fn find_marker_by_index(&self, index: usize) -> Option<Marker> {
        if self.markers.is_empty() {
            None
        } else {
            self.markers.get(index % self.markers.len()).cloned()
        }
    }

    pub fn find_marker_by_title(&self, title: &str, rng: &mut StdRng) -> Option<Marker> {
        self.markers
            .iter()
            .filter(|m| &m.title == title)
            .choose(rng)
            .cloned()
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
}
