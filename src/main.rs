mod lib;
use lib::{init_centroids, iterate, create_img, get_raw_image};
use image::*;
use std::str::{FromStr};
use std::env;
use std::time::Instant;

// Command line args as follows - path to input image, path of output image, 
// k value, minkowski distance p value, # of iterations

fn main() {
   // get params from command line args
   let in_path = env::args().nth(1).unwrap();
   let out_path = env::args().nth(2).unwrap();

   let means = u16::from_str(&env::args().nth(3).unwrap()).unwrap();
   let distance = f64::from_str(&env::args().nth(4).unwrap()).unwrap();
   let iterations = u16::from_str(&env::args().nth(5).unwrap()).unwrap();

   // get start time
   let start = Instant::now();

   // load image, get dimensions and raw RGBA data as 2d vec of 4 element arrays
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
   final_img = create_img(final_img, &raw_data, &centroids, &distance);
   
   // save image
   final_img.save(out_path).unwrap();

   // get duration 
   let duration = start.elapsed();
   println!("Executed In {duration:?} Seconds");
} 
