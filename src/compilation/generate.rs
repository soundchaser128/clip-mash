use std::process::Output;

use crate::{server::handlers::CreateVideoBody, stash::api::Marker, Result};
use camino::{Utf8Path, Utf8PathBuf};
use futures::lock::Mutex;
use lazy_static::lazy_static;
use serde::Serialize;
use tokio::process::Command;

use super::clip::Clip;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Progress {
    pub finished: usize,
    pub total: usize,
}

lazy_static! {
    static ref PROGRESS: Mutex<Progress> = Default::default();
}

#[derive(Clone)]
pub struct CompilationGenerator {
    path: Utf8PathBuf,
    pub video_dir: Utf8PathBuf,
}

pub fn find_stream_url(marker: &Marker) -> &str {
    const LABEL_PRIORITIES: &[&str] = &["Direct stream", "webm", "HLS"];

    let streams = &marker.scene.scene_streams;
    for stream in streams {
        for label in LABEL_PRIORITIES {
            if let Some(l) = &stream.label {
                if l == label {
                    tracing::debug!("returning stream {stream:?}");
                    return &stream.url;
                }
            }
        }
    }
    // fallback to returning the first URL
    tracing::info!(
        "could not find any stream URL with the preferred labels, returning {:?}",
        streams[0]
    );
    &streams[0].url
}

fn commandline_error<T>(output: Output) -> Result<T> {
    use color_eyre::eyre::eyre;

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    Err(eyre!(
        "ffmpeg failed with exit code {}, stdout:\n{}\nstderr:\n{}",
        output.status.code().unwrap_or(1),
        stdout,
        stderr
    ))
}

pub async fn get_progress() -> Progress {
    PROGRESS.lock().await.clone()
}

impl CompilationGenerator {
    pub async fn new() -> Result<Self> {
        let path = crate::download_ffmpeg::download().await?;

        Ok(CompilationGenerator {
            path,
            video_dir: Utf8PathBuf::from("./videos"),
        })
    }

    async fn create_clip(
        &self,
        url: &str,
        start: u32,
        duration: u32,
        width: u32,
        height: u32,
        fps: f64,
        out_file: &Utf8Path,
    ) -> Result<()> {
        let clip_str = duration.to_string();
        let seconds_str = start.to_string();
        let filter = format!("scale={width}:{height}:force_original_aspect_ratio=decrease,pad={width}:{height}:-1:-1:color=black,fps={fps}",
            fps=fps,
        );

        let args = vec![
            "-hide_banner",
            "-loglevel",
            "warning",
            "-ss",
            seconds_str.as_str(),
            "-i",
            url,
            "-t",
            clip_str.as_str(),
            "-c:v",
            "libx264",
            "-preset",
            "slow",
            "-crf",
            "22",
            "-acodec",
            "aac",
            "-vf",
            &filter,
            "-ar",
            "48000",
            out_file.as_str(),
        ];
        tracing::info!("executing command ffmpeg {}", args.join(" "));

        let output = Command::new(self.path.as_str()).args(args).output().await?;
        if !output.status.success() {
            commandline_error(output)
        } else {
            Ok(())
        }
    }

    async fn initialize_progress(&self, total_items: usize) {
        let mut progress = PROGRESS.lock().await;
        progress.total = total_items;
    }

    async fn increase_progress(&self) {
        let mut progress = PROGRESS.lock().await;
        progress.finished += 1;
    }

    async fn reset_progress(&self) {
        let mut progress = PROGRESS.lock().await;
        *progress = Default::default();
    }

    pub async fn gather_clips(&self, output: &CreateVideoBody) -> Result<Vec<Utf8PathBuf>> {
        tokio::fs::create_dir_all(&self.video_dir).await?;
        let clips = &output.clips;
        let total_items = clips.len();
        self.initialize_progress(total_items).await;

        let mut paths = vec![];
        for Clip {
            range: (start, end),
            marker_id,
            ..
        } in clips
        {
            let marker = output
                .markers
                .iter()
                .find(|m| &m.id == marker_id)
                .expect(&format!("no marker with ID {marker_id} found"));
            let url = find_stream_url(marker);
            let (width, height) = output.output_resolution.resolution();
            let out_file = self
                .video_dir
                .join(format!("{}_{}-{}.mp4", marker.scene.id, start, end));
            if !out_file.is_file() {
                tracing::info!("creating clip {out_file}");
                self.create_clip(
                    url,
                    *start,
                    end - start,
                    width,
                    height,
                    output.output_fps as f64,
                    &out_file,
                )
                .await?;
            } else {
                tracing::info!("clip {out_file} already exists, skipping");
            }
            self.increase_progress().await;
            paths.push(out_file);
        }
        self.reset_progress().await;
        Ok(paths)
    }

    pub async fn compile_clips(
        &self,
        options: &CreateVideoBody,
        clips: Vec<Utf8PathBuf>,
    ) -> Result<Utf8PathBuf> {
        let file_name = &options.file_name;
        tracing::info!(
            "assembling {} clips into video with file name '{}'",
            options.clips.len(),
            file_name
        );
        let lines: Vec<_> = clips
            .into_iter()
            .map(|file| format!("file '{}", file.file_name().unwrap()))
            .collect();
        let file_content = lines.join("\n");
        tokio::fs::write(self.video_dir.join("clips.txt"), file_content).await?;
        let destination = self.video_dir.join(file_name);

        let args = vec![
            "-hide_banner",
            "-y",
            "-loglevel",
            "warning",
            "-f",
            "concat",
            "-i",
            "clips.txt",
            "-c",
            "copy",
            file_name,
        ];

        let output = Command::new(self.path.as_str())
            .args(args)
            .current_dir(self.video_dir.canonicalize()?)
            .output()
            .await?;

        if !output.status.success() {
            return commandline_error(output);
        }

        tracing::info!("finished assembling video, result at {destination}");

        Ok(destination)
    }
}
