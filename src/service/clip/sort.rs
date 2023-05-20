use super::Clip;

use rand::{rngs::StdRng, seq::SliceRandom, Rng};

use std::fmt::Debug;
use tracing::info;

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
        clips.sort_by_key(|(clip, random)| {
            (clip.index_within_video, clip.index_within_marker, *random)
        });
        clips.into_iter().map(|(clip, _)| clip).collect()
    }
}
