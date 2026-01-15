// System information commands

/// Get system platform and version information
#[tauri::command]
pub async fn get_system_info() -> serde_json::Value {
    #[cfg(target_os = "windows")]
    let platform = "windows";
    #[cfg(target_os = "macos")]
    let platform = "macos";
    #[cfg(target_os = "linux")]
    let platform = "linux";

    serde_json::json!({
        "platform": platform,
        "version": env!("CARGO_PKG_VERSION")
    })
}
