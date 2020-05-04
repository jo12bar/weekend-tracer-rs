//! Textures.
//!
//! A texture is just some function that makes the colors on a surface
//! procedural. So, rather than have a bunch of structs floating around, this
//! module mostly contains functions that return `Texture` functions.

pub mod constant;

use crate::hittable::UVCoord;
use crate::vec3::Vec3;

/// All textures should implement this trait for the sake of type-checking.
pub trait TextureTrait {
    /// Get the colour of a texture at surface coordinates (u, v) and point `point`.
    fn value(&self, uv: UVCoord, point: &Vec3) -> Vec3;
}

#[derive(Debug, Clone, Copy)]
pub enum Texture {
    Constant(constant::Constant),
}

impl Texture {
    /// Create a constant-colour texture.
    pub fn constant(color: Vec3) -> Self {
        Self::Constant(constant::Constant::new(color))
    }

    /// Get the value of the texture at surface coordinate (u, v) and point `point`.
    pub fn value(&self, uv: UVCoord, point: &Vec3) -> Vec3 {
        match self {
            Texture::Constant(c) => c.value(uv, point),
        }
    }
}

/// For Vec3's, just turn it directly into a constant-colour texture.
impl From<Vec3> for Texture {
    fn from(v: Vec3) -> Self {
        Self::constant(v)
    }
}
