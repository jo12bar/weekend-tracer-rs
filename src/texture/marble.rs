use crate::{
    texture::{perlin::turbulence, Texture},
    vec3::{Axis::*, Vec3},
};
use std::sync::Arc;

/// A texture with a simple marble-like effect, using perlin noise and turbulence.
pub fn simple_marble(scale: f32) -> Texture {
    Texture(Arc::new(move |_uv_coords, hit_point| {
        Vec3::from(1.0)
            * 0.5
            * (1.0 + (scale * hit_point[Z] + 10.0 * turbulence(hit_point, None)).sin())
    }))
}
