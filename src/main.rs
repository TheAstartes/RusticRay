use rt::{
    Camera,
    Render,
};

fn main() {


    let camera = Camera::new(400);

    Render::render(camera.pixel00_loc, camera.pixel_vec_u, camera.pixel_vec_v,
         camera.camera_center, camera.image_width, camera.image_height)


}
