use std::collections::HashMap;

use clip_mash_types::Clip;
use rand::{
    distributions::WeightedIndex, prelude::Distribution, rngs::StdRng, seq::IteratorRandom,
};
use tracing::info;

use crate::service::Marker;

use super::pmv::PmvClipLengths;

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

pub struct RoundRobinClipPickerOptions {
    pub length: f64,
    pub clip_lengths: PmvClipLengths,
}

pub struct RoundRobinClipPicker;

impl ClipPicker for RoundRobinClipPicker {
    type Options = RoundRobinClipPickerOptions;

    fn pick_clips(
        &mut self,
        markers: Vec<Marker>,
        mut options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip> {
        let mut marker_idx = 0;
        let mut start_times: HashMap<i64, (f64, usize)> = markers
            .iter()
            .map(|m| (m.id.inner(), (m.start_time, 0)))
            .collect();
        let mut clips = vec![];
        let mut total_duration = 0.0;
        let has_music = matches!(options.clip_lengths, PmvClipLengths::Songs(_));

        while total_duration <= options.length {
            let marker = &markers[marker_idx % markers.len()];
            let clip_duration = options.clip_lengths.pick_duration(rng);
            if clip_duration.is_none() {
                break;
            }
            let clip_duration = clip_duration.unwrap();

            let (start, index) = start_times[&marker.id.inner()];
            let end = (start + clip_duration).min(marker.end_time);
            let duration = end - start;
            if has_music || duration >= MIN_DURATION {
                info!(
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

        clips
    }
}

pub struct WeightedRandomClipPicker;

pub struct WeightedRandomClipPickerOptions {
    pub weights: HashMap<String, f64>,
    pub length: f64,
    pub clip_lengths: PmvClipLengths,
}

impl ClipPicker for WeightedRandomClipPicker {
    type Options = WeightedRandomClipPickerOptions;

    fn pick_clips(
        &mut self,
        markers: Vec<Marker>,
        mut options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip> {
        let choices: Vec<(String, f64)> = options.weights.into_iter().collect();
        let distribution = WeightedIndex::new(choices.iter().map(|item| item.1))
            .expect("could not build distribution");
        let mut total_duration = 0.0;
        let mut start_times: HashMap<i64, (f64, usize)> = markers
            .iter()
            .map(|m| (m.id.inner(), (m.start_time, 0)))
            .collect();
        let mut clips = vec![];

        while total_duration <= options.length {
            let marker_tag = &choices[distribution.sample(rng)].0;
            let next_marker = markers
                .iter()
                .filter(|m| &m.title == marker_tag)
                .choose(rng);
            if let Some(marker) = next_marker {
                let (start, index) = start_times[&marker.id.inner()];
                let clip_duration = options.clip_lengths.pick_duration(rng);
                if clip_duration.is_none() {
                    break;
                }
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

                start_times.insert(marker.id.inner(), (end, index + 1));
                total_duration += duration;
            }
        }

        clips
    }
}

pub struct EqualLengthClipPicker;

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_weighted_random_clips() {
        
    }
}