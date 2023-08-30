use std::ops::{Add, Mul, Sub, Div, Neg};
use rand::Rng;


#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
    

    // fn random_vec3() -> Vec3 {
    //     let mut random = rand::thread_rng();

    //     Vec3 { 
    //         x: random.gen::<f64>(),
    //         y: random.gen::<f64>(),
    //         z: random.gen::<f64>() 
    //     }
    // }

    fn random_vec3_minmax(min: f64, max: f64) -> Vec3 {
        let mut random = rand::thread_rng();

        Vec3 { 
            x: random.gen_range(min..max),
            y: random.gen_range(min..max),
            z: random.gen_range(min..max),
        }
    }

    fn random_vec3_unit_sphere() -> Vec3 {
        loop  {
            let vector = Vec3::random_vec3_minmax(-1.0, 1.0);

            if vector.length_squared() < 1.0 {
                return vector
            }
        }
    }

    pub fn random_unit_vec3() -> Vec3 {
        Vec3::random_vec3_unit_sphere().unit_vector()
    }

    pub fn random_vec3_on_hemisphere(normal: &Vec3) -> Vec3 {

        let unit_sphere_vec3 = Vec3::random_unit_vec3();

        //Check if unit vector is in the same hempisphere as sphere normal
        if unit_sphere_vec3.dot(normal) > 0.0 {
            unit_sphere_vec3
        }else {
            unit_sphere_vec3 *  -1.0
        }
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
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
            z: self.z + num,
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
