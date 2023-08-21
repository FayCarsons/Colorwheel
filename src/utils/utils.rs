pub use super::bucket::Bucket;
use image::*;
use rand::{self, seq::SliceRandom, thread_rng};
use rayon::prelude::*;

pub fn init_centroids(data: &Vec<&[u8]>, means: &u16) -> Vec<Bucket> {
    let mut rng = thread_rng();
    (0..*means)
        .map(|_| loop {
            if let Some(pixel) = data.choose(&mut rng) {
                return Bucket::new(u8_to_f64(pixel));
            } else {
                continue;
            }
        })
        .collect()
}

pub fn iterate(img: &Vec<&[u8]>, buckets: Vec<Bucket>, k: usize) -> Vec<Bucket> {
    let res = img.par_iter().fold(|| vec![Bucket::empty_new(); k as usize], |mut new_buckets, pixel| {
        let fpixel = u8_to_f64(pixel);
        let bucket = nearest_centroid(&fpixel, &buckets);
        new_buckets[bucket] = new_buckets[bucket].add_pixel(fpixel);
        new_buckets
    }).reduce(|| vec![Bucket::empty_new(); k as usize], |mut res, current| {
        for i in 0..k {
            res[i] = res[i] + current[i]
        };
        res
    });
    
    res.iter().map(|c| c.average()).collect()
}

fn nearest_centroid(pixel: &[f64; 3], buckets: &Vec<Bucket>) -> usize {
    let mut min_dist = u64::MAX as f64;
    let mut centroid_id = 0;

    for (c, val) in buckets.iter().enumerate() {
        let dist = distance(&val.to_rgb(), &pixel);
        if dist < min_dist {
            min_dist = dist;
            centroid_id = c;
        }
    }

    centroid_id
}

fn nearest_color(idx: u32, data: &Vec<&[u8]>, buckets: &Vec<Bucket>) -> Rgb<u8> {
    let pixel = data[idx as usize];
    let fpixel = u8_to_f64(&pixel);

    let bucket = nearest_centroid(&fpixel, buckets);

    let pixel = buckets[bucket].to_rgb_u8();
    image::Rgb(pixel)
}

pub fn create_img(
    size: (u32, u32),
    data: Vec<&[u8]>,
    buckets: Vec<Bucket>,
    mode: &str,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = size;
    if mode == "n" {
        ImageBuffer::from_fn(width, height, |x, y| {
            nearest_color(y * width + x, &data, &buckets)
        })
    } else if mode == "y" {
        ImageBuffer::from_fn(buckets.len() as u32, 1, |x, _| {
            buckets[x as usize].to_pixel()
        })
    } else {
        panic!("Invalid mode!!")
    }
}

pub fn u8_to_f64(pixel: &[u8]) -> [f64; 3] {
    let [r, g, b] = [pixel[0], pixel[1], pixel[2]];
    [
        r as f64 / u8::MAX as f64,
        g as f64 / u8::MAX as f64,
        b as f64 / u8::MAX as f64,
    ]
}

// 3D minkowski distance function
// p=1 manhattan, p=2 euclidean, p=25+ chebyshev
pub fn distance(to: &[f64; 3], from: &[f64; 3]) -> f64 {
    let [to_r, to_g, to_b] = to;
    let [from_r, from_g, from_b] = from;

    let dx = (to_r - from_r) * (to_r - from_r);
    let dy = (to_g - from_g) * (to_g - from_g);
    let dz = (to_b - from_b) * (to_b - from_b);

    dx + dy + dz
}