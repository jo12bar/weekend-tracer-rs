use clap::{clap_app, crate_version};
use std::sync::Arc;

#[cfg(feature = "gui-support")]
use minifb::{Key, Window, WindowOptions};

use weekend_tracer_rs::{
    bvh::BVH, camera::Camera, hittable::Hittable, renderer, scenes, vec3, vec3::Vec3,
};

// Some defaults
#[cfg(debug_assertions)]
const WIDTH: usize = 100;
#[cfg(not(debug_assertions))]
const WIDTH: usize = 400;
#[cfg(debug_assertions)]
const HEIGHT: usize = 100;
#[cfg(not(debug_assertions))]
const HEIGHT: usize = 400;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_REFLECTION_DEPTH: usize = 50;

const BACKGROUND_COLOR: Vec3 = vec3!();

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
            (@arg OUTPUT_FILE: required_unless[gui compute_pi compute_int_x_squared] "The file to be outputted to.")
            (@arg ppm: -p --ppm "Output to an ASCII PPM file (e.g. test.ppm, image.ppm, etc...).")
        )
        (@arg dimensions: -d --dimensions <WIDTH> <HEIGHT> !required "Set the dimensions for the render. 300x300 by default.")
        (@arg samples: -s --samples <SAMPLES_PER_PIXEL> !required "Sets the number of samples to be taken per pixel.")
        (@arg reflections: -r --max_reflection_depth <DEPTH> !required "Sets the maximum reflection depth.")
        (@arg compute_pi: --compute_pi conflicts_with[dimensions samples reflections image_output gui compute_int_x_squared] "Computes pi (because why not?).")
        (@arg compute_int_x_squared: --compute_int_x_squared conflicts_with[dimensions samples reflections image_output gui compute_pi] "Computes the integral of x^2 between x=0 and x=2.")
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

    if matches.is_present("compute_pi") {
        compute_pi();
    }

    if matches.is_present("compute_int_x_squared") {
        compute_int_x_squared();
    }

    let dimensions = if let Some(v) = matches.values_of("dimensions") {
        v.map(str::parse::<usize>)
            .map(|x| {
                x.unwrap_or_else(|e| {
                    panic!(
                        "<WIDTH> or <HEIGHT> could not be parsed into a positive integer!\n{}",
                        e
                    )
                })
            })
            .collect::<Vec<_>>()
    } else {
        vec![WIDTH, HEIGHT]
    };

    let width = dimensions[0];
    let height = dimensions[1];
    let aspect_ratio = (width as f32) / (height as f32);

    let samples_per_pixel = matches
        .value_of("samples")
        .unwrap_or(&SAMPLES_PER_PIXEL.to_string())
        .parse::<usize>()
        .unwrap_or_else(|e| {
            panic!(
                "Could not parse <SAMPLES_PER_PIXEL> into a positive integer!\n{}",
                e
            )
        });

    let max_reflection_depth = matches
        .value_of("reflections")
        .unwrap_or(&MAX_REFLECTION_DEPTH.to_string())
        .parse::<usize>()
        .unwrap_or_else(|e| panic!("Could not parse <DEPTH> into a positive integer!\n{}", e));

    let (world, lights, camera) = scenes::cornell_box(aspect_ratio);
    let lights = Arc::new(lights);
    let bvh = BVH::new(world.objects, 0.0, 1.0);

    // let lookfrom = vec3!(478.0, 278.0, -600.0);
    // let lookat = vec3!(278.0, 278.0, 0.0);
    // let vup = vec3!(0.0, 1.0);
    // let dist_to_focus = (vec3!(260.0, 150.0, 45.0) - lookfrom).length();
    // let aperture = 0.5;
    // let vfov = 40.0; // degrees

    // let camera = Camera::new(
    //     lookfrom,
    //     lookat,
    //     vup,
    //     vfov,
    //     aspect_ratio,
    //     aperture,
    //     dist_to_focus,
    //     0.0,
    //     1.0,
    // );

    if matches.is_present("gui") {
        #[cfg(feature = "gui-support")]
        gui_output(
            bvh,
            lights,
            camera,
            width,
            height,
            samples_per_pixel,
            max_reflection_depth,
        );
    } else {
        // Calling .unwrap() is safe here because we require that the OUTPUT_FILE
        // is present if --gui/-g is not present.
        let output_file = matches.value_of("OUTPUT_FILE").unwrap();

        if matches.is_present("ppm") {
            ppm_output(
                output_file,
                bvh,
                lights,
                camera,
                width,
                height,
                samples_per_pixel,
                max_reflection_depth,
            )
            .unwrap();
        } else {
            image_output(
                output_file,
                bvh,
                lights,
                camera,
                width,
                height,
                samples_per_pixel,
                max_reflection_depth,
            );
        }
    }
}

/// Render to a simple cross-platform window using the `minifb` crate.
#[cfg(feature = "gui-support")]
fn gui_output(
    bvh: BVH,
    lights: Arc<dyn Hittable>,
    camera: Camera,
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    max_reflection_depth: usize,
) {
    let buffer: Vec<u32> = renderer::convert_to_argb(renderer::render(
        width,
        height,
        samples_per_pixel,
        max_reflection_depth,
        bvh,
        lights,
        camera,
        BACKGROUND_COLOR,
    ))
    .collect();

    let mut window = Window::new(
        "weekend-tracer-rs - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

/// Render to an ASCII PPM `.ppm` file.
#[allow(clippy::too_many_arguments)]
fn ppm_output(
    filename: &str,
    bvh: BVH,
    lights: Arc<dyn Hittable>,
    camera: Camera,
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    max_reflection_depth: usize,
) -> std::io::Result<()> {
    let output = renderer::render(
        width,
        height,
        samples_per_pixel,
        max_reflection_depth,
        bvh,
        lights,
        camera,
        BACKGROUND_COLOR,
    )
    .into_iter()
    .map(|(r, g, b)| format!("{} {} {}", r, g, b))
    .fold(format!("P3\n{} {}\n255\n", width, height), |s, pixel| {
        s + &pixel + "\n"
    });

    std::fs::write(filename, output)
}

/// Render to some arbritrary image file type. Whatever the `image` crate
/// supports.
#[allow(clippy::too_many_arguments)]
fn image_output(
    filename: &str,
    bvh: BVH,
    lights: Arc<dyn Hittable>,
    camera: Camera,
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    max_reflection_depth: usize,
) {
    let rendered = renderer::render(
        width,
        height,
        samples_per_pixel,
        max_reflection_depth,
        bvh,
        lights,
        camera,
        BACKGROUND_COLOR,
    )
    .into_iter()
    .map(|(r, g, b)| vec![r as u8, g as u8, b as u8])
    .flatten()
    .collect::<Vec<_>>();

    image::save_buffer(
        filename,
        &rendered[..],
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
}

/// Compute pi (for reasons)
fn compute_pi() -> ! {
    use rand::prelude::*;

    let mut rng = thread_rng();
    let mut inside_circle: u64 = 0;
    let mut inside_circle_stratified: u64 = 0;
    let sqrt_n = 10_000;

    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            let mut x: f64 = rng.gen_range(-1.0, 1.0);
            let mut y: f64 = rng.gen_range(-1.0, 1.0);

            if x * x + y * y < 1.0 {
                inside_circle += 1;
            }

            x = 2.0 * ((i as f64 + rng.gen::<f64>()) / sqrt_n as f64) - 1.0;
            y = 2.0 * ((j as f64 + rng.gen::<f64>()) / sqrt_n as f64) - 1.0;

            if x * x + y * y < 1.0 {
                inside_circle_stratified += 1;
            }
        }
    }

    let n = (sqrt_n * sqrt_n) as f64;

    println!(
        "\
        Regular estimate of Pi    = {:.12}\n\
        Stratified estimate of Pi = {:.12}",
        4.0 * (inside_circle as f64) / n,
        4.0 * (inside_circle_stratified as f64) / n,
    );

    std::process::exit(0)
}

/// Computes the integral of x^2 between x=0.0 and x=2.0.
fn compute_int_x_squared() -> ! {
    use rand::prelude::*;

    let mut rng = thread_rng();

    #[inline]
    fn pdf(_x: f64) -> f64 {
        0.5
    }

    let n = 1_000_000;
    let mut sum = 0.0;

    for _ in 0..n {
        let x = rng.gen_range(0.0_f64, 2.0_f64).sqrt();
        sum += x * x / pdf(x);
    }

    println!(
        "    2
I = ∫ x^2 dx ≈ {:.12}
    0",
        sum / (n as f64),
    );

    std::process::exit(0)
}
