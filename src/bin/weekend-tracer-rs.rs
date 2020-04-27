use clap::{clap_app, crate_version};
use minifb::{Key, Window, WindowOptions};
use weekend_tracer_rs::renderer;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

fn gui_output() {
    let buffer = renderer::render_bgra(WIDTH, HEIGHT);

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

fn ppm_output() {
    let buffer = renderer::render(WIDTH, HEIGHT);

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

    if matches.is_present("version") {
        println!("weekend-tracer-rs {}", crate_version!());
    } else if matches.is_present("gui") {
        gui_output();
    } else {
        ppm_output();
    }
}
