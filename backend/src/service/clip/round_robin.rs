use float_cmp::approx_eq;
use rand::rngs::StdRng;
use tracing::info;

use super::length_picker::ClipLengthPicker;
use super::ClipPicker;
use crate::server::types::{Clip, ClipLengthOptions, RoundRobinClipOptions};
use crate::service::clip::state::{MarkerState, MarkerStateInfo};
use crate::service::clip::trim_clips;
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
        info!("using RoundRobinClipPicker to make clips");
        let song_duration = match &options.clip_lengths {
            ClipLengthOptions::Songs(options) => {
                Some(options.songs.iter().map(|s| s.length as f64).sum())
            }
            _ => None,
        };
        if !options.lenient_duration {
            let marker_duration = markers.iter().map(|m| m.duration()).sum::<f64>();
            assert!(
                marker_duration >= options.length,
                "marker duration {} must be greater or equal to target duration {}",
                marker_duration,
                options.length
            );
        }

        let max_duration = options.length;
        let mut clips = vec![];
        let mut marker_idx = 0;
        let has_music = matches!(options.clip_lengths, ClipLengthOptions::Songs(_));
        let min_duration = options.min_clip_duration.unwrap_or(1.5);
        let clip_lengths =
            ClipLengthPicker::new(options.clip_lengths, max_duration, min_duration, rng);
        let clip_lengths = clip_lengths.durations();
        info!("clip lengths: {:?}", clip_lengths);

        let mut marker_state = MarkerState::new(markers, clip_lengths, options.length);

        while !marker_state.finished() {
            // info!("marker state: {marker_state:#?}");

            if let Some(MarkerStateInfo {
                start,
                end,
                marker,
                skipped_duration,
            }) = marker_state.find_marker_by_index(marker_idx)
            {
                assert!(
                    end >= start,
                    "end time {} must be greater than start time {}",
                    end,
                    start
                );
                let duration = end - start;
                if (has_music && duration > 0.0) || (!has_music && duration >= min_duration) {
                    info!(
                        "adding clip for video {} with duration {duration} (skipped {skipped_duration}) and title {}",
                        marker.video_id, marker.title
                    );

                    clips.push(Clip {
                        index_within_marker: marker_idx,
                        index_within_video: marker.index_within_video,
                        marker_id: marker.id,
                        range: (start, end),
                        source: marker.source,
                        video_id: marker.video_id.clone(),
                        marker_title: marker.title.clone(),
                    });
                }

                marker_state.update(marker.id, end, duration, skipped_duration);
            }
            marker_idx += 1;
        }

        let clips_duration: f64 = clips.iter().map(|c| c.duration()).sum();
        if let Some(song_duration) = song_duration {
            assert!(
                approx_eq!(f64, clips_duration, song_duration) || clips_duration >= song_duration,
                "clips duration {} must be greater or equal to song duration {}",
                clips_duration,
                song_duration
            )
        }

        trim_clips(&mut clips, options.length);

        clips
    }
}

#[cfg(test)]
mod test {
    use float_cmp::assert_approx_eq;
    use tracing_test::traced_test;

    use crate::server::types::{
        Beats, ClipLengthOptions, MeasureCount, RoundRobinClipOptions, SongClipOptions,
    };
    use crate::service::clip::round_robin::RoundRobinClipPicker;
    use crate::service::clip::ClipPicker;
    use crate::service::fixtures;
    use crate::util::create_seeded_rng;

    #[traced_test]
    #[test]
    fn test_songs_clips_too_short() {
        let songs = fixtures::songs();
        let song_duration = songs.iter().map(|s| s.length as f64).sum();

        let options = RoundRobinClipOptions {
            length: song_duration,
            clip_lengths: ClipLengthOptions::Songs(SongClipOptions {
                beats_per_measure: 4,
                cut_after_measures: MeasureCount::Fixed { count: 4 },
                songs,
            }),
            lenient_duration: false,
            min_clip_duration: None,
        };
        let markers = fixtures::markers();
        let mut rng = create_seeded_rng(None);
        let mut picker = RoundRobinClipPicker;
        let _clips = picker.pick_clips(markers, options, &mut rng);
    }

    #[traced_test]
    #[test]
    fn test_songs_clips_simple() {
        let songs = vec![
            Beats {
                length: 10.0,
                offsets: (0..10).map(|n| n as f32).collect(),
            },
            Beats {
                length: 10.0,
                offsets: (0..10).map(|n| n as f32).collect(),
            },
        ];
        let song_duration = songs.iter().map(|s| s.length as f64).sum();

        let options = RoundRobinClipOptions {
            length: song_duration,
            clip_lengths: ClipLengthOptions::Songs(SongClipOptions {
                beats_per_measure: 4,
                cut_after_measures: MeasureCount::Fixed { count: 4 },
                songs,
            }),
            lenient_duration: false,
            min_clip_duration: None,
        };

        let markers = fixtures::markers();
        let mut rng = create_seeded_rng(None);
        let mut picker = RoundRobinClipPicker;
        let clips = picker.pick_clips(markers, options, &mut rng);
        let clip_duration = clips.iter().map(|c| c.duration()).sum::<f64>();

        assert_approx_eq!(f64, clip_duration, song_duration, epsilon = 0.01);
    }
}
