
use rand::{Rng, rngs::ThreadRng};
use crate::{vector, ray, hit_record};
use palette::Srgb;

pub trait Scatterable {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hit_record::HitRecord) -> Option<(ray::Ray, Srgb)>;
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metallic(Metal),
    Glass(Glass),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hit_record::HitRecord) -> Option<(ray::Ray, Srgb)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metallic(m) => m.scatter(ray, hit_record),
            Material::Glass(g) => g.scatter(ray, hit_record),
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
    fuzz: f64
}

impl Metal {
    pub fn new( albedo: Srgb, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Default for Metal{
    fn default() -> Self {
        Metal { albedo: Srgb::new(0.0, 0.0, 0.0), fuzz: 0.0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Glass {
    pub refraction_index: f64,
}

impl Glass {
    pub fn new(refraction_index: f64) -> Glass {
        Glass { refraction_index }
    }
}

fn reflect(v: &vector::Vec3, n: &vector::Vec3) -> vector::Vec3 {
    *v - *n * (2.0 * v.dot(n))
}

fn refract(v: &vector::Vec3, n: &vector::Vec3, eta_quotient: f64) -> vector::Vec3 {
    let cos_theta = ((-*v).dot(n)).min(1.0);
    
    let r_out_perp = (*v + *n * cos_theta) * eta_quotient;
    let r_out_parallel = *n * (-1.0 * (1.0 - r_out_perp.length_squared().abs().sqrt() ));

    r_out_parallel + r_out_perp
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);

    let r0_squared = r0.powi(2);

    r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5) 
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hit_record::HitRecord) -> Option<(ray::Ray, Srgb)> {
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let scattered = ray::Ray::new(hit_record.point, reflected + vector::Vec3::random_unit_vec3() * self.fuzz);
        let attenuation = self.albedo;
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

impl Scatterable for Glass {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hit_record::HitRecord) -> Option<(ray::Ray, Srgb)> {
        let mut rng = rand::thread_rng();
        let attenuation = Srgb::new(1.0 as f32, 1.0 as f32, 1.0 as f32);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };


        let unit_vec_direction = ray.direction.unit_vector();
        let cos_theta = (-unit_vec_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let can_refract = refraction_ratio * sin_theta > 1.0;


        if can_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() {
            let reflected = reflect(&unit_vec_direction, &hit_record.normal);
            let scattered = ray::Ray::new(hit_record.point, reflected);
            Some((scattered, attenuation))
        } else {
            let direction = refract(&unit_vec_direction, &hit_record.normal, refraction_ratio);
            let scattered = ray::Ray::new(hit_record.point, direction);
            Some((scattered, attenuation))
        }
    }
}