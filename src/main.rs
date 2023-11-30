#![feature(slice_take)]
mod utils;
use image::*;
use utils::{
    lib::{init_buckets, iterate, render, Bucket},
    prompts::Options,
};

fn main() {
    // get command line args
    let options = Options::new();

    // load image and convert to flat array of pixels
    let img = image::open(options.input_path).unwrap();
    let rgbf = img.to_rgb32f();
    let raw_data: Vec<&[f32]> = rgbf.as_raw().chunks_exact(3).collect();

    // create buckets from k random pixels in image
    let buckets = init_buckets(&raw_data, &options.k);

    // do clustering/averaging
    let buckets = (0..options.k).fold(buckets, |acc: Vec<Bucket>, _| {
        iterate(&raw_data, acc, options.k)
    });

    // create final image
    let final_img = render(img.dimensions(), &raw_data, buckets, options.mode);

    // save image
    final_img.save(options.output_path).unwrap();
}
