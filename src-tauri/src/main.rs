// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod decode_img;
mod utils;

use std::path::Path;
use utils::ctm_type::temp_dir_path;

fn main() {
    if Path::new(&temp_dir_path()).is_dir() {
        std::fs::remove_dir_all(&temp_dir_path()).unwrap();
    }
    println!("{:?}", temp_dir_path().display());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::img_seed_extract,
            commands::img_decode_v2,
            commands::save_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
