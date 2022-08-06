use crate::Image;

const CHARS: &str = " -0=%$@#";

pub struct Converter {
    ascii: String
}

impl Converter {
    pub fn convert(image: Image, pixel_size: u32) -> Self {
        let size = image.get_size();

        let char_width = pixel_size;
        let char_height = pixel_size*2;


        for y in 0..size.height / char_height {
            for x in 0..size.width / char_width {
                let pixel = image.get_pixel(
                    (x * char_width).clamp(0, size.width-1),
                    (y * char_height).clamp(0, size.height-1)
                );

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