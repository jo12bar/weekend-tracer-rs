use crate::vec3;
use crate::vec3::Vec3;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;

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

    (0..(width * height))
        .into_par_iter()
        .progress_with(pb)
        .map(|screen_pos| {
            let j = height - 1 - screen_pos / width;
            let i = screen_pos % width;

            let color = vec3!(
                (i as f32) / (width as f32),
                (j as f32) / (height as f32),
                0.2
            );

            let ir = (255.999 * color.x) as u32;
            let ig = (255.999 * color.y) as u32;
            let ib = (255.999 * color.z) as u32;

            (ir, ig, ib)
        })
        .collect()
}
