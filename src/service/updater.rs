/// Self-update logic ported over from https://github.com/mitsuhiko/rye/blob/ecfc17e7c31137d060d43e22d93637541aa0b051/rye/src/cli/rye.rs#L131-L190
use crate::Result;
use axum::body::Bytes;
use camino::Utf8Path;
use color_eyre::eyre::bail;
use std::env::consts::OS;

use tracing::info;

const GITHUB_REPO: &str = "https://github.com/soundchaser128/clip-mash";

async fn download_url(url: &str) -> Result<Bytes> {
    info!("downloading binary from URL {url}");
    let response = reqwest::get(url).await?.error_for_status()?;
    let bytes = response.bytes().await?;
    Ok(bytes)
}

// executables come in a .tar.gz archive for Mac OS and Linux
#[cfg(unix)]
fn unzip_file(bytes: Bytes, destination: impl AsRef<Utf8Path>) -> Result<()> {
    use libflate::gzip::Decoder;
    use std::io::Read;
    use tar::Archive;

    let destination = destination.as_ref();
    info!("unzipping binary to {destination}");

    let mut decoder = Decoder::new(&*bytes)?;
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data)?;

    let mut archive = Archive::new(&*decoded_data);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        let path = Utf8Path::from_path(path.as_ref()).unwrap();
        if let Some("clip-mash") = path.file_name() {
            entry.unpack(&destination)?;
        }
    }

    Ok(())
}

// executables are in a ZIP archive for Windows
#[cfg(not(unix))]
fn unzip_file(bytes: Bytes, destination: impl AsRef<Utf8Path>) -> Result<()> {
    use std::fs::File;
    use zip::ZipArchive;

    let reader = Cursor::new(&bytes);
    let mut zip = ZipArchive::new(reader)?;
    let mut dest_file = File::create(destination)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.name().contains("clip-mash.exe") {
            std::io::copy(&mut file, &mut dest_file)?;
        }
    }

    Ok(())
}

pub async fn self_update(tag: Option<&str>) -> Result<()> {
    let version = tag.unwrap_or("latest");
    info!("Updating to {version}");
    let binary = match OS {
        "linux" => "clip-mash-x86_64-unknown-linux-gnu.tar.gz",
        "macos" => "clip-mash-x86_64-apple-darwin.tar.gz",
        "windows" => "clip-mash-x86_64-pc-windows-msvc.zip",
        os => bail!("unsupported OS {os}"),
    };
    let url = if version == "latest" {
        format!("{GITHUB_REPO}/releases/latest/download/{binary}")
    } else {
        format!("{GITHUB_REPO}/releases/download/{version}/{binary}")
    };
    let bytes = download_url(&url).await?;
    let tmp = tempfile::NamedTempFile::new()?;
    let path = Utf8Path::from_path(tmp.path()).expect("path must be utf-8");
    unzip_file(bytes, path)?;

    info!("unzipped executable, replacing self");
    self_replace::self_replace(tmp.path())?;
    Ok(())
}
