use crate::Image;

const CHARS: &str = " -0=%$@#";

pub struct Converter {
    ascii: String
}

impl Converter {
    pub fn convert(image: Image) -> Self {
        let size = image.get_size();

        for x in 0..size.width/30 {
            for y in 0..size.height/10 {
                let pixel = image.get_pixel(x*30, y*10);

                let c = pixel.average() / 32;

                print!("{}", CHARS.chars().skip(c as usize).next().unwrap());
            }

            print!("\n");
        }

        Self {
            ascii: "".to_string()
        }
    }
}