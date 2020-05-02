use clap::{clap_app, crate_version};
use minifb::{Key, Window, WindowOptions};
use std::sync::Arc;

use weekend_tracer_rs::{
    camera::Camera,
    create_world,
    hittable::{sphere::Sphere, world::World},
    material::Material,
    renderer, vec3,
    vec3::Vec3,
};

const WIDTH: usize = 200;
const HEIGHT: usize = 100;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_REFLECTION_DEPTH: usize = 50;

const ASPECT_RATIO: f32 = (WIDTH as f32) / (HEIGHT as f32);

fn gui_output(world: World, camera: Camera) {
    let buffer = renderer::render_bgra(
        WIDTH,
        HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_REFLECTION_DEPTH,
        world,
        camera,
    );

    let mut window = Window::new(
        "weekend-tracer-rs - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn ppm_output(world: World, camera: Camera) {
    let buffer = renderer::render(
        WIDTH,
        HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_REFLECTION_DEPTH,
        world,
        camera,
    );

    print!("P3\n{} {}\n255\n", WIDTH, HEIGHT);

    for (r, g, b) in buffer.iter() {
        println!("{} {} {}", r, g, b);
    }
}

fn main() {
    let matches = clap_app!(weekend_tracer_rs =>
        (version: crate_version!())
        (author: "Johann M. Barnard <johann.b@telus.net>")
        (about: "A simple ray-tracing renderer. If no options are passed, the \
                 image is outputted in a ASCII PPM image format to \
                 stdout. This can be redirected to a .ppm file.")
        (@arg version: -v --version "Outputs version information.")
        (@arg gui: -g --gui "Render to a window instead of to stdout.")
    )
    .get_matches();

    let world = create_world!(
        // Middle diffuse sphere:
        Sphere::new(
            vec3!(0.0, 0.0, -1.0),
            0.5,
            Material::lambertian(vec3!(0.1, 0.2, 0.5))
        ),
        // Right metallic sphere:
        Sphere::new(
            vec3!(1.0, 0.0, -1.0),
            0.5,
            Material::metal(vec3!(0.8, 0.6, 0.2), 0.3)
        ),
        // Left hollow glass sphere:
        Sphere::new(vec3!(-1.0, 0.0, -1.0), 0.5, Material::dielectric(1.5)),
        Sphere::new(vec3!(-1.0, 0.0, -1.0), -0.45, Material::dielectric(1.5)),
        // Ground:
        Sphere::new(
            vec3!(0.0, -100.5, -1.0),
            100.0,
            Material::lambertian(vec3!(0.8, 0.8))
        ),
    );

    let camera = Camera::new(
        vec3!(-3.0, 3.0, 2.0),
        vec3!(0.0, 0.0, -1.0),
        vec3!(0.0, 1.0),
        20.0,
        ASPECT_RATIO,
        2.0,
        (vec3!(-3.0, 3.0, 2.0) - vec3!(0.0, 0.0, -1.0)).length(),
    );

    if matches.is_present("version") {
        println!("weekend-tracer-rs {}", crate_version!());
    } else if matches.is_present("gui") {
        gui_output(world, camera);
    } else {
        ppm_output(world, camera);
    }
}
