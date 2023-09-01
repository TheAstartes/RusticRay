use crate::vector;

#[derive(Debug)]
pub struct Camera{
    pub image_width: i32,
    pub image_height: i32,
    pub pixel00_loc: vector::Vec3,
    pub center: vector::Vec3,
    pub pixel_vec_u: vector::Vec3,
    pub pixel_vec_v: vector::Vec3,
    pub max_depth: u32,
}

impl Camera{
    pub fn new(image_width: i32, max_depth: u32) -> Camera{

        //Image
        const ASPECT_RATIO: f64 = 16.0/9.0;

        // Calculate the image height, and ensure that it's at least 1.
        let mut image_height =  image_width / ASPECT_RATIO as i32;
        
        image_height =  match image_height {
            0..=1 => 1,
            _ => image_height,
        };
        
        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_height / image_width) as f64;
        let center = vector::Vec3::new(0.0, 0.0, 0.0);

        // Vectors for horizontal and vertical lines
        let vecotr_u = vector::Vec3::new(viewport_width, 0.0, 0.0);
        let vector_v = vector::Vec3::new(0.0, -viewport_height, 0.0);

        // Delta vectors for pixels
        let pixel_vec_u = vecotr_u / image_width as f64;
        let pixel_vec_v = vector_v / image_height as f64;

        // Calculate location of the upper left pixel
        let viewpoert_upper_left = center - vector::Vec3::new(0.0, 0.0, focal_length) - (vecotr_u/2.0) - (vector_v/2.0);
        let mut pixel00_loc = (pixel_vec_u + pixel_vec_v) * 0.5;
        pixel00_loc = pixel00_loc + viewpoert_upper_left;

        Camera { 
            image_width,
            image_height,
            pixel00_loc,
            pixel_vec_u,
            pixel_vec_v,
            center,
            max_depth
        }
    }
}