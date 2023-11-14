use array2::Array2;
use csc411_image;
use csc411_image::{Read, Rgb, RgbImage, Write};
use csc411_rpegio::{output_rpeg_data, input_rpeg_data};

use crate::{bitpack, pack_2x2_elements, unpack_2x2_pixels};
//use crate::compute_cv_byte;
use crate::quantize::encode;
use crate::to_component_video::to_ypbpr;
use crate::to_component_video::{from_ypbpr, YPbPr};
use crate::to_rgb_float::{to_rgbf32, from_rgb32};
use crate::trim_to_even_dimensions;

use crate::unpack_bits;
use crate::chroma_of_index;
use crate::reverse_luminosity_coeffs;

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

    // let check3 = to_rgb(&arr_cv);
    //  set array to 2x2 pixels and values we need
    let check4 = pack_2x2_elements(arr_cv);

    //  for (x, y, &ref element) in check3.iter_row_major() {
    //       println!("{}, {}, : {:?}", x, y, element);
    //    }

    /*    for (x, y, &ref element) in check4.iter_row_major() {

               let mut modified_element = element.clone();

               modified_element.0 = encode(element.0, 9, 0.3) as f32;
               modified_element.1 = encode(element.1, 5, 0.3) as f32;
               modified_element.2 = encode(element.2, 5, 0.3) as f32;
               modified_element.3 = encode(element.3, 5, 0.3) as f32;

               println!("{}, {}, : {:?}", x, y, modified_element);
        }
    */

    let mut empty_vec = vec![];
    //println!("Before println");
    for (x, y, &ref element) in check4.iter_row_major() {
        let qa = encode(element.0, 9, 0.3) as u32;
        let qb = encode(element.1 * 100.0, 5, 0.3) as i32;
        let qc = encode(element.2, 5, 0.3) as i32;
        let qd = encode(element.3, 5, 0.3) as i32;
        //println!("Line, {}, {}", element.1, qb);

        let test = bitpack(qa, qb, qc, qd, element.4 as u32, element.5 as u32).unwrap();
        empty_vec.push(test);

    }


    let compressed_data: Vec<[u8; 4]> = empty_vec.into_iter().map(u32::to_be_bytes).collect();

    output_rpeg_data(&compressed_data, width, height).unwrap();
}


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

    let image = from_array2(&returned_float_arr);
    let _ = RgbImage::write(&image, None);


    //let mut vec = vec![];

    /*for (x, y, &ref element) in arr_rgb.iter_row_major() {

        let modified_element = element.clone();

        println!("{}, {}, : {:?}", x, y, modified_element);
    }*/

    //let image = from_array2(&arr_rgb);
    //let _ = RgbImage::write(&image, None);
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
