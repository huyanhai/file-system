use image::{load_from_memory_with_format, ImageFormat};

use crate::img::jpeg;
use crate::img::png;

pub fn save_img(buf: &Vec<u8>, format: ImageFormat) -> Vec<u8> {
    let img = load_from_memory_with_format(buf, format).unwrap();

    let buffer = match format {
        ImageFormat::Jpeg => unsafe { jpeg::compress(&img, 80) },
        _ => png::compress(&img),
    };

    buffer
}
