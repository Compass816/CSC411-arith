use array2::Array2;
use csc411_image;
use csc411_image::{Read, Rgb, RgbImage, Write};
use csc411_rpegio::{output_rpeg_data, input_rpeg_data};

use crate::{bitpack, pack_2x2_elements, unpack_2x2_pixels};
//use crate::compute_cv_byte;
use crate::quantize::{encodes, encodeu};
use crate::to_component_video::to_ypbpr;
use crate::to_component_video::{from_ypbpr};
use crate::to_rgb_float::{to_rgbf32, from_rgb32};
use crate::trim_to_even_dimensions;

use crate::unpack_bits;

/// Performs all functions to compress an image, including trimming the image, converting to RgbF32, then to component video,
/// then packing the pixels into 2x2 groups, quantizing, and then bitpacking.
/// 
/// # Arguments:
/// * `filename`: an option &str that is the filename of the ppm image to compress
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

    // testing to see if float values are printed (they are)

    // set array to 2x2 pixels and values we need
    let packed_arr = pack_2x2_elements(arr_cv);

    let mut empty_vec = vec![];
    for (_x, _y, &ref element) in packed_arr.iter_row_major() {
        let qa = encodeu(element.0, 9, 0.3);
        let qb = encodes(element.1, 5, 0.3);
        let qc = encodes(element.2, 5, 0.3);
        let qd = encodes(element.3, 5, 0.3);

        /*println!(
            "{}, {}, {}, {}, {}, {}, {}, {}",
            element.0, qa, element.1, qb, element.2, qc, element.3, qd
        );*/

        let test = bitpack(qa, qb, qc, qd, element.4 as u32, element.5 as u32).unwrap();
        empty_vec.push(test);
    }

    let compressed_data: Vec<[u8; 4]> = empty_vec.into_iter().map(u32::to_be_bytes).collect();

    output_rpeg_data(&compressed_data, width, height).unwrap();
}


/// Performs all functions to decompress an image, including unpacking the Array2, converting to component video, and then to RGB.
/// 
/// # Arguments:
/// * `filename`: an option &str that is the filename of the rpeg compressed data to decompress
pub fn decompress(filename: Option<&str>) {
    let file = input_rpeg_data(filename);
    let file = file.unwrap();

    let width = file.1 / 2 as usize;
    let height = file.2 / 2 as usize;

    let mut decompressed_vec = vec![];

    for word in file.0 {
        let vals = unpack_bits(u32::from_be_bytes(word));
        decompressed_vec.push(vals);
    }

    let decompressed_arr = Array2 {
        width,
        height,
        data: decompressed_vec,
    };

    let unpacked_arr = unpack_2x2_pixels(decompressed_arr);
    let returned_cv_arr = from_ypbpr(&unpacked_arr);

    let returned_float_arr = from_rgb32(&returned_cv_arr);

    for (x, y, &ref element) in returned_cv_arr.iter_row_major() {
        let modified_element = element.clone();

        println!("{}, {}, {}", modified_element.red, modified_element.green, modified_element.blue);
    }

    let image = from_array2(&returned_float_arr);
    let _ = RgbImage::write(&image, None);


    //let mut vec = vec![];

    /*for (x, y, &ref element) in arr_rgb.iter_row_major() {

        let modified_element = element.clone();

        println!("{}, {}, : {:?}", x, y, modified_element);
    }*/

}

fn from_array2(arr: &Array2<Rgb>) -> RgbImage {
    let width = arr.width() as u32;
    let height = arr.height() as u32;
    let denominator = 255;
    let pixels: Vec<Rgb> = arr.data().to_vec();

    RgbImage {
        width,
        height,
        denominator,
        pixels,
    }
}
