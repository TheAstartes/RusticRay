use crate::{vector, interval,sphere::Sphere, hit_record, material::Scatterable};
use palette::Srgb;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: vector::Vec3,
    pub direction: vector::Vec3,
}

impl Ray{
    
    pub fn new(origin: vector::Vec3, direction: vector::Vec3) -> Ray{
        Ray { origin: origin, direction: direction }
    }

    pub fn at(&self, t: f64) -> vector::Vec3{
        self.origin() + (self.direction() * t)
    }

    pub fn origin(&self) -> vector::Vec3{
        self.origin.clone()
    }

    pub fn direction(&self) -> vector::Vec3{
        self.direction.clone()
    }

    pub fn ray_color(ray: &Ray, intensity: interval::Interval, world: &Vec<Sphere>, depth: u32) -> Srgb {
    
        let hit = hit_record::HitRecord::hit_world(world, ray, intensity);
        match hit{
            Some(HitRecord) => {
                let scattered = HitRecord.material.scatter(ray, &HitRecord);
            
                match scattered {
                    Some((scattered_ray, albedo)) => {
                        let target_color = Ray::ray_color(&scattered_ray, intensity, world, depth);
    
                        return Srgb::new(
                            albedo.red * target_color.red,
                            albedo.green * target_color.green,
                            albedo.blue * target_color.blue,
                        ); 
                    }
                    None => {
                        return Srgb::new(0.0, 0.0, 0.0)
                    }
                }
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
}