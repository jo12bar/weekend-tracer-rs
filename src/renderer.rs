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
pub fn render(width: usize, height: usize) -> Vec<(u32, u32, u32)> {
    (0..(width * height))
        .map(|screen_pos| {
            let j = height - 1 - screen_pos / width;
            let i = screen_pos % width;

            let r = (i as f64) / (width as f64);
            let g = (j as f64) / (height as f64);
            let b = 0.2;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            (ir, ig, ib)
        })
        .collect()
}
