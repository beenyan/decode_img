use image::{ImageBuffer, Rgb, Rgba};
use std::path::{Path, PathBuf};

pub type PngA = ImageBuffer<Rgba<u8>, Vec<u8>>;
pub type Png = ImageBuffer<Rgb<u8>, Vec<u8>>;

pub fn temp_dir_path() -> PathBuf {
    Path::new(&std::env::temp_dir()).join("Snowflake")
}
