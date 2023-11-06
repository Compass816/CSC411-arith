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

pub fn to_rgbf32(arr: &Array2<Rgb>) -> Array2<RgbF32> {
    let new_data: Vec<RgbF32> = arr
    .iter_row_major()
    .map(|(_, _, element)| {
        let r = element.red as f32 / 255 as f32;
        let g = element.green as f32 / 255 as f32;
        let b = element.blue as f32 / 255 as f32;
        RgbF32 {
            red: r,
            green: g,
            blue: b,
        }
    })
    .collect();

    return Array2::from_row_major(arr.width(), arr.height(), new_data).unwrap();
}
