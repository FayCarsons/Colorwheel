use std::ops::AddAssign;

/// Centroid/Bucket, holding one of K values for pixels to be placed into
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Bucket {
    r: f32,
    g: f32,
    b: f32,
    num_pixels: u64,
}

impl Bucket {
    pub fn new(pixel: &[f32]) -> Bucket {
        // Pixels should never have < 3 values so we can panic here
        let [r, g, b] = pixel[0..3] else {
            panic!("Invalid pixel format!")
        };
        Bucket {
            r,
            g,
            b,
            num_pixels: 1,
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
}

impl AddAssign<Bucket> for Bucket {
    fn add_assign(&mut self, rhs: Bucket) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.num_pixels += rhs.num_pixels;
    }
}

impl AddAssign<&[f32]> for Bucket {
    fn add_assign(&mut self, rhs: &[f32]) {
        let [r, g, b] = rhs else {
            // A pixel should *always* have 3 values so we can panic here
            panic!("Received invalid pixel!")
        };

        self.r += r;
        self.g += g;
        self.b += b;
        self.num_pixels += 1;
    }
}

impl From<&Bucket> for [f32; 3] {
    fn from(value: &Bucket) -> Self {
        [value.r, value.g, value.b]
    }
}

impl From<Bucket> for image::Rgb<u8> {
    fn from(value: Bucket) -> Self {
        Self([
            (value.r * u8::MAX as f32) as u8,
            (value.g * u8::MAX as f32) as u8,
            (value.b * u8::MAX as f32) as u8,
        ])
    }
}
