use rand::rngs::StdRng;
use rand::seq::IndexedRandom;
use tracing::{debug, info};

use super::ClipPicker;
use crate::server::types::{Clip, EqualLengthClipOptions};
use crate::service::Marker;
use crate::service::clip::{get_divisors, trim_clips};

pub struct EqualLengthClipPicker;

impl ClipPicker for EqualLengthClipPicker {
    type Options = EqualLengthClipOptions;

    fn pick_clips(
        &mut self,
        markers: Vec<Marker>,
        options: Self::Options,
        rng: &mut StdRng,
    ) -> Vec<Clip> {
        info!("using EqualLengthClipPicker to make clips: {options:?}");
        let min_duration = options.min_clip_duration.unwrap_or(1.5);

        let duration = options.clip_duration;
        let divisors = get_divisors(options.spread);
        let clip_lengths: Vec<f64> = divisors
            .into_iter()
            .map(|d| (duration / d).max(min_duration))
            .collect();
        let mut clips = vec![];
        let mut len = 0.0;
        for marker in markers {
            let start = marker.start_time;
            let end = marker.end_time;

            if let Some(max_len) = options.length {
                if len >= max_len {
                    trim_clips(&mut clips, max_len);
                    break;
                }
            }

            let mut index = 0;
            let mut offset = start;
            while offset < end {
                let duration = clip_lengths.choose(rng).unwrap();
                let start = offset;
                let end = (offset + duration).min(end);
                let duration = end - start;
                if duration > min_duration {
                    debug!(
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
                        marker_title: marker.title.clone(),
                    });
                    index += 1;
                    len += duration;
                }
                offset += duration;
            }
        }

        clips
    }
}
