use image_to_ascii::Image;
use image_to_ascii::converter::Converter;

fn main() {
    let image = Image::load("/");

    let size = image.get_size();
    println!("{}:{}", size.width, size.height);

    Converter::convert(image, 3);
}
