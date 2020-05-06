use crate::{texture::Texture, vec3::Axis::*};
use std::sync::Arc;

/// A alternating checkerboard pattern between two other textures.
pub fn checkerboard(odd: Texture, even: Texture) -> Texture {
    Texture(Arc::new(move |uv_coords, hit_point| {
        let sines =
            (10.0 * hit_point[X]).sin() * (10.0 * hit_point[Y]).sin() * (10.0 * hit_point[Z]).sin();

        if sines < 0.0 {
            odd.0(uv_coords, hit_point)
        } else {
            even.0(uv_coords, hit_point)
        }
    }))
}
