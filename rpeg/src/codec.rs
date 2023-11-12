use csc411_image;
use csc411_image::{Rgb, RgbImage, Read};
use array2::Array2;
use csc411_rpegio::{output_rpeg_data, debug_output_rpeg_data};

use crate::trim_to_even_dimensions;
use crate::pack_2x2_pixels;
use crate::average_pbpr;
use crate::get_luminosity_coeffs;
use crate::to_rgb_float::to_rgbf32;
use crate::to_component_video::to_ypbpr;
use crate::to_component_video::to_rgb;
use crate::quantize::encode;
use crate::quantize::scale_sat;
use crate::quantize::smax;
use crate::bitpack;
use std::f32::consts::E;





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
    
    //println!("{}, {}", arr_cv.width(), arr_cv.height());



    // testing to see if float values are printed (they are)
  

   // let check3 = to_rgb(&arr_cv);
  //  set array to 2x2 pixels and values we need 
    let check4 = pack_2x2_pixels(&arr_cv);


  

  //  for (x, y, &ref element) in check3.iter_row_major() {
 //       println!("{}, {}, : {:?}", x, y, element);
//    }
 //   for (x, y, &ref element) in check4.iter_row_major() {
//           println!("{}, {}, : {:?}", x, y, element);
    //   }

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
for (x, y, &ref element) in check4.iter_row_major() {

    let qa = encode(element.0, 9, 0.3) as u32;
    let qb = encode(element.1, 5, 0.3);
    let qc = encode(element.2, 5, 0.3);
    let qd = encode(element.3, 5, 0.3);

    let test = bitpack(qa, qb, qc, qd, element.4 as u32, element.5 as u32).unwrap();
    empty_vec.push(test);

 
  //  println!("{}, {}, : {:?}", x, y, modified_element);
}
let compressed_data: Vec<[u8; 4]> = empty_vec.into_iter().map(u32::to_be_bytes).collect();

debug_output_rpeg_data(&compressed_data, width as u32, height as u32);







// Output the rpeg data to stdout

    
}

pub fn decompress(filename: Option<&str>) {
    todo!();

}
