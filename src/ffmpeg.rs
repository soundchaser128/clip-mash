use std::process::Output;

use camino::Utf8PathBuf;
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use regex::Regex;
use tokio::process::Command;

use crate::{
    api::find_markers_query::{FindMarkersQueryFindSceneMarkersSceneMarkers as Marker, GenderEnum},
    cli::CompilationInfo,
    Result,
};

pub struct OutputFormat {
    pub order: ClipOrder,
    pub video_name: String,
    pub resolution: (u32, u32),
    pub fps: f64,
    pub clip_duration: u32,
}

impl From<CompilationInfo> for OutputFormat {
    fn from(value: CompilationInfo) -> Self {
        OutputFormat {
            order: value.clip_order,
            video_name: value.video_name,
            resolution: value.output_resolution,
            fps: value.output_fps,
            clip_duration: value.clip_duration,
        }
    }
}

pub struct Ffmpeg {
    path: Utf8PathBuf,
    video_dir: Utf8PathBuf,
}

pub fn find_stream_url(marker: &Marker) -> &str {
    const LABEL_PRIORITIES: &[&str] = &["Direct stream", "webm", "HLS"];

    let streams = &marker.scene.scene_streams;
    for stream in streams {
        for label in LABEL_PRIORITIES {
            if let Some(l) = &stream.label {
                if l == label {
                    tracing::info!("returning stream {stream:?}");
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


fn formatted_scene(marker: &Marker) -> String {
    let female_performers: Vec<_> = marker
        .scene
        .performers
        .iter()
        .filter(|p| {
            matches!(
                &p.gender.as_ref().unwrap_or(&GenderEnum::FEMALE),
                GenderEnum::FEMALE
            )
        })
        .map(|p| p.name.as_str())
        .collect();

    let title = marker.scene.title.as_deref().unwrap_or("<no title>");
    format!("'{}' ({})", title, female_performers.join(", "))
}

fn commandline_error<T>(output: Output) -> Result<T> {
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    Err(format!(
        "ffmpeg failed with nonzero exit code, stdout:\n{}\nstderr:\n{}",
        stdout, stderr
    )
    .into())
}

#[derive(Clone, Copy, Debug)]
pub enum ClipOrder {
    Random,
    SceneOrder,
}

fn clip_sort_key(filename: &str) -> (u32, u32) {
    lazy_static! {
        static ref FILE_REGEX: Regex = Regex::new(r#"(\d+)_(\d+)-(\d+)\.mp4"#).unwrap();
    }
    let matches = FILE_REGEX.captures(filename);
    match matches {
        Some(matches) => {
            let scene_id = matches.get(1).unwrap().as_str();
            let scene_id: u32 = scene_id.parse().unwrap();

            let start = matches.get(2).unwrap().as_str();
            let start: u32 = start.parse().unwrap();

            (scene_id, start)
        }
        None => (0, 0),
    }
}

impl Ffmpeg {
    pub fn new() -> Self {
        Ffmpeg {
            path: Utf8PathBuf::from("ffmpeg"),
            video_dir: Utf8PathBuf::from("./videos"),
        }
    }

    pub async fn gather_clips(
        &self,
        markers: Vec<Marker>,
        output: &OutputFormat,
    ) -> Result<Vec<Utf8PathBuf>> {
        tokio::fs::create_dir_all(&self.video_dir).await?;

        let pb = ProgressBar::new(markers.len() as u64);
        pb.set_style(ProgressStyle::with_template(
            "{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )?);
        let mut paths = vec![];
        for marker in markers {
            let url = find_stream_url(&marker);
            let seconds = marker.seconds as u32;
            let out_file = self.video_dir.join(format!(
                "{}_{}-{}.mp4",
                marker.scene.id,
                seconds,
                seconds + output.clip_duration
            ));
            pb.set_message(format!(
                "Creating clip for scene {} at seconds {} to {}",
                formatted_scene(&marker),
                seconds,
                seconds + output.clip_duration
            ));

            if !out_file.is_file() {
                let clip_str = output.clip_duration.to_string();
                let seconds_str = seconds.to_string();
                let (width, height) = output.resolution;
                let filter = format!("scale={width}:{height}:force_original_aspect_ratio=decrease,pad={width}:{height}:-1:-1:color=black,fps={fps}",
                    fps=output.fps,
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
                let output = Command::new(self.path.as_str()).args(args).output().await?;
                if !output.status.success() {
                    return commandline_error(output);
                }
            }

            paths.push(out_file);
            pb.inc(1);
        }
        pb.finish();
        Ok(paths)
    }

    pub async fn compile_clips(
        &self,
        mut clips: Vec<Utf8PathBuf>,
        output_format: OutputFormat,
    ) -> Result<Utf8PathBuf> {
        use rand::seq::SliceRandom;

        match output_format.order {
            ClipOrder::Random => {
                let mut rng = rand::thread_rng();
                clips.shuffle(&mut rng);
            }
            ClipOrder::SceneOrder => {
                clips.sort_by_key(|str| clip_sort_key(str.file_name().unwrap()));
            }
        }

        let lines: Vec<_> = clips
            .into_iter()
            .map(|file| format!("file '{}", file.file_name().unwrap()))
            .collect();
        let file_content = lines.join("\n");
        tokio::fs::write(self.video_dir.join("clips.txt"), file_content).await?;
        let file_name = format!("{}.mp4", output_format.video_name);
        let destination = self.video_dir.join(&file_name);

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
            &file_name,
        ];

        let output = Command::new(self.path.as_str())
            .args(args)
            .current_dir(self.video_dir.canonicalize()?)
            .output()
            .await?;

        if !output.status.success() {
            return commandline_error(output);
        }

        Ok(destination)
    }
}
