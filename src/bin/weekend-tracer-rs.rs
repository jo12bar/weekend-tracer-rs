use clap::{clap_app, crate_version};

#[cfg(feature = "gui-support")]
use minifb::{Key, Window, WindowOptions};

use weekend_tracer_rs::{camera::Camera, hittable::world::World, renderer, vec3, vec3::Vec3};

const WIDTH: usize = 200;
const HEIGHT: usize = 100;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_REFLECTION_DEPTH: usize = 50;

const ASPECT_RATIO: f32 = (WIDTH as f32) / (HEIGHT as f32);

#[cfg(feature = "gui-support")]
fn gui_output(world: World, camera: Camera) {
    let buffer: Vec<u32> = renderer::convert_to_argb(renderer::render(
        WIDTH,
        HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_REFLECTION_DEPTH,
        world,
        camera,
    ))
    .collect();

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
    #[allow(unused_mut)]
    let mut app = clap_app!(weekend_tracer_rs =>
        (version: crate_version!())
        (author: "Johann M. Barnard <johann.b@telus.net>")
        (about: "A simple ray-tracing renderer. If no options are passed, the \
                 image is outputted in a ASCII PPM image format to \
                 stdout. This can be redirected to a .ppm file.")
        (@arg version: -v --version "Outputs version information.")
    );

    #[cfg(feature = "gui-support")]
    {
        app = app.arg(
            clap::Arg::with_name("gui")
                .short("g")
                .long("gui")
                .help("Render to a window instead of to stdout."),
        );
    }

    let matches = app.get_matches();

    let world = World::random_scene(&mut rand::thread_rng());

    let lookfrom = vec3!(13.0, 2.0, 3.0);
    let lookat = vec3!(0.0, 0.0, 0.0);
    let vup = vec3!(0.0, 1.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    if matches.is_present("version") {
        println!("weekend-tracer-rs {}", crate_version!());
    } else if matches.is_present("gui") {
        #[cfg(feature = "gui-support")]
        gui_output(world, camera);
    } else {
        ppm_output(world, camera);
    }
}
