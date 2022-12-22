extern crate tuple_map;
extern crate image;

mod lib;
use lib::{init_centroids, iterate, create_img, get_raw_image};
use image::*;
use std::str::{FromStr};
use std::env;
use std::time::Instant;

// Image compressor that yields a cell-shading effect
// using the kmeans algorithm

// Command line args as follows - path to input image, path of output image, 
// k value, minkowski distance p value, # of iterations


fn main() {
   // get params from user input
   let mut params = Vec::new();
   for arg in env::args().skip(1) {
      params.push(arg);
   }

   let in_path = &params[0];
   let out_path = &params[1];

   let means = i32::from_str(&params[2]).unwrap();
   let distance = f64::from_str(&params[4]).unwrap();
   let iterations = i32::from_str(&params[3]).unwrap();

   // get start time
   let start = Instant::now();

   // load image, get dimensions and raw data as 1d vec of 3 element tuples
   let img: DynamicImage = image::open(in_path).unwrap();
   let dim: (u32, u32) = img.dimensions();
   let raw_data = get_raw_image(&img);
   

   // place centroids, random points on image
   let mut centroids: Vec<(f64, f64, f64)> = init_centroids(&raw_data, means);

   // iterate centroids/do clustering
   for _ in 0..=iterations {
      centroids = iterate(&raw_data, &centroids, &distance);
   }
   
   // create final image
   let mut final_img: RgbaImage = RgbaImage::new(dim.0, dim.1);
   final_img = create_img(final_img, &img, &centroids, &distance);
   
   // save image
   final_img.save(out_path).unwrap();

   // get duration 
   let duration = start.elapsed();
   println!("Executed In {:?} Seconds", duration);
} 
