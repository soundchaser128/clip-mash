use std::env::consts::{ARCH, EXE_EXTENSION, OS};
use axum::body::Bytes;
use std::path::Path;
use crate::Result;

const GITHUB_REPO: &str = "https://github.com/soundchaser128/clip-mash";

async fn download_url(url: &str) -> Result<Bytes> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes)
}

#[cfg(unix)]
fn unzip_file(bytes: Bytes, path: &Path) -> Result<()> {

    todo!()
}

pub async fn self_update() -> Result<()> {
    let version = "latest";
    eprintln!("Updating to {version}");
    let binary = format!("rye-{ARCH}-{OS}");
    let ext = if cfg!(unix) { ".gz" } else { ".exe" };
    let url = if version == "latest" {
        format!("{GITHUB_REPO}/releases/latest/download/{binary}{ext}")
    } else {
        format!("{GITHUB_REPO}/releases/download/{version}/{binary}{ext}")
    };
    let bytes = download_url(&url).await?;
    let tmp = tempfile::NamedTempFile::new()?;
    unzip_file(bytes, tmp.path())?;

    self_replace::self_replace(tmp.path())?;
    Ok(())
}
