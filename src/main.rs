mod lib;
use lib::{create_img, get_raw_image, init_centroids, iterate, Centroid};

use image::*;
use std::{io::stdin, path::Path, str::FromStr, time::Instant};

enum Prompt {
    Path,
    Safe,
    Num,
    Mode,
}

struct Options {
    input_path: String,
    output_path: String,
    k: u16,
    iterations: u16,
    mode: String,
}

impl Options {
    fn new() -> Options {
        let prompts = [
            "Enter input path: ",
            "Enter output path: ",
            "Enter K value: ",
            "Enter # of iterations: ",
            "Create palette? (y/n): ",
        ];

        let parse = [
            Prompt::Path,
            Prompt::Safe,
            Prompt::Num,
            Prompt::Num,
            Prompt::Mode,
        ];

        let opts = prompts
            .iter()
            .zip(parse)
            .map(|(pr, pa)| prompt(pr, pa))
            .collect::<Vec<String>>();

        Options {
            input_path: opts[0].clone(),
            output_path: opts[1].clone(),
            k: u16::from_str(&opts[2]).unwrap(),
            iterations: u16::from_str(&opts[3]).unwrap(),
            mode: opts[4].clone(),
        }
    }
}

fn prompt(line: &str, parse: Prompt) -> String {
    let mut first = true;
    let mut input = String::new();
    loop {
        if first {
            println!("{line}")
        } else {
            println!("invalid input \n \"{input}\" \n please try again");
            input.clear();
        }

        stdin().read_line(&mut input).unwrap();
        let parse_fn = match parse {
            Prompt::Path => |p: &str| -> bool { Path::new(&p.trim_matches('\n')).exists() },
            Prompt::Safe => |_: &str| true,
            Prompt::Num => |n: &str| -> bool { u16::from_str(n.trim()).is_ok() },
            Prompt::Mode => |m: &str| -> bool { m.trim() == "y" || m.trim() == "n" },
        };

        if parse_fn(&input) {
            return input.trim().to_string();
        } else {
            first = false;
            continue;
        }
    }
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
    let iteration_time = start.elapsed() - raw_time;
    println!("Iterations completed in {iteration_time:?}");

    // create final image
    let final_img = create_img(img.dimensions(), raw_data, centroids, &options.mode);
    // save image
    final_img.save(options.output_path).unwrap();
    let img_time = start.elapsed() - iteration_time;
    println!("Image assembled and saved in {img_time:?}");

    // get duration
    let duration = start.elapsed();
    println!("Total time: {duration:?}");
}
