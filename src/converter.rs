use crate::Image;
use ocl::{
    Device,
    Context,
    Program,
    Platform,
    Image as ClImage,
};

const CHARS: &str = " -0=%$@#";

pub struct Converter {
    device: Device,
    context: Context,
    platform: Platform
}

impl Converter {
    pub fn init() -> Self {
        let platform = Platform::first().unwrap();
        println!("{}", platform);
        let device = Device::first(platform).unwrap();
        println!("{}", device);

        let context = Context::builder()
            .platform(platform)
            .devices(device)
            .build().unwrap();

        let program = Program::builder()
            .devices(device)
            .source_file("src/opencl/kernel.cl")
            .build(&context).unwrap();

        println!("{:?}", context);
        println!("{:?}", program.to_string());

        Self {
            device,
            context,
            platform
        }
    }
}