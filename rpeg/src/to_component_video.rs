use csc411_image::Rgb;
use array2::Array2;
use std::fmt;

// This struc is for our temp type of f32s
#[derive(Clone, Debug)]
pub struct RgbF32 {
    red: f32,
    green: f32,
    blue: f32,
}

impl fmt::Display for RgbF32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RgbF32 {{ red: {}, green: {}, blue: {} }}",
            self.red, self.green, self.blue
        )
    }
}

pub fn to_comp_vid(arr: &Array2<RgbF32>) -> Array2<RgbF32> {
    let new_data: Vec<RgbF32Temp> = ben2.iter_row_major()
    .map(|(_, _, element)| {
        let r = element.red as f32 / img.denominator as f32;
        let g = element.green as f32 / img.denominator as f32;
        let b = element.blue as f32 / img.denominator as f32;

        let y = 0.299 * r + 0.587 * g + 0.114 * b;
        let pb = -0.168736 * r - 0.331264 * g + 0.5 * b;
        let pr = 0.5 * r - 0.418688 * g - 0.081312 * b;

        RgbF32Temp { red: y, green: pb, blue: pr }
    })
    .collect();

}


    return Array2::from_row_major(arr.width(), arr.height(), new_data).unwrap();
