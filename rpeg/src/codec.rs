use csc411_image;
use csc411_image::{Rgb, RgbImage, Read};
use array2::Array2;

use crate::trim_to_even_dimensions;

pub fn compress(filename: Option<&str>) {
    // Construct an Array2
    let img = RgbImage::read(filename).unwrap();
    let height = img.height.try_into().unwrap();
    let width = img.width.try_into().unwrap();
    let usize_vec: Vec<csc411_image::Rgb> = img.pixels.clone();
    let arr: Array2<Rgb> = Array2::from_row_major(width, height, usize_vec).unwrap();

    // Trim rows and/or cols to be an even number
    trim_to_even_dimensions(&arr);

}

pub fn decompress(filename: Option<&str>) {
    todo!();
}
