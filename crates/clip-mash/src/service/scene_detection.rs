use std::sync::Arc;

use color_eyre::eyre::bail;
use lazy_static::lazy_static;
use regex::Regex;
use tracing::{debug, info};

use super::commands::ffmpeg::{Ffmpeg, FfmpegLocation};
use crate::Result;
use crate::server::handlers::AppState;
use crate::server::types::{CreateMarker, MarkerDto, MarkerDtoConverter};
use crate::service::preview_image::PreviewGenerator;

lazy_static! {
    static ref PTS_REGEX: Regex = Regex::new(r"pts_time:([\d\.]+)").unwrap();
}

const MIN_MARKER_DURATION: f64 = 5.0;

async fn detect_scenes(
    input: &str,
    threshold: f64,
    ffmpeg_location: &FfmpegLocation,
) -> Result<Vec<f64>> {
    info!(
        "detecting scenes in input {} with threshold {}",
        input, threshold
    );
    let output = Ffmpeg::new(ffmpeg_location, "-")
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
struct DetectedMarker {
    pub start: f64,
    pub end: f64,
}

fn detect_markers(mut timestamps: Vec<f64>, total_duration: f64) -> Vec<DetectedMarker> {
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
            let duration = end - start;
            if duration >= MIN_MARKER_DURATION {
                info!("adding marker with start {} and end {}", start, end);
                markers.push(DetectedMarker {
                    start: *start,
                    end: *end,
                });
            }
        }
    }
    markers
}

pub async fn find_and_persist_markers(
    video_id: &str,
    threshold: f64,
    state: Arc<AppState>,
) -> Result<Vec<MarkerDto>> {
    let video = state.database.videos.get_video(video_id).await?;
    if video.is_none() {
        bail!("no video found for id {}", video_id);
    }
    let video = video.unwrap();
    let timestamps = detect_scenes(&video.file_path, threshold, &state.ffmpeg_location).await?;
    let markers = detect_markers(timestamps, video.duration);

    let mut created_markers = vec![];
    let preview_generator: PreviewGenerator = state.clone().into();
    let stash_api = state.stash_api().await?;
    let converter = MarkerDtoConverter::new(stash_api);
    for (index, marker) in markers.into_iter().enumerate() {
        let preview_image = preview_generator
            .generate_preview(&video.id, &video.file_path, marker.start)
            .await?;
        let db_marker = state
            .database
            .markers
            .create_new_marker(CreateMarker {
                video_id: video.id.clone(),
                title: "Untitled".to_string(),
                start: marker.start,
                end: marker.end,
                preview_image_path: Some(preview_image.to_string()),
                index_within_video: index as i64,
                video_interactive: video.interactive,
                created_on: None,
                marker_stash_id: None,
            })
            .await?;
        info!("created marker {db_marker:?}");
        created_markers.push(converter.from_db(db_marker, &video));
    }
    Ok(created_markers)
}
