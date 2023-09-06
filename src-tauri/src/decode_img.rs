use crate::utils::error::MyError;
use base64::{engine::general_purpose, Engine as _};
use image::{ImageBuffer, Rgba, RgbaImage};
use std::{io::Cursor, path::PathBuf};

type Png = ImageBuffer<Rgba<u8>, Vec<u8>>;

const DIR_CATCH: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ImageData {
    pub seed: usize,
    pub base64: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ImageSaveStruct {
    pub filename: String,
    pub base64: String,
}

pub fn save_image(base64: &str, path: PathBuf) -> Result<(), MyError> {
    let img: Png = base64_to_png(base64)?;
    if let Err(error) = img.save(path) {
        return Err(MyError::new(&format!("Save Image Error: {error}")));
    }

    Ok(())
}

pub fn base64_to_png(base64: &str) -> Result<Png, MyError> {
    match general_purpose::STANDARD.decode(base64) {
        Ok(decode_data) => match image::load_from_memory(&decode_data) {
            Ok(img) => Ok(img.into_rgba8()),
            Err(error) => Err(MyError::new(&format!("Convert to Image Error: {error}"))),
        },
        Err(error) => Err(MyError::new(&format!("Base64 Decode Error: {error}"))),
    }
}

pub fn png_to_base64(img: &Png) -> String {
    let mut buf: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)
        .unwrap();
    general_purpose::STANDARD.encode(&buf)
}

/**
 * DecodeV2 Rotate
 */
fn decode_v2_step1(img: &mut Png) {
    let (width, height) = img.dimensions();
    let mut new_img: RgbaImage = ImageBuffer::new(width, height);
    let mut border = 0;
    let mut dir = 0;
    let mut pos: (i32, i32) = (0, 0);
    for pixel in img.pixels_mut() {
        if dir == 0 && pos.0 == width as i32 - 1 - border
            || dir == 1 && pos.1 == height as i32 - 1 - border
            || dir == 2 && pos.0 == border
            || dir == 3 && pos.1 == border
        {
            dir = (dir + 1) % 4;

            if dir == 3 {
                border += 1;
            }
        }

        new_img.put_pixel(pos.0 as u32, pos.1 as u32, *pixel);

        pos.0 += DIR_CATCH[dir].0;
        pos.1 += DIR_CATCH[dir].1;
    }

    for (x, y, pixel) in new_img.enumerate_pixels() {
        img.put_pixel(x, y, *pixel);
    }
}

/**
 * DecodeV2 hash
 */
fn decode_v2_step2(img: &mut Png, mut seed: usize) -> Result<(), MyError> {
    let filename = format!("{seed}.png");
    seed = seed >> 1;
    let carry_count = seed % 10;
    seed /= 10;
    let offset: usize = seed % 100;
    seed /= 100;
    let mut check_sum = seed + carry_count + offset;

    let mut count = 0;
    for (index, color) in img.iter_mut().enumerate() {
        let index = index % 256;

        if index != 0 && index % offset == 0 {
            count += carry_count;
        }

        if *color % 2 == 1 {
            if check_sum == 0 {
                return Err(MyError::new(&format!("{filename} Seed mistake")));
            } else {
                check_sum -= 1;
            }
        }

        *color = ((*color as usize + 256 - (index * index + count) % 256) % 256) as u8;
    }

    if check_sum != 0 {
        return Err(MyError::new(&format!("{filename} Seed mistake")));
    }

    Ok(())
}

pub fn decode_v2(img: &mut Png, seed: usize) -> Result<(), MyError> {
    decode_v2_step1(img);
    decode_v2_step2(img, seed)
}
