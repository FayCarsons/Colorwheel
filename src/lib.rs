use image::*;
use rand::{self, Rng};
use rayon::prelude::*;
use std::boxed::Box;
//use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};

#[derive(Clone, Copy, Debug)]
pub struct Centroid {
    r: f64,
    g: f64,
    b: f64,
    num_pixels: u64
}

pub fn u8_to_f64(x: u8) -> f64 {
    x as f64 / u8::MAX as f64
}

impl Centroid {
    pub fn new(pixel: (f64, f64, f64)) -> Centroid {
        let (r, g, b) = pixel;
        Centroid {
            r,
            g,
            b,
            num_pixels: 1,
        }
    }

    pub fn to_rgb(self) -> (f64, f64, f64) {
        (self.r, self.g, self.b)
    }

    pub fn average(&self) -> Centroid {
        Centroid {
            r: self.r / (self.num_pixels as f64),
            g: self.g / (self.num_pixels as f64),
            b: self.b / (self.num_pixels as f64),
            num_pixels: 1,
        }
    }

    pub fn to_rgb_u8(self) -> (u8, u8, u8) {
        (
            (self.r * u8::MAX as f64) as u8,
            (self.g * u8::MAX as f64) as u8,
            (self.b * u8::MAX as f64) as u8,
        )
    }

    fn to_pixel(self) -> image::Rgb<u8> {
        let (r, g, b) = self.to_rgb_u8();
        image::Rgb([r, g, b])
    }

    #[inline(always)]
    pub fn to_atom(self) -> AtomicPtr<Centroid> {
        let ptr: *mut Centroid = Box::<Centroid>::into_raw(Box::new(self));
        AtomicPtr::new(ptr)
    }
}

pub fn get_raw_image(img: &DynamicImage) -> Vec<Vec<[u8; 4]>> {
    let (x, y) = img.dimensions();
    (0..y)
        .into_par_iter()
        .map(|y| {
            (0..x)
                .into_par_iter()
                .map(|x| img.get_pixel(x, y).0)
                .collect()
        })
        .collect()
}

pub fn init_centroids(data: &Vec<Vec<[u8; 4]>>, means: &u16) -> Vec<Centroid> {
    let mut centroids: Vec<Centroid> = Vec::with_capacity(*means as usize);
    let mut thread_rng = rand::thread_rng();

    for _ in 0..*means {
        let rnd_x = thread_rng.gen_range(0..data[0].len());
        let rnd_y = thread_rng.gen_range(0..data.len());

        let pixel = (
            data[rnd_y][rnd_x][0] as f64 / 255.0,
            data[rnd_y][rnd_x][1] as f64 / 255.0,
            data[rnd_y][rnd_x][2] as f64 / 255.0,
        );

        centroids.push(Centroid::new(pixel));
    }
    centroids
}

pub fn iterate(
    img: &[Vec<[u8; 4]>],
    centroids: &Vec<Centroid>,
    x: usize,
    y: usize,
) -> Vec<Centroid> {
    let new_centroids: Vec<AtomicPtr<Centroid>> = centroids
        .iter()
        .map(|c| c.to_atom())
        .collect::<Vec<AtomicPtr<Centroid>>>();

    (0..y).into_par_iter().for_each(|y| {
        (0..x).for_each(|x| {
            let new_pixel = (
                img[y][x][0] as f64 / 255.0,
                img[y][x][1] as f64 / 255.0,
                img[y][x][2] as f64 / 255.0,
            );

            let mut min_dist = u64::MAX as f64;
            let mut centroid_id = centroids.len() + 1;
            for (c, centroid) in centroids.iter().enumerate() {
                let dist = minkowski_distance(&centroid.to_rgb(), &new_pixel, 2.0);
                if dist < min_dist {
                    min_dist = dist;
                    centroid_id = c;
                }
            }
            let atom = new_centroids[centroid_id].load(Ordering::Relaxed);
            unsafe {
                new_centroids[centroid_id].store(
                    Box::into_raw(Box::new(Centroid {
                        r: (*atom).r + new_pixel.0,
                        g: (*atom).g + new_pixel.1,
                        b: (*atom).b + new_pixel.2,
                        num_pixels: (*atom).num_pixels + 1,
                    })),
                    Ordering::Relaxed,
                );
            }
        })
    });
    
    let new_centroids = new_centroids
        .iter()
        .map(|c| unsafe { *c.load(Ordering::Relaxed) }.average())
        .collect();
    new_centroids
}

fn nearest_color(
    x: usize,
    y: usize,
    data: &[Vec<[u8; 4]>],
    centroids: &[Centroid],
    p: f64,
) -> Rgb<u8> {
    let [r, g, b, _] = data[y][x];
    let pixel = (u8_to_f64(r), u8_to_f64(g), u8_to_f64(b));
    let mut min_dist = u64::MAX as f64;
    let mut centroid_id = 0;

    for (c, val) in centroids.iter().enumerate() {
        let dist = minkowski_distance(&val.to_rgb(), &pixel, p);
        if dist < min_dist {
            min_dist = dist;
            centroid_id = c;
        }
    }

    let (out_r, out_g, out_b) = centroids[centroid_id].to_rgb_u8();
    image::Rgb([out_r, out_g, out_b])
}

pub fn create_img(
    size: (u32, u32),
    data: Vec<Vec<[u8; 4]>>,
    centroids: Vec<Centroid>,
    p: f64,
    mode: &str
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (x_size, y_size) = size;
    if mode == "n" {
        ImageBuffer::from_fn(x_size, y_size, |x,y| 
        nearest_color(x as usize, y as usize, &data, &centroids,p)) 
    } else if mode == "y" {
        ImageBuffer::from_fn(centroids.len() as u32, 1, |x,_|
        centroids[x as usize].to_pixel())
    } else {
        panic!("Invalid mode")
    }
} 

// 3D minkowski distance function
// p=1 manhattan, p=2 euclidean, p=25+ chebyshev
pub fn minkowski_distance(to: &(f64, f64, f64), from: &(f64, f64, f64), p: f64) -> f64 {
    let dx = (to.0 - from.0).powf(p);
    let dy = (to.1 - from.1).powf(p);
    let dz = (to.2 - from.2).powf(p);

    (dx.abs() + dy.abs() + dz.abs()).powf(1.0 / p)
}
