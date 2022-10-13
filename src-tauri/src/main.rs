#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use image::ImageFormat;
use std::collections::HashMap;
use tauri::Manager;
use window::WindowExt;

mod download;
mod imager;
mod window;

pub mod img;

#[tauri::command]
fn save_img(buffer: HashMap<String, Vec<u8>>, ext: u8) -> Vec<u8> {
    let buf = match buffer.get("source") {
        Some(buffer) => buffer,
        _ => panic!(""),
    };

    let format = match ext {
        1 => ImageFormat::Jpeg,
        2 => ImageFormat::WebP,
        _ => ImageFormat::Png,
    };

    imager::save_img(buf, format)
}

#[tauri::command]
async fn down(url: String) -> Result<String, String> {
    download::download(url).await
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_transparent_titlebar();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_img, down])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
