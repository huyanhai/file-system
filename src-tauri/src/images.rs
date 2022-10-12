use image::{load_from_memory_with_format, ImageFormat};

use crate::img::jepg::encode;

pub fn save_img(buf: &Vec<u8>) -> Vec<u8> {
    let img = load_from_memory_with_format(buf, ImageFormat::Jpeg).unwrap();
    unsafe { encode(&img, 80) }
}
