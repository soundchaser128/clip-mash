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
    info!(
        "detecting scenes in input {} with threshold {}",
        input, threshold
    );
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
            debug!("found pts: {}", pts);
            if let Ok(pts) = pts.parse::<f64>() {
                timestamps.push(pts);
            }
        }
    }

    Ok(timestamps)
}

#[derive(Debug, Clone)]
pub struct DetectedMarker {
    pub start: f64,
    pub end: f64,
}

pub fn detect_markers(mut timestamps: Vec<f64>, total_duration: f64) -> Vec<DetectedMarker> {
    if timestamps.is_empty() {
        return vec![DetectedMarker {
            start: 0.0,
            end: total_duration,
        }];
    }

    if timestamps[0] != 0.0 {
        timestamps.insert(0, 0.0);
    }

    if timestamps.last() != Some(&total_duration) {
        timestamps.push(total_duration);
    }

    let mut markers = vec![];
    for window in timestamps.windows(2) {
        if let [start, end] = window {
            info!("adding marker with start {} and end {}", start, end);
            markers.push(DetectedMarker {
                start: *start,
                end: *end,
            })
        }
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
        let _scenes = detect_scenes(input, 0.4, ffmpeg_location).await;
    }
}
