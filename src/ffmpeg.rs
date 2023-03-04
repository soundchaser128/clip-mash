use std::process::Output;

use camino::Utf8PathBuf;
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use regex::Regex;
use tokio::process::Command;

use crate::{
    api::find_markers_query::{FindMarkersQueryFindSceneMarkersSceneMarkers as Marker, GenderEnum},
    Result,
};

pub struct Ffmpeg {
    path: Utf8PathBuf,
    video_dir: Utf8PathBuf,
}

pub fn find_direct_stream(marker: &Marker) -> &str {
    marker
        .scene
        .scene_streams
        .iter()
        .find(|v| v.label == Some("Direct stream".to_string()))
        .map(|v| &v.url)
        .unwrap()
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
        clip_duration: u32,
    ) -> Result<Vec<Utf8PathBuf>> {
        let pb = ProgressBar::new(markers.len() as u64);
        pb.set_style(ProgressStyle::with_template(
            "{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )?);
        let mut paths = vec![];
        for marker in markers {
            let url = find_direct_stream(&marker);
            let seconds = marker.seconds as u32;
            let out_file = self.video_dir.join(format!(
                "{}_{}-{}.mp4",
                marker.scene.id,
                seconds,
                seconds + clip_duration
            ));
            pb.set_message(format!(
                "Creating clip for scene {} at seconds {} to {}",
                formatted_scene(&marker),
                seconds,
                seconds + clip_duration
            ));

            if !out_file.is_file() {
                let clip_str = clip_duration.to_string();
                let seconds_str = seconds.to_string();

                let args = vec![
                    "-hide_banner",
                    "-loglevel",
                    "warning",
                    "-ss",
                    seconds_str.as_str(),
                    "-i",
                    &url,
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
                    "scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:-1:-1:color=black,fps=60",
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
        order: ClipOrder,
    ) -> Result<Utf8PathBuf> {
        use rand::seq::SliceRandom;

        match order {
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
        let destination = self.video_dir.join("compilation.mp4");

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
            "compilation.mp4",
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
