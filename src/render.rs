use rand::{Rng, rngs::ThreadRng};
use palette::Srgb;
use crate::{vector, interval, sphere::Sphere, ray};

#[derive(Debug)]
pub struct Render {}

impl Render{

    pub fn render(pixel00_loc: vector::Vec3, pixel_delta_u: vector::Vec3, pixel_delta_v: vector::Vec3, camera_center: vector::Vec3,
                image_width: i32, image_height: i32, max_depth: u32
    ){
        static I: f64 = 255.999;

        let intensity = interval::Interval::new(0.001, std::f64::MAX);

        println!("P3");
        println!("{} {}", image_height, image_width);
        println!("{}", image_height-1);

        let samples_per_pixel = 32;
        let mut random: ThreadRng = rand::thread_rng();


        let mut world: Vec<Sphere> = Vec::new();

        world.push(Sphere::new(vector::Vec3::new(0.0, 0.0, -1.0), 0.5));
        world.push(Sphere::new(vector::Vec3::new(0.0, -100.5, -2.0), 100.0));

        for y in 0..image_height {
            eprint!("\rScanlines remaining: {} ", image_height - y);
            for x in 0..image_width{

            let mut color = Srgb::new(0.0, 0.0, 0.0);
            for _z in 0..samples_per_pixel{

                let pixel_center = pixel00_loc + (pixel_delta_u * x as f64 /*+ random.gen::<f64>()*/) + (pixel_delta_v * y as f64 /*+ random.gen::<f64>()*/);
                let pixel_sample = pixel_center + Render::pixel_sample_square(random.gen::<f64>(), pixel_delta_u, pixel_delta_v);

                let ray_direction = pixel_sample - camera_center;

                let r = ray::Ray::new(camera_center,   ray_direction);
                color = color + ray::Ray::ray_color(&r, intensity, &world, max_depth);    

               }
               // new func for Interval -> line too long 
               println!("{} {} {}", (intensity.sample(color.red, samples_per_pixel) * I) as i32, (intensity.sample(color.green, samples_per_pixel)* I) as i32, (intensity.sample(color.blue, samples_per_pixel)* I) as i32);

            }
        }


        eprint!("\nDone                    \n")
    }

    fn pixel_sample_square(rng: f64, pixel_delta_u: vector::Vec3, pixel_delta_v: vector::Vec3) -> vector::Vec3 {

        let px = -0.5 + rng;
        let py = -0.5 + rng;

        (pixel_delta_u * px) + (pixel_delta_v * py)
    }  

}