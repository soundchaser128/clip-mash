use crate::{
    http::{CreateClipsBody, FilterMode},
    stash_api::Marker,
    util,
};
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClipOrder {
    Random,
    SceneOrder,
}

pub struct ClipSettings {
    pub order: ClipOrder,
    pub max_clip_length: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub scene_id: String,
    pub marker_id: String,
    /// Start and endpoint inside the video in seconds.
    pub range: (u32, u32),
    pub marker_index: usize,
    pub scene_index: usize,
}

impl Clip {
    pub fn range_millis(&self) -> (u32, u32) {
        (self.range.0 * 1000, self.range.1 * 1000)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkerWithClips {
    pub marker: Marker,
    pub clips: Vec<Clip>,
}

pub fn get_time_range(marker: &Marker, max_duration: Option<u32>) -> (u32, Option<u32>) {
    let start = marker.start;
    let next_marker = marker
        .scene
        .scene_markers
        .iter()
        .find(|m| m.start > marker.start);
    if let Some(max_duration) = max_duration {
        (start, Some(start + max_duration))
    } else if let Some(next) = next_marker {
        (start, Some(next.start))
    } else {
        (start, None)
    }
}

pub fn get_clips(
    marker: &Marker,
    settings: &ClipSettings,
    max_duration: Option<u32>,
    rng: &mut StdRng,
) -> MarkerWithClips {
    let clip_lengths = [
        (settings.max_clip_length / 2).max(2),
        (settings.max_clip_length / 3).max(2),
        (settings.max_clip_length / 4).max(2),
    ];

    let (start, end) = get_time_range(marker, max_duration);
    let end = end.unwrap_or(start + settings.max_clip_length);

    let mut index = 0;
    let mut offset = start;
    let mut clips = vec![];
    while offset < end {
        let duration = clip_lengths.choose(rng).unwrap();
        clips.push(Clip {
            scene_id: marker.scene.id.clone(),
            marker_id: marker.id.clone(),
            range: (offset, offset + duration),
            marker_index: index,
            scene_index: marker.index_in_scene,
        });
        offset += duration;
        index += 1;
    }

    MarkerWithClips {
        clips,
        marker: marker.clone(),
    }
}

pub fn get_all_clips(output: &CreateClipsBody) -> Vec<MarkerWithClips> {
    let mut rng = util::create_seeded_rng();
    let settings = ClipSettings {
        max_clip_length: output.clip_duration,
        order: output.clip_order,
    };
    tracing::debug!("creating clips for options {output:?}");
    output
        .markers
        .iter()
        .filter(|m| output.selected_markers.iter().any(|c| c.id == m.id))
        .map(|marker| {
            let selected_marker = output
                .selected_markers
                .iter()
                .find(|c| c.id == marker.id)
                .unwrap();
            if output.split_clips {
                get_clips(marker, &settings, selected_marker.duration, &mut rng)
            } else {
                MarkerWithClips {
                    marker: marker.clone(),
                    clips: vec![Clip {
                        marker_id: marker.id.clone(),
                        scene_id: marker.scene.id.clone(),
                        marker_index: marker.index_in_scene,
                        scene_index: 0,
                        range: (
                            marker.start,
                            marker.start + selected_marker.duration.or(marker.end).unwrap_or(15),
                        ),
                    }],
                }
            }
        })
        .collect()
}

pub fn compile_clips(clips: Vec<MarkerWithClips>, order: ClipOrder, mode: FilterMode) -> Vec<Clip> {
    let mut rng = util::create_seeded_rng();

    match order {
        ClipOrder::SceneOrder => match mode {
            FilterMode::Performers | FilterMode::Tags => {
                let mut clips: Vec<_> = clips
                    .into_iter()
                    .flat_map(|m| m.clips)
                    .map(|c| (c, rng.gen::<u32>()))
                    .collect();

                clips.sort_by_key(|(clip, random)| (clip.marker_index, *random));
                clips.into_iter().map(|(clip, _)| clip).collect()
            }
            FilterMode::Scenes => {
                let mut clips: Vec<_> = clips
                    .into_iter()
                    .flat_map(|m| m.clips)
                    .map(|c| (c, rng.gen::<u32>()))
                    .collect();

                clips.sort_by_key(|(clip, random)| (clip.scene_index, *random));
                clips.into_iter().map(|(clip, _)| clip).collect()
            }
        },
        ClipOrder::Random => {
            let mut clips: Vec<_> = clips.into_iter().flat_map(|c| c.clips).collect();
            clips.shuffle(&mut rng);
            clips
        }
    }
}
