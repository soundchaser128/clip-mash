use std::fmt::Debug;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use tracing::{debug, info};

use super::Clip;

pub trait ClipSorter {
    fn sort_clips(&self, clips: Vec<Clip>, rng: &mut StdRng) -> Vec<Clip>;
}

#[derive(Debug)]
pub struct RandomClipSorter;

impl ClipSorter for RandomClipSorter {
    fn sort_clips(&self, mut clips: Vec<Clip>, rng: &mut StdRng) -> Vec<Clip> {
        info!("sorting clips with RandomClipSorter");
        clips.shuffle(rng);
        clips
    }
}

#[derive(Debug)]
pub struct SceneOrderClipSorter;

impl ClipSorter for SceneOrderClipSorter {
    fn sort_clips(&self, clips: Vec<Clip>, rng: &mut StdRng) -> Vec<Clip> {
        info!("sorting clips with SceneOrderClipSorter");
        let mut clips: Vec<_> = clips.into_iter().map(|c| (c, rng.gen::<usize>())).collect();
        debug!("clips: {:#?}", clips);
        clips.sort_by_key(|(clip, random)| {
            (clip.index_within_video, clip.index_within_marker, *random)
        });
        debug!("sorted clips: {:#?}", clips);
        clips.into_iter().map(|(clip, _)| clip).collect()
    }
}

#[derive(Debug)]
pub struct FixedOrderClipSorter {
    pub marker_titles: Vec<String>,
}

impl ClipSorter for FixedOrderClipSorter {
    fn sort_clips(&self, clips: Vec<Clip>, rng: &mut StdRng) -> Vec<Clip> {
        info!("sorting clips with FixedOrderClipSorter");
        let mut clips: Vec<_> = clips.into_iter().map(|c| (c, rng.gen::<usize>())).collect();
        debug!("clips: {:#?}", clips);
        clips.sort_by_key(|(clip, random)| {
            (
                self.marker_titles
                    .iter()
                    .position(|title| title == &clip.marker_title)
                    .unwrap_or(usize::MAX),
                clip.index_within_video,
                clip.index_within_marker,
                *random,
            )
        });
        debug!("sorted clips: {:#?}", clips);
        clips.into_iter().map(|(clip, _)| clip).collect()
    }
}
