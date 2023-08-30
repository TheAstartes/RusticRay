use crate::{vector, material, sphere::{self, Hittable}, interval, ray};
pub struct HitRecord{
    pub point: vector::Vec3,
    pub normal: vector::Vec3,
    pub material: material::Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord{
    pub fn new(t: f64, point: vector::Vec3, normal: vector::Vec3, front_face: bool, material: material::Material) -> HitRecord {
        HitRecord {
            t,
            point,
            normal,
            front_face,
            material,
        }
    }

    pub fn hit_world(world: &Vec<sphere::Sphere>, ray: &ray::Ray, intensity: interval::Interval) -> Option<HitRecord> {
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
}
