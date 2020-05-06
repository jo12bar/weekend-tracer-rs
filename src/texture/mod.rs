//! Textures.
//!
//! A texture is just some function that makes the colors on a surface
//! procedural. So, rather than have a bunch of structs floating around, this
//! module mostly contains functions that return `Texture` functions.

pub mod constant;
pub use constant::constant;

pub mod checkerboard;
pub use checkerboard::checkerboard;

pub mod perlin;
pub use perlin::{perlin_noise, perlin_turbulence};

pub mod marble;
pub use marble::simple_marble;

pub mod image;
pub use crate::texture::image::image;

use crate::hittable::UVCoord;
use crate::vec3::Vec3;
use std::sync::Arc;

/// A texture function. Takes in (u, v) surface coordinates the a hit point,
/// and outputs the resulting colour of that point.
#[derive(Clone)]
pub struct Texture(pub Arc<dyn Fn(UVCoord, &Vec3) -> Vec3 + Send + Sync>);

/// Allows `Texture` to implement `Debug`.
impl std::fmt::Debug for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Texture")
            .field(&String::from(
                "Arc<dyn Fn(UVCoord, Vec3) -> Vec3 + Send + Sync>",
            ))
            .finish()
    }
}

/// Convert colours directly into a `constant` texture.
impl From<Vec3> for Texture {
    #[inline]
    fn from(v: Vec3) -> Self {
        constant(v)
    }
}
