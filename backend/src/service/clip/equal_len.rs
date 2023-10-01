use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use tracing::info;

use super::ClipPicker;
use crate::server::types::{Clip, EqualLengthClipOptions};
use crate::service::clip::MIN_DURATION;
use crate::service::Marker;

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
                        source: marker.source,
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
