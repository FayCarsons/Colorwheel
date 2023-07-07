mod lib;
use lib::{create_img, get_raw_image, init_centroids, iterate, Centroid};

use image::*;
use std::io::stdin;
use std::time::Instant;
use std::str::FromStr;


fn prompt(line: &str) -> String {
    let mut input = String::new();
    println!("{line}");
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_args() -> (String, String, u16, u16, f64, String) {    
    let input_path = prompt("Enter input path: ");
    let output_path = prompt("Enter output path: ");
    let k = u16::from_str(&prompt("Enter K value: ")).unwrap();
    let iterations = u16::from_str(&prompt("Enter # of iterations: ")).unwrap();
    let p = f64::from_str(&prompt("Enter distance metric: ")).unwrap();
    let palette_mode = prompt("Use palette mode? (y/n): ").to_lowercase();
    
    (input_path, output_path, k, iterations, p, palette_mode)
}

fn main() {
    // get command line args
    let (in_path, out_path, k, iterations, distance, mode) = get_args();    

    // get start time
    let start = Instant::now();

    // load image, get dimensions and raw RGBA data as 2d vec of 4 element arrays
    let img: DynamicImage = image::open(in_path).unwrap();
    let (x_size, y_size) = img.dimensions();
    let raw_data: Vec<Vec<[u8; 4]>> = get_raw_image(&img);
    let raw_time = start.elapsed();
    println!("Acquired raw image data in {raw_time:?}");

    // place centroids, random pixels in image
    let mut centroids: Vec<Centroid> = init_centroids(&raw_data, &k);

    // iterate centroids/do clustering
    for _ in 0..iterations {
        centroids = iterate(&raw_data, &centroids, x_size as usize, y_size as usize);
    }
    let iteration_time = start.elapsed();
    println!("Iterations completed in {iteration_time:?}");

    // create final image
    let final_img = create_img(img.dimensions(), raw_data, centroids, distance, &mode);
    // save image
    final_img.save(out_path).unwrap();
    let img_time = start.elapsed() - iteration_time;
    println!("Image assembled and saved in {img_time:?}");

    // get duration
    let duration = start.elapsed();
    println!("Executed In {duration:?}");
}
