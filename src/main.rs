#![feature(slice_take)]
mod utils;
use utils::{utils::{create_img, init_centroids, iterate, Bucket}, prompts::Options};
use image::*;
use std::time::Instant;


fn main() {
    // get command line args
    let options = Options::new();

    // get start time
    let start = Instant::now();

    // load image, get dimensions and raw RGBA data as 2d vec of 4 element arrays
    let img: DynamicImage = image::open(options.input_path).unwrap();
    let rgb8 = img.to_rgb8();
    let raw_data: Vec<&[u8]> = rgb8.as_bytes().chunks_exact(3).collect::<Vec<&[u8]>>();


    // place centroids, random pixels in image
    let init_centroids = init_centroids(&raw_data, &options.k);

    // do clustering/averaging
    let buckets = (0..options.k).fold(init_centroids, |acc: Vec<Bucket>, _| {
        iterate(&raw_data, acc, options.k as usize)
    });

    // create final image
    let final_img = create_img(img.dimensions(), raw_data, buckets, &options.mode);
    // save image
    final_img.save(options.output_path).unwrap();

    // get duration
    let duration = start.elapsed();
    println!("Total time: {duration:?}");
}
