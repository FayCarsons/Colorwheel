use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Bucket {
    r: f64,
    g: f64,
    b: f64,
    num_pixels: u64,
}

impl Bucket {
    pub fn new(pixel: [f64; 3]) -> Bucket {
        let [r, g, b] = pixel;
        Bucket {
            r,
            g,
            b,
            num_pixels: 1,
        }
    }

    pub fn empty_new() -> Self {
        let [r,g,b] = [0.;3];
        Bucket {
            r,g,b, num_pixels: 1
        }
    }

    pub fn to_rgb(self) -> [f64; 3] {
        [self.r, self.g, self.b]
    }

    pub fn average(&self) -> Bucket {
        Bucket {
            r: self.r / (self.num_pixels as f64),
            g: self.g / (self.num_pixels as f64),
            b: self.b / (self.num_pixels as f64),
            num_pixels: 1,
        }
    }

    pub fn to_rgb_u8(self) -> [u8; 3] {
        [
            (self.r * u8::MAX as f64) as u8,
            (self.g * u8::MAX as f64) as u8,
            (self.b * u8::MAX as f64) as u8,
        ]
    }

    pub fn add_pixel(&self, pixel: [f64; 3]) -> Self {
        let [r, g, b] = pixel;
        Bucket {
            r: self.r + r,
            g: self.g + g,
            b: self.b + b,
            num_pixels: self.num_pixels + 1,
        }
    }

    #[inline(always)]
    pub fn to_pixel(self) -> image::Rgb<u8> {
        image::Rgb(self.to_rgb_u8())
    }
}

impl Add<Bucket> for Bucket {
    type Output = Self;

    fn add(self, other: Bucket) -> Self {
        Bucket {
            r: self.r + other.r, 
            g: self.g + other.g, 
            b: self.b + other.b,
            num_pixels: self.num_pixels + other.num_pixels
        }
    }
}

unsafe impl Send for Bucket {}
unsafe impl Sync for Bucket {}