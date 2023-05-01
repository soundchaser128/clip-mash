use serde::{Deserialize, Serialize};

pub mod clip;
pub mod funscript;
pub mod generate;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: i64,
    pub title: String,
    pub interactive: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Marker {
    pub id: i64,
    pub start_time: f64,
    pub end_time: f64,
    pub index_within_scene: usize,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub video_id: i64,
    pub marker_id: i64,
    /// Start and endpoint inside the video in seconds.
    pub range: (f64, f64),
    pub index_within_video: usize,
    pub index_within_marker: usize,
}

impl Clip {
    pub fn range_millis(&self) -> (u32, u32) {
        ((self.range.0 as u32) * 1000, (self.range.1 as u32) * 1000)
    }
}
