use crate::util;
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use serde::Deserialize;

use super::{Clip, Marker, Video};

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClipOrder {
    Random,
    SceneOrder,
}

pub struct MarkerWithClips {
    pub marker: Marker,
    pub clips: Vec<Clip>,
}

pub fn get_clips(
    marker: &Marker,
    options: &CreateClipsOptions,
    rng: &mut StdRng,
) -> MarkerWithClips {
    let clip_lengths = [
        (options.clip_duration / 2).max(2) as f64,
        (options.clip_duration / 3).max(2) as f64,
        (options.clip_duration / 4).max(2) as f64,
    ];

    let start = marker.start_time;
    let end = marker.end_time;

    let mut index = 0;
    let mut offset = start;
    let mut clips = vec![];
    while offset < end {
        let duration = clip_lengths.choose(rng).unwrap();
        clips.push(Clip {
            source: options.video.source,
            video_id: options.video.id,
            marker_id: marker.id,
            range: (offset, offset + duration),
            index_within_marker: index,
            index_within_video: marker.index_within_video,
        });
        offset += duration;
        index += 1;
    }

    todo!()
}

#[derive(Debug)]
pub struct CreateClipsOptions {
    pub clip_duration: u32,
    pub clip_order: ClipOrder,
    pub markers: Vec<Marker>,
    pub video: Video,
    pub split_clips: bool,
    pub max_duration: Option<u32>,
}

pub fn get_all_clips(options: &CreateClipsOptions) -> Vec<MarkerWithClips> {
    let mut rng = util::create_seeded_rng();
    tracing::debug!("creating clips for options {options:?}");
    options
        .markers
        .iter()
        .map(|marker| {
            if options.split_clips {
                get_clips(marker, &options, &mut rng)
            } else {
                MarkerWithClips {
                    marker: marker.clone(),
                    clips: vec![Clip {
                        source: options.video.source,
                        video_id: options.video.id,
                        marker_id: marker.id,
                        range: (marker.start_time, marker.end_time),
                        index_within_marker: 0,
                        index_within_video: marker.index_within_video,
                    }],
                }
            }
        })
        .collect()
}

pub fn compile_clips(clips: Vec<MarkerWithClips>, order: ClipOrder) -> Vec<Clip> {
    let mut rng = util::create_seeded_rng();

    match order {
        ClipOrder::SceneOrder => {
            let mut clips: Vec<_> = clips
                .into_iter()
                .flat_map(|m| m.clips)
                .map(|c| (c, rng.gen::<u32>()))
                .collect();
            // TODO parameter to control order by
            clips.sort_by_key(|(clip, random)| (clip.index_within_video, *random));
            clips.into_iter().map(|(clip, _)| clip).collect()
        }
        ClipOrder::Random => {
            let mut clips: Vec<_> = clips.into_iter().flat_map(|c| c.clips).collect();
            clips.shuffle(&mut rng);
            clips
        }
    }
}
