use std::{
    io::ErrorKind,
    process::exit
};
use image_to_ascii::{
    Args,
    Image,
    converter::convert
};
use clap::Parser;

fn main() {
    let args = Args::parse();

    let image = match Image::load(&args.path) {
        Ok(i) => i,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => print_and_exit("File not found"),
                ErrorKind::PermissionDenied => print_and_exit("You cannot read this file"),
                ErrorKind::InvalidData => print_and_exit(&e.to_string()),
                _ => panic!("Invalid error")
            }
        }
    };

    println!("{}", convert(image, args.step));
}

fn print_and_exit(message: &str) -> ! {
    println!("{}", message);
    exit(1)
}