use std::fmt::Display;

#[derive(serde::Serialize)]
pub struct DecodeInfo<T> {
    success: bool,
    message: T,
    seed: usize,
}

impl<T: Display> DecodeInfo<T> {
    pub fn success(data: T, seed: usize) -> DecodeInfo<String> {
        DecodeInfo {
            success: true,
            message: format!("{data}"),
            seed,
        }
    }

    pub fn fail(message: T) -> DecodeInfo<String> {
        DecodeInfo {
            success: false,
            message: format!("{message}"),
            seed: 0,
        }
    }
}
