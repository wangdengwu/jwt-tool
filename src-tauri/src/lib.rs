#[tauri::command]
fn show_message() -> Result<String, String> {
    Ok("Hello from Rust!".into())
}

#[tauri::command]
fn get_system_info() -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        platform: std::env::consts::OS.to_string(),
        tauri_version: env!("CARGO_PKG_VERSION").to_string(),
        app_name: "My Tauri App".to_string(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![show_message,get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(serde::Serialize)]
struct SystemInfo {
    platform: String,
    tauri_version: String,
    app_name: String,
}