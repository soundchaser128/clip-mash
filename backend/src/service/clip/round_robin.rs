use clip_mash_types::{Clip, PmvClipOptions, RoundRobinClipOptions};
use rand::rngs::StdRng;
use tracing::info;

use super::length_picker::ClipLengthPicker;
use super::ClipPicker;
use crate::service::clip::state::{MarkerState, MarkerStateInfo};
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
        info!("using RoundRobinClipPicker to make clips");

        let song_duration = match &options.clip_lengths {
            PmvClipOptions::Songs(options) => {
                Some(options.songs.iter().map(|s| s.length as f64).sum())
            }
            _ => None,
        };
        let marker_duration = markers.iter().map(|m| m.duration()).sum::<f64>();
        assert!(
            marker_duration >= options.length,
            "marker duration {} must be greater or equal to target duration {}",
            marker_duration,
            options.length
        );

        let max_duration = options.length;
        let mut clips = vec![];
        let mut marker_idx = 0;
        let has_music = matches!(options.clip_lengths, PmvClipOptions::Songs(_));
        let clip_lengths = ClipLengthPicker::new(options.clip_lengths, max_duration, rng);
        let clip_lengths = clip_lengths.durations();
        info!("clip lengths: {:?}", clip_lengths);

        let mut marker_state = MarkerState::new(markers, clip_lengths, options.length);

        while !marker_state.finished() {
            if let Some(MarkerStateInfo {
                start,
                end,
                marker,
                skipped_duration,
            }) = marker_state.find_marker_by_index(marker_idx)
            {
                let duration = end - start;
                if has_music || duration >= MIN_DURATION {
                    info!(
                        "adding clip for video {} with duration {duration} (skipped {skipped_duration}) and title {}",
                        marker.video_id, marker.title
                    );
                    assert!(
                        end > start,
                        "end time {} must be greater than start time {}",
                        end,
                        start
                    );
                    clips.push(Clip {
                        index_within_marker: marker_idx,
                        index_within_video: marker.index_within_video,
                        marker_id: marker.id,
                        range: (start, end),
                        source: marker.video_id.source(),
                        video_id: marker.video_id.clone(),
                    });
                }

                marker_state.update(&marker.id, end, duration, skipped_duration);
                marker_idx += 1;
            }
        }

        let clips_duration: f64 = clips.iter().map(|c| c.duration()).sum();
        if let Some(song_duration) = song_duration {
            assert!(
                clips_duration >= song_duration,
                "clips duration {} must be greater or equal to song duration {}",
                clips_duration,
                song_duration
            )
        }

        if clips_duration > max_duration {
            let slack = (clips_duration - max_duration) / clips.len() as f64;
            info!("clip duration {clips_duration} longer than permitted maximum duration {max_duration}, making each clip {slack} shorter");
            for clip in &mut clips {
                clip.range.1 -= slack;
            }
        }

        clips
    }
}

#[cfg(test)]
mod test {
    use clip_mash_types::{
        Beats, MeasureCount, PmvClipOptions, RoundRobinClipOptions, SongClipOptions,
    };
    use float_cmp::assert_approx_eq;
    use tracing_test::traced_test;

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
            clip_lengths: PmvClipOptions::Songs(SongClipOptions {
                beats_per_measure: 4,
                cut_after_measures: MeasureCount::Fixed { count: 4 },
                songs,
            }),
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
                offsets: (0..10).into_iter().map(|n| n as f32).collect(),
            },
            Beats {
                length: 10.0,
                offsets: (0..10).into_iter().map(|n| n as f32).collect(),
            },
        ];
        let song_duration = songs.iter().map(|s| s.length as f64).sum();

        let options = RoundRobinClipOptions {
            length: song_duration,
            clip_lengths: PmvClipOptions::Songs(SongClipOptions {
                beats_per_measure: 4,
                cut_after_measures: MeasureCount::Fixed { count: 4 },
                songs,
            }),
        };

        let markers = fixtures::markers();
        let mut rng = create_seeded_rng(None);
        let mut picker = RoundRobinClipPicker;
        let clips = picker.pick_clips(markers, options, &mut rng);
        let clip_duration = clips.iter().map(|c| c.duration()).sum::<f64>();

        assert_approx_eq!(f64, clip_duration, song_duration, epsilon = 0.01);
    }
}
