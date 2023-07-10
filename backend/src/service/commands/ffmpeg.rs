use std::fs;
use std::io::Write;
use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::bail;
use tokio::process::Command;
use tracing::{debug, info};

use crate::service::directories::Directories;
use crate::util::commandline_error;
use crate::Result;

fn download_url() -> Result<(&'static str, &'static str)> {
    if cfg!(not(target_arch = "x86_64")) {
        bail!("Downloads must be manually provided for non-x86_64 architectures")
    }

    if cfg!(target_os = "windows") {
        Ok((
            "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip",
            "ffmpeg.zip",
        ))
    } else if cfg!(target_os = "unix") || cfg!(target_os = "linux") {
        Ok((
            "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz",
            "ffmpeg.tar.xz",
        ))
    } else {
        bail!("Sorry, unsupported platform.");
    }
}

fn get_ffmpeg_location(base_dir: &Utf8Path) -> Option<FfmpegLocation> {
    use std::process::{Command, Stdio};

    if base_dir.join("ffmpeg").is_file() || base_dir.join("ffmpeg.exe").is_file() {
        Some(FfmpegLocation::Local(Arc::new(base_dir.to_owned())))
    } else {
        let is_system = Command::new("ffmpeg")
            .arg("-version")
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or_else(|_| false);
        if is_system {
            Some(FfmpegLocation::System)
        } else {
            None
        }
    }
}

#[cfg(target_os = "linux")]
fn unzip(path: impl AsRef<Utf8Path>, destination_folder: impl AsRef<Utf8Path>) -> Result<()> {
    use std::fs::File;
    use std::io::BufReader;

    use tar::Archive;
    use xz2::bufread::XzDecoder;

    let file = BufReader::new(File::open(path.as_ref())?);
    let reader = XzDecoder::new(file);
    let mut archive = Archive::new(reader);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        let path = Utf8Path::from_path(path.as_ref()).unwrap();
        if let Some("ffmpeg") | Some("ffprobe") = path.file_name() {
            let dest_file = destination_folder.as_ref().join(path.file_name().unwrap());
            entry.unpack(&dest_file)?;
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn unzip(path: impl AsRef<Utf8Path>, destination_folder: impl AsRef<Utf8Path>) -> Result<()> {
    use std::fs::File;
    use std::io::BufReader;

    use zip::ZipArchive;

    let file = BufReader::new(File::open(path.as_ref())?);
    let mut zip = ZipArchive::new(file)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.name().contains("ffmpeg.exe") || file.name().contains("ffprobe.exe") {
            let path = Utf8Path::new(file.name());
            let file_name = path.file_name().unwrap();

            let dest_path = destination_folder.as_ref().join(file_name);
            let mut dest_file = File::create(&dest_path)?;
            std::io::copy(&mut file, &mut dest_file)?;
        }
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn unzip(_path: impl AsRef<Utf8Path>, _destination_folder: impl AsRef<Utf8Path>) -> Result<()> {
    bail!("Please use `brew` to install ffmpeg manually.")
}

async fn download_archive(url: &str, destination: &Utf8Path) -> Result<()> {
    let mut response = reqwest::get(url).await?.error_for_status()?;
    let mut file = fs::File::create(destination)?;
    info!("downloading file {} to {}", url, destination);
    let content_length = response.content_length().unwrap_or_default();

    let mut bytes_written = 0;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;
        bytes_written += chunk.len();
        debug!("wrote {bytes_written} / {content_length} bytes");
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub enum FfmpegLocation {
    System,
    Local(Arc<Utf8PathBuf>),
}

impl FfmpegLocation {
    pub fn ffmpeg(&self) -> Utf8PathBuf {
        match self {
            FfmpegLocation::System => Utf8PathBuf::from("ffmpeg"),
            FfmpegLocation::Local(path) => path.join("ffmpeg"),
        }
    }

    pub fn ffprobe(&self) -> Utf8PathBuf {
        match self {
            FfmpegLocation::System => Utf8PathBuf::from("ffprobe"),
            FfmpegLocation::Local(path) => path.join("ffprobe"),
        }
    }
}

pub async fn download_ffmpeg(directories: &Directories) -> Result<FfmpegLocation> {
    let dest = directories.cache_dir().join("ffmpeg");
    fs::create_dir_all(&dest)?;

    let existing_install = get_ffmpeg_location(&dest);
    if let Some(location) = existing_install {
        info!("found existing ffmpeg binaries at {location:?}");
        return Ok(location);
    }

    info!("ffmpeg not found, downlaoding...");
    let (url, file_name) = download_url()?;
    info!("downloading ffmpeg from {}", url);

    let archive_dest = dest.join(file_name);
    if !archive_dest.is_file() {
        download_archive(url, &archive_dest).await?;
    }

    info!("downloaded ffmpeg archive to {archive_dest}, extracting...");
    unzip(&archive_dest, &dest)?;
    fs::remove_file(archive_dest)?;

    Ok(FfmpegLocation::Local(Arc::new(dest)))
}

#[derive(Default)]
pub struct Ffmpeg {
    executable_path: Utf8PathBuf,
    inputs: Vec<String>,
    start: Option<String>,
    extra_args: Vec<String>,
    working_directory: Option<Utf8PathBuf>,
    filter: Option<String>,
    format: Option<String>,
    log_level: Option<String>,
    output_file: String,
}

impl Ffmpeg {
    pub fn new(location: &FfmpegLocation, output_file: impl Into<String>) -> Self {
        Ffmpeg {
            output_file: output_file.into(),
            executable_path: location.ffmpeg(),
            ..Default::default()
        }
    }

    pub fn input(&mut self, input: impl Into<String>) -> &mut Self {
        self.inputs.push(input.into());
        self
    }

    #[allow(unused)]
    pub fn working_directory(&mut self, working_directory: impl Into<Utf8PathBuf>) -> &mut Self {
        self.working_directory = Some(working_directory.into());
        self
    }

    pub fn start(&mut self, start: f64) -> &mut Self {
        self.start = Some(start.to_string());
        self
    }

    pub fn video_filter(&mut self, filter: impl Into<String>) -> &mut Self {
        self.filter = Some(filter.into());
        self
    }

    pub fn format(&mut self, format: impl Into<String>) -> &mut Self {
        self.format = Some(format.into());
        self
    }

    pub fn log_level(&mut self, log_level: impl Into<String>) -> &mut Self {
        self.log_level = Some(log_level.into());
        self
    }

    pub fn extra_arg(&mut self, arg: impl Into<String>) -> &mut Self {
        self.extra_args.push(arg.into());
        self
    }

    fn build_args(&self) -> Vec<&str> {
        let mut args = vec!["-hide_banner"];
        args.push("-loglevel");
        if let Some(log_level) = &self.log_level {
            args.push(log_level);
        } else {
            args.push("warning");
        }

        if let Some(start) = &self.start {
            args.push("-ss");
            args.push(start);
        }

        for input in &self.inputs {
            args.push("-i");
            args.push(input);
        }

        if let Some(format) = &self.format {
            args.push("-f");
            args.push(format);
        }

        if let Some(filter) = &self.filter {
            args.push("-filter:v");
            args.push(filter);
        }

        for extra_arg in &self.extra_args {
            args.push(extra_arg);
        }

        args.push(&self.output_file);
        info!("running ffmpeg with arguments {:?}", args);
        args
    }

    fn command(&self) -> Command {
        let args = self.build_args();
        let mut command = Command::new(self.executable_path.as_str());
        command.args(args);

        if let Some(dir) = &self.working_directory {
            command.current_dir(dir);
        }
        command
    }

    pub async fn run(&self) -> Result<()> {
        let mut command = self.command();
        let output = command.output().await?;

        if output.status.success() {
            Ok(())
        } else {
            commandline_error(self.executable_path.as_str(), output)
        }
    }

    pub async fn output(&self) -> Result<String> {
        let mut command = self.command();
        let output = command.output().await?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stderr).into())
        } else {
            commandline_error(self.executable_path.as_str(), output)
        }
    }
}
