use image::{load_from_memory_with_format, ImageFormat};
use std::io::Cursor;

pub fn save_img(buf: &Vec<u8>) -> Vec<u8> {
    let img = load_from_memory_with_format(buf, ImageFormat::Jpeg).unwrap();

    let mut buffer = Vec::new();

    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Jpeg)
        .unwrap();

    buffer
}
