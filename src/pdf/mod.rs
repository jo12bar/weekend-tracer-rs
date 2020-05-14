//! Probability Density Functions (PDFs), and their associated structs and methods.

pub mod cosine_pdf;
pub mod hittable_pdf;

use crate::{hittable::Hittable, vec3::Vec3};
use rand::prelude::*;

/// A probability density function. Supports generating either floats or vectors.
#[derive(Clone, Debug)]
pub enum PDF {
    Cosine(cosine_pdf::CosinePDF),
    Hittable(hittable_pdf::HittablePDF),
}

impl PDF {
    /// Create a new cosine density PDF.
    pub fn cosine(w: Vec3) -> Self {
        Self::Cosine(cosine_pdf::CosinePDF::new(w))
    }

    /// Create a new PDF for some `Hittable` object.
    pub fn hittable(obj: Box<dyn Hittable>, origin: Vec3) -> Self {
        Self::Hittable(hittable_pdf::HittablePDF::new(obj, origin))
    }

    /// Generate a float for some vector.
    pub fn value(&self, direction: &Vec3) -> f32 {
        match self {
            Self::Cosine(c) => c.value(direction),
            Self::Hittable(h) => h.value(direction),
        }
    }

    /// Generate a vector.
    pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        match self {
            Self::Cosine(c) => c.generate(rng),
            Self::Hittable(h) => h.generate(rng),
        }
    }
}
