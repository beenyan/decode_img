// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod decode_img;
mod utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::img_decode_v2,
            commands::save_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
