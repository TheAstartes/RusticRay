use crate::{vector, ray, hit_record};
use palette::Srgb;

pub trait Scatterable {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hit_record::HitRecord) -> Option<(ray::Ray, Srgb)>;
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metallic(Metal),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hit_record::HitRecord) -> Option<(ray::Ray, Srgb)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metallic(m) => m.scatter(ray, hit_record),
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Srgb,
}

impl Lambertian {
    pub fn new(albedo: Srgb) -> Lambertian {
        Lambertian {albedo} 
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian { albedo: Srgb::new(0.0, 0.0, 0.0) }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &ray::Ray, hit_record: &hit_record::HitRecord) -> Option<(ray::Ray, Srgb)> {

        let mut scatter_direction = hit_record.normal + vector::Vec3::random_unit_vec3();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let target_ray = hit_record.point + scatter_direction;
        let scattered = ray::Ray::new(hit_record.point, target_ray - hit_record.point);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Srgb,
}

impl Default for Metal{
    fn default() -> Self {
        Metal { albedo: Srgb::new(0.0, 0.0, 0.0) }
    }
}

fn reflect(v: &vector::Vec3, n: &vector::Vec3) -> vector::Vec3 {
    *v - *n * (2.0 * v.dot(n))
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hit_record::HitRecord) -> Option<(ray::Ray, Srgb)> {
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let scattered = ray::Ray::new(hit_record.point, reflected);
        let attenuation = self.albedo;
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}