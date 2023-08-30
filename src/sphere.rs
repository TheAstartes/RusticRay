use crate::{ray, hit_record, vector, material::{Material, Lambertian}};
use palette::Srgb;
pub trait Hittable{
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<hit_record::HitRecord>;
} 

pub struct Sphere{
    center: vector::Vec3,
    radius: f64,
    material: Material,
}

impl Sphere{
    pub fn new(center: vector::Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
            material: Material::Lambertian(Lambertian::new(Srgb::new(
                0.5 as f32, 0.5 as f32, 0.5 as f32,
            ))),
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<hit_record::HitRecord> {
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

                return Some(hit_record::HitRecord {
                    t: temp,
                    point: p,
                    normal: if front_face {normal } else { -normal},
                    front_face,
                    material: self.material,
                })
            }
        }

        None
    }
}