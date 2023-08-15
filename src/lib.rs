use std::ops::{Add, Mul, Sub, Div};
use std::cmp::PartialEq;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

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

    pub fn cross(&self, vec3: &Vec3) -> Vec3 {
        Vec3 { 
            x: (self.y * vec3.z()) - (self.z * vec3.y()),
            y: (self.z * vec3.x()) - (self.x * vec3.z()),
            z: (self.x * vec3.y()) - (self.y * vec3.x()),
        }
    }

    //needs to be fixed
    pub fn unit_vector(self) -> Vec3{
        self / self.length()
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

#[test]
fn test_new_vecotr(){
    let test_vec = Vec3{x:0.11, y:0.22, z:0.33};

    assert_approx_eq!(test_vec.x(), 0.11);
    assert_approx_eq!(test_vec.y(), 0.22);
    assert_approx_eq!(test_vec.z(), 0.33);

    let test_vec2 = Vec3::new(0.11, 0.22, 0.33);

    assert_approx_eq!(test_vec2.x(), test_vec.x());
    assert_approx_eq!(test_vec2.y(), test_vec.y());
    assert_approx_eq!(test_vec2.z(), test_vec.z());
}

#[test]
fn test_add_vector(){

    let test_vec = Vec3{x:0.11, y:0.22, z:0.33};
    let test_vec2 = Vec3::new(0.12, 0.23, 0.34);

    let addition_vector = test_vec + test_vec2;

    assert_approx_eq!(addition_vector.x(), 0.23);
    assert_approx_eq!(addition_vector.y(), 0.45);
    assert_approx_eq!(addition_vector.z(), 0.67);
}

#[test]
fn test_sub_vector(){

    let test_vec = Vec3{x:0.11, y:0.22, z:0.33};
    let test_vec2 = Vec3::new(0.12, 0.23, 0.34);

    let sub_vector = test_vec2 - test_vec;

    assert_approx_eq!(sub_vector.x(), 0.01);
    assert_approx_eq!(sub_vector.y(), 0.01);
    assert_approx_eq!(sub_vector.z(), 0.01);
}

#[test]
fn test_mul_vector(){

    let test_vec = Vec3{x:0.11, y:0.22, z:0.33};
    let test_vec2 = Vec3::new(2.0, 3.0, 4.0);

    let mul_vector = test_vec2 * test_vec;

    assert_approx_eq!(mul_vector.x(), 0.22);
    assert_approx_eq!(mul_vector.y(), 0.66);
    assert_approx_eq!(mul_vector.z(), 1.32);
}

#[test]
fn test_mul_vector_by_num(){
    let test_vec = Vec3{x:0.11, y:0.22, z:0.33};

    let mul_vector = test_vec * 2.0;

    assert_approx_eq!(mul_vector.x(), 0.22);
    assert_approx_eq!(mul_vector.y(), 0.44);
    assert_approx_eq!(mul_vector.z(), 0.66);

}

#[test]
fn test_div_vector(){

    let test_vec = Vec3{x:0.22, y:0.66, z:0.88};
    let test_vec2 = Vec3::new(1.0, 3.0, 4.0);

    let div_vector = test_vec / test_vec2;
    let div_vector_by_num = test_vec / 2.0;


    assert_approx_eq!(div_vector.x(), 0.22);
    assert_approx_eq!(div_vector.y(), 0.22);
    assert_approx_eq!(div_vector.z(), 0.22);
    assert_approx_eq!(div_vector_by_num.x(), 0.11);
    assert_approx_eq!(div_vector_by_num.y(), 0.33);
    assert_approx_eq!(div_vector_by_num.z(), 0.44);
}

#[test]
fn test_vector_length(){
    
    let test_vec = Vec3::new(1.0, -2.0, 3.0 );
    let test_vec2 = Vec3::new(1.0, -2.0, 3.0 );

    let test_vec_length = test_vec.length();
    let test_vec_length2 = test_vec2.length();


    assert_approx_eq!(test_vec_length, 3.31662479036, 1f64);
    assert_approx_eq!(test_vec_length, test_vec_length2);

    println!("Value of vector test length ----> {}", test_vec_length);
}

#[test]
fn test_vector_cross(){

    let test_vec = Vec3::new(2.0, 4.0, 6.0);
    let test_vec2 = Vec3::new(3.0, 2.0, 1.0);

    let coross_vector = test_vec.cross(&test_vec2);

    assert_approx_eq!(coross_vector.x(), -8.0);
    assert_approx_eq!(coross_vector.y(), 16.0);
    assert_approx_eq!(coross_vector.z(), -8.0);
}

#[test]
fn test_unit_vecotr(){

    let test_vec = Vec3::new(2.0, 4.0, 6.0);

    let unit_vector = test_vec.unit_vector();

    assert_approx_eq!(unit_vector.x(), 0.26726124191);
    //assert_approx_eq!(unit_vector.y(), 0.53452248382);
    //assert_approx_eq!(unit_vector.z(), 0.80178372573);

}

#[test]
fn test_new_ray(){
    let test_ray = Ray::new(
    Vec3::new(2.0, 4.0, 6.0),
    Vec3::new(1.0, 2.0, 3.0));

    assert_approx_eq!(test_ray.origin().x(), 2.0);
}

#[test]
fn test_ray_at(){
    let test_ray = Ray::new(
    Vec3::new(2.0, 4.0, 6.0),
    Vec3::new(1.0, 2.0, 3.0));

    let new_origin = test_ray.at(2.0);

    assert_approx_eq!(new_origin.x(), 4.0);
    assert_approx_eq!(new_origin.y(), 8.0);
    assert_approx_eq!(new_origin.z(), 12.0);
}