use crate::Image;

const CHARS: &str = " -0=%$@#";


pub fn default_convert(image: Image, step: u32) -> String {
    convert(image, step, |x| x)
}

pub fn convert_inverted(image: Image, step: u32) -> String {
    convert(image, step, |x| u8::MAX - x)
}


pub fn convert<F>(image: Image, step: u32, pred: F) -> String
    where F: Fn(u8) -> u8
{
    let mut output = String::new();


    let size = image.get_size();

    let width_step = step;
    let height_step = step*2;


    for y in 0..size.height / height_step {
        for x in 0..size.width / width_step {
            let pixel = image.get_pixel(
                (x * width_step).clamp(0, size.width-1),
                (y * height_step).clamp(0, size.height-1)
            );

            let c = (pred(pixel.average()) / 32) as usize;

            output.push_str(&CHARS[c..c+1]);
        }

        output.push('\n');
    }

    output
}