use crate::camera::Camera;
use crate::hittable::{world::World, Hittable};
use crate::ray::Ray;
use crate::util::clamp;
use crate::vec3;
use crate::vec3::Vec3;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

/// Linearly blends white and blue depending on the height of the passed-in
/// ray's y coordinate, *after* scaling the ray direction to unit length (so
/// -1.0 <= y <= 1.0).
fn ray_color<R: Rng + ?Sized>(
    rng: &mut R,
    ray: &Ray,
    world: &World,
    reflection_depth: usize,
) -> Vec3 {
    if reflection_depth == 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        vec3!()
    } else if let Some(hit_record) = world.hit(ray, 0.001, f32::INFINITY) {
        //                                          ^^^^^
        //                                            |
        // This `0.001` is so that we don't get weird "shadow acne" due to
        // floating-point errors.
        //
        // We hit something! Reflect the ray off in some random manner to create
        // a diffuse effect.
        let target = hit_record.hit_point + hit_record.normal + Vec3::random_in_unit_sphere(rng);
        0.5 * ray_color(
            rng,
            &Ray::new(hit_record.hit_point, target - hit_record.hit_point),
            world,
            reflection_depth - 1,
        )
    } else {
        // Didn't hit anything! Just render the sky by blending blue and white
        // linearly based on the y-direction of the ray.
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);

        // Linearly blend white and light blue.
        ((1.0 - t) * vec3!(1.0, 1.0, 1.0)) + (t * vec3!(0.5, 0.7, 1.0))
    }
}

/// Render the scene. Outputs a vector of `u32`'s, one for each pixel:
/// - The upper 8 bits is for the alpha channel.
/// - The next 8 bits is for the red channel.
/// - The next 8 bits is for the blue channel.
/// - The lowest 8 bits is for the green channel.
pub fn render_bgra(
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    max_reflection_depth: usize,
    world: World,
    camera: Camera,
) -> Vec<u32> {
    render(
        width,
        height,
        samples_per_pixel,
        max_reflection_depth,
        world,
        camera,
    )
    .into_iter()
    .map(|pixel| {
        let (r, g, b) = pixel;
        (255 << 24) | (r << 16) | (g << 8) | b
    })
    .collect()
}

/// Render the scene. Outputs a vector of (r, g, b) integer triples, one for
/// each pixel, which can range from 0 to 255.
#[allow(clippy::many_single_char_names)]
pub fn render(
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    max_reflection_depth: usize,
    world: World,
    camera: Camera,
) -> Vec<(u32, u32, u32)> {
    let pb_style = ProgressStyle::default_bar()
        .template("{spinner} {msg} [{elapsed_precise}] [{bar:30.yellow/blue}] {pos}/{len}")
        .progress_chars("=>-");

    let pb = ProgressBar::new((width * height) as u64);
    pb.set_style(pb_style);

    (0..(width * height))
        .into_par_iter()
        .progress_with(pb)
        .map_init(thread_rng, |rng, screen_pos| {
            let j = height - 1 - screen_pos / width;
            let i = screen_pos % width;

            // Take a whole bunch of samples within a pixel, and average out the
            // pixel's colour.
            let mut color = vec3!();
            for _ in 0..samples_per_pixel {
                // Each sample is offset by a small, random amount.
                let u = ((i as f32) + rng.gen::<f32>()) / (width as f32);
                let v = ((j as f32) + rng.gen::<f32>()) / (height as f32);

                let ray = camera.get_ray(u, v);
                color += ray_color(rng, &ray, &world, max_reflection_depth);
            }

            // Divide the color total by the number of samples and gamma-correct
            // for a gamma value of 2.0.
            let scale = 1.0 / (samples_per_pixel as f32);
            let r = (scale * color.x).sqrt();
            let g = (scale * color.y).sqrt();
            let b = (scale * color.z).sqrt();

            let ir = (256.0 * clamp(r, 0.0, 0.999)) as u32;
            let ig = (256.0 * clamp(g, 0.0, 0.999)) as u32;
            let ib = (256.0 * clamp(b, 0.0, 0.999)) as u32;

            (ir, ig, ib)
        })
        .collect()
}
