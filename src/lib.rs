use std::ops::{Add, Mul, Sub, Div, Neg};
use std::cmp::{PartialEq, max};
use palette::Srgb;

use rand::{Rng, rngs::ThreadRng};

mod test;


#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}


impl Vec3 { 
    pub fn new(x: f64, y: f64, z:f64) -> Vec3{
        Vec3 { x, y, z}
    }


    pub fn x(&self) -> f64 
    {
        self.x
    }

    pub fn y(&self) -> f64
    {
        self.y
    }

    pub fn z(&self) -> f64 
    {
        self.z
    }

    pub fn length(&self) -> f64
    {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.length().powi(2)
    }

    pub fn cross(&self, vec3: &Vec3) -> Vec3 {
        Vec3 { 
            x: (self.y * vec3.z()) - (self.z * vec3.y()),
            y: (self.z * vec3.x()) - (self.x * vec3.z()),
            z: (self.x * vec3.y()) - (self.y * vec3.x()),
        }
    }

    pub fn unit_vector(self) -> Vec3{
        self / self.length()
    }

    pub fn dot(&self, point: &Vec3) -> f64{
        self.x * point.x() + self.y * point.y() + self.z * point.z()
    }
    
}

impl Neg for Vec3
{
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Vec3> for Vec3
{    
    type Output = Vec3;

    fn add(self, vec3: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + vec3.x(),
            y: self.y + vec3.y(),
            z: self.z + vec3.z(),
            
        }
    }
}

impl Add<f64> for Vec3
{
    type Output = Vec3;

    fn add(self, num: f64) -> Vec3 {
        Vec3 {
            x: self.x + num,
            y: self.y + num,
            z: self.y + num,
        }
    }
}
       
impl Sub for Vec3
{
    type Output = Vec3;

    fn sub(self, vec3: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - vec3.x(),
            y: self.y - vec3.y(),
            z: self.z - vec3.z(),
            
        }
    }
}

impl Mul<Vec3> for Vec3
{  
    type Output = Vec3;

    fn mul(self, vec3: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * vec3.x(),
            y: self.y * vec3.y(),
            z: self.z * vec3.z(),
            
        }
    }
}


impl Mul<f64> for Vec3
{  
    type Output = Vec3;

    fn mul(self, multiplier: f64) -> Vec3 {
        Vec3 {
            x: self.x * multiplier,
            y: self.y * multiplier,
            z: self.z * multiplier,
            
        }
    }
}

impl Div<Vec3> for Vec3
{
    type Output = Vec3;

    fn div(self, vec3: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / vec3.x(),
            y: self.y / vec3.y(),
            z: self.z / vec3.z(),
            
        }
    }
}

impl Div<f64> for Vec3{
    
    type Output = Vec3;

    fn div(self, num: f64) -> Vec3{

        let c: f64 = 1.0/num;

        Vec3{
            x: c * self.x,
            y: c * self.y,
            z: c * self.z,
        }
    }
}

impl PartialEq for Vec3{
    fn eq(&self, vec3: &Vec3) -> bool{
        self.x == vec3.x() && self.y == vec3.y() && self.z == vec3.z()
    }
}

impl Default for Vec3{
    fn default() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray{
    
    pub fn new(origin: Vec3, direction: Vec3) -> Ray{
        Ray { origin: origin, direction: direction }
    }

    pub fn at(&self, t: f64) -> Vec3{
        self.origin() + (self.direction() * t)
    }

    pub fn origin(&self) -> Vec3{
        self.origin.clone()
    }

    pub fn direction(&self) -> Vec3{
        self.direction.clone()
    }
}


impl Default for HitRecord {
    fn default() -> Self {
        HitRecord { point: Vec3::default(), t: 0.0, normal: Vec3::default(), front_face: true}
    }
}

pub struct HitRecord{
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord{
    pub fn new(t: f64, point: Vec3, normal: Vec3, front_face: bool) -> HitRecord {
        HitRecord {
            t,
            point,
            normal,
            front_face
        }
    }
}

pub trait Hittable{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
} 

pub struct Sphere{
    center: Vec3,
    radius: f64,
}

impl Sphere{
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;

            if temp < t_max && temp > t_min {

                let p = ray.at(temp);
                let normal = (p - self.center) / self.radius;
                let front_face = ray.direction().dot(&normal) < 0.0;

                return Some(HitRecord {
                    t: temp,
                    point: p,
                    normal: if front_face {normal } else { -normal},
                    front_face
                })
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct Camera{
    pub image_width: i32,
    pub image_height: i32,
    pub pixel00_loc: Vec3,
    pub camera_center: Vec3,
    pub pixel_vec_u: Vec3,
    pub pixel_vec_v: Vec3
}

impl Camera{
    pub fn new(image_width: i32) -> Camera{

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
        let camera_center = Vec3::new(0.0, 0.0, 0.0);

        // Vectors for horizontal and vertical lines
        let vecotr_u = Vec3::new(viewport_width, 0.0, 0.0);
        let vector_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Delta vectors for pixels
        let pixel_vec_u = vecotr_u / image_width as f64;
        let pixel_vec_v = vector_v / image_height as f64;

        // Calculate location of the upper left pixel
        let viewpoert_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - (vecotr_u/2.0) - (vector_v/2.0);
        let mut pixel00_loc = (pixel_vec_u + pixel_vec_v) * 0.5;
        pixel00_loc = pixel00_loc + viewpoert_upper_left;

        Camera { 
            image_width,
            image_height,
            pixel00_loc,
            pixel_vec_u,
            pixel_vec_v,
            camera_center
        }
    }
}

fn hit_world(world: &Vec<Sphere>, ray: &Ray, intensity: Interval) -> Option<HitRecord> {
    let mut closest = intensity.max;

    let mut hit_record = None;

    for sphere in world {
        if let Some(hit) = sphere.hit(ray, intensity.min, closest) {
            closest = hit.t;
            hit_record = Some(hit);
        }
    }

    hit_record
}

fn ray_color(ray: &Ray, intensity: Interval, world: &Vec<Sphere>) -> Srgb {
    
    let hit = hit_world(world, ray, intensity);
    
    match hit{
        Some(HitRecord) => {
            let normal = (ray.at(HitRecord.t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();

            return Srgb::new(
                (0.5 * normal.x() as f32 + 0.5) *0.8,
                (0.5 * normal.y() as f32 + 0.5) * 0.3,
                (0.5 * normal.z() as f32 + 0.5) *1.0,
            )
        }
        None => {
            let t: f32 = 0.5 * (ray.direction().unit_vector().y() as f32 + 1.0);

            return Srgb::new(
                (1.0 - t) * 1.0 + t * 0.5,
                (1.0 - t) * 1.0 + t * 0.7,
                (1.0 - t) * 1.0 + t * 1.0,
            );
        }
    }


}

#[derive(Debug)]
pub struct Render {}

impl Render{

    pub fn render(pixel00_loc: Vec3, pixel_delta_u: Vec3, pixel_delta_v: Vec3, camera_center: Vec3,
                image_width: i32, image_height: i32
    ){
        static I: f64 = 255.999;

        let intensity = Interval::new(0.001, std::f64::MAX);

        println!("P3");
        println!("{} {}", image_height, image_width);
        println!("{}", image_height-1);

        let samples_per_pixel = 32;
        let mut random: ThreadRng = rand::thread_rng();


        let mut world: Vec<Sphere> = Vec::new();

        world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
        world.push(Sphere::new(Vec3::new(0.0, -100.5, -2.0), 100.0));

        for y in 0..image_height {
            eprint!("\rScanlines remaining: {} ", image_height - y);
            for x in 0..image_width{

            let mut color = Srgb::new(0.0, 0.0, 0.0);
            for _z in 0..samples_per_pixel{

                let pixel_center = pixel00_loc + (pixel_delta_u * x as f64 /*+ random.gen::<f64>()*/) + (pixel_delta_v * y as f64 /*+ random.gen::<f64>()*/);
                let pixel_sample = pixel_center + Render::pixel_sample_square(random.gen::<f64>(), pixel_delta_u, pixel_delta_v);

                let ray_direction = pixel_sample - camera_center;

                let r = Ray::new(camera_center,   ray_direction);
                color = color + ray_color(&r, intensity, &world);    

               }
               // new func for Interval -> line too long 
               println!("{} {} {}", (intensity.sample(color.red, samples_per_pixel) * I) as i32, (intensity.sample(color.green, samples_per_pixel)* I) as i32, (intensity.sample(color.blue, samples_per_pixel)* I) as i32);

            }
        }


        eprint!("\nDone                    \n")
    }

    fn pixel_sample_square(rng: f64, pixel_delta_u: Vec3, pixel_delta_v: Vec3) -> Vec3 {

        let px = -0.5 + rng;
        let py = -0.5 + rng;

        (pixel_delta_u * px) + (pixel_delta_v * py)
    }  

}


#[derive(Debug, Clone, Copy)]
struct Interval{
    min: f64,
    max: f64
}

impl Interval{
    fn new(min: f64, max: f64) -> Interval{
        Interval { min, max }
    }

    fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    fn surrounds(&self, x: f64) -> bool{
        self.min < x && x < self.max
    }

    fn clamp(&self, x: f32) -> f64 {
        if self.min > x as f64 {return self.min}
        if self.max < x as f64 {return self.max}
    
        x as f64
    }

    fn sample(&self,color: f32, pixels_per_sample: i32) -> f64 {

        let scale = 1.0 / pixels_per_sample as f32;

        let new_color = color * scale;

        self.clamp(new_color)
    }
}

