//! Textures and methods for working with images.

use crate::{texture::Texture, util::clamp, vec3, vec3::Vec3};
use image as i;
use std::{path::Path, sync::Arc};

/// Renders an image as a texture.
pub fn image<P>(path: P) -> Texture
where
    P: AsRef<Path>,
{
    let img = i::open(path)
        .unwrap_or_else(|e| panic!("Could not open image for texture!\n{}", e))
        .into_rgb();

    let (width, height) = img.dimensions();

    Texture(Arc::new(move |(u, v), _p| {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0); // Flip v to image coordinates!

        let mut i = (u * width as f32) as u32;
        let mut j = (v * height as f32) as u32;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= width {
            i = width - 1;
        }
        if j >= height {
            j = height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pixel = img.get_pixel(i, j);

        color_scale * vec3!(pixel.0[0].into(), pixel.0[1].into(), pixel.0[2].into())
    }))
}
