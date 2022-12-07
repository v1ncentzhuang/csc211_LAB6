use csc411_image::{Rgb, RgbImage};
use crate::structs::{CVC, FloatRgb};

/// Converts RGB floating points into CVC pixels
/// Returns a new vector of CVC pixels
/// # Arguments:
/// * 'floating_pixels' Vector of floating RGB values
pub fn convert_rgb_to_cvc(floating_pixels: Vec<FloatRgb>) -> Vec<CVC> {
    let mut cvc_pixels:Vec<CVC> = Vec::new();

    for i in floating_pixels.iter() {
        let temp = CVC {
            y: 0.299 * i.red + 0.587 * i.green + 0.114 * i.blue,
            pb: -0.168736 * i.red - 0.331264 * i.green + 0.5 * i.blue,
            pr: 0.5 * i.red - 0.418688 * i.green - 0.081312 * i.blue,
        };
        cvc_pixels.push(temp);
    }
    cvc_pixels
}
/// Convert CVC pixels to RGB pixels (floats)
/// Returns an array of RGB pixels (floats)
/// # Arguments
/// * 'cvc_pixels' Vector of CVC pixels
pub fn convert_cvc_to_rgb(cvc_pixels: Vec<CVC>) -> Vec<FloatRgb> {

    let mut rgb_floats = Vec::new();
    for i in cvc_pixels.iter() {
        let temp = FloatRgb {
            red: 1.0 * i.y + 0.0 * i.pb + 1.402 * i.pr,
            green: 1.0 * i.y - 0.344136 * i.pb - 0.714136 * i.pr,
            blue: 1.0 * i.y + 1.772 * i.pb + 0.0 * i.pr,
        };
        rgb_floats.push(temp);
    }
    rgb_floats
}

/// Converts RGB floating points to RGB integers
/// Returns a vector of RGB pixels (integers)
/// # Arguments:
/// * 'rgb_floats' Vector of floating RGB values
pub fn floats_to_rgb(rgb_floats: Vec<FloatRgb>) -> Vec<Rgb> {
    let mut rgb_pixels = Vec::new();

    for i in rgb_floats.iter() {
        let temp = Rgb {
            red: (i.red * 255.0) as u16,
            green: (i.green * 255.0) as u16,
            blue: (i.blue * 255.0) as u16,
        };
        rgb_pixels.push(temp);
    }
    rgb_pixels
}

///Converts RGB values to floating point representations
/// Returns a vector of RGB values (integers)
/// # Arguments:
/// * 'rgb_pixels' Vector of RGB values (integers)
pub fn rgb_to_floats(rgb_pixels:&Vec<Rgb>) -> Vec<FloatRgb> {

    let mut floating_pixels:Vec<FloatRgb> = Vec::new();

    for i in rgb_pixels.iter() {
        let temp = FloatRgb {
            red: i.red as f64 / 255 as f64,
            blue: i.blue as f64 / 255 as f64,
            green: i.green as f64 / 255 as f64,
        };
        floating_pixels.push(temp);
    }
    floating_pixels
}

/// Trims the width and height of an image to ensure they are NOT odd
/// Returns a tuple containing the adjusted (width, height)
/// # Argument:
/// 'img' A csc411 RgbImage
///
pub fn trimmed_size(img: &RgbImage) -> (usize, usize) {
    let mut width = img.width.clone();
    let mut height = img.height.clone();

    width = width & !1_u32;
    height = height & !1_u32;
    return (width as usize, height as usize);
}
