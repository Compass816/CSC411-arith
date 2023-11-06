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

    //
    let new_data: Vec<RgbF32Temp> = arr
    .iter_row_major()
    .map(|(_, _, element)| {
        let r = element.red as f32 / img.denominator as f32;
        let g = element.green as f32 / img.denominator as f32;
        let b = element.blue as f32 / img.denominator as f32;
        RgbF32Temp {
            red: r,
            green: g,
            blue: b,
        }
    })
    .collect();

}

pub fn decompress(filename: Option<&str>) {
    todo!();
}
