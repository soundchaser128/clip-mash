use std::env::consts::{ARCH, EXE_EXTENSION, OS};
use crate::Result;

const GITHUB_REPO: &str = "https://github.com/soundchaser128/clip-mash";

async fn download_url(url: &str) -> Result<Vec<u8>> {
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

    // unix currently comes compressed, windows comes uncompressed
    #[cfg(unix)]
    {
        use tokio::fs;
        
        // let mut decoder = flate2::bufread::GzDecoder::new(&bytes[..]);
        let mut rv = Vec::new();
        decoder.read_to_end(&mut rv)?;
        fs::write(tmp.path(), rv)?;
    }
    #[cfg(windows)]
    {
        fs::write(tmp.path(), bytes)?;
    }

    self_replace::self_replace(tmp.path())?;
    Ok(())
}
