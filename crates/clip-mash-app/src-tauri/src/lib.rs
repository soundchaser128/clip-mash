use anyhow::anyhow;
use tauri::Manager;
use tauri_plugin_shell::{ShellExt, process::CommandChild};

struct AppState {
    sidecar_process: CommandChild,
}

// TODO no sidcear, but start server in a separate thread in the same process?
fn start_sidecar(app: &tauri::AppHandle) -> tauri::Result<CommandChild> {
    let handle = app
        .shell()
        .sidecar("../../../target/release/clip-mash-server")
        .map_err(|e| anyhow!("Failed to get sidecar: {}", e))?;

    let (_rx, child) = handle
        .spawn()
        .map_err(|e| anyhow!("Failed to spawn sidecar: {}", e))?;

    Ok(child)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
