use clap::Parser;

#[derive(Parser)]
#[clap(about, version)]
pub struct Args {
    /// Path to image
    pub path: String,

    /// Pixel step(the larger the lower the resolution)
    #[clap(value_parser, default_value="10")]
    pub step: u32,

    /// Invert colors(if set, colors inverted)
    #[clap(short, long, value_parser, default_value="false")]
    pub inverted: bool
}