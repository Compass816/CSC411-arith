pub mod codec;
pub mod to_rgb_float;
pub mod to_component_video;
pub mod quantize;
pub mod bitpack;

use array2::Array2;
use csc411_image::Rgb;
use to_component_video::YPbPr;
use csc411_arith::{index_of_chroma, chroma_of_index};
use bitpack::{fitss, fitsu, news, newu, gets, getu};

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

/// TODO
/// Returns: an Array2 of quantized values, a 6-tuple containing a, b, c, d, pb ave, and pr ave
pub fn pack_2x2_pixels(arr: &Array2<YPbPr>) -> Array2<(f32, f32, f32, f32, usize, usize)>{

    let mut packed_arr = Array2::blank_state(arr.width()/2, arr.height()/2, (0.0, 0.0, 0.0, 0.0, 0, 0));

    // Loop invariant: arr will always have an even number of rows and cols
    for (x, y, value) in iter_row_major_2x2(arr) {
        let group: [[&YPbPr; 2]; 2] = [[arr.get(x, y), arr.get(x+1, y)], [arr.get(x, y+1), arr.get(x+1, y+1)]];

        let chroma_vals = average_pbpr(group);
        let luminosity_coeffs = get_luminosity_coeffs(group);

        packed_arr.get_mut(x/2, y/2).0 = luminosity_coeffs.0;
        packed_arr.get_mut(x/2, y/2).1 = luminosity_coeffs.1;
        packed_arr.get_mut(x/2, y/2).2 = luminosity_coeffs.2;
        packed_arr.get_mut(x/2, y/2).3 = luminosity_coeffs.3;
        packed_arr.get_mut(x/2, y/2).4 = chroma_vals.0;
        packed_arr.get_mut(x/2, y/2).5 = chroma_vals.1;
    }

    return packed_arr;
}


pub fn iter_row_major_2x2<'a, T: Clone>(arr: &'a Array2<T>) -> impl Iterator<Item = (usize, usize, T)> + 'a {
    (0..arr.height())
        .step_by(2)
        .flat_map(move |y| {
            (0..arr.width())
                .step_by(2)
                .map(move |x| (x, y, arr.get(x, y).clone()))
        })
}


pub fn average_pbpr(group: [[&YPbPr; 2]; 2]) -> (usize, usize) {
    let pb_ave = (group[0][0].pb() + group[0][1].pb() + group[1][0].pb() + group[1][1].pb()) / 4 as f32;
    let pr_ave = (group[0][0].pr() + group[0][1].pr() + group[1][0].pr() + group[1][1].pr()) / 4 as f32;
    
    (index_of_chroma(pb_ave), index_of_chroma(pr_ave))
}


pub fn get_luminosity_coeffs(group: [[&YPbPr; 2]; 2]) -> (f32, f32, f32, f32) {
    let y1 = group[0][0].y();
    let y2 = group[0][1].y();
    let y3 = group[1][0].y();
    let y4 = group[1][1].y();

    let a = (y4 + y3 + y2 + y1) / 4.0;
    let b = (y4 + y3 - y2 - y1) / 4.0;
    let c = (y4 - y3 + y2 - y1) / 4.0;
    let d = (y4 - y3 - y2 + y1) / 4.0;

    (a, b, c, d)
}

pub fn reverse_luminosity_coeffs(a: u32, b: i32, c: i32, d: i32) -> (u32, i32, i32, i32) {
    let y1 = a as f32 - b as f32 - c as f32 + d as f32;
    let y2 = a as f32 - b as f32 + c as f32 - d as f32;
    let y3 = a as f32 + b as f32 - c as f32 - d as f32;
    let y4 = a as f32 + b as f32 + c as f32 + d as f32;

    (y1 as u32, y2 as i32, y3 as i32, y4 as i32)
}


pub fn unpack_bits(packed_value: u32) -> (u32, i32, i32, i32, u32, u32) {
    let a = getu(packed_value as u64, 9, 23) as u32;
    let b = gets(packed_value as u64, 5, 18) as i32;
    let c = gets(packed_value as u64, 5, 13) as i32;
    let d = gets(packed_value as u64, 5, 8) as i32;
    let pb = getu(packed_value as u64, 4, 4) as u32;
    let pr = getu(packed_value as u64, 4, 0) as u32;

    (a, b, c, d, pb, pr)
}


fn compute_cv_byte(bytes: [u8; 4], pos_x: usize, pos_y: usize) -> (f32, f32, f32) {
    // Read in bytes in big=endian order
    let vals = unpack_bits(u32::from_be_bytes(bytes));
  
    let a = vals.0;
    let b = vals.1;
    let c = vals.2;
    let d = vals.3;
  
    let pb_chroma = chroma_of_index(vals.4 as usize) as f32;
    let pr_chroma = chroma_of_index(vals.5 as usize) as f32;
  
    let y = reverse_luminosity_coeffs(a, b, c, d);
    
    let pixel_x = pos_x % 2;
    let pixel_y = pos_y % 2;
    let pixel_position = pixel_y * 2 + pixel_x;

    let result = match pixel_position {
        1 => (y.0 as f32, pb_chroma, pr_chroma),
        2 => (y.1 as f32, pb_chroma, pr_chroma),
        3 => (y.2 as f32, pb_chroma, pr_chroma),
        4 => (y.3 as f32, pb_chroma, pr_chroma),
        _ => (0.0, 0.0, 0.0),
    };

    result
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
