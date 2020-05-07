//! A light-emitting material.
use crate::{
    hittable::{HitRecord, UVCoord},
    material::Scatter,
    ray::Ray,
    texture::Texture,
    vec3::Vec3,
};
use rand::prelude::*;

/// A light-emitting material. Can hold any texture. Will not reflect rays.
#[derive(Debug, Clone)]
pub struct DiffuseLight {
    /// The emitting texture.
    pub emit: Texture,
}

impl DiffuseLight {
    /// Create a new diffuse light.
    pub fn new(emit: Texture) -> Self {
        Self { emit }
    }

    pub fn scatter<R: Rng + ?Sized>(
        &self,
        _rng: &mut R,
        _ray: &Ray,
        _rec: &HitRecord,
    ) -> Option<Scatter> {
        None
    }

    pub fn emitted(&self, uv_coord: UVCoord, point: &Vec3) -> Vec3 {
        self.emit.0(uv_coord, point)
    }
}
