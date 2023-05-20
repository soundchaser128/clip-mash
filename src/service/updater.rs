use crate::Result;
use axum::body::Bytes;
use std::path::Path;
use std::{
    env::consts::{ARCH, EXE_EXTENSION, OS},
    io::Cursor,
};

const GITHUB_REPO: &str = "https://github.com/soundchaser128/clip-mash";

async fn download_url(url: &str) -> Result<Bytes> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes)
}

// executables come in a .tar.gz archive for Mac OS and Linux
#[cfg(unix)]
fn unzip_file(bytes: Bytes, destination: &Path) -> Result<()> {
    use camino::Utf8Path;
    use libflate::gzip::Decoder;
    use std::io::Read;
    use tar::Archive;

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
fn unzip_file(bytes: Bytes, destination: &Path) -> Result<()> {
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
