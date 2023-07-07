mod lib;
use lib::{create_img, get_raw_image, init_centroids, iterate, Centroid};

use image::*;
use std::io::stdin;
use std::str::FromStr;
use std::time::Instant;

struct Options {
    input_path: String,
    output_path: String,
    k: u16,
    iterations: u16,
    mode: String,
}

impl Options {
    fn new() -> Options {
        let prompts = vec![
        "Enter input path: ",
        "Enter output path: ",
        "Enter K value: ",
        "Enter # of iterations: ",
        "Enter distance metric: ",
        "Create palette? (y/n): ",
        ];

        let opts = prompts.iter().map(|p| prompt(p)).collect::<Vec<String>>();

        Options {
            input_path: opts[0].clone(),
            output_path: opts[1].clone(),
            k: u16::from_str(&opts[2]).unwrap(),
            iterations: u16::from_str(&opts[3]).unwrap(),
            mode: opts[5].clone(),
        }
    }
}

fn prompt(line: &str) -> String {
    let mut input = String::new();
    println!("{line}");
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    // get command line args
    let options = Options::new();

    // get start time
    let start = Instant::now();

    // load image, get dimensions and raw RGBA data as 2d vec of 4 element arrays
    let img: DynamicImage = image::open(options.input_path).unwrap();
    let (x_size, y_size) = img.dimensions();
    let raw_data: Vec<Vec<[u8; 4]>> = get_raw_image(&img);
    let raw_time = start.elapsed();
    println!("Acquired raw image data in {raw_time:?}");

    // place centroids, random pixels in image
    let mut centroids: Vec<Centroid> = init_centroids(&raw_data, &options.k);

    // iterate centroids/do clustering
    for _ in 0..options.iterations {
        centroids = iterate(&raw_data, &centroids, x_size as usize, y_size as usize);
    }
    let iteration_time = start.elapsed();
    println!("Iterations completed in {iteration_time:?}");

    // create final image
    let final_img = create_img(img.dimensions(), raw_data, centroids, &options.mode);
    // save image
    final_img.save(options.output_path).unwrap();
    let img_time = start.elapsed() - iteration_time;
    println!("Image assembled and saved in {img_time:?}");

    // get duration
    let duration = start.elapsed();
    println!("Executed In {duration:?}");
}
