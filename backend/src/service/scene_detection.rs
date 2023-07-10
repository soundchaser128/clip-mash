use clip_mash_types::DetectedMarker;
use lazy_static::lazy_static;
use regex::Regex;
use tracing::{debug, info};

use super::commands::ffmpeg::{Ffmpeg, FfmpegLocation};
use crate::Result;

lazy_static! {
    static ref PTS_REGEX: Regex = Regex::new(r"pts_time:([\d\.]+)").unwrap();
}

pub async fn detect_scenes(
    input: &str,
    threshold: f64,
    ffmpeg_location: FfmpegLocation,
) -> Result<Vec<f64>> {
    let output = Ffmpeg::new(&ffmpeg_location, "-")
        .input(input)
        .format("null")
        .video_filter(format!("select='gt(scene,{threshold})',showinfo'"))
        .log_level("info")
        .output()
        .await?;
    debug!("output: {}", output);
    let mut timestamps = vec![];
    for line in output.split('\n') {
        if let Some(captures) = PTS_REGEX.captures(line) {
            let pts = captures.get(1).unwrap().as_str();
            info!("pts: {}", pts);
            if let Ok(pts) = pts.parse::<f64>() {
                timestamps.push(pts);
            }
        }
    }

    Ok(timestamps)
}

pub fn detect_markers(mut timestamps: Vec<f64>, total_duration: f64) -> Vec<DetectedMarker> {
    if timestamps.is_empty() {
        return vec![];
    }

    let mut markers = vec![];
    if timestamps[0] != 0.0 {
        timestamps.insert(0, 0.0);
    }

    if timestamps.last() != Some(&total_duration) {
        timestamps.push(total_duration);
    }

    let mut markers = vec![];
    for window in timestamps.windows(2) {
        let [start, end] = window;
        markers.push(DetectedMarker { start, end })
    }
    markers
}

#[cfg(test)]
mod test {
    use tracing_test::traced_test;

    use super::detect_scenes;
    use crate::service::commands::ffmpeg::FfmpegLocation;

    #[traced_test]
    #[tokio::test]
    async fn test_detect_scenes() {
        let input = "/Users/martin/stuff/3D PMV [petty-wellworn-wuerhosaurus].mp4";
        let ffmpeg_location = FfmpegLocation::System;
        let scenes = detect_scenes(input, 0.4, ffmpeg_location).await;
    }
}
