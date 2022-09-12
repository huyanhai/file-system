#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod files;

#[tauri::command]
fn dir_lists(name: &str) -> Vec<files::PathInfo> {
    let result = files::dir_lists(name);
    return result;
}

#[tauri::command]
fn content(path: &str) -> String {
    return files::file_content(path);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![dir_lists, content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
