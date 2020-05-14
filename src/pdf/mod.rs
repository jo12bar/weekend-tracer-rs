//! Probability Density Functions (PDFs), and their associated structs and methods.

pub mod cosine_pdf;

use crate::vec3::Vec3;
use rand::prelude::*;

/// A probability density function. Supports generating either floats or vectors.
#[derive(Clone, Copy, Debug)]
pub enum PDF {
    Cosine(cosine_pdf::CosinePDF),
}

impl PDF {
    /// Create a new cosine density PDF.
    pub fn cosine(w: Vec3) -> Self {
        Self::Cosine(cosine_pdf::CosinePDF::new(w))
    }

    /// Generate a float for some vector.
    pub fn value(&self, direction: &Vec3) -> f32 {
        match self {
            Self::Cosine(c) => c.value(direction),
        }
    }

    /// Generate a vector.
    pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        match self {
            Self::Cosine(c) => c.generate(rng),
        }
    }
}
