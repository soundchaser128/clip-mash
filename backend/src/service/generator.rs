use std::ffi::OsStr;
use std::time::Instant;

use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
use tokio::process::Command;
use tracing::{debug, enabled, info, Level};

use super::commands::ffmpeg::FfmpegLocation;
use super::directories::Directories;
use super::encoding_optimization::EncodingOptimizationService;
use super::streams::{LocalVideoSource, StreamUrlService};
use super::Marker;
use crate::data::database::{Database, DbSong};
use crate::helpers::estimator::Estimator;
use crate::server::types::{Clip, EncodingEffort, VideoCodec, VideoQuality};
use crate::util::{commandline_error, debug_output, format_duration, generate_id};
use crate::Result;

#[derive(Debug)]
pub struct CompilationOptions {
    pub video_id: String,
    pub clips: Vec<Clip>,
    pub markers: Vec<Marker>,
    pub output_resolution: (u32, u32),
    pub output_fps: u32,
    pub file_name: String,
    pub songs: Vec<DbSong>,
    pub music_volume: f64,
    pub video_codec: VideoCodec,
    pub video_quality: VideoQuality,
    pub encoding_effort: EncodingEffort,
}

fn get_clip_file_name(
    video_id: &String,
    start: f64,
    end: f64,
    codec: VideoCodec,
    (x_res, y_res): (u32, u32),
) -> String {
    format!("{video_id}_{start}-{end}-{codec}-{x_res}x{y_res}.mp4")
}

#[derive(Debug)]
struct CreateClip<'a> {
    url: &'a str,
    start: f64,
    duration: f64,
    width: u32,
    height: u32,
    fps: f64,
    out_file: &'a Utf8Path,
    codec: VideoCodec,
    quality: VideoQuality,
    effort: EncodingEffort,
    re_encode: bool,
}

#[derive(Clone)]
pub struct CompilationGenerator {
    directories: Directories,
    ffmpeg_path: Utf8PathBuf,
    database: Database,
    encoding_optimization: EncodingOptimizationService,
    stream_urls: StreamUrlService,
}

impl CompilationGenerator {
    pub async fn new(
        directories: Directories,
        ffmpeg_location: &FfmpegLocation,
        database: Database,
    ) -> Result<Self> {
        let ffmpeg_path = ffmpeg_location.ffmpeg();
        let encoding_optimization =
            EncodingOptimizationService::new(ffmpeg_location.clone(), database.clone()).await;
        let streams_service = StreamUrlService::new(database.clone()).await;
        info!("using ffmpeg at {ffmpeg_path}");
        Ok(CompilationGenerator {
            directories,
            ffmpeg_path,
            database: database,
            encoding_optimization,
            stream_urls: streams_service,
        })
    }

    async fn ffmpeg(&self, args: Vec<impl AsRef<OsStr>>) -> Result<()> {
        if enabled!(Level::DEBUG) {
            let string = args.iter().map(|s| s.as_ref().to_string_lossy()).join(" ");
            debug!("running command '{} {}'", self.ffmpeg_path, string);
        }

        let output = Command::new(self.ffmpeg_path.as_str())
            .args(args)
            .current_dir(self.directories.temp_video_dir())
            .output()
            .await?;
        if !output.status.success() {
            commandline_error(self.ffmpeg_path.as_str(), output)
        } else {
            debug_output(output);
            Ok(())
        }
    }

    fn video_encoding_parameters(
        &self,
        codec: VideoCodec,
        quality: VideoQuality,
        effort: EncodingEffort,
    ) -> Vec<&str> {
        let encoder = match codec {
            VideoCodec::H264 => "libx264",
            VideoCodec::H265 => "libx265",
            VideoCodec::Av1 => "libsvtav1",
        };
        let effort = match codec {
            VideoCodec::Av1 => match effort {
                EncodingEffort::Low => "3",
                EncodingEffort::Medium => "7",
                EncodingEffort::High => "10",
            },
            _ => match effort {
                EncodingEffort::Low => "veryfast",
                EncodingEffort::Medium => "medium",
                EncodingEffort::High => "slow",
            },
        };
        let crf = match codec {
            VideoCodec::H264 => match quality {
                VideoQuality::Low => "28",
                VideoQuality::Medium => "24",
                VideoQuality::High => "19",
                VideoQuality::Lossless => "16",
            },
            VideoCodec::H265 => match quality {
                VideoQuality::Low => "32",
                VideoQuality::Medium => "28",
                VideoQuality::High => "24",
                VideoQuality::Lossless => "16",
            },
            VideoCodec::Av1 => match quality {
                VideoQuality::Low => "35",
                VideoQuality::Medium => "30",
                VideoQuality::High => "26",
                VideoQuality::Lossless => "20",
            },
        };

        vec!["-c:v", encoder, "-preset", effort, "-crf", crf]
    }

    async fn create_clip(&self, clip: CreateClip<'_>) -> Result<()> {
        let clip_str = clip.duration.to_string();
        let seconds_str = clip.start.to_string();
        let filter = format!("scale={width}:{height}:force_original_aspect_ratio=decrease,pad={width}:{height}:-1:-1:color=black,fps={fps}",
            width=clip.width,
            height=clip.height,
            fps=clip.fps,
        );

        let mut args = vec![
            "-hide_banner",
            "-loglevel",
            "warning",
            "-ss",
            seconds_str.as_str(),
            "-i",
            clip.url,
            "-t",
            clip_str.as_str(),
        ];
        if clip.re_encode {
            args.extend(self.video_encoding_parameters(clip.codec, clip.quality, clip.effort));
            args.extend(&["-acodec", "aac", "-vf", &filter, "-ar", "48000"]);
        } else {
            args.extend(&["-c:v", "copy", "-c:a", "copy"]);
        }
        args.push(clip.out_file.as_str());

        self.ffmpeg(args).await
    }

    async fn initialize_progress(&self, video_id: &str, total_items: f64) -> Result<()> {
        self.database
            .progress
            .insert_progress(video_id, total_items, "Starting...")
            .await?;
        Ok(())
    }

    async fn increase_progress(
        &self,
        video_id: &str,
        seconds: f64,
        eta: f64,
        message: &str,
    ) -> Result<()> {
        self.database
            .progress
            .update_progress(video_id, seconds, eta, message)
            .await?;
        Ok(())
    }

    async fn finish_progress(&self, video_id: &str) -> Result<()> {
        self.database.progress.finish_progress(video_id).await?;
        Ok(())
    }

    fn get_video_ids<'a>(&self, options: &'a CompilationOptions) -> Vec<&'a str> {
        let mut ids: Vec<_> = options.clips.iter().map(|c| c.video_id.as_str()).collect();
        ids.sort();
        ids.dedup();

        ids
    }

    pub async fn gather_clips(&self, options: &CompilationOptions) -> Result<Vec<Utf8PathBuf>> {
        let mut estimator = Estimator::new(Instant::now());
        let clips = &options.clips;
        let total_duration = clips.iter().map(|c| c.duration()).sum();
        self.initialize_progress(&options.video_id, total_duration)
            .await?;
        let video_dir = self.directories.temp_video_dir();
        tokio::fs::create_dir_all(&video_dir).await?;
        let video_ids = self.get_video_ids(&options);
        let stream_urls = self
            .stream_urls
            .get_video_streams(&video_ids, LocalVideoSource::File)
            .await?;
        let can_concatenate = self
            .encoding_optimization
            .needs_re_encode(&video_ids)
            .await?;
        info!("can concatenate without re-encoding: {}", can_concatenate);

        let total = clips.len();
        let mut paths = vec![];
        let mut completed = 0.0;
        for (index, clip) in clips.iter().enumerate() {
            let Clip {
                range: (start, end),
                marker_id,
                ..
            } = clip;
            let marker = options
                .markers
                .iter()
                .find(|m| &m.id == marker_id)
                .expect(&format!("no marker with ID {marker_id} found"));
            let url = &stream_urls[&marker.video_id];
            let (width, height) = options.output_resolution;
            let out_file = video_dir.join(get_clip_file_name(
                &marker.video_id,
                *start,
                *end,
                options.video_codec,
                options.output_resolution,
            ));
            if !out_file.is_file() {
                info!("creating clip {} / {} at {out_file}", index + 1, total);
                self.create_clip(CreateClip {
                    url,
                    start: *start,
                    duration: end - start,
                    width,
                    height,
                    fps: options.output_fps as f64,
                    out_file: &out_file,
                    codec: options.video_codec,
                    quality: options.video_quality,
                    effort: options.encoding_effort,
                    re_encode: !can_concatenate,
                })
                .await?;
            } else {
                info!("clip {out_file} already exists, skipping");
            }
            let message = format!(
                "Encoding clip for marker '{}' from {} to {}",
                marker.title,
                format_duration(*start),
                format_duration(*end)
            );
            completed += clip.duration();
            estimator.record(completed as u64, Instant::now());

            let steps_per_second = estimator.steps_per_second(Instant::now());
            let eta = (total_duration - completed) / steps_per_second;

            self.increase_progress(&options.video_id, clip.duration(), eta, &message)
                .await?;
            paths.push(out_file);
        }

        Ok(paths)
    }

    async fn concat_songs(&self, songs: &[DbSong], video_id: &str) -> Result<Utf8PathBuf> {
        let file_name = format!("{}.aac", generate_id());
        let music_dir = self.directories.music_dir();

        let lines: Vec<_> = songs
            .iter()
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
            return commandline_error(self.ffmpeg_path.as_str(), output);
        }

        self.increase_progress(video_id, 1.0, 0.0, "Stitching together songs")
            .await?;

        Ok(destination)
    }

    pub async fn compile_clips(
        &self,
        options: &CompilationOptions,
        clips: Vec<Utf8PathBuf>,
    ) -> Result<Utf8PathBuf> {
        let video_dir = self.directories.temp_video_dir();
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
        let destination = self.directories.compilation_video_dir().join(file_name);

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
                destination.as_str(),
            ]
            .into_iter()
            .map(From::from)
            .collect()
        } else {
            let audio_path = if options.songs.len() >= 2 {
                self.concat_songs(&options.songs, &options.video_id).await?
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
                destination.as_str(),
            ]
            .into_iter()
            .map(From::from)
            .collect()
        };

        self.ffmpeg(args).await?;

        info!("finished assembling video, result at {destination}");
        self.increase_progress(&options.video_id, 1.0, 0.0, "Compiling clips together")
            .await?;
        self.finish_progress(&options.video_id).await?;
        Ok(destination)
    }
}
