use std::time::Duration;

use color_eyre::Result;
use rand::Rng;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let port: u32 = rand::thread_rng().gen_range(1024..65535);
    eprintln!("Starting server on port {}", port);
    // killall clip-mash

    let mut process = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("0.0.0.0")
        .arg(port.to_string())
        .spawn()?;

    loop {
        let response = reqwest::get(&format!("http://localhost:{}/api/system/health", port)).await;
        if let Ok(response) = response {
            if response.status().is_success() {
                break;
            }
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    let response =
        reqwest::get(&format!("http://localhost:{}/api-docs/openapi.json", port)).await?;
    let spec = response.text().await?;

    let file_name = std::env::args().nth(1).unwrap_or("openapi.json".into());
    tokio::fs::write(file_name, &spec).await?;

    process.kill().await?;

    Ok(())
}
