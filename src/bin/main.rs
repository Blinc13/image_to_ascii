use std::{
    io::ErrorKind,
    process::exit
};
use image_to_ascii::{
    Args,
    Image,
    converter::Converter
};
use clap::Parser;

fn main() {
    let conv = Converter::init();
}

fn print_and_exit(message: &str) -> ! {
    eprintln!("{}", message);
    exit(1)
}