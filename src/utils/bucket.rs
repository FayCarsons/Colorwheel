use std::ops::Add;

/// Centroid/Bucket, holding one of K values for pixels to be placed into
#[derive(Clone, Copy, Debug)]
pub struct Bucket {
    r: f32,
    g: f32,
    b: f32,
    num_pixels: u64,
}

impl Bucket {
    pub fn empty() -> Self {
        let [r, g, b] = [0.; 3];
        Bucket {
            r,
            g,
            b,
            num_pixels: 1,
        }
    }

    pub fn new(pixel: &[f32]) -> Bucket {
        // Pixels should never have < 3 values so we can panic here
        let [r, g, b] = pixel[0..3] else { panic!() };
        Bucket {
            r,
            g,
            b,
            num_pixels: 1,
        }
    }

    fn to_rgb_u8(self) -> [u8; 3] {
        [
            (self.r * u8::MAX as f32) as u8,
            (self.g * u8::MAX as f32) as u8,
            (self.b * u8::MAX as f32) as u8,
        ]
    }

    pub fn to_rgb_f32(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    pub fn add_pixel(&self, pixel: &[f32]) -> Self {
        // Pixels should never have < 3 values so we can panic here
        let [r, g, b] = pixel[0..3] else { panic!("Invalid pixel format!") };
        Bucket {
            r: self.r + r,
            g: self.g + g,
            b: self.b + b,
            num_pixels: self.num_pixels + 1,
        }
    }

    pub fn average(&self) -> Bucket {
        Bucket {
            r: self.r / (self.num_pixels as f32),
            g: self.g / (self.num_pixels as f32),
            b: self.b / (self.num_pixels as f32),
            num_pixels: 1,
        }
    }

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
            num_pixels: self.num_pixels + other.num_pixels,
        }
    }
}
