use super::{Clip, ClipCreator, Marker};
use crate::service::clip::MIN_DURATION;
use rand::{rngs::StdRng, seq::SliceRandom};

use std::{collections::HashMap, fmt::Debug};
use tracing::{debug, info};

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
            (duration / 2.0).max(MIN_DURATION),
            (duration / 3.0).max(MIN_DURATION),
            (duration / 4.0).max(MIN_DURATION),
        ];

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
            let clip_duration = clip_lengths.choose(rng).expect("must find one element");

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
