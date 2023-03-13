use std::{cmp::Reverse, process::Output};

use camino::{Utf8Path, Utf8PathBuf};
use futures::lock::Mutex;
use lazy_static::lazy_static;
use rand::{rngs::StdRng, seq::SliceRandom, RngCore, SeedableRng};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

use crate::{
    clip::{self, Clip, ClipOrder, MarkerWithClips},
    download_ffmpeg,
    http::CreateVideoBody,
    stash_api::find_markers_query::{
        FindMarkersQueryFindSceneMarkersSceneMarkers as Marker, GenderEnum,
    },
    util, Result,
};

#[derive(Debug, Default, Clone, Serialize)]
pub struct Progress {
    pub finished: usize,
    pub total: usize,
}

lazy_static! {
    static ref PROGRESS: Mutex<Progress> = Default::default();
}

#[derive(Clone)]
pub struct Ffmpeg {
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

pub fn formatted_scene(marker: &Marker) -> String {
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

    let title = match &marker.scene.title {
        Some(title) if title.trim().len() > 0 => title,
        _ => &marker.scene.files[0].basename,
    };
    let performers = female_performers.join(",");
    let performers = match performers.as_str() {
        "" => "<no performers found>",
        _ => &performers,
    };
    format!("'{}' ({})", title, performers)
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

pub async fn get_progress() -> Progress {
    PROGRESS.lock().await.clone()
}

#[derive(Debug)]
struct ParsedClip {
    scene_id: u32,
    path: Utf8PathBuf,
}

impl ParsedClip {
    fn from_path(path: Utf8PathBuf) -> Self {
        lazy_static! {
            static ref FILE_REGEX: Regex = Regex::new(r#"(\d+)_(\d+)-(\d+)\.mp4"#).unwrap();
        }

        let filename = path.file_name().expect("path must have file name");
        let matches = FILE_REGEX.captures(filename);
        let (scene_id, _) = match matches {
            Some(matches) => {
                let scene_id = matches.get(1).unwrap().as_str();
                let scene_id: u32 = scene_id.parse().unwrap();

                let start = matches.get(2).unwrap().as_str();
                let start: u32 = start.parse().unwrap();

                (scene_id, start)
            }
            None => (0, 0),
        };
        ParsedClip { path, scene_id }
    }
}

fn intersperse_scene_clips(clips: Vec<Utf8PathBuf>) -> Vec<Utf8PathBuf> {
    use itertools::Itertools;

    let mut clips: Vec<_> = clips.into_iter().map(ParsedClip::from_path).collect();
    clips.sort_by_key(|c| c.scene_id);

    let iter = clips.into_iter().group_by(|c| c.scene_id);
    let mut rng = util::create_seeded_rng();
    let mut clips = vec![];
    for (_, group) in &iter {
        for (idx, clip) in group.enumerate() {
            let rand = rng.next_u32();
            clips.push((idx, rand, clip));
        }
    }

    clips.sort_by_key(|(idx, rand, _)| Reverse((*idx, *rand)));
    clips.into_iter().map(|(_, _, c)| c.path).collect()
}

impl Ffmpeg {
    pub async fn new() -> Result<Self> {
        let path = download_ffmpeg::download().await?;

        Ok(Ffmpeg {
            path,
            video_dir: Utf8PathBuf::from("./videos"),
        })
    }

    // fn get_clip_offsets(
    //     &self,
    //     marker: &Marker,
    //     clip_duration_max: u32,
    //     max_duration: Option<u32>,
    // ) -> Vec<(u32, u32)> {
    //     let clip_lengths = [
    //         (clip_duration_max / 2).max(2),
    //         (clip_duration_max / 3).max(2),
    //         (clip_duration_max / 4).max(2),
    //     ];

    //     let mut rng = util::create_seeded_rng();
    //     let (start, end) = self.get_time_range(marker, max_duration);
    //     let end = end.unwrap_or(start + clip_duration_max);

    //     let mut offset = start;
    //     let mut offsets = vec![];
    //     while offset < end {
    //         let duration = clip_lengths.choose(&mut rng).unwrap();
    //         offsets.push((offset, *duration));
    //         offset += duration;
    //     }
    //     offsets
    // }

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
        let output = Command::new(self.path.as_str()).args(args).output().await?;
        if !output.status.success() {
            commandline_error(output)
        } else {
            Ok(())
        }
    }

    async fn write_markers_with_offsets(
        &self,
        id: &str,
        markers: &[(&Marker, Vec<(u32, u32)>)],
    ) -> Result<()> {
        #[derive(Serialize)]
        struct MarkerJson<'a> {
            scene: String,
            scene_id: &'a str,
            offsets: &'a [(u32, u32)],
            tag: &'a str,
        }

        let path = self.video_dir.join(format!("markers-{id}.json"));
        let markers: Vec<_> = markers
            .iter()
            .map(|(marker, offsets)| MarkerJson {
                scene: formatted_scene(marker),
                offsets,
                scene_id: marker.scene.id.as_str(),
                tag: &marker.primary_tag.name,
            })
            .collect();
        let contents = serde_json::to_string_pretty(&markers)?;
        tokio::fs::write(path, contents).await?;
        Ok(())
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

        let clips = clip::get_all_clips(&output);
        // let markers: Vec<_> = output
        //     .markers
        //     .iter()
        //     .map(|m| {
        //         (
        //             m,
        //             self.get_clip_offsets(
        //                 m,
        //                 output.clip_duration,
        //                 output
        //                     .selected_markers
        //                     .iter()
        //                     .find(|n| n.id == m.id)
        //                     .and_then(|m| m.duration),
        //             ),
        //         )
        //     })
        //     .collect();

        let total_items = clips
            .iter()
            .fold(0, |count, marker| count + marker.clips.len());
        self.initialize_progress(total_items).await;

        let mut paths = vec![];
        for MarkerWithClips { clips, marker } in clips {
            let url = find_stream_url(&marker);
            let (width, height) = output.output_resolution.resolution();
            tracing::info!("computed {} clips for marker {}", clips.len(), marker.id);
            for Clip {
                range: (start, end),
                ..
            } in clips
            {
                let out_file = self
                    .video_dir
                    .join(format!("{}_{}-{}.mp4", marker.scene.id, start, end));
                if !out_file.is_file() {
                    tracing::info!("creating clip {out_file}");
                    self.create_clip(
                        url,
                        start,
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
        }
        self.reset_progress().await;
        Ok(paths)
    }

    pub async fn compile_clips(
        &self,
        mut clips: Vec<Utf8PathBuf>,
        options: &CreateVideoBody,
    ) -> Result<Utf8PathBuf> {
        tracing::info!("assembling {} clips into video", clips.len());

        let clips = match options.clip_order {
            ClipOrder::Random => {
                let mut rng = rand::thread_rng();
                clips.shuffle(&mut rng);
                clips
            }
            ClipOrder::SceneOrder => intersperse_scene_clips(clips),
        };

        let lines: Vec<_> = clips
            .into_iter()
            .map(|file| format!("file '{}", file.file_name().unwrap()))
            .collect();
        let file_content = lines.join("\n");
        tokio::fs::write(self.video_dir.join("clips.txt"), file_content).await?;
        let file_name = format!("{}.mp4", options.id);
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

        tracing::info!("finished assembling video, result at {destination}");

        Ok(destination)
    }
}
