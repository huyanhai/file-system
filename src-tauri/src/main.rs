#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use window::WindowExt;

mod window;

use window::ToolbarThickness;

#[tauri::command]
fn content() -> String {
    return String::from("test");
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_transparent_titlebar(ToolbarThickness::Thick);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
