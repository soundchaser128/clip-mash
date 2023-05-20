use std::ffi::OsStr;

use crate::{
    data::{database::DbSong, stash_api::StashMarker},
    service::MarkerInfo,
    util::{commandline_error, debug_output},
    Result,
};
use camino::{Utf8Path, Utf8PathBuf};
use futures::lock::Mutex;
use itertools::Itertools;
use lazy_static::lazy_static;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tracing::{debug, enabled, info, Level};

use super::{directories::Directories, Clip, Marker};

#[derive(Debug, Default, Clone, Serialize)]
pub struct Progress {
    pub finished: usize,
    pub total: usize,
    pub done: bool,
}

lazy_static! {
    static ref PROGRESS: Mutex<Progress> = Default::default();
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum VideoResolution {
    #[serde(rename = "720")]
    SevenTwenty,
    #[serde(rename = "1080")]
    TenEighty,
    #[serde(rename = "4K")]
    FourK,
}

impl VideoResolution {
    fn resolution(&self) -> (u32, u32) {
        match self {
            Self::SevenTwenty => (1280, 720),
            Self::TenEighty => (1920, 1080),
            Self::FourK => (3840, 2160),
        }
    }
}

#[derive(Debug)]
pub struct CompilationOptions {
    pub clips: Vec<Clip>,
    pub markers: Vec<Marker>,
    pub output_resolution: VideoResolution,
    pub output_fps: u32,
    pub file_name: String,
    pub songs: Vec<DbSong>,
    pub music_volume: f64,
}

pub fn find_stash_stream_url(marker: &StashMarker) -> &str {
    const LABEL_PRIORITIES: &[&str] = &["Direct stream", "webm", "HLS"];

    let streams = &marker.streams;
    for stream in streams {
        for label in LABEL_PRIORITIES {
            if let Some(l) = &stream.label {
                if l == label {
                    debug!("returning stream {stream:?}");
                    return &stream.url;
                }
            }
        }
    }
    // fallback to returning the first URL
    info!(
        "could not find any stream URL with the preferred labels, returning {:?}",
        streams[0]
    );
    &streams[0].url
}

pub fn find_stream_url(marker: &Marker) -> &str {
    match &marker.info {
        MarkerInfo::Stash { marker } => find_stash_stream_url(marker),
        MarkerInfo::LocalFile { marker } => &marker.file_path,
    }
}

pub async fn get_progress() -> Progress {
    PROGRESS.lock().await.clone()
}

#[derive(Clone)]
pub struct CompilationGenerator {
    directories: Directories,
    ffmpeg_path: Utf8PathBuf,
}

impl CompilationGenerator {
    pub async fn new(directories: Directories) -> Result<Self> {
        use crate::service::download_ffmpeg;

        let ffmpeg_path = download_ffmpeg::download(&directories).await?;
        Ok(CompilationGenerator {
            directories,
            ffmpeg_path,
        })
    }

    async fn ffmpeg(&self, args: Vec<impl AsRef<OsStr>>) -> Result<()> {
        if enabled!(Level::DEBUG) {
            let string = args.iter().map(|s| s.as_ref().to_string_lossy()).join(" ");
            debug!("running command 'ffmpeg {}'", string);
        }

        let output = Command::new(self.ffmpeg_path.as_str())
            .args(args)
            .current_dir(self.directories.video_dir())
            .output()
            .await?;
        if !output.status.success() {
            commandline_error(output)
        } else {
            debug_output(output);
            Ok(())
        }
    }

    async fn create_clip(
        &self,
        url: &str,
        start: f64,
        duration: f64,
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
        self.ffmpeg(args).await
    }

    async fn initialize_progress(&self, total_items: usize) {
        let mut progress = PROGRESS.lock().await;
        progress.total = total_items;
        progress.done = false;
        progress.finished = 0;
        debug!("setting progress total to {total_items}");
    }

    async fn increase_progress(&self) {
        let mut progress = PROGRESS.lock().await;
        progress.finished += 1;
        if progress.finished == progress.total {
            info!(
                "finished all items, setting done = true ({} items)",
                progress.finished
            );
            progress.done = true;
        }
        debug!("bumping progress, count = {}", progress.finished);
    }

    async fn reset_progress(&self) {
        let mut progress = PROGRESS.lock().await;
        *progress = Default::default();
        info!("reset progress to default");
    }

    pub async fn gather_clips(&self, options: &CompilationOptions) -> Result<Vec<Utf8PathBuf>> {
        let clips = &options.clips;
        self.reset_progress().await;
        let progress_items = clips.len() + if options.songs.len() >= 2 { 2 } else { 1 };
        self.initialize_progress(progress_items).await;
        let video_dir = self.directories.video_dir();
        tokio::fs::create_dir_all(&video_dir).await?;

        let mut paths = vec![];
        for Clip {
            range: (start, end),
            marker_id,
            ..
        } in clips
        {
            let marker = options
                .markers
                .iter()
                .find(|m| &m.id == marker_id)
                .expect(&format!("no marker with ID {marker_id} found"));
            let url = find_stream_url(marker);
            let (width, height) = options.output_resolution.resolution();
            let out_file = video_dir.join(format!("{}_{}-{}.mp4", marker.video_id, start, end));
            if !out_file.is_file() {
                info!("creating clip {out_file}");
                self.create_clip(
                    url,
                    *start,
                    end - start,
                    width,
                    height,
                    options.output_fps as f64,
                    &out_file,
                )
                .await?;
            } else {
                info!("clip {out_file} already exists, skipping");
            }
            self.increase_progress().await;
            paths.push(out_file);
        }

        Ok(paths)
    }

    async fn concat_songs(&self, songs: &[DbSong]) -> Result<Utf8PathBuf> {
        let file_name = format!("{}.aac", nanoid!(8));
        let music_dir = self.directories.music_dir();

        let lines: Vec<_> = songs
            .into_iter()
            .map(|file| format!("file '{}'", file.file_path))
            .collect();
        let file_content = lines.join("\n");
        tokio::fs::write(music_dir.join("songs.txt"), file_content).await?;
        let destination = music_dir.join(file_name);

        let args = vec![
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            "songs.txt",
            "-c:a",
            "aac",
            "-b:a",
            "128k",
            destination.as_str(),
        ];

        let output = Command::new(self.ffmpeg_path.as_str())
            .args(args)
            .current_dir(music_dir.canonicalize()?)
            .output()
            .await?;

        if !output.status.success() {
            return commandline_error(output);
        }

        self.increase_progress().await;

        Ok(destination)
    }

    pub async fn compile_clips(
        &self,
        options: &CompilationOptions,
        clips: Vec<Utf8PathBuf>,
    ) -> Result<Utf8PathBuf> {
        let video_dir = self.directories.video_dir();
        let file_name = &options.file_name;
        info!(
            "assembling {} clips into video with file name '{}'",
            options.clips.len(),
            file_name
        );
        let lines: Vec<_> = clips
            .into_iter()
            .map(|file| format!("file '{}", file.file_name().unwrap()))
            .collect();
        let file_content = lines.join("\n");
        tokio::fs::write(video_dir.join("clips.txt"), file_content).await?;
        let destination = video_dir.join(file_name);

        let music_volume = options.music_volume;
        let original_volume = 1.0 - options.music_volume;
        let filter = format!("[0:a:0]volume={original_volume}[a1];[1:a:0]volume={music_volume}[a2];[a1][a2]amix=inputs=2[a]");

        let args: Vec<String> = if options.songs.is_empty() {
            vec![
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
            ]
            .into_iter()
            .map(From::from)
            .collect()
        } else {
            let audio_path = if options.songs.len() >= 2 {
                self.concat_songs(&options.songs).await?
            } else {
                options.songs[0].file_path.clone().into()
            };

            info!("using audio from {audio_path}");

            vec![
                "-hide_banner",
                "-y",
                "-loglevel",
                "warning",
                "-f",
                "concat",
                "-i",
                "clips.txt",
                "-i",
                audio_path.as_str(),
                "-filter_complex",
                &filter,
                "-map",
                "0:v:0",
                "-map",
                "[a]",
                "-c:v",
                "copy",
                "-c:a",
                "aac",
                "-b:a",
                "128k",
                file_name,
            ]
            .into_iter()
            .map(From::from)
            .collect()
        };

        self.ffmpeg(args).await?;

        info!("finished assembling video, result at {destination}");
        self.increase_progress().await;
        Ok(destination)
    }
}
