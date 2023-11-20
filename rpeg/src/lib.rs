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
use quantize::{encodes, encodeu};
use to_component_video::YPbPr;

/// Trims an Array2<Rgb> to have an even width and height
/// # Returns:
/// * `Array2<Rgb>` : a trimmed Array2<Rgb>
///
/// # Arguments:
/// * `arr`: a ref Array2<Rgb>
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

/// Packs pixels of a coponent video Array2 into 2x2 groups and computes the luminosity and chroma index values, and packaging them into a 6-tuple.
/// # Returns:
/// * `Array2<(f32, f32, f32, f32, usize, usize)>` : a 6-tuple containing a, b, c, d, pb ave, and pr ave
///
/// # Arguments:
/// * `arr`: a Array2<YPbPr>, a Array2 of component video pixels
fn pack_2x2_elements(arr: Array2<YPbPr>) -> Array2<(f32, f32, f32, f32, usize, usize)> {
    let mut packed_elements =
        Array2::blank_state(arr.width / 2, arr.height / 2, (0.0, 0.0, 0.0, 0.0, 0, 0));

    // Iterate through 2x2 groups
    for x in (0..arr.width).step_by(2) {
        for y in (0..arr.height).step_by(2) {
            // Extract the 2x2 group
            let e1 = arr.get(x, y);
            let e2 = arr.get(x + 1, y);
            let e3 = arr.get(x, y + 1);
            let e4 = arr.get(x + 1, y + 1);

            let group = [e1, e2, e3, e4];
            let luminosity_coeffs = get_luminosity_coeffs(group);
            let chroma_vals = average_pbpr(group);

            let val = packed_elements.get_mut(x / 2, y / 2);
            *val = (
                luminosity_coeffs.0,
                luminosity_coeffs.1,
                luminosity_coeffs.2,
                luminosity_coeffs.3,
                chroma_vals.0,
                chroma_vals.1,
            );
        }
    }

    packed_elements
}

/// Unpacks a pixels of a Array2 that contains a tuple of a, b, c, d, pb ave, and pr ave into pixels
/// # Returns:
/// * `Array2<YPbPr>` : a Array2 of component video pixels
///
/// # Arguments:
/// * `arr`: a Array2<(u32, i32, i32, i32, usize, usize)>, a 6-tuple containing a, b, c, d, pb ave, and pr ave
pub fn unpack_2x2_pixels(arr: Array2<(u32, i32, i32, i32, usize, usize)>) -> Array2<YPbPr> {
    let mut unpacked_elements = Array2::blank_state(
        arr.width * 2,
        arr.height * 2,
        YPbPr {
            y: 0.0,
            pb: 0.0,
            pr: 0.0,
        },
    );

    // Iterate through packed elements
    for x in 0..arr.width {
        for y in 0..arr.height {
            let y_coeffs = reverse_luminosity_coeffs(
                arr.get(x, y).0,
                arr.get(x, y).1,
                arr.get(x, y).2,
                arr.get(x, y).3,
            );

            let pb_chroma = chroma_of_index(arr.get(x, y).4);
            let pr_chroma = chroma_of_index(arr.get(x, y).5);

            // Unpack the values into the corresponding 2x2 group
            let e1 = unpacked_elements.get_mut(x * 2, y * 2);
            e1.y = y_coeffs.0 as f32;
            e1.pb = pb_chroma;
            e1.pr = pr_chroma;

            let e2 = unpacked_elements.get_mut(x * 2 + 1, y * 2);
            e2.y = y_coeffs.1 as f32;
            e2.pb = pb_chroma;
            e2.pr = pr_chroma;

            let e3 = unpacked_elements.get_mut(x * 2, y * 2 + 1);
            e3.y = y_coeffs.2 as f32;
            e3.pb = pb_chroma;
            e3.pr = pr_chroma;

            let e4 = unpacked_elements.get_mut(x * 2 + 1, y * 2 + 1);
            e4.y = y_coeffs.3 as f32;
            e4.pb = pb_chroma;
            e4.pr = pr_chroma;
        }
    }

    unpacked_elements
}

/// Unpacks a pixels of a Array2 that contains a tuple of a, b, c, d, pb ave, and pr ave into pixels
/// # Returns:
/// * `Array2<YPbPr>` : a Array2 of component video pixels
///
/// # Arguments:
/// * `arr`: a Array2<(u32, i32, i32, i32, usize, usize)>, a 6-tuple containing a, b, c, d, pb ave, and pr ave
pub fn unpack_2x2_pixels_from_float(
    arr: Array2<(f32, f32, f32, f32, usize, usize)>,
) -> Array2<YPbPr> {
    let mut unpacked_elements = Array2::blank_state(
        arr.width * 2,
        arr.height * 2,
        YPbPr {
            y: 0.0,
            pb: 0.0,
            pr: 0.0,
        },
    );

    // Iterate through packed elements
    for x in 0..arr.width {
        for y in 0..arr.height {
            let y_coeffs = reverse_luminosity_coeffs_from_float(
                arr.get(x, y).0,
                arr.get(x, y).1,
                arr.get(x, y).2,
                arr.get(x, y).3,
            );

            let pb_chroma = chroma_of_index(arr.get(x, y).4);
            let pr_chroma = chroma_of_index(arr.get(x, y).5);

            // Unpack the values into the corresponding 2x2 group
            let e1 = unpacked_elements.get_mut(x * 2, y * 2);
            e1.y = y_coeffs.0 as f32;
            e1.pb = pb_chroma;
            e1.pr = pr_chroma;

            let e2 = unpacked_elements.get_mut(x * 2 + 1, y * 2);
            e2.y = y_coeffs.1 as f32;
            e2.pb = pb_chroma;
            e2.pr = pr_chroma;

            let e3 = unpacked_elements.get_mut(x * 2, y * 2 + 1);
            e3.y = y_coeffs.2 as f32;
            e3.pb = pb_chroma;
            e3.pr = pr_chroma;

            let e4 = unpacked_elements.get_mut(x * 2 + 1, y * 2 + 1);
            e4.y = y_coeffs.3 as f32;
            e4.pb = pb_chroma;
            e4.pr = pr_chroma;
        }
    }

    unpacked_elements
}

/// Computes the average chroma index for pb and pr
/// # Returns:
/// * `(usize, uszie), a tuple of pb and pr ave values
///
/// # Arguments:
/// * `group` `: [YPbPr; 4], a slice of 4 component video pixels
pub fn average_pbpr(group: [&YPbPr; 4]) -> (usize, usize) {
    let pb_ave = (group[0].pb() + group[1].pb() + group[2].pb() + group[3].pb()) / 4 as f32;
    let pr_ave = (group[0].pr() + group[1].pr() + group[2].pr() + group[3].pr()) / 4 as f32;

    (index_of_chroma(pb_ave), index_of_chroma(pr_ave))
}

/// Computes a, b, c, and d
/// # Returns:
/// * `(usize, uszie), a tuple of pb and pr ave values
/// # Arguments:
/// * `group` `: [YPbPr; 4], a slice of 4 component video pixels
pub fn get_luminosity_coeffs(group: [&YPbPr; 4]) -> (f32, f32, f32, f32) {
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

/// Computes y1-4 from a, b, c, d
/// # Returns:
/// * `(f32, f32, f32, f32)`, a tuple of y1, y2, y3, y4
/// # Arguments:
/// * `group` `: (a: f32, b: f32, c: f32, d: f32)
pub fn reverse_luminosity_coeffs_from_float(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
) -> (f32, f32, f32, f32) {
    let y1 = a - b - c + d;
    let y2 = a - b + c - d;
    let y3 = a + b - c - d;
    let y4 = a + b + c + d;

    (y1, y2, y3, y4)
}

/// Computes y1-4 from a, b, c, d
/// # Returns:
/// * (u32, i32, i32, i32)`, a tuple of y1, y2, y3, y4
/// # Arguments:
/// * `group` `: (a: u32, b: i32, c: i32, d: i32)
pub fn reverse_luminosity_coeffs(a: u32, b: i32, c: i32, d: i32) -> (u32, i32, i32, i32) {
    let y1 = a as i32 - b - c + d;
    let y2 = a as i32 - b + c - d;
    let y3 = a as i32 + b - c - d;
    let y4 = a as i32 + b + c + d;

    (y1 as u32, y2 as i32, y3 as i32, y4 as i32)
}

/// Takes a, b, c, d, pb, and pr and triggers bitpacking functions with the given width and lsb
/// # Returns:
/// * `Option<u32>`, a u32 bit-packed word
/// # Arguments:
/// * `(a: u32, b: i32, c: i32, d: i32, pb: u32, pr: u32)`, a tuple of values to bitpack
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

/// Takes a, b, c, d, pb, and pr and unpacks the bits for the corresponding value, given a width and lsb
/// # Returns:
/// * `(u32, i32, i32, i32, usize, usize)`, a tuple of values
/// # Arguments:
/// * `packed value: u32`, a u32 word
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
    use std::io::{self, Write};

    use crate::{
        bitpack, pack_2x2_elements, quantize::encodes, quantize::encodeu,
        to_component_video::from_ypbpr, to_component_video::to_ypbpr, to_rgb_float::from_rgb32,
        to_rgb_float::to_rgbf32, trim_to_even_dimensions, unpack_2x2_pixels,
        unpack_2x2_pixels_from_float, unpack_bits,
    };
    use array2::Array2;
    use csc411_image::{Read, Rgb, RgbImage, Write as RgbWrite};
    use csc411_rpegio::{output_rpeg_data, output_rpeg_data_debug};

    #[test]
    fn start_to_ypbpr_and_back() {
        // Construct an Array2
        let img = RgbImage::read(Some("frost.ppm")).unwrap();
        let height = img.height.try_into().unwrap();
        let width = img.width.try_into().unwrap();
        let usize_vec: Vec<csc411_image::Rgb> = img.pixels.clone();
        let arr: Array2<Rgb> = Array2::from_row_major(width, height, usize_vec).unwrap();

        // Trim rows and/or cols to be an even number
        let arr_trimmed = trim_to_even_dimensions(&arr);

        /*for (x, y, &ref element) in arr_trimmed.iter_row_major() {
            let modified_element = element.clone();

            println!("{}, {}, : {:?}", x, y, modified_element);
        }*/

        let float_arr = to_rgbf32(&arr_trimmed);

        /*for (x, y, &ref element) in float_arr.iter_row_major() {
            let modified_element = element.clone();

            println!("{}, {}, : {:?}", x, y, modified_element);
        }*/

        let cv_arr = to_ypbpr(&float_arr);
        let returned_cv_arr = from_ypbpr(&cv_arr);
        let returned_float_arr = from_rgb32(&returned_cv_arr);

        let out_image = RgbImage {
            height: height as u32,
            width: width as u32,
            denominator: img.denominator,
            pixels: returned_float_arr.data,
        };

        out_image.write(Some("new_out_test1.ppm")).unwrap();
    }

    #[test]
    fn start_pack2x2_and_back() {
        // Construct an Array2
        let img = RgbImage::read(Some("frost.ppm")).unwrap();
        let height = img.height.try_into().unwrap();
        let width = img.width.try_into().unwrap();
        let usize_vec: Vec<csc411_image::Rgb> = img.pixels.clone();
        let arr: Array2<Rgb> = Array2::from_row_major(width, height, usize_vec).unwrap();

        // Trim rows and/or cols to be an even number
        let arr_trimmed = trim_to_even_dimensions(&arr);

        /*for (x, y, &ref element) in arr_trimmed.iter_row_major() {
            let modified_element = element.clone();

            println!("{}, {}, : {:?}", x, y, modified_element);
        }*/

        let float_arr = to_rgbf32(&arr_trimmed);

        /*for (x, y, &ref element) in float_arr.iter_row_major() {
            let modified_element = element.clone();

            println!("{}, {}, : {:?}", x, y, modified_element);
        }*/

        let cv_arr = to_ypbpr(&float_arr);
        let packed_arr = pack_2x2_elements(cv_arr);

        /*for (x, y, &ref element) in packed_arr.iter_row_major() {
            let modified_element = element.clone();

            println!("{}, {}, : {:?}", x, y, modified_element);
        }*/

        let unpacked_arr = unpack_2x2_pixels_from_float(packed_arr);

        let returned_cv_arr = from_ypbpr(&unpacked_arr);
        let returned_float_arr = from_rgb32(&returned_cv_arr);

        let out_image = RgbImage {
            height: height as u32,
            width: width as u32,
            denominator: img.denominator,
            pixels: returned_float_arr.data,
        };

        out_image.write(Some("new_out_test2.ppm")).unwrap();
    }

    #[test]
    fn start_encode_and_back() {
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

        // Read in the rpeg data
        let input = Some("output.rpeg");
        let (compressed_data, width, height) = csc411_rpegio::input_rpeg_data(input).unwrap();

        let mut decompressed_vec = vec![];

        for word in compressed_data {
            let vals = unpack_bits(u32::from_be_bytes(word));
            decompressed_vec.push(vals);
        }

        let decompressed_arr = Array2::from_row_major(packed_arr.width, packed_arr.height, decompressed_vec).unwrap();
        let unpacked_arr = unpack_2x2_pixels(decompressed_arr);
        let returned_cv_arr = from_ypbpr(&unpacked_arr);

        /*for (x, y, &ref element) in returned_cv_arr.iter_row_major() {
            let modified_element = element.clone();

            println!("{}, {}, {}", modified_element.red, modified_element.green, modified_element.blue);
        }*/

        let returned_float_arr = from_rgb32(&returned_cv_arr);

        let out_image = RgbImage {
            height: height as u32,
            width: width as u32,
            denominator: img.denominator,
            pixels: returned_float_arr.data,
        };

        //out_image.write(Some("new_out.ppm")).unwrap();
    }
}
