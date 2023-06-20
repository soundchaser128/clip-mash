/// Self-update logic ported over from https://github.com/mitsuhiko/rye/blob/ecfc17e7c31137d060d43e22d93637541aa0b051/rye/src/cli/rye.rs#L131-L190
use std::env;
use std::env::consts::{EXE_EXTENSION, OS};
use std::process::Command;
use std::{fs, process};

use axum::body::Bytes;
use camino::Utf8Path;
use clip_mash_types::AppVersion;
use color_eyre::eyre::bail;
use reqwest::Client;
use semver::Version;
use serde_json::Value;
use tracing::info;

use crate::data::database::Database;
use crate::Result;

const GITHUB_USER: &str = "soundchaser128";
const GITHUB_REPO_NAME: &str = "clip-mash";
const GITHUB_REPO_URL: &str = "https://github.com/soundchaser128/clip-mash";

async fn download_url(url: &str) -> Result<Bytes> {
    info!("downloading binary from URL {url}");
    let response = reqwest::get(url).await?.error_for_status()?;
    let bytes = response.bytes().await?;
    Ok(bytes)
}

// executables come in a .tar.gz archive for Mac OS and Linux
#[cfg(unix)]
fn unzip_file(bytes: Bytes, destination: impl AsRef<Utf8Path>) -> Result<()> {
    use std::io::Read;

    use libflate::gzip::Decoder;
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
    use std::io::Cursor;

    use zip::ZipArchive;

    let reader = Cursor::new(&bytes);
    let mut zip = ZipArchive::new(reader)?;
    let mut dest_file = File::create(destination.as_ref())?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.name().contains("clip-mash.exe") {
            std::io::copy(&mut file, &mut dest_file)?;
        }
    }

    Ok(())
}

async fn fetch_release(client: &Client, database: &Database) -> Result<Value> {
    if let Some(release) = database.latest_release().await? {
        info!("found cached release JSON");
        Ok(release)
    } else {
        let url = format!(
            "https://api.github.com/repos/{GITHUB_USER}/{GITHUB_REPO_NAME}/releases/latest"
        );
        info!("sending request to {url}");
        let response = client
            .get(&url)
            .header("User-Agent", "clip-mash")
            .send()
            .await?
            .error_for_status()?;
        let release = response.json::<Value>().await?;
        database.persist_release(&release).await?;
        Ok(release)
    }
}

pub async fn check_for_updates(client: &Client, database: &Database) -> Result<AppVersion> {
    let release = fetch_release(client, database).await?;
    let name = release["tag_name"].as_str().unwrap();
    info!("latest release is {name}");
    // compare it to the current version
    let version = &name[1..];
    let version = Version::parse(version)?;
    let current_version = Version::parse(env!("CARGO_PKG_VERSION"))?;
    info!("current version is {current_version}, latest version is {version}");

    Ok(AppVersion {
        new_version: version.to_string(),
        current_version: current_version.to_string(),
        needs_update: version > current_version,
    })
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
        format!("{GITHUB_REPO_URL}/releases/latest/download/{binary}")
    } else {
        format!("{GITHUB_REPO_URL}/releases/download/{version}/{binary}")
    };
    let bytes = download_url(&url).await?;
    let path = Utf8Path::new("clip-mash-new").with_extension(EXE_EXTENSION);
    unzip_file(bytes, &path)?;
    // TODO remove assets folder before starting new version
    info!("unzipped executable to {path}, replacing self");
    self_replace::self_replace(&path)?;
    fs::remove_file(&path)?;

    let current_executable = env::current_exe()?;
    info!("starting process from {}", current_executable.display());
    let _process = Command::new(current_executable).spawn()?;
    process::exit(0);
}
