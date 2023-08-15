use std::fmt::Debug;

use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::Rng;
use tracing::{debug, info};

use super::MIN_DURATION;
use crate::server::types::{Beats, MeasureCount, PmvClipOptions};

#[derive(Debug)]
pub struct RandomizedClipLengthPicker<'a> {
    rng: &'a mut StdRng,
    divisors: Vec<f64>,
    base_duration: f64,

    total_duration: f64,
    current_duration: f64,
}

impl<'a> RandomizedClipLengthPicker<'a> {
    pub fn new(
        rng: &'a mut StdRng,
        divisors: Vec<f64>,
        base_duration: f64,
        total_duration: f64,
    ) -> Self {
        assert!(!divisors.is_empty(), "divisors must not be empty");

        Self {
            rng,
            divisors,
            base_duration,
            total_duration,
            current_duration: 0.0,
        }
    }
}

impl<'a> Iterator for RandomizedClipLengthPicker<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining_duration = self.total_duration - self.current_duration;
        if remaining_duration > 0.0 {
            let time = self
                .divisors
                .iter()
                .map(|d| (self.base_duration / d).max(MIN_DURATION))
                .choose(self.rng)
                .unwrap();
            self.current_duration += time;

            Some(time)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct SongClipLengthPicker<'a> {
    rng: &'a mut StdRng,
    songs: Vec<Beats>,
    beats_per_measure: usize,
    cut_after_measure_count: MeasureCount,
    song_index: usize,
    beat_index: usize,
}

impl<'a> SongClipLengthPicker<'a> {
    pub fn new(
        rng: &'a mut StdRng,
        mut songs: Vec<Beats>,
        beats_per_measure: usize,
        cut_after_measure_count: MeasureCount,
    ) -> Self {
        assert!(!songs.is_empty(), "songs must not be empty");

        for beats in &mut songs {
            if beats.offsets.first() != Some(&0.0) {
                beats.offsets.insert(0, 0.0);
            }

            if beats.offsets.last() != Some(&beats.length) {
                beats.offsets.push(beats.length);
            }
        }

        Self {
            rng,
            songs,
            beats_per_measure,
            cut_after_measure_count,
            song_index: 0,
            beat_index: 0,
        }
    }
}

impl<'a> Iterator for SongClipLengthPicker<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        debug!(
            "state: song_index = {}, beat_index = {}",
            self.song_index, self.beat_index
        );
        if self.song_index >= self.songs.len() {
            info!(
                "no more songs to pick from, stopping (song index = {}, len = {})",
                self.song_index,
                self.songs.len()
            );
            return None;
        }

        let beats = &self.songs[self.song_index].offsets;
        let num_measures = match self.cut_after_measure_count {
            MeasureCount::Fixed { count } => count,
            MeasureCount::Random { min, max } => self.rng.gen_range(min..max),
        };
        let num_beats_to_advance = self.beats_per_measure * num_measures;
        let next_beat_index = (self.beat_index + num_beats_to_advance).min(beats.len() - 1);
        let start = beats[self.beat_index];
        let end = beats[next_beat_index];
        let duration = (end - start) as f64;

        debug!("advancing by {num_beats_to_advance} beats, next clip from {start} - {end} seconds ({duration} seconds long)");
        debug!(
            "next beat index: {}, number of beats: {}",
            next_beat_index,
            beats.len()
        );

        if next_beat_index == beats.len() - 1 {
            self.song_index += 1;
            self.beat_index = 0;
        } else {
            self.beat_index = next_beat_index;
        }
        Some(duration)
    }
}

#[derive(Debug)]
pub enum ClipLengthPicker<'a> {
    Randomized(RandomizedClipLengthPicker<'a>),
    Songs(SongClipLengthPicker<'a>),
}

impl<'a> ClipLengthPicker<'a> {
    pub fn new(options: PmvClipOptions, total_duration: f64, rng: &'a mut StdRng) -> Self {
        match options {
            PmvClipOptions::Randomized(options) => {
                ClipLengthPicker::Randomized(RandomizedClipLengthPicker::new(
                    rng,
                    options.divisors,
                    options.base_duration,
                    total_duration,
                ))
            }
            PmvClipOptions::Songs(options) => ClipLengthPicker::Songs(SongClipLengthPicker::new(
                rng,
                options.songs,
                options.beats_per_measure,
                options.cut_after_measures,
            )),
        }
    }

    pub fn durations(self) -> Vec<f64> {
        match self {
            ClipLengthPicker::Randomized(picker) => picker.collect(),
            ClipLengthPicker::Songs(picker) => picker.collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use ordered_float::OrderedFloat;
    use tracing_test::traced_test;

    use crate::server::types::{Beats, MeasureCount};
    use crate::service::clip::length_picker::{RandomizedClipLengthPicker, SongClipLengthPicker};
    use crate::service::fixtures;
    use crate::util::create_seeded_rng;

    #[traced_test]
    #[test]
    fn clip_lengths_beats() {
        let mut rng = create_seeded_rng(None);
        let beats = vec![
            Beats {
                length: 250.0,
                offsets: (0..250).into_iter().map(|n| n as f32).collect(),
            },
            Beats {
                length: 250.0,
                offsets: (0..250).into_iter().map(|n| n as f32).collect(),
            },
        ];
        let songs = SongClipLengthPicker::new(&mut rng, beats, 4, MeasureCount::Fixed { count: 1 });
        let durations: Vec<_> = songs.collect();
        assert_eq!(126, durations.len());
    }

    #[traced_test]
    #[test]
    fn clip_lengths_beats_randomized() {
        let mut rng = create_seeded_rng(None);
        let beats = vec![
            Beats {
                length: 10.0,
                offsets: (0..10).into_iter().map(|n| n as f32).collect(),
            },
            Beats {
                length: 10.0,
                offsets: (0..10).into_iter().map(|n| n as f32).collect(),
            },
        ];
        let songs =
            SongClipLengthPicker::new(&mut rng, beats, 4, MeasureCount::Random { min: 1, max: 3 });
        let durations: Vec<_> = songs.collect();
        assert_eq!(vec![4.0, 4.0, 2.0, 4.0, 4.0, 2.0], durations);
        assert_eq!(6, durations.len());
        let total_duration = durations.iter().sum::<f64>();
        assert!(
            total_duration >= 20.0,
            "total duration was {} but expected at least 20",
            total_duration
        );
    }

    #[traced_test]
    #[test]
    fn clip_lengths_songs() {
        let mut rng = create_seeded_rng(None);
        let songs = fixtures::songs();
        let expected_duration: f64 = songs.iter().map(|s| s.length as f64).sum();
        let state = SongClipLengthPicker::new(&mut rng, songs, 1, MeasureCount::Fixed { count: 1 });
        let total: f64 = state.sum();
        assert!(
            total >= expected_duration,
            "total duration was {} but expected at least {}",
            total,
            expected_duration
        );
    }

    #[traced_test]
    #[test]
    fn randomized_clip_lengths() {
        let mut rng = create_seeded_rng(None);
        let picker = RandomizedClipLengthPicker::new(&mut rng, vec![2.0, 3.0, 4.0], 30.0, 600.0);
        let durations: Vec<_> = picker.collect();
        let total = durations.iter().sum::<f64>();
        assert!(
            total >= 600.0,
            "total duration was {} but expected at least 600",
            total
        );

        let distinct_values: HashSet<_> =
            durations.iter().map(|n| OrderedFloat::from(*n)).collect();
        assert_eq!(3, distinct_values.len());
    }
}
