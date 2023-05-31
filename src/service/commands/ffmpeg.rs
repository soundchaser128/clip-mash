use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::bail;
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
            "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip",
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

fn is_installed(dest: &Utf8Path) -> bool {
    if dest.join("ffmpeg").is_file() || dest.join("ffmpeg.exe").is_file() {
        true
    } else {
        Command::new("ffmpeg")
            .arg("-version")
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or_else(|_| false)
    }
}

#[cfg(target_os = "linux")]
fn unzip(
    path: impl AsRef<Utf8Path>,
    destination_folder: impl AsRef<Utf8Path>,
) -> Result<Utf8PathBuf> {
    use std::fs::File;
    use std::io::BufReader;

    use tar::Archive;
    use xz2::bufread::XzDecoder;

    let file = BufReader::new(File::open(path.as_ref())?);
    let reader = XzDecoder::new(file);
    let mut archive = Archive::new(reader);
    let dest_file = destination_folder.as_ref().join("ffmpeg");
    info!("unzipping to {dest_file}");

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        let path = Utf8Path::from_path(path.as_ref()).unwrap();
        if let Some("ffmpeg") = path.file_name() {
            entry.unpack(&dest_file)?;
        }
    }

    Ok(dest_file)
}

#[cfg(target_os = "windows")]
fn unzip(
    path: impl AsRef<Utf8Path>,
    destination_folder: impl AsRef<Utf8Path>,
) -> Result<Utf8PathBuf> {
    use std::fs::File;
    use std::io::BufReader;

    use zip::ZipArchive;

    let file = BufReader::new(File::open(path.as_ref())?);
    let mut zip = ZipArchive::new(file)?;
    let dest_path = destination_folder.as_ref().join("ffmpeg.exe");
    let mut dest_file = File::create(&dest_path)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.name().contains("ffmpeg.exe") {
            std::io::copy(&mut file, &mut dest_file)?;
            return Ok(dest_path);
        }
    }
    bail!("no file found")
}

#[cfg(target_os = "macos")]
fn unzip(
    _path: impl AsRef<Utf8Path>,
    _destination_folder: impl AsRef<Utf8Path>,
) -> Result<Utf8PathBuf> {
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

pub async fn download_ffmpeg(directories: &Directories) -> Result<Utf8PathBuf> {
    let dest = directories.cache_dir().join("ffmpeg");
    fs::create_dir_all(&dest)?;
    if is_installed(&dest) {
        info!("ffmpeg already installed, not doing anything.");
        return Ok("ffmpeg".into());
    }

    let (url, file_name) = download_url()?;

    let archive_dest = dest.join(file_name);
    if !archive_dest.is_file() {
        download_archive(url, &archive_dest).await?;
    }

    info!("downloaded ffmpeg archive");
    let ffmpeg_path = unzip(&archive_dest, dest)?;
    fs::remove_file(archive_dest)?;

    Ok(ffmpeg_path)
}

#[allow(unused)]
#[derive(Default)]
pub struct Ffmpeg {
    executable_path: Utf8PathBuf,
    inputs: Vec<String>,
    output: Option<String>,
    start: Option<String>,
    duration: Option<String>,
    video_codec: Option<String>,
    audio_codec: Option<String>,
    crf: Option<String>,
    preset: Option<String>,
    extra_args: Vec<String>,
    video_filter: Option<String>,
    working_directory: Option<Utf8PathBuf>,
    output_file: String,
}

impl Ffmpeg {
    pub fn new(executable: impl AsRef<Utf8Path>, output_file: String) -> Self {
        Ffmpeg {
            output_file,
            executable_path: executable.as_ref().to_owned(),
            ..Default::default()
        }
    }

    pub fn input(&mut self, input: impl Into<String>) -> &mut Self {
        self.inputs.push(input.into());
        self
    }

    pub fn working_directory(&mut self, working_directory: impl Into<Utf8PathBuf>) -> &mut Self {
        self.working_directory = Some(working_directory.into());
        self
    }

    pub fn start(&mut self, start: f64) -> &mut Self {
        self.start = Some(start.to_string());
        self
    }

    pub fn duration(&mut self, duration: f64) -> &mut Self {
        self.duration = Some(duration.to_string());
        self
    }

    pub async fn run(&self) -> Result<()> {
        let mut args = vec!["-hide-banner", "-loglevel", "warning"];

        if let Some(start) = &self.start {
            args.push("-ss");
            args.push(&start);
        }

        for input in &self.inputs {
            args.push("-i");
            args.push(&input);
        }

        args.push(&self.output_file);
        let mut command = Command::new(self.executable_path.as_str());
        command.args(args);

        if let Some(dir) = &self.working_directory {
            command.current_dir(dir);
        }

        let output = command.output()?;

        if output.status.success() {
            Ok(())
        } else {
            commandline_error(self.executable_path.as_str(), output)
        }
    }
}
