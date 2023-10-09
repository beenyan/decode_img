use crate::{
    decode_img::{decode_v2, rgba8_to_rgb8, ImageData, ImageSaveStruct},
    utils::{ctm_type::temp_dir_path, result::DecodeInfo},
};
use image::{self, DynamicImage};
use std::{ffi::OsStr, fs, path::Path};

#[tauri::command]
pub async fn img_seed_extract(path: std::path::PathBuf) -> usize {
    path.file_stem()
        .unwrap_or(OsStr::new("0"))
        .to_str()
        .unwrap_or("0")
        .parse::<usize>()
        .unwrap_or(0)
}

#[tauri::command]
pub async fn img_decode_v2(data_array: Vec<ImageData>) -> Vec<DecodeInfo<String>> {
    let mut result = Vec::with_capacity(data_array.len());
    for img_data in data_array {
        if let Ok(dynamic_image) = image::open(img_data.file_path) {
            let (decoded_image, img) = match dynamic_image {
                DynamicImage::ImageRgb8(mut img) => (decode_v2(&mut img, img_data.seed), img),
                DynamicImage::ImageRgba8(mut img) => {
                    (decode_v2(&mut img, img_data.seed), rgba8_to_rgb8(&img))
                }
                _ => {
                    result.push(DecodeInfo::fail("Unknown Image."));
                    continue;
                }
            };

            if let Err(err) = decoded_image {
                result.push(DecodeInfo::fail(err));
                continue;
            }

            let file_path = Path::new(&temp_dir_path()).join(&format!("{}.png", img_data.seed));
            if let Err(err) = fs::create_dir_all(&temp_dir_path()) {
                result.push(DecodeInfo::fail(&format!(
                    "Create Temp Directory Failed: {err}"
                )));
            } else if let Err(err) = img.save(&file_path) {
                result.push(DecodeInfo::fail(&format!("Save Image Failed: {err}")));
            }

            result.push(DecodeInfo::success(&file_path.display(), img_data.seed));
        }
    }

    result
}

#[tauri::command]
pub fn save_image(data_array: Vec<ImageSaveStruct>) -> Vec<DecodeInfo<String>> {
    let mut result = Vec::with_capacity(data_array.len());
    for data in data_array {
        let to = Path::new(&data.dir_path).join(&data.filename);
        match fs::copy(data.file_path, to) {
            Ok(_) => result.push(DecodeInfo::success(
                &format!("{} save successed", data.filename),
                0,
            )),
            Err(err) => result.push(DecodeInfo::fail(err)),
        }
    }

    result
}
