use rpeg::codec::{compress, decompress};
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    assert!(argnum == 2 || argnum == 3);
    let filename = args.iter().nth(2).unwrap();

    match args[1].as_str() {
        "-c" => compress(Some(filename)),
        "-d" => decompress(Some(filename)),
        _ => {
            eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]")
        }
    }










    /* This struc is for our temp type of f32s
    #[derive(Clone, Debug)]
    pub struct RgbF32Temp {
        red: f32,
        green: f32,
        blue: f32,
    }
    impl fmt::Display for RgbF32Temp {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "RgbF32Temp {{ red: {}, green: {}, blue: {} }}",
                self.red, self.green, self.blue
            )
        }
    }*/

    // read in
    /*
    let input = env::args().nth(1);

    // set rgb image with th filename
    let img = RgbImage::read(input.as_deref()).unwrap();

    // get our values from grayimage to the correct types for our structure
    let height2 = img.height.try_into().unwrap();
    let width2 = img.width.try_into().unwrap();

    // Got to figure out how we're representing RGB
    let usize_vec: Vec<csc411_image::Rgb> = img.pixels.clone();

    //iter().map(|rgb| (rgb.red, rgb.green, rgb.blue)).collect();
    let try_2 = Array2::from_row_major(width2, height2, usize_vec).unwrap();
    /*for (x, y, &ref element) in try_2.iter_row_major(){
        println!("{}, {}, {:?}", x, y, element);
    }*/
    // match case for rotate
    */
    // trim to even with function call

    //let ben = try_2.trim_to_even_dimensions();

    // here is the process for making the values f32s.
    /*let new_data: Vec<RgbF32Temp> = ben
        .iter_row_major()
        .map(|(_, _, element)| {
            let r = element.red as f32 / img.denominator as f32;
            let g = element.green as f32 / img.denominator as f32;
            let b = element.blue as f32 / img.denominator as f32;
            RgbF32Temp {
                red: r,
                green: g,
                blue: b,
            }
        })
        .collect();

    // creating new instnace for f 32s

    let ben2 = Array2::from_row_major(ben.width(), ben.height(), new_data).unwrap();

    // testing to make sure dimensions function is working*/

    println!("{}, {}", ben.width(), ben.height());

    // testing to see if float values are printed (they are)
    for (x, y, element) in ben2.iter_row_major() {
        println!("{}, {}, : {}", x, y, element);
    }
}
