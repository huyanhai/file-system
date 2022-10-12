#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use tauri::Manager;
use window::WindowExt;

mod images;
mod window;

pub mod img;

#[tauri::command]
fn save_img(buffer: HashMap<String, Vec<u8>>) -> Vec<u8> {
    let buf = match buffer.get("name") {
        Some(buffer) => buffer,
        _ => panic!(""),
    };
    images::save_img(buf)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_transparent_titlebar();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_img])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
