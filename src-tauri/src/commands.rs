use crate::{
    decode_img::{self, base64_to_png, decode_v2, png_to_base64, ImageData, ImageSaveStruct},
    utils::result::DecodeInfo,
};

/**
 * Base64 to Png
 */
#[tauri::command]
pub async fn img_decode_v2(data_array: Vec<ImageData>) -> Vec<DecodeInfo<String>> {
    let mut result = Vec::with_capacity(data_array.len());
    for img_data in data_array {
        match base64_to_png(&img_data.base64) {
            Ok(mut img) => {
                if let Err(err) = decode_v2(&mut img, img_data.seed) {
                    result.push(DecodeInfo::fail(err));
                    continue;
                }

                let base64 = png_to_base64(&img);
                result.push(DecodeInfo::success(&base64, img_data.seed));
            }
            Err(err) => result.push(DecodeInfo::fail(err)),
        }
    }

    result
}

#[tauri::command]
pub fn save_image(data_array: Vec<ImageSaveStruct>) -> Vec<DecodeInfo<String>> {
    let mut result = Vec::with_capacity(data_array.len());
    for data in data_array {
        match decode_img::save_image(&data.base64, (&data.filename).into()) {
            Ok(_) => result.push(DecodeInfo::success(
                &format!("{} save successed", data.filename),
                0,
            )),
            Err(err) => result.push(DecodeInfo::fail(err)),
        }
    }

    result
}
