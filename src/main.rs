use rt::*;

mod render;


fn main() {
    let camera = rt::camera::Camera::new(400, 50);

    render::Render::render(camera.pixel00_loc, camera.pixel_vec_u, camera.pixel_vec_v,
         camera.camera_center, camera.image_width, camera.image_height, camera.max_depth)
}
