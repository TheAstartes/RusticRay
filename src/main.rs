use rt::HitRecord;
use rt::Hittable;
//use palette::encoding::srgb;
use rt::Ray;
use rt::Vec3;
use palette::Srgb;
use rt::Sphere;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;


fn ray_color(ray: &Ray) -> Srgb {

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0 ), 0.5);

    
    let hit = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    
    if hit > 0.0 {
        let n = (ray.at(hit) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Srgb::new(0.5*n.x() as f32 + 0.7, 0.5*n.y() as f32 + 0.5, 0.5*n.z() as f32 + 0.5);
    }
    
    let t: f32 = 0.5 * (ray.direction().unit_vector().y() as f32 + 1.0);
    Srgb::new
    (
        (1.0 - t) * 1.0 + t * 0.3, 
        (1.0 - t) * 1.0 + t * 0.6,
        (1.0 - t) * 1.0 + t * 1.0,
    )
}


fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().dot(&r.direction());
    let b = 2.0 * oc.dot(&r.direction());
    let c = oc.dot(&oc) - radius * radius;

    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b -discriminant.sqrt()) / (2.0 *a);
    }
}

#[derive(Debug)]
struct Render {

    image_width: i32,
    image_height: i32,
}
impl Render{
    fn new(x: i32, y: i32) -> Self{
        Self { image_width: x, image_height: y }
    }

    fn render(&self, pixel00_loc: Vec3, pixel_delta_u: Vec3, pixel_delta_v: Vec3, camera_center: Vec3){
        static I: f32 = 255.999;

        //let inverse_height_minus_one = 1.0 / (self.image_height as f64 - 1.0);
        //let inverse_width_minus_one = 1.0 / (self.image_width as f64 - 1.0);
    
        for y in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - y);
            for x in 0..self.image_width{
            
                let pixel_center = pixel00_loc + (pixel_delta_u * x as f64) + (pixel_delta_v * y as f64);
                let ray_direction = pixel_center - camera_center;

                let r = Ray::new(camera_center,   ray_direction);
                let color = ray_color(&r);    

                

                println!("{} {} {}", (color.red * I) as i32, (color.green * I) as i32, (color.blue* I) as i32);
            }
        }

        eprint!("\nDone                    \n")
    }
    
}

#[derive(Debug)]
struct Camera{
    image_width: i32
}

impl Camera{
    fn new(img_width: i32) -> Camera{
        Camera { image_width: img_width }
    }

    fn calc_ray(&self) {
        
        //Image
        const ASPECT_RATIO: f64 = 16.0/9.0;
        let image_width = self.image_width;

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
        let camera_center = Vec3::new(0.0, 0.0, 0.0);

        // Vectors for horizontal and vertical lines
        let vecotr_u = Vec3::new(viewport_width, 0.0, 0.0);
        let vector_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Delta vectors for pixels
        let vecotr_u_pixel = vecotr_u / image_width as f64;
        let vector_v_pixel = vector_v / image_height as f64;

        // Calculate location of the upper left pixel

        let viewpoert_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - (vecotr_u/2.0) - (vector_v/2.0);
        let mut pixel00_loc = (vecotr_u_pixel + vector_v_pixel) * 0.5;
        pixel00_loc = pixel00_loc + viewpoert_upper_left;

        let rndr = Render::new(image_width, image_height);

        println!("P3");
        println!("{} {}", rndr.image_height, rndr.image_width);
        println!("{}", rndr.image_height-1);

        rndr.render(pixel00_loc, vecotr_u_pixel, vector_v_pixel, camera_center);
    }
}

fn main() {

    let camera = Camera::new(400);

    camera.calc_ray()
}

#[test]
fn test_ray_color(){
    let p = Vec3::new(0.0, 0.0, 0.0);
    let q = Vec3::new(1.0, 0.0, 0.0);
    let r = Ray::new(p, q);
   
    assert_approx_eq!(ray_color(&r).red, Srgb::new(0.75, 0.85, 1.0).red);
    assert_approx_eq!(ray_color(&r).green, Srgb::new(0.75, 0.85, 1.0).green);
    assert_approx_eq!(ray_color(&r).blue, Srgb::new(0.75, 0.85, 1.0).blue);
}
