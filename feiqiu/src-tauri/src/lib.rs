#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ipc;
pub mod services;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to 飞秋!", name)
}

#[tauri::command]
async fn get_system_info() -> serde_json::Value {
    #[cfg(target_os = "windows")]
    let platform = "windows";
    #[cfg(target_os = "macos")]
    let platform = "macos";
    #[cfg(target_os = "linux")]
    let platform = "linux";

    serde_json::json!({
        "platform": platform,
        "version": "1.0.0"
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
