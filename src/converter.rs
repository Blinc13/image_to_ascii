use crate::Image;
use ocl::{
    Device,
    Context,
    Platform,
    Image as ClImage,
};

const CHARS: &str = " -0=%$@#";

pub struct Converter {
    device: Device,
    context: Context
}

impl Converter {
    pub fn init() -> () {
        let platform = Platform::first().unwrap();
        println!("{}", platform);
        let device = Device::first(platform).unwrap();
        println!("{}", device);

        let context = Context::builder()
            .platform(platform)
            .devices(device)
            .build();

        println!("{:?}", context);
    }
}