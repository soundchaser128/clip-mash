use clip_mash_types::{Clip, PmvClipOptions, RoundRobinClipOptions};
use rand::rngs::StdRng;
use tracing::{debug, info};

use super::length_picker::ClipLengthPicker;
use super::ClipPicker;
use crate::service::clip::state::{MarkerStart, MarkerState};
use crate::service::clip::MIN_DURATION;
use crate::service::Marker;

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
        let mut clip_lengths: ClipLengthPicker = options.clip_lengths.into();
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
                        assert!(
                            end > *start,
                            "end time {} must be greater than start time {}",
                            end,
                            start
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
