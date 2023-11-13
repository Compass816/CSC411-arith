use array2::Array2;
use std::fmt;

use csc411_image::Rgb;

use crate::to_rgb_float::RgbF32;

// This struc is for our temp type of f32s
#[derive(Clone, Debug)]
pub struct YPbPr {
    y: f32,
    pb: f32,
    pr: f32,
}

impl fmt::Display for YPbPr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "YPbPr {{ y: {}, pb: {}, pr: {} }}",
            self.y, self.pb, self.pr
        )
    }

}

impl YPbPr {
    pub fn new(y: f32, pb: f32, pr: f32) -> Self {
        YPbPr { y, pb, pr }
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn pb(&self) -> f32 {
        self.pb
    }

    pub fn pr(&self) -> f32 {
        self.pr
    }
}

pub fn to_ypbpr(arr: &Array2<RgbF32>) -> Array2<YPbPr> {
    let new_data: Vec<YPbPr> = arr
    .iter_row_major()
    .map(|(_, _, element)| {
        let r = element.red() as f32;
        let g = element.green() as f32;
        let b = element.blue() as f32;

        let y = 0.299 * r + 0.587 * g + 0.114 * b;
        let pb = -0.168736 * r - 0.331264 * g + 0.5 * b;
        let pr = 0.5 * r - 0.418688 * g - 0.081312 * b;

        YPbPr { y, pb, pr }
    })
    .collect();

    return Array2::from_row_major(arr.width(), arr.height(), new_data).unwrap();
}


pub fn to_rgb(arr: &Array2<YPbPr>) -> Array2<Rgb> {
    let new_data: Vec<Rgb> = arr
    .iter_row_major()
    .map(|(_, _, element)| {
        let y = element.y() as f32;
        let pr = element.pb() as f32;
        let pr = element.pr() as f32;

        let red = (1.0 * y + 0.0 * pb + 1.402 * pr) * 255.0;
        let green = (1.0 * y - 0.344136 * pb - 0.714136 * pr) * 255.0;
        let blue = (1.0 * y + 1.772 * pb + 0.0 * pr) * 255.0;
        
        Rgb {
            red,
            green,
            blue
        }

    })
    .collect();

    return Array2::from_row_major(arr.width(), arr.height(), new_data).unwrap();
}




