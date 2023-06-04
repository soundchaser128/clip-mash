use std::fmt::Debug;

use clip_mash_types::{Beats, MeasureCount, PmvClipOptions};
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::Rng;
use tracing::debug;

const MIN_DURATION: f64 = 1.5;

#[derive(Debug)]
pub struct PmvSongs {
    pub songs: Vec<Beats>,
    pub beats_per_measure: usize,
    pub cut_after_measure_count: MeasureCount,

    song_index: usize,
    beat_index: usize,
}

impl PmvSongs {
    pub fn new(
        songs: Vec<Beats>,
        beats_per_measure: usize,
        cut_after_measure_count: MeasureCount,
    ) -> Self {
        Self {
            songs,
            beats_per_measure,
            cut_after_measure_count,
            song_index: 0,
            beat_index: 0,
        }
    }

    pub fn next_duration(&mut self, rng: &mut StdRng) -> Option<f64> {
        debug!(
            "state: song_index = {}, beat_index = {}",
            self.song_index, self.beat_index
        );
        if self.song_index >= self.songs.len() {
            return None;
        }

        let beats = &self.songs[self.song_index].offsets;
        let num_measures = match self.cut_after_measure_count {
            MeasureCount::Fixed { count } => count,
            MeasureCount::Random { min, max } => rng.gen_range(min..max),
        };
        let num_beats_to_advance = self.beats_per_measure * num_measures;
        let next_beat_index = (self.beat_index + num_beats_to_advance).min(beats.len() - 1);
        let start = beats[self.beat_index];
        let end = beats[next_beat_index];

        debug!("advancing by {num_beats_to_advance} beats, next clip from {start} - {end} seconds");

        if next_beat_index == beats.len() - 1 {
            self.song_index += 1;
            self.beat_index = 0;
        } else {
            self.beat_index = next_beat_index;
        }

        Some((end - start) as f64)
    }
}

#[derive(Debug)]
pub enum PmvClipLengths {
    Randomized {
        base_duration: f64,
        divisors: Vec<f64>,
    },
    Songs(PmvSongs),
}

impl PmvClipLengths {
    pub fn pick_duration(&mut self, rng: &mut StdRng) -> Option<f64> {
        match self {
            PmvClipLengths::Randomized {
                base_duration,
                divisors,
            } => divisors
                .iter()
                .map(|d| (*base_duration / *d).max(MIN_DURATION))
                .choose(rng),
            PmvClipLengths::Songs(songs) => songs.next_duration(rng),
        }
    }
}

impl From<PmvClipOptions> for PmvClipLengths {
    fn from(value: PmvClipOptions) -> Self {
        match value {
            PmvClipOptions::Randomized(options) => PmvClipLengths::Randomized {
                base_duration: options.base_duration,
                divisors: options.divisors,
            },
            PmvClipOptions::Songs(options) => PmvClipLengths::Songs(PmvSongs::new(
                options.songs,
                options.beats_per_measure,
                options.cut_after_measures,
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use clip_mash_types::{Beats, MeasureCount};
    use tracing_test::traced_test;

    use super::PmvSongs;
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
        let mut songs = PmvSongs::new(beats, 4, MeasureCount::Fixed { count: 1 });
        let mut durations = vec![];
        while let Some(duration) = songs.next_duration(&mut rng) {
            durations.push(duration);
        }

        assert_eq!(126, durations.len());
    }

    #[traced_test]
    #[test]
    fn clip_lengths_beats_randomized() {
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
        let mut songs = PmvSongs::new(beats, 4, MeasureCount::Random { min: 1, max: 3 });
        let mut durations = vec![];
        while let Some(duration) = songs.next_duration(&mut rng) {
            durations.push(duration);
        }

        assert_eq!(79, durations.len());
    }
}
