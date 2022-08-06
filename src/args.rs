use clap::Parser;

#[derive(Parser)]
#[clap(about, version)]
pub struct Args {
    /// Path to image
    #[clap(short, long, value_parser)]
    pub path: String,

    /// Pixel step(the larger the lower the resolution)
    #[clap(short, long, value_parser, default_value="10")]
    pub step: u32
}