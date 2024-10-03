// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use serde::Serialize;

#[derive(Serialize)]
struct FoiRequest {
    id: u32,
    url: String,
    title: String,
}

#[tauri::command]
fn get_foirequests() -> Vec<FoiRequest> {
    vec![
        FoiRequest {
            id: 1,
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
        },
        FoiRequest {
            id: 2,
            url: "https://example.org".to_string(),
            title: "Example 2".to_string(),
        },
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_foirequests])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
