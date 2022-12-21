use num_traits::Pow;
use image::*;
use rand::{self, Rng};

pub fn init_centroids(img: &DynamicImage, k: i32) -> Vec<(f64,f64,f64)> {
   let mut centroids = Vec::new();   
   let mut thread_rng = rand::thread_rng();
   for _ in 0..k {
      let rnd_x = thread_rng.gen_range(0..img.dimensions().0);
      let rnd_y = thread_rng.gen_range(0..img.dimensions().1); 
      let pixel = (img.get_pixel(rnd_x, rnd_y).0[0] as f64, 
                                    img.get_pixel(rnd_x, rnd_y).0[1] as f64,
                                    img.get_pixel(rnd_x, rnd_y).0[2] as f64);  
      centroids.push(pixel);
   }                                                                                
centroids                                                   
}

pub fn iterate(img: &DynamicImage, 
                   centroids: &Vec<(f64, f64, f64)>, 
                   distance_p: &f64) 
                   -> Vec<(f64, f64, f64)>{
   

   let mut container: Vec<(f64, f64, f64)> = Vec::new();
   let mut num_pixels: Vec<usize> = Vec::new();
   for _ in 0..centroids.len() {
      container.push((0.0,0.0,0.0));
      num_pixels.push(0);
   }
   
   for y in 0..img.dimensions().1 { 
     for  x in 0..img.dimensions().0 {
      let data = (img.get_pixel(x,y).0[0] as f64, 
                                   img.get_pixel(x,y).0[1] as f64,
                                   img.get_pixel(x,y).0[2] as f64);
      let mut min_dist = f64::MAX;
      let mut centroid_id = 0;
      
      for c in 0..centroids.len() {
         let dist = get_distance_3d(&centroids[c], &data, distance_p);
         if dist < min_dist {
            min_dist = dist;
            centroid_id = c;
         }
      }

      container[centroid_id] = tup3_pairwise_addition(container[centroid_id], &data);
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
               dim: &(u32, u32), 
               img: &image::DynamicImage, 
               centroids: &Vec<(f64, f64, f64)>, p: &f64) 
               -> RgbaImage{
   

    
   
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
//p=2 euclidean
pub fn get_distance_3d (to: &(f64, f64, f64), 
                     from: &(f64, f64, f64), 
                     p: &f64 ) -> f64 {
   let dx = (to.0 - from.0).pow(p);
   let dy = (to.1 - from.1).pow(p);
   let dz = (to.2 - from.2).pow(p);
   
   let distance = (dx.abs() + dy.abs() + dz.abs()).pow(1.0 / p);

   distance
}
