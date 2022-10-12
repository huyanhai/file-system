use image::{imageops::FilterType, load_from_memory_with_format, DynamicImage, ImageFormat};
use std::io::Cursor;

use imager::{
    api::OutMeda,
    classifier,
    codec::{jpeg, png, webp},
    data::OutputFormat,
};

pub fn output_format(
    input: DynamicImage,
    format: OutputFormat,
    extreme_mode: bool,
) -> Result<(Vec<u8>), ()> {
    match format {
        OutputFormat::Webp => {
            let (out, meta) = webp::opt::opt(&input);
            Ok(out)
        }
        OutputFormat::Jpeg => {
            let (out, meta) = jpeg::OptContext::from_image(input.clone()).run_search(extreme_mode);
            Ok(out)
        }
        OutputFormat::Png => {
            let out = png::basic_optimize(&input);
            Ok(out)
        }
    }
}

pub fn save_img(buf: &Vec<u8>) -> Vec<u8> {
    let img = load_from_memory_with_format(buf, ImageFormat::JPEG).unwrap();
    let out = output_format(img, OutputFormat::Jpeg, true).expect("msg");
    out
}
