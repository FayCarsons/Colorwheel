#![feature(slice_take)]
mod utils;
use utils::{utils::{render, init_centroids, iterate, Bucket}, prompts::Options};
use image::*;

fn main() {
    // get command line args
    let options = Options::new();

    // load image and convert to flat array of pixels
    let img = image::open(options.input_path).unwrap();
    let rgbf = img.to_rgb32f();
    let raw_data = rgbf.as_raw().chunks_exact(3).collect();

    // create buckets from k random pixels in image
    let init_centroids = init_centroids(&raw_data, &options.k);

    // do clustering/averaging
    let buckets = (0..options.k).fold(init_centroids, |acc: Vec<Bucket>, _| {
        iterate(&raw_data, acc, options.k)
    });

    // create final image
    let final_img = render(img.dimensions(), raw_data, buckets, options.mode);
    
    // save image
    final_img.save(options.output_path).unwrap();
}
