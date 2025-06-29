use std::process::Stdio;
use std::time::Duration;

use color_eyre::Result;
use color_eyre::eyre::bail;
use rand::Rng;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let timeout = Duration::from_secs(60);
    let interval_duration = Duration::from_millis(100);

    let port: u32 = rand::rng().random_range(1024..65535);
    eprintln!("Starting server on port {port}");

    let mut process = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("0.0.0.0")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;

    let mut elapsed = Duration::from_secs(0);
    loop {
        if elapsed >= timeout {
            bail!("Server did not start in {} seconds", timeout.as_secs());
        }

        let response = reqwest::get(&format!("http://localhost:{port}/api/system/health")).await;
        if let Ok(response) = response {
            if response.status().is_success() {
                break;
            }
        }

        tokio::time::sleep(interval_duration).await;
        elapsed += interval_duration;
    }

    let response = reqwest::get(&format!("http://localhost:{port}/api-docs/openapi.json")).await?;
    let spec = response.text().await?;

    let file_name = std::env::args().nth(1).unwrap_or("openapi.json".into());
    tokio::fs::write(file_name, &spec).await?;

    process.kill().await?;

    Ok(())
}
