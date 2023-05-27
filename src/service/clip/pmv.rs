use std::collections::HashMap;
use std::fmt::Debug;

use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::Rng;
use tracing::{debug, info};

use super::{Clip, ClipCreator, Marker};
use crate::service::beats::Beats;
use crate::service::clip::MIN_DURATION;

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
        info!(
            "state: song_index = {}, beat_index = {}",
            self.song_index, self.beat_index
        );
        if self.song_index >= self.songs.len() {
            return None;
        }

        let beats = &self.songs[self.song_index].offsets;
        let num_measures = match self.cut_after_measure_count {
            MeasureCount::Fixed(n) => n,
            MeasureCount::Randomized { min, max } => rng.gen_range(min..max),
        };
        let num_beats_to_advance = self.beats_per_measure * num_measures;
        let next_beat_index = (self.beat_index + num_beats_to_advance).min(beats.len() - 1);
        let start = beats[self.beat_index];
        let end = beats[next_beat_index];

        info!("start = {}, end = {}", self.beat_index, next_beat_index);

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
pub enum MeasureCount {
    Fixed(usize),
    Randomized { min: usize, max: usize },
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

#[derive(Debug)]
pub struct PmvClipCreatorOptions {
    pub seed: Option<String>,
    pub video_duration: f64,
    pub clip_lengths: PmvClipLengths,
}

pub struct PmvClipCreator;

impl ClipCreator for PmvClipCreator {
    type Options = PmvClipCreatorOptions;

    fn create_clips(
        &self,
        markers: Vec<Marker>,
        mut options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip> {
        info!(
            "using PmvClipCreator to create clips, options: {:?}",
            options
        );

        let max_duration = options.video_duration;
        let mut total_duration = 0.0;
        let mut clips = vec![];
        let mut marker_idx = 0;

        let mut start_times: HashMap<i64, (f64, usize)> = markers
            .iter()
            .map(|m| (m.id.inner(), (m.start_time, 0)))
            .collect();

        while total_duration <= max_duration {
            let marker = &markers[marker_idx % markers.len()];
            let clip_duration = options.clip_lengths.pick_duration(rng);
            if clip_duration.is_none() {
                break;
            }
            let clip_duration = clip_duration.unwrap();

            let (start, index) = start_times[&marker.id.inner()];
            let end = (start + clip_duration).min(marker.end_time);
            let duration = end - start;
            if duration >= MIN_DURATION {
                debug!(
                    "adding clip for video {} from {start} - {end}",
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
            for clip in &mut clips {
                clip.range.1 = clip.range.1 - slack;
            }
        }

        clips
    }
}

#[cfg(test)]
mod test {
    use tracing_test::traced_test;

    use super::{MeasureCount, PmvSongs};
    use crate::service::beats::Beats;
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
        let mut songs = PmvSongs::new(beats, 4, MeasureCount::Fixed(1));
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
        let mut songs = PmvSongs::new(beats, 4, MeasureCount::Randomized { min: 1, max: 3 });
        let mut durations = vec![];
        while let Some(duration) = songs.next_duration(&mut rng) {
            durations.push(duration);
        }

        assert_eq!(79, durations.len());
    }
}
