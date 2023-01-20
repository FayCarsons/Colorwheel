use image::*;
use rand::{self, Rng};

pub fn get_raw_image(img: &DynamicImage) -> Vec<Vec<[u8; 4]>> {
   let dim = img.dimensions();
   (0..dim.1).map(|y|
      (0..dim.0).map(|x|
         img.get_pixel(x, y).0
      ).collect()
   ).collect()
}

pub fn init_centroids(data: &Vec<Vec<[u8; 4]>>, means: u16) 
                     -> Vec<(f64,f64,f64)> {
   let mut centroids = Vec::with_capacity(means as usize);   
   let mut thread_rng = rand::thread_rng();

   for _ in 0..means {
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
               -> Vec<(f64, f64, f64)> {
   let mut container = vec![(0.0,0.0,0.0); centroids.len()];
   let mut num_pixels = vec![0; centroids.len()];
   
   for y in 0..data.len() { 
      for  x in 0..data[0].len() {
         let pixel = (data[y][x][0] as f64, 
                                       data[y][x][1] as f64,
                                       data[y][x][2] as f64);
         let mut min_dist = u64::MAX as f64;
         let mut centroid_id = 0;
      
      for (c, val) in centroids.iter().enumerate() {
         let dist = minkowski_distance(val, &pixel, distance_p);
         if dist < min_dist {
            min_dist = dist;
            centroid_id = c;
         }
      }

      container[centroid_id] = tup3_add(container[centroid_id], &pixel);
      num_pixels[centroid_id] += 1;}
   }
   
   for v in 0..container.len() {
      container[v].0 /= num_pixels[v] as f64;
      container[v].1 /= num_pixels[v] as f64;
      container[v].2 /= num_pixels[v] as f64;

   }
   container
}

pub fn create_img (mut target: image::RgbaImage, 
                  data: &Vec<Vec<[u8; 4]>>, 
                  centroids: &Vec<(f64, f64, f64)>, p: &f64) 
                  -> RgbaImage {

   let dim = (target.dimensions().0 as usize, 
                              target.dimensions().1 as usize); 
   
   for x in 0..dim.0 {
      for y in 0..dim.1 {
         let r = data[y][x][0] as f64;
         let g = data[y][x][1] as f64;
         let b = data[y][x][2] as f64;
         let data = (r, g, b);

         let mut min_dist = u64::MAX as f64;
         let mut centroid_id = 0; 
         
         for (c, val) in centroids.iter().enumerate() 
         {
            let dist = minkowski_distance(val, &data, p);
            if dist < min_dist {
               min_dist = dist;
               centroid_id = c;
            }
         }
         let (out_r, out_g, out_b) = centroids[centroid_id];
         let pixel: Rgba<u8> = Rgba([out_r as u8, out_g as u8, out_b as u8, 1]);
         target.put_pixel(x as u32, y as u32, pixel);
      }
   }
   target
}

pub fn tup3_add(mut tuple_one: (f64, f64, f64), tuple_two: &(f64, f64, f64)) 
               -> (f64, f64, f64) {
   tuple_one.0 += tuple_two.0;
   tuple_one.1 += tuple_two.1;
   tuple_one.2 += tuple_two.2;
   
   tuple_one
}

// minkowski distance function 
// p=1 manhattan, p=2 euclidean, p=25+ chebyshev
pub fn minkowski_distance (to: &(f64, f64, f64), 
                        from: &(f64, f64, f64), 
                        p: &f64 ) -> f64 {
   let dx = (to.0 - from.0).powf(*p);
   let dy = (to.1 - from.1).powf(*p);
   let dz = (to.2 - from.2).powf(*p);
   
   (dx.abs() + dy.abs() + dz.abs()).powf(1.0 / p)
}
