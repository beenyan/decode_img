use crate::utils::ctm_type::{Png, PngA};
use crate::utils::error::MyError;
use image::ImageBuffer;
use std::path::PathBuf;

const DIR_CATCH: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ImageData {
    pub seed: usize,
    pub file_path: PathBuf,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ImageSaveStruct {
    pub filename: String,
    pub file_path: PathBuf,
    pub dir_path: PathBuf,
}

/**
 * DecodeV2 Rotate
 */
fn decode_v2_step1<T: image::Pixel<Subpixel = u8>>(img: &mut ImageBuffer<T, Vec<T::Subpixel>>) {
    let (width, height) = img.dimensions();
    let mut new_img = ImageBuffer::new(width, height);
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
fn decode_v2_step2<T: image::Pixel<Subpixel = u8>>(
    img: &mut ImageBuffer<T, Vec<T::Subpixel>>,
    mut seed: usize,
) -> Result<(), MyError> {
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

pub fn decode_v2<T: image::Pixel<Subpixel = u8>>(
    img: &mut ImageBuffer<T, Vec<T::Subpixel>>,
    seed: usize,
) -> Result<(), MyError> {
    decode_v2_step1(img);
    decode_v2_step2(img, seed)
}

pub fn rgba8_to_rgb8(input: &PngA) -> Png {
    let width = input.width() as usize;
    let height = input.height() as usize;

    // Get the raw image data as a vector
    let input: &Vec<u8> = input.as_raw();

    // Allocate a new buffer for the RGB image, 3 bytes per pixel
    let mut output_data = vec![0u8; width * height * 3];

    for (output, chunk) in output_data.chunks_exact_mut(3).zip(input.chunks_exact(4)) {
        // ... and copy each of them to output, leaving out the A byte
        output.copy_from_slice(&chunk[0..3]);
    }

    // Construct a new image
    image::ImageBuffer::from_raw(width as u32, height as u32, output_data).unwrap()
}
