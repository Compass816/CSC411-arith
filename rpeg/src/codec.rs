use csc411_image;
use csc411_image::{Rgb, RgbImage, Read};
use array2::Array2;

use crate::trim_to_even_dimensions;
use crate::pack_2x2_pixels;
use crate::average_pbpr;
use crate::get_luminosity_coeffs;
use crate::to_rgb_float::to_rgbf32;
use crate::to_component_video::to_ypbpr;

pub fn compress(filename: Option<&str>) {
    // Construct an Array2
    let img = RgbImage::read(filename).unwrap();
    let height = img.height.try_into().unwrap();
    let width = img.width.try_into().unwrap();
    let usize_vec: Vec<csc411_image::Rgb> = img.pixels.clone();
    let arr: Array2<Rgb> = Array2::from_row_major(width, height, usize_vec).unwrap();

    // Trim rows and/or cols to be an even number
    let arr_trimmed = trim_to_even_dimensions(&arr);

    // Convert pixels to a triplet of f32s
    let arr_f = to_rgbf32(&arr_trimmed);

    // Convert to component video
    let arr_cv = to_ypbpr(&arr_f);
    
    println!("{}, {}", arr_cv.width(), arr_cv.height());

    // Group pixels 2x2, average Pb and Pr, discrete cosine on Ys, and group b, c, and d^c
    // in progress


    // testing to see if float values are printed (they are)
    for (x, y, element) in arr_cv.iter_row_major() {
        println!("{}, {}, : {}", x, y, element);
    }

}

pub fn decompress(filename: Option<&str>) {
    todo!();
}
