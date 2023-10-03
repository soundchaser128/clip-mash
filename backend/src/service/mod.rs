pub mod clip;
pub mod commands;
pub mod directories;
pub mod funscript;
pub mod generator;
pub mod migrations;
pub mod music;
pub mod preview_image;
pub mod scene_detection;
pub mod stash_config;
pub mod streams;
pub mod video;

#[cfg(test)]
pub mod fixtures;

use serde::{Deserialize, Serialize};

use crate::data::database::VideoSource;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marker {
    pub id: i64,
    pub start_time: f64,
    pub end_time: f64,
    pub index_within_video: usize,
    pub video_id: String,
    pub title: String,
    pub loops: usize,
    pub source: VideoSource,
}

impl Marker {
    pub fn duration(&self) -> f64 {
        self.end_time - self.start_time
    }
}
