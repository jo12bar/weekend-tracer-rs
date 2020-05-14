//! A PDF that samples directions towards a `Hittable`, like a light (for example).

use crate::{hittable::Hittable, vec3::Vec3};
use rand::prelude::*;

/// A PDF that will bias vectors to point towards some `Hittable` object.
#[derive(Debug, Clone)]
pub struct HittablePDF {
    pub origin: Vec3,
    pub obj: Box<dyn Hittable>,
}

impl HittablePDF {
    /// Create a new `HittablePDF`.
    pub fn new(obj: Box<dyn Hittable>, origin: Vec3) -> Self {
        Self { obj, origin }
    }

    pub fn value(&self, direction: &Vec3) -> f32 {
        self.obj.pdf_value(&self.origin, direction)
    }

    pub fn generate<R: Rng + ?Sized>(&self, _rng: &mut R) -> Vec3 {
        self.obj.random(&self.origin)
    }
}
