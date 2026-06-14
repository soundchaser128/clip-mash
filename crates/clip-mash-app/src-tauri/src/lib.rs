use log::{error, info};
use tauri::Manager;
use tauri::async_runtime::JoinHandle;

struct AppState {
    _server_handle: JoinHandle<()>,
}

fn start_server() -> Result<JoinHandle<()>, Box<dyn std::error::Error>> {
    use tauri::async_runtime::spawn;

    // Spawn the server on the runtime
    let server_handle = spawn(async {
        info!("Starting clip-mash server in background thread");
        if let Err(e) = clip_mash_server::start_server().await {
            error!("Server failed to start: {}", e);
        }
    });

    Ok(server_handle)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .setup(|app| {
            let server_handle = start_server().map_err(|e| {
                error!("Failed to start server: {}", e);
                e
            })?;
            app.manage(AppState {
                _server_handle: server_handle,
            });
            info!("Server starting in background...");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
