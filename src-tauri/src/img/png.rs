use exoquant::*;
use exoquant::{
    ditherer, optimizer, optimizer::Optimizer, Color, Histogram, Quantizer, Remapper,
    SimpleColorSpace,
};
use image::{DynamicImage, GenericImageView};

use lodepng;

#[derive(Debug, Clone, PartialEq)]
pub enum ImageMode {
    Text,
}

fn encode_indexed(palette: &[Color], image: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut state = lodepng::State::new();
    for color in palette {
        unsafe {
            lodepng::ffi::lodepng_palette_add(
                &mut state.info_png_mut().color,
                color.r,
                color.g,
                color.b,
                color.a,
            );
            lodepng::ffi::lodepng_palette_add(
                &mut state.info_raw_mut(),
                color.r,
                color.g,
                color.b,
                color.a,
            );
        }
    }
    state.info_png_mut().color.set_bitdepth(8);
    state.info_png_mut().color.colortype = lodepng::ColorType::PALETTE;
    state.info_raw_mut().set_bitdepth(8);
    state.info_raw_mut().colortype = lodepng::ColorType::PALETTE;
    state
        .encode(image, width as usize, height as usize)
        .expect("encode png data")
}

pub fn compress(source: &DynamicImage, mode: ImageMode, num_colors: usize) -> Vec<u8> {
    let input_pixels = source
        .pixels()
        .map(|(_, _, px)| Color::new(px.0[0], px.0[1], px.0[2], px.0[3]))
        .collect::<Vec<Color>>();

    let colorspace = SimpleColorSpace::default();
    let optimizer = optimizer::KMeans;

    let histogram = input_pixels[..].iter().cloned().collect();

    let mut quantizer = Quantizer::new(&histogram, &colorspace);

    while quantizer.num_colors() < 256 {
        quantizer.step();
        // very optional optimization, !very slow!
        // you probably only want to do this every N steps, if at all.
        if quantizer.num_colors() % 64 == 0 {
            quantizer = quantizer.optimize(&optimizer, 4);
        }
    }

    let palette = quantizer.colors(&colorspace);
    // this optimization is more useful than the above and a lot less slow
    let palette = optimizer.optimize_palette(&colorspace, &palette, &histogram, 16);

    let ditherer = ditherer::FloydSteinberg::new();
    let remapper = Remapper::new(&palette, &colorspace, &ditherer);
    let indexed_data = remapper.remap(&input_pixels[..], source.width() as usize);

    encode_indexed(&palette, &indexed_data, source.width(), source.height())
}
