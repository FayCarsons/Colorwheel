pub use super::{bucket::Bucket, prompts::Mode};
use image::*;
use rand::{seq::SliceRandom, thread_rng};
use rayon::prelude::*;

pub fn init_buckets(data: &[&[f32]], means: &usize) -> Vec<Bucket> {
    let mut rng = thread_rng();
    (0..*means)
        .map(|_| {
            // Program relies on image not being empty so we can panic here
            let pixel = data.choose(&mut rng).expect("Image contains no pixels");
            Bucket::new(pixel)
        })
        .collect()
}

// Parallel fold + reduce is the secret sauce here
// buckets are accumulated, adding pixels who's value is "near", then averaged
pub fn iterate(img: &[&[f32]], buckets: Vec<Bucket>, k: usize) -> Vec<Bucket> {
    img.par_iter()
        // Fold creates multiple copies of buckets with image pixels added to them
        .fold(
            || buckets.clone(),
            |mut new_buckets, pixel| {
                let bucket = nearest_centroid(pixel, &buckets);
                new_buckets[bucket] += *pixel;
                new_buckets
            },
        )
        // and reduce adds them all together
        .reduce(
            || buckets.clone(),
            |mut res, curr| {
                for i in 0..k {
                    res[i] += curr[i]
                }
                res
            },
        )
        .iter()
        // Buckets are then averaged
        .map(|c| c.average())
        .collect()
}

fn nearest_centroid(pixel: &[f32], buckets: &[Bucket]) -> usize {
    let mut min_dist = u64::MAX as f32;
    let mut centroid_id = 0;

    for (c, val) in buckets.iter().enumerate() {
        let dist = distance(&<[f32; 3]>::from(val), pixel);
        if dist < min_dist {
            min_dist = dist;
            centroid_id = c;
        }
    }

    centroid_id
}

fn nearest_color(idx: u32, data: &[&[f32]], buckets: &[Bucket]) -> Rgb<u8> {
    let pixel = data[idx as usize];
    let bucket = nearest_centroid(pixel, buckets);

    Rgb::from(buckets[bucket])
}

pub fn render(
    size: (u32, u32),
    data: &[&[f32]],
    buckets: Vec<Bucket>,
    mode: Mode,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = size;
    match mode {
        Mode::Image => ImageBuffer::from_fn(width, height, |x, y| {
            nearest_color(y * width + x, data, &buckets)
        }),
        Mode::Palette => ImageBuffer::from_fn(buckets.len() as u32 * 100, 100, |x, _| {
            Rgb::from(buckets[x as usize / 100])
        }),
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
