use clap::{clap_app, crate_version};

#[cfg(feature = "gui-support")]
use minifb::{Key, Window, WindowOptions};

use weekend_tracer_rs::{camera::Camera, hittable::world::World, renderer, vec3, vec3::Vec3};

const WIDTH: usize = 200;
const HEIGHT: usize = 100;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_REFLECTION_DEPTH: usize = 50;

const ASPECT_RATIO: f32 = (WIDTH as f32) / (HEIGHT as f32);

/// Render to a simple cross-platform window using the `minifb` crate.
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

/// Render to an ASCII PPM `.ppm` file.
fn ppm_output(filename: &str, world: World, camera: Camera) -> std::io::Result<()> {
    let output = renderer::render(
        WIDTH,
        HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_REFLECTION_DEPTH,
        world,
        camera,
    )
    .into_iter()
    .map(|(r, g, b)| format!("{} {} {}", r, g, b))
    .fold(format!("P3\n{} {}\n255\n", WIDTH, HEIGHT), |s, pixel| {
        s + &pixel + "\n"
    });

    std::fs::write(filename, output)
}

/// Render to some arbritrary image file type. Whatever the `image` crate
/// supports.
fn image_output(filename: &str, world: World, camera: Camera) {
    let rendered = renderer::render(
        WIDTH,
        HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_REFLECTION_DEPTH,
        world,
        camera,
    )
    .into_iter()
    .map(|(r, g, b)| vec![r as u8, g as u8, b as u8])
    .flatten()
    .collect::<Vec<_>>();

    image::save_buffer(
        filename,
        &rendered[..],
        WIDTH as u32,
        HEIGHT as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
}

fn main() {
    #[allow(unused_mut)]
    let mut app = clap_app!(weekend_tracer_rs =>
        (version: crate_version!())
        (author: "Johann M. Barnard <johann.b@telus.net>")
        (about: "A simple ray-tracing renderer. If no options are passed, the \
                 image is outputted in some image format to \
                 <OUTPUT_FILE> (e.g. image.png, image.JPEG, etc...). The output \
                 format is deduced from the file name extension. JPEG, PNG, \
                 GIF, BMP, TIFF, ICO, and PPM (the binary version) formats are \
                 supported.")
        (@group image_output +multiple =>
            (@arg OUTPUT_FILE: required_unless[gui] "The file to be outputted to.")
            (@arg ppm: -p --ppm "Output to an ASCII PPM file (e.g. test.ppm, image.ppm, etc...).")
        )
    );

    #[cfg(feature = "gui-support")]
    {
        app = app.arg(
            clap::Arg::with_name("gui")
                .short("g")
                .long("gui")
                .help("Render to a window instead of to stdout.")
                .conflicts_with("image_output"),
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
        0.0,
        0.0,
    );

    if matches.is_present("gui") {
        #[cfg(feature = "gui-support")]
        gui_output(world, camera);
    } else {
        // Calling .unwrap() is safe here because we require that the OUTPUT_FILE
        // is present if --gui/-g is not present.
        let output_file = matches.value_of("OUTPUT_FILE").unwrap();

        if matches.is_present("ppm") {
            ppm_output(output_file, world, camera).unwrap();
        } else {
            image_output(output_file, world, camera);
        }
    }
}
