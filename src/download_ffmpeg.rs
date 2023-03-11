use std::process::{Command, Stdio};

use camino::{Utf8Path, Utf8PathBuf};
use tokio::{fs, io::AsyncWriteExt};

use crate::Result;

fn download_url() -> Result<(&'static str, &'static str)> {
    if cfg!(not(target_arch = "x86_64")) {
        return Err("Downloads must be manually provided for non-x86_64 architectures".into());
    }

    if cfg!(target_os = "windows") {
        Ok((
            "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip",
            "ffmpeg.zip",
        ))
    } else if cfg!(target_os = "macos") {
        Ok(("https://evermeet.cx/ffmpeg/getrelease", "ffmpeg.7z"))
    } else if cfg!(target_os = "unix") || cfg!(target_os = "linux") {
        Ok((
            "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz",
            "ffmpeg.tar.xz",
        ))
    } else {
        Err("Unsupported platform".into())
    }
}

fn is_installed(dest: &Utf8Path) -> bool {
    if dest.join("ffmpeg").is_file() {
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
    use std::{fs::File, io::BufReader};
    use tar::Archive;
    use xz2::bufread::XzDecoder;

    let file = BufReader::new(File::open(path.as_ref())?);
    let reader = XzDecoder::new(file);
    let mut archive = Archive::new(reader);
    let dest_file = destination_folder.as_ref().join("ffmpeg");
    tracing::info!("unzipping to {dest_file}");

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

async fn download_archive(url: &str, destination: &Utf8Path) -> Result<()> {
    let mut response = reqwest::get(url).await?.error_for_status()?;
    let mut file = fs::File::create(destination).await?;
    tracing::info!("downloading file {} to {}", url, destination);
    let content_length = response.content_length().unwrap_or_default();

    let mut bytes_written = 0;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        bytes_written += chunk.len();
        tracing::debug!("wrote {bytes_written} / {content_length} bytes");
    }

    Ok(())
}

pub async fn download() -> Result<Utf8PathBuf> {
    let dest = Utf8Path::new("ffmpeg");
    fs::create_dir_all(dest).await?;
    if is_installed(&dest) {
        tracing::info!("ffmpeg already installed, not doing anything.");
        return Ok("ffmpeg".into());
    }

    let (url, file_name) = download_url()?;

    let archive_dest = dest.join(file_name);
    if !archive_dest.is_file() {
        download_archive(url, &archive_dest).await?;
    }

    tracing::info!("downloaded ffmpeg archive");
    unzip(&archive_dest, dest)
}
