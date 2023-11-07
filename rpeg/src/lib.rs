pub mod codec;
pub mod to_rgb_float;
pub mod to_component_video;

use array2::Array2;
use csc411_image::Rgb;
use to_component_video::YPbPr;
use csc411_arith::index_of_chroma;

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


pub fn pack_2x2_pixels(arr: &Array2<YPbPr>) {

    // Loop invariant: arr will always have an even number of rows and cols
    for y in 0..arr.height() {
        for x in 0..arr.width() {
            // Extract a 2x2 group of elements
            let group = [
                [arr.get(x, y), arr.get(x+1, y)], [arr.get(x, y+1), arr.get(x+1, y+1)]
            ];

            let chroma_vals = average_pbpr(group);
            let luminosity_coeffs = get_luminosity_coeffs(group);

        }
    }
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
