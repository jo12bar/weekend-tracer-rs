use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Vec3;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;

/// Linearly blends white and blue depending on the height of the passed-in
/// ray's y coordinate, *after* scaling the ray direction to unit length (so
/// -1.0 <= y <= 1.0).
fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    // Linearly blend white and light blue.
    ((1.0 - t) * vec3!(1.0, 1.0, 1.0)) + (t * vec3!(0.5, 0.7, 1.0))
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
