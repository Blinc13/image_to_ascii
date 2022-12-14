use std::io::{
    Error,
    Result,
    ErrorKind::InvalidData
};
use image::{
    DynamicImage,
    GenericImageView,
    io::Reader,
    Pixel as px,
    Rgb
};

pub struct Image {
    data: DynamicImage
}

impl Image {
    pub fn from_raw(data: &[u8]) -> Result<Self> {
        let data = match image::load_from_memory(data) {
            Ok(i) => i,
            Err(_) => return Err(Error::new(InvalidData,
                                            "Failed to handle image"
            ))
        };

        Ok(
            Self {
                data
            }
        )
    }

    pub fn load(path: &str) -> Result<Self> {
        let reader = Reader::open(path)?;

        Ok (
            Self {
                data: match reader.decode() {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(Error::new(
                            InvalidData,
                            "Wrong file format"
                        ))
                    }
                }
            }
        )
    }

    pub fn get_raw(&self) -> &[u8] {
        let r = self.data.as_flat_samples_u8()
                .unwrap().as_slice() as *const [u8];

        unsafe {
            &*r
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let pixel = self.data.get_pixel(x, y);

        Pixel {
            color: pixel.to_rgb()
        }
    }

    pub fn get_size(&self) -> Size {
        Size {
            height: self.data.height(),
            width: self.data.width()
        }
    }
}

pub struct Pixel {
    color: Rgb<u8>
}

impl Pixel {
    pub fn red(&self) -> u8 {
        self.color.0[0]
    }

    pub fn green(&self) -> u8 {
        self.color.0[1]
    }

    pub fn blue(&self) -> u8 {
        self.color.0[2]
    }

    /// Returns the average color value
    pub fn average(&self) -> u8 {
        let sum = self.red() as u16
            + self.green() as u16
            + self.blue() as u16;

        (sum / 3) as u8
    }
}

pub struct Size {
    pub height: u32,
    pub width: u32
}