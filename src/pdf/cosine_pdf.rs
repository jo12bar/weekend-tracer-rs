//! A cosine density PDF.

use crate::{onb::ONB, vec3::Vec3};
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct CosinePDF {
    pub uvw: ONB,
}

impl CosinePDF {
    /// Create a new cosine density PDF.
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: ONB::build_from_w(w),
        }
    }

    pub fn value(&self, direction: &Vec3) -> f32 {
        let cosine = direction.unit_vector().dot(&self.uvw.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / std::f32::consts::PI
        }
    }

    pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        self.uvw.local(&Vec3::random_cosine_direction(rng))
    }
}
