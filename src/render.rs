use rand::{Rng, rngs::ThreadRng};
use palette::Srgb;
use crate::{vector, interval, sphere::Sphere, ray, camera::Camera, material::{*, self}};

#[derive(Debug)]
pub struct Render {}

impl Render{

    pub fn render(camera: Camera){
        static I: f64 = 255.999;

        let intensity = interval::Interval::new(0.001, std::f64::MAX);

        println!("P3");
        println!("{} {}", camera.image_height, camera.image_width);
        println!("{}", camera.image_height-1);

        let samples_per_pixel = 32;
        let mut random: ThreadRng = rand::thread_rng();


        let mut world: Vec<Sphere> = Vec::new();


        //Materials
        let metal_sphere = material::Material::Metallic(Metal::new(Srgb::new(0.5, 0.5, 0.5), 0.0));
        let fuzz_metal_sphere = material::Material::Metallic(Metal::new(Srgb::new(0.5, 0.0, 0.9), 0.3));
        let normal_sphere = material::Material::Lambertian(Lambertian::new(Srgb::new(0.5, 0.5, 0.5)));
        let glass_sphere = material::Material::Glass(Glass::new(3.0));

        //Spheres
        world.push(Sphere::new(vector::Vec3::new(0.0, 0.0, -2.0), 0.5, normal_sphere));
        world.push(Sphere::new(vector::Vec3::new(0.0, -100.5, -3.0), 100.0, normal_sphere));
        world.push(Sphere::new(vector::Vec3::new(1.0, 0.0, -2.3), 0.5, metal_sphere));
        world.push(Sphere::new(vector::Vec3::new(0.7, -0.3, -1.4), 0.2, fuzz_metal_sphere));
        world.push(Sphere::new(vector::Vec3::new(-0.7, -0.3, -1.4), 0.2, glass_sphere));


        for y in 0..camera.image_height {
            eprint!("\rScanlines remaining: {} ", camera.image_height - y);
            for x in 0..camera.image_width{

            let mut color = Srgb::new(0.0, 0.0, 0.0);
            for _z in 0..samples_per_pixel{

                let pixel_center = camera.pixel00_loc + (camera.pixel_vec_u * x as f64) + (camera.pixel_vec_v * y as f64);
                let pixel_sample = pixel_center + Render::pixel_sample_square(random.gen::<f64>(), camera.pixel_vec_u, camera.pixel_vec_v);

                let ray_direction = pixel_sample - camera.center;

                let r = ray::Ray::new(camera.center,   ray_direction);
                color = color + ray::Ray::ray_color(&r, intensity, &world, camera.max_depth);    

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