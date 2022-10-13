use image::{DynamicImage, GenericImageView};

use libc;

// copy as https://github.com/imager-io/imager/blob/244df6721db203ba04d28391318395e9f195bf6d/imager/src/codec/jpeg.rs#L43

#[allow(non_snake_case)]
const TRUE: mozjpeg_sys::boolean = true as mozjpeg_sys::boolean;
#[allow(non_snake_case)]
const FALSE: mozjpeg_sys::boolean = false as mozjpeg_sys::boolean;

const COLOR_SPACE: mozjpeg_sys::J_COLOR_SPACE = mozjpeg_sys::J_COLOR_SPACE::JCS_RGB;
const COLOR_SPACE_COMPONENTS: libc::c_int = 3 as libc::c_int;

pub unsafe fn compress(source: &DynamicImage, quality: u8) -> Vec<u8> {
    ///////////////////////////////////////////////////////////////////////////
    // INPUT
    ///////////////////////////////////////////////////////////////////////////
    let rgb_source = source
        .to_rgb8()
        .pixels()
        .flat_map(|x| x.0.to_vec())
        .collect::<Vec<_>>();

    let (width, height) = source.dimensions();

    ///////////////////////////////////////////////////////////////////////////
    // INIT ENCODER CONTEXT
    ///////////////////////////////////////////////////////////////////////////
    let mut err = std::mem::zeroed();
    let mut cinfo: mozjpeg_sys::jpeg_compress_struct = std::mem::zeroed();
    let mut outbuffer: *mut libc::c_uchar = std::ptr::null_mut();
    let mut outsize: libc::c_ulong = 0;

    cinfo.common.err = mozjpeg_sys::jpeg_std_error(&mut err);
    mozjpeg_sys::jpeg_create_compress(&mut cinfo);
    mozjpeg_sys::jpeg_mem_dest(&mut cinfo, &mut outbuffer, &mut outsize);

    ///////////////////////////////////////////////////////////////////////////
    // ENCODER CONFIG
    ///////////////////////////////////////////////////////////////////////////
    cinfo.image_width = width;
    cinfo.image_height = height;
    cinfo.input_components = COLOR_SPACE_COMPONENTS;
    let row_stride = cinfo.image_width as usize * cinfo.input_components as usize;
    cinfo.in_color_space = COLOR_SPACE;
    mozjpeg_sys::jpeg_set_defaults(&mut cinfo);
    cinfo.dct_method = mozjpeg_sys::J_DCT_METHOD::JDCT_ISLOW;
    cinfo.write_JFIF_header = FALSE;
    cinfo.optimize_coding = TRUE;
    mozjpeg_sys::jpeg_simple_progression(&mut cinfo);
    mozjpeg_sys::jpeg_c_set_bool_param(
        &mut cinfo,
        mozjpeg_sys::JBOOLEAN_USE_SCANS_IN_TRELLIS,
        TRUE,
    );
    mozjpeg_sys::jpeg_c_set_bool_param(
        &mut cinfo,
        mozjpeg_sys::JBOOLEAN_USE_LAMBDA_WEIGHT_TBL,
        TRUE,
    );
    mozjpeg_sys::jpeg_set_quality(&mut cinfo, quality as i32, TRUE);

    ///////////////////////////////////////////////////////////////////////////
    // GO!
    ///////////////////////////////////////////////////////////////////////////
    mozjpeg_sys::jpeg_start_compress(&mut cinfo, TRUE);
    while cinfo.next_scanline < cinfo.image_height {
        let offset = cinfo.next_scanline as usize * row_stride;
        let jsamparray = [rgb_source[offset..].as_ptr()];
        mozjpeg_sys::jpeg_write_scanlines(&mut cinfo, jsamparray.as_ptr(), 1);
    }
    mozjpeg_sys::jpeg_finish_compress(&mut cinfo);
    mozjpeg_sys::jpeg_destroy_compress(&mut cinfo);

    ///////////////////////////////////////////////////////////////////////////
    // OUTPUT
    ///////////////////////////////////////////////////////////////////////////
    let output_data = std::slice::from_raw_parts(outbuffer, outsize as usize).to_vec();

    ///////////////////////////////////////////////////////////////////////////
    // CLEANUP
    ///////////////////////////////////////////////////////////////////////////
    if !outbuffer.is_null() {
        // FREE MEMORY DEST
        libc::free(outbuffer as *mut mozjpeg_sys::c_void);
        outbuffer = std::ptr::null_mut();
        outsize = 0;
    }

    ///////////////////////////////////////////////////////////////////////////
    // DONE
    ///////////////////////////////////////////////////////////////////////////
    output_data
}
