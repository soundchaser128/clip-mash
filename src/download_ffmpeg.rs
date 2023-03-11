use std::process::{Command, Stdio};

use camino::Utf8PathBuf;
use tokio::{fs, io::{self, AsyncWriteExt}};

use crate::Result;

fn download_url() -> Result<(&'static str, &'static str)> {
    if cfg!(not(target_arch = "x86_64")) {
        return Err("Downloads must be manually provided for non-x86_64 architectures".into());
    }

    if cfg!(target_os = "windows") {
        Ok(("https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip", "ffmpeg.zip"))
    } else if cfg!(target_os = "macos") {
        Ok(("https://evermeet.cx/ffmpeg/getrelease", "ffmpeg.7z"))
    } else if cfg!(target_os = "linux") {
        Ok(("https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz", "ffmpeg.tar.xz"))
    } else {
        Err("Unsupported platform".into())
    }
}

fn is_installed() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or_else(|_| false)
}

pub async fn download() -> Result<Utf8PathBuf> {
    if is_installed() {
        return Ok("ffmpeg".into())
    }
    
    let (url, file_name) = download_url()?;
    let mut response = reqwest::get(url).await?.error_for_status()?;
    let mut file = fs::File::create(file_name).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }

    // todo decompress file
    // sevenz-rust, xz, tar, zip

    todo!()
}
