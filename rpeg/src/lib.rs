pub mod codec;
pub mod quantize;
pub mod to_component_video;
pub mod to_rgb_float;

use core::f32;
use std::{usize, vec};

use array2::Array2;
use bitpack::bitpack::{fitss, fitsu, gets, getu, news, newu};
use csc411_arith::{chroma_of_index, index_of_chroma};
use csc411_image::Rgb;
use to_component_video::YPbPr;

pub fn trim_to_even_dimensions(arr: &Array2<Rgb>) -> Array2<Rgb> {
    let new_width = if arr.width() % 2 == 0 {
        arr.width()
    } else {
        arr.width() - 1
    };

    let new_height = if arr.height() % 2 == 0 {
        arr.height()
    } else {
        arr.height() - 1
    };

    let mut new_data = Vec::with_capacity(new_width * new_height);

    for y in 0..new_height {
        for x in 0..new_width {
            let element = arr.get(x, y).clone();
            new_data.push(element);
        }
    }

    return Array2::from_row_major(new_width, new_height, new_data).unwrap();
}

/* Returns: an Array2 of quantized values, a 6-tuple containing a, b, c, d, pb ave, and pr ave
pub fn pack_2x2_pixels(arr: &Array2<YPbPr>) -> Array2<(f32, f32, f32, f32, usize, usize)> {
    let mut packed_arr: Array2<(f32, f32, f32, f32, usize, usize)> = Array2 {
        width: arr.width / 2,
        height: arr.height / 2,
        data: vec![],
    };

    // Loop invariant: arr will always have an even number of rows and cols
    for (x, y, _value) in arr.iter_row_major() {
        let group: [[&YPbPr; 2]; 2] = [
            [arr.get(x, y), arr.get(x + 1, y)],
            [arr.get(x, y + 1), arr.get(x + 1, y + 1)],
        ];

        let chroma_vals = average_pbpr(group);
        let luminosity_coeffs: (f32, f32, f32, f32) = get_luminosity_coeffs(group);

        packed_arr.data.push((
            luminosity_coeffs.0, 
            luminosity_coeffs. 1, 
            luminosity_coeffs. 2, 
            luminosity_coeffs. 3, 
            chroma_vals.0, 
            chroma_vals.1
        ));
    }

    return packed_arr;
}*/


fn pack_2x2_elements(elements: Array2<YPbPr>) -> Array2<(f32, f32, f32, f32, usize, usize)> {
    let mut packed_elements = Array2 {
        width: elements.width / 2,
        height: elements.height / 2,
        data: vec![],
    };

    // Iterate through the elements in groups of 4 to pack them into tuples.
    for i in (0..elements.data.len()).step_by(4) {
        if i + 3 < elements.data.len() {
            let group: [YPbPr; 4] = [elements.data[i], elements.data[i + 1], elements.data[i + 2], elements.data[i + 3]];
            let luminosity_coeffs = get_luminosity_coeffs(group);
            let chroma_vals = average_pbpr(group);

            packed_elements.data.push((luminosity_coeffs.0, luminosity_coeffs.1, luminosity_coeffs.2, luminosity_coeffs.3, chroma_vals.0, chroma_vals.1));
        } else {
            // Handle cases where there are fewer than 4 elements remaining.
            let remaining_elements = elements.data.len() - i;
            let mut sub_vector: [YPbPr; 4] = Default::default();

            for j in 0..remaining_elements {
                sub_vector[j] = elements.data[i + j];
            }

            let luminosity_coeffs = get_luminosity_coeffs(sub_vector);
            let chroma_vals = average_pbpr(sub_vector);

            packed_elements.data.push((luminosity_coeffs.0, luminosity_coeffs.1, luminosity_coeffs.2, luminosity_coeffs.3, chroma_vals.0, chroma_vals.1));
        }
    }

    packed_elements
}





pub fn compute_cv_value(group: [YPbPr; 4], pos_x: usize, pos_y: usize) -> YPbPr {
    let pixel_x = pos_x % 2;
    let pixel_y = pos_y % 2;
    let pixel_position = pixel_y * 2 + pixel_x;

    match pixel_position {
        0 => group[0],
        1 => group[1],
        2 => group[2],
        3 => group[3],
        _ => YPbPr { y: 0.0, pb: 0.0, pr: 0.0 },
    }
}


pub fn unpack_2x2_pixels_from_float(arr: Array2<(f32, f32, f32, f32, usize, usize)>) -> Array2<YPbPr> {
    let mut grouped_pixels = vec![];
    for (_x, _y, i) in arr.iter_row_major() {
        
        let y = reverse_luminosity_coeffs_from_float(i.0, i.1, i.2, i.3);

        let pb_chroma = chroma_of_index(i.4);
        let pr_chroma = chroma_of_index(i.5);
    
        let e1 = YPbPr {y: y.0, pb: pb_chroma, pr: pr_chroma};
        let e2 = YPbPr {y: y.1, pb: pb_chroma, pr: pr_chroma};
        let e3 = YPbPr {y: y.2, pb: pb_chroma, pr: pr_chroma};
        let e4 = YPbPr {y: y.3, pb: pb_chroma, pr: pr_chroma};

        grouped_pixels.push([e1, e2, e3, e4]);
    }

    let mut unpacked_arr = Array2 {
        width: arr.width * 2,
        height: arr.height * 2,
        data: vec![],
    };

    for pixel_group in &grouped_pixels {
        for &pixel in pixel_group.iter() {
            unpacked_arr.data.push(pixel);
        }
    }
    unpacked_arr
}



pub fn unpack_2x2_pixels(arr: Array2<(u32, i32, i32, i32, usize, usize)>) -> Array2<YPbPr> {
    let mut grouped_pixels = vec![];
    for (_x, _y, i) in arr.iter_row_major() {
        
        let y = reverse_luminosity_coeffs(i.0, i.1, i.2, i.3);

        let pb_chroma = chroma_of_index(i.4);
        let pr_chroma = chroma_of_index(i.5);
    
        let e1 = YPbPr {y: y.0 as f32, pb: pb_chroma, pr: pr_chroma};
        let e2 = YPbPr {y: y.1 as f32, pb: pb_chroma, pr: pr_chroma};
        let e3 = YPbPr {y: y.2 as f32, pb: pb_chroma, pr: pr_chroma};
        let e4 = YPbPr {y: y.3 as f32, pb: pb_chroma, pr: pr_chroma};

        grouped_pixels.push([e1, e2, e3, e4]);
    }

    let mut unpacked_arr = Array2 {
        width: arr.width * 2,
        height: arr.height * 2,
        data: vec![],
    };

    for pixel_group in &grouped_pixels {
        for &pixel in pixel_group.iter() {
            unpacked_arr.data.push(pixel);
        }
    }
    unpacked_arr
}


pub fn iter_row_major_2x2<'a, T: Clone>(
    arr: &'a Array2<T>,
) -> impl Iterator<Item = (usize, usize, T)> + 'a {
    (0..arr.height()).step_by(2).flat_map(move |y| {
        (0..arr.width())
            .step_by(2)
            .map(move |x| (x, y, arr.get(x, y).clone()))
    })
}


pub fn average_pbpr(group: [YPbPr; 4]) -> (usize, usize) {
    let pb_ave =
        (group[0].pb() + group[1].pb() + group[2].pb() + group[3].pb()) / 4 as f32;
    let pr_ave =
        (group[0].pr() + group[1].pr() + group[2].pr() + group[3].pr()) / 4 as f32;

    (index_of_chroma(pb_ave), index_of_chroma(pr_ave))
}


pub fn get_luminosity_coeffs(group: [YPbPr; 4]) -> (f32, f32, f32, f32) {
    let y1 = group[0].y();
    let y2 = group[1].y();
    let y3 = group[2].y();
    let y4 = group[3].y();

    let a = (y4 + y3 + y2 + y1) / 4.0;
    let b = (y4 + y3 - y2 - y1) / 4.0;
    let c = (y4 - y3 + y2 - y1) / 4.0;
    let d = (y4 - y3 - y2 + y1) / 4.0;

    (a, b, c, d)
}


pub fn reverse_luminosity_coeffs_from_float(a: f32, b: f32, c: f32, d: f32) -> (f32, f32, f32, f32) {
    let y1 = a - b - c + d;
    let y2 = a - b + c - d;
    let y3 = a + b - c - d;
    let y4 = a + b + c + d;

    (y1, y2, y3, y4)
}


pub fn reverse_luminosity_coeffs(a: u32, b: i32, c: i32, d: i32) -> (u32, i32, i32, i32) {
    let y1 = a as f32 - b as f32 - c as f32 + d as f32;
    let y2 = a as f32 - b as f32 + c as f32 - d as f32;
    let y3 = a as f32 + b as f32 - c as f32 - d as f32;
    let y4 = a as f32 + b as f32 + c as f32 + d as f32;

    (y1 as u32, y2 as i32, y3 as i32, y4 as i32)
}


pub fn bitpack(a: u32, b: i32, c: i32, d: i32, pb: u32, pr: u32) -> Option<u32> {
    let mut val: u64 = 0;

    if fitsu(a as u64, 9) {
        val = match newu(val, 9, 23, a as u64) {
            Some(val) => val,
            None => return None,
        };
    } else {
        return None;
    }

    if fitss(b as i64, 5) {
        val = match news(val, 5, 18, b as i64) {
            Some(val) => val,
            None => return None,
        };
    } else {
        return None;
    }

    if fitss(c as i64, 5) {
        val = match news(val, 5, 13, c as i64) {
            Some(val) => val,
            None => return None,
        };
    } else {
        return None;
    }

    if fitss(d as i64, 5) {
        val = match news(val, 5, 8, d as i64) {
            Some(val) => val,
            None => return None,
        };
    } else {
        return None;
    }

    if fitsu(pb as u64, 4) {
        val = match newu(val, 4, 4, pb as u64) {
            Some(val) => val,
            None => return None,
        };
    } else {
        return None;
    }

    if fitsu(pr as u64, 4) {
        val = match newu(val, 4, 0, pr as u64) {
            Some(val) => val,
            None => return None,
        };
    } else {
        return None;
    }

    Some(val as u32)
}


pub fn unpack_bits(packed_value: u32) -> (u32, i32, i32, i32, usize, usize) {
    let a = getu(packed_value as u64, 9, 23) as u32;
    let b = gets(packed_value as u64, 5, 18) as i32;
    let c = gets(packed_value as u64, 5, 13) as i32;
    let d = gets(packed_value as u64, 5, 8) as i32;
    let pb = getu(packed_value as u64, 4, 4) as usize;
    let pr = getu(packed_value as u64, 4, 0) as usize;

    (a, b, c, d, pb, pr)
}



#[cfg(test)]
mod tests {
    use crate::{
        pack_2x2_elements, unpack_2x2_pixels, to_component_video::from_ypbpr, to_component_video::to_ypbpr,
        to_rgb_float::from_rgb32, to_rgb_float::to_rgbf32, trim_to_even_dimensions, quantize::encode, bitpack, unpack_bits, unpack_2x2_pixels_from_float
    };
    use array2::Array2;
    use csc411_image::{Read, Rgb, RgbImage, Write};
    use csc411_rpegio::{output_rpeg_data, output_rpeg_data_debug};

    #[test]
    fn trim() {
        // Construct an Array2
        let img = RgbImage::read(Some("frost.ppm")).unwrap();
        let height = img.height.try_into().unwrap();
        let width = img.width.try_into().unwrap();
        let usize_vec: Vec<csc411_image::Rgb> = img.pixels.clone();
        let arr: Array2<Rgb> = Array2::from_row_major(width, height, usize_vec).unwrap();

        // Trim rows and/or cols to be an even number
        let arr_trimmed = trim_to_even_dimensions(&arr);

        let float_arr = to_rgbf32(&arr_trimmed);
        let cv_arr = to_ypbpr(&float_arr);
        let packed_arr = pack_2x2_elements(cv_arr);

        //let unpacked_arr = unpack_2x2_pixels_from_float(packed_arr);

        let mut empty_vec = vec![];
        for (_x, _y, &ref element) in packed_arr.iter_row_major() {
            let qa = encode(element.0, 9, 0.3) as u32;
            let qb = encode(element.1, 5, 0.3);
            let qc = encode(element.2, 5, 0.3);
            let qd = encode(element.3, 5, 0.3);
    
            let test = bitpack(qa, qb, qc, qd, element.4 as u32, element.5 as u32).unwrap();
            empty_vec.push(test);
        }
    
        let compressed_data: Vec<[u8; 4]> = empty_vec.into_iter().map(u32::to_be_bytes).collect();
        output_rpeg_data_debug(&compressed_data, width, height).unwrap();

        // End of compression

        let mut decompressed_vec = vec![];

        for word in compressed_data {
            let vals = unpack_bits(u32::from_be_bytes(word));
            decompressed_vec.push(vals);
        }

        let decompressed_arr = Array2::from_row_major(packed_arr.width, packed_arr.height, decompressed_vec).unwrap();
        let unpacked_arr = unpack_2x2_pixels(decompressed_arr);
        let returned_cv_arr = from_ypbpr(&unpacked_arr);
        let returned_float_arr = from_rgb32(&returned_cv_arr);

        let out_image = RgbImage {
            height: height as u32,
            width: width as u32,
            denominator: img.denominator,
            pixels: returned_float_arr.data,
        };

        out_image.write(Some("new_out.ppm")).unwrap();
    }
}
