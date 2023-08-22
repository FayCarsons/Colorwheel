pub use super::bucket::Bucket;
use super::prompts::Mode;
use image::*;
use rand::{self, seq::SliceRandom, thread_rng};
use rayon::prelude::*;

pub fn init_centroids(data: &Vec<&[f32]>, means: &usize) -> Vec<Bucket> {
    let mut rng = thread_rng();
    (0..*means)
        .map(|_| loop {
            if let Some(pixel) = data.choose(&mut rng) {
                return Bucket::new(*pixel);
            } else {
                continue;
            }
        })
        .collect()
}

// Parallel fold + reduce is the secret sauce here
// buckets are accumulated, adding pixels who's value is "near", then averaged
pub fn iterate(img: &Vec<&[f32]>, buckets: Vec<Bucket>, k: usize) -> Vec<Bucket> {
    img.par_iter()
        .fold(
            || vec![Bucket::empty(); k],
            |mut new_buckets, pixel| {
                let bucket = nearest_centroid(*pixel, &buckets);
                new_buckets[bucket] = new_buckets[bucket].add_pixel(*pixel);
                new_buckets
            },
        )
        .reduce(
            || vec![Bucket::empty(); k],
            |mut res, curr| {
                for i in 0..k {
                    res[i] = res[i] + curr[i]
                }
                res
            },
        )
        .iter()
        .map(|c| c.average())
        .collect()
}

fn nearest_centroid(pixel: &[f32], buckets: &Vec<Bucket>) -> usize {
    let mut min_dist = u64::MAX as f32;
    let mut centroid_id = 0;

    for (c, val) in buckets.iter().enumerate() {
        let dist = distance(&val.to_rgb_f32(), pixel);
        if dist < min_dist {
            min_dist = dist;
            centroid_id = c;
        }
    }

    centroid_id
}

fn nearest_color(idx: u32, data: &Vec<&[f32]>, buckets: &Vec<Bucket>) -> Rgb<u8> {
    let pixel = data[idx as usize];
    let bucket = nearest_centroid(pixel, buckets);

    buckets[bucket].to_pixel()
}

pub fn render(
    size: (u32, u32),
    data: Vec<&[f32]>,
    buckets: Vec<Bucket>,
    mode: Mode,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = size;
    match mode {
        Mode::Image => ImageBuffer::from_fn(width, height, |x, y| {
            nearest_color(y * width + x, &data, &buckets)
        }),
        Mode::Palette => {
            ImageBuffer::from_fn(buckets.len() as u32, 1, |x, _| {
                buckets[x as usize].to_pixel()
            })
        }
    }
}

/// Euclidean distance for rgb32f
pub fn distance(to: &[f32], from: &[f32]) -> f32 {
    from.iter()
        .zip(to.iter())
        .map(|(t, f)| {
            let d = t - f;
            d * d
        })
        .sum()
}
