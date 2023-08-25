

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;
use super::*;

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

#[test]
fn test_hit(){
    let center = Vec3:: new(0.0, 0.0, 0.0);
    let sphere = Sphere::new(center, 1.0);

    let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
    let hit = sphere.hit(&ray, 0.0, f64::INFINITY);
    assert_approx_eq!(hit.unwrap().t, 4.0);
}
