use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Vec3;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;

/// For a `Ray`, **p&#x20D7;**(t) = **a** + **b&#x20D7;**t (where **a** is the
/// origin and **b&#x20D7;** is the direction, a vector), this function returns
/// the value t at which the `Ray` hits a sphere with some radius and a center
/// at some point.
fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> Option<f32> {
    // See the raytracing in one weekend book, chapter 5, for this formula.
    // We found a quadratic formula for hit-testing a sphere.
    let oc = ray.origin - *center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - (radius * radius);
    let discriminant = (half_b * half_b) - (a * c);

    // If the discriminant is bigger than 0, then the ray hit the sphere.
    if discriminant < 0.0 {
        None
    } else {
        Some(-half_b - discriminant.sqrt() / a)
    }
}

/// Linearly blends white and blue depending on the height of the passed-in
/// ray's y coordinate, *after* scaling the ray direction to unit length (so
/// -1.0 <= y <= 1.0).
fn ray_color(ray: &Ray) -> Vec3 {
    if let Some(t) = hit_sphere(&vec3!(0.0, 0.0, -1.0), 0.5, ray) {
        // `norm` is the normal vector to the point that got hit.
        let norm = Vec3::unit_vector(&(ray.at(t) - vec3!(0.0, 0.0, -1.0)));

        // Map x/y/z of the normal vector to r/g/b
        0.5 * vec3!(norm.x + 1.0, norm.y + 1.0, norm.z + 1.0)
    } else {
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
pub fn render_bgra(width: usize, height: usize) -> Vec<u32> {
    render(width, height)
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
pub fn render(width: usize, height: usize) -> Vec<(u32, u32, u32)> {
    let pb_style = ProgressStyle::default_bar()
        .template("{spinner} {msg} [{elapsed_precise}] [{bar:30.yellow/blue}] {pos}/{len}")
        .progress_chars("=>-");

    let pb = ProgressBar::new((width * height) as u64);
    pb.set_style(pb_style);

    // Some reference vectors
    let lower_left_corner = vec3!(-2.0, -1.0, -1.0);
    let horizontal = vec3!(4.0, 0.0, 0.0);
    let vertical = vec3!(0.0, 2.0, 0.0);
    let origin = vec3!(0.0, 0.0, 0.0);

    (0..(width * height))
        .into_par_iter()
        .progress_with(pb)
        .map(|screen_pos| {
            let j = height - 1 - screen_pos / width;
            let i = screen_pos % width;

            let u = (i as f32) / (width as f32);
            let v = (j as f32) / (height as f32);

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let color = ray_color(&ray);

            let ir = (255.999 * color.x) as u32;
            let ig = (255.999 * color.y) as u32;
            let ib = (255.999 * color.z) as u32;

            (ir, ig, ib)
        })
        .collect()
}
