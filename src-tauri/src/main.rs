#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod show;

#[tauri::command]
fn greet(name: &str) -> Vec<show::PathInfo> {
    let result = show::dir_lists(name);
    return result;
}

#[tauri::command]
fn content(path: &str) -> String {
    return show::file_content(path);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
