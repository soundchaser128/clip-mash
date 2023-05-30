use std::fmt::Debug;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use tracing::{debug, info};

use super::{Clip, ClipCreator, Marker};

// Specifc minimum duration for default clip generation
const MIN_DURATION: f64 = 1.5;

#[derive(Debug)]
pub struct DefaultClipOptions {
    pub clip_duration: u32,
    pub seed: Option<String>,
}

pub struct DefaultClipCreator;

impl ClipCreator for DefaultClipCreator {
    type Options = DefaultClipOptions;

    fn create_clips(
        &self,
        markers: Vec<Marker>,
        options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip> {
        info!("using DefaultClipCreator to create clips, options: {options:#?}",);
        let duration = options.clip_duration as f64;
        let clip_lengths = [
            (duration / 1.0).max(MIN_DURATION),
            (duration / 2.0).max(MIN_DURATION),
            (duration / 3.0).max(MIN_DURATION),
            (duration / 4.0).max(MIN_DURATION),
        ];
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
