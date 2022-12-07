use csc411_image::{RgbImage, Read, Write};
use crate::blocks::{blocks_to_cvc, cvc_to_blocks};
use crate::conversions::{convert_cvc_to_rgb, convert_rgb_to_cvc, floats_to_rgb, rgb_to_floats, trimmed_size};
use crate::words::{extract_words, packing, unpacking};

/// Compresses an image to standard out
/// # Arguments:
/// * 'filename' Name of ppm file to be compressed
pub fn compress(filename: &str) {
    let img = RgbImage::read(Some(filename)).unwrap();

    let size = trimmed_size(&img);

    let rgb_floats = rgb_to_floats(&img.pixels.clone());

    let cvc_pixels = convert_rgb_to_cvc(rgb_floats);

    let blocks = cvc_to_blocks(cvc_pixels.clone(), size.0, size.1);

    packing(blocks, size.0 as u32, size.1 as u32);
}

/// Decompresses an image to standard out
/// # Arguments:
/// * 'filename' Name of bytes file to be decompressed
pub fn decompress(filename: &str) {
    let img = csc411_rpegio::read_in_rpeg_data(Some(filename));

    let words = extract_words(img.0);

    let blocks = unpacking(words);

    let cvc_pixels = blocks_to_cvc(blocks, img.1 as usize, img.2 as usize);

    let rgb_floats = convert_cvc_to_rgb(cvc_pixels);

    let rgb = floats_to_rgb(rgb_floats);

    let image = RgbImage {
        pixels: rgb,
        denominator: 255,
        width: img.1,
        height: img.2
    };

    image.write(None).unwrap();
}
