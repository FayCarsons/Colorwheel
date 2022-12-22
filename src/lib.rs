use num_traits::Pow;
use image::*;
use rand::{self, Rng};

pub fn get_raw_image(img: &DynamicImage) -> Vec<Vec<[u8; 4]>> {
   let dim = img.dimensions();
   let raw_data = (0..dim.1).map(|y|(0..dim.0).map(|x|
                                          img.get_pixel(x, y).0).collect()
                                       ).collect();
      
   raw_data
}

pub fn init_centroids(data: &Vec<Vec<[u8; 4]>>, k: i32) -> Vec<(f64,f64,f64)> {
   let mut centroids = Vec::new();   
   let mut thread_rng = rand::thread_rng();
   for _ in 0..k {
      let rnd_x = thread_rng.gen_range(0..data[0].len());
      let rnd_y = thread_rng.gen_range(0..data.len()); 

      let pixel = (data[rnd_y][rnd_x][0] as f64,
                                    data[rnd_y][rnd_x][1] as f64,
                                    data[rnd_y][rnd_x][2] as f64); 

      centroids.push(pixel);
   }                                                                                
centroids                                                   
}

pub fn iterate(data: &Vec<Vec<[u8; 4]>>, 
                   centroids: &Vec<(f64, f64, f64)>, 
                   distance_p: &f64) 
                   -> Vec<(f64, f64, f64)>{
   
   let mut container: Vec<(f64, f64, f64)> = Vec::new();
   let mut num_pixels: Vec<usize> = Vec::new();
   for _ in 0..centroids.len() {
      container.push((0.0,0.0,0.0));
      num_pixels.push(0);
   }
   
   for y in 0..data.len() as usize { 
     for  x in 0..data[0].len() as usize {
      let pixel = (data[y][x][0] as f64, 
                                   data[y][x][1] as f64,
                                   data[y][x][2] as f64);
      let mut min_dist = f64::MAX;
      let mut centroid_id = 0;
      
      for c in 0..centroids.len() {
         let dist = get_distance_3d(&centroids[c], &pixel, distance_p);
         if dist < min_dist {
            min_dist = dist;
            centroid_id = c;
         }
      }

      container[centroid_id] = tup3_pairwise_addition(container[centroid_id], &pixel);
      num_pixels[centroid_id] += 1;}
   }
   
   for v in 0..container.len() {
      container[v].0 = container[v].0 / (num_pixels[v] as f64);
      container[v].1 = container[v].1 / (num_pixels[v] as f64);
      container[v].2 = container[v].2 / (num_pixels[v] as f64);

   }
   container
}

pub fn create_img (mut target: image::RgbaImage, 
                  img: &image::DynamicImage, 
                  centroids: &Vec<(f64, f64, f64)>, p: &f64) 
                  -> RgbaImage{
   let dim = img.dimensions(); 
   
   for x in 0..dim.0 {
      for y in 0..dim.1 {
         
         let r = img.get_pixel(x, y).0[0] as f64;
         let g = img.get_pixel(x, y).0[1] as f64;
         let b = img.get_pixel(x, y).0[2] as f64;
         let data = (r, g, b);

         let mut min_dist = 1000000000.0;
         let mut centroid_id = 0; 
         
         for c in 0..centroids.len() {
            let dist = get_distance_3d(&centroids[c], &data, p);
            if dist < min_dist {
               min_dist = dist;
               centroid_id = c;
            }
         }
      
         let out_r = centroids[centroid_id].0 as u8;
         let out_g = centroids[centroid_id].1 as u8;
         let out_b = centroids[centroid_id].2 as u8;
         let pixel: Rgba<u8> = Rgba([out_r, out_g, out_b, 1]);
         target.put_pixel(x, y, pixel);
      }
   }
   target
}

pub fn tup3_pairwise_addition(mut tuple_one: (f64, f64, f64), tuple_two: &(f64, f64, f64)) -> (f64, f64, f64) {
   tuple_one.0 = tuple_one.0 + tuple_two.0;
   tuple_one.1 = tuple_one.1 + tuple_two.1;
   tuple_one.2 = tuple_one.2 + tuple_two.2;
   
   tuple_one
}

// generalized(minkowski) distance function, when p=1 its manhattan, 
//p=2 euclidean, 25+ chebyshev
pub fn get_distance_3d (to: &(f64, f64, f64), 
                     from: &(f64, f64, f64), 
                     p: &f64 ) -> f64 {
   let dx = (to.0 - from.0).pow(p);
   let dy = (to.1 - from.1).pow(p);
   let dz = (to.2 - from.2).pow(p);
   
   let distance = (dx.abs() + dy.abs() + dz.abs()).pow(1.0 / p);

   distance
}
