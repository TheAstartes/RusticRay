# RusticRay 
Simple Ray Tracer [1] - Adaptation of fun and engaging project from scratch in one of my recently favorite programming languages (Rust). 

## Render example - output
![Render_example](https://github.com/TheAstartes/RusticRay/assets/46090816/9f48d90a-6eea-40d8-8084-100619e09bac)
## Features
- Vector utilities
- Surface normals shading
- Antialiasing
- Material properties for objects: Metal, Dielectrics, Diffuse
## Installation
I have used Rust Toolchain (compilation target + release channel) to create this project, recommended way is to install it via rustup. Instructions can be found under: https://rustup.rs/
## Usage
After installation, we have to go into our project directory.

### Running project
```
cargo run > name.pnn
```
### Resolution
inside of **main.rs** we change value of resolution. In reality we are using **width** value to calculate aspect ratio of 16:9 because it's very common to do so. An image with 800 pixels wide by 400 pixels high has aspect ratio of 2:1.
```
let camera = rt::camera::Camera::new(width, 50);
```
### Gamma correction
Picture may be very dark or too bright. Pixel values are stored in linear space, they are not transformed. Image viewers are not taking this into account thus creating some inaccuracies, it's best to experiment with **gamma** vlaue inside of file **intervals.rs** in **sample** function
```
gamma_value * self.clamp(new_color)
```
Usually, the higher the resolution the higher the gamma value should be. For example a value of **6.5** works well with **width** value of 1920. In future it would be best to calculate optimal value automatically (though I like picking my value by hand).
## Sources
[1] Peter Shirley, Trevor David Black, Steve Hollasch "Ray Tracing in One Weekend" Version 4.0.0-alpha.1, 2023-08-06 https: https://raytracing.github.io/books/RayTracingInOneWeekend.html#surfacenormalsandmultipleobjects/shadingwithsurfacenormals
