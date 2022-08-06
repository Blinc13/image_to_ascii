use image_to_ascii::Image;

fn main() {
    let image = Image::load("/");

    let size = image.get_size();
    println!("{}:{}", size.width, size.height);
}
