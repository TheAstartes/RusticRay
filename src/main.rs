use rt::*;

mod render;


fn main() {
    let camera = rt::camera::Camera::new(400, 50);

    render::Render::render(camera);
}
