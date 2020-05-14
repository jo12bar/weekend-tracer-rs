//! Probability Density Functions (PDFs), and their associated structs and methods.

pub mod cosine_pdf;
pub mod hittable_pdf;
pub mod mixture_pdf;

use crate::{hittable::Hittable, vec3::Vec3};
use rand::prelude::*;
use std::sync::Arc;

/// A probability density function. Supports generating either floats or vectors.
#[derive(Clone, Debug)]
pub enum PDF {
    Cosine(cosine_pdf::CosinePDF),
    Hittable(hittable_pdf::HittablePDF),
    Mixture(mixture_pdf::MixturePDF),
}

impl PDF {
    /// Create a new cosine density PDF.
    pub fn cosine(w: Vec3) -> Self {
        Self::Cosine(cosine_pdf::CosinePDF::new(w))
    }

    /// Create a new PDF for some `Hittable` object.
    pub fn hittable(obj: Arc<dyn Hittable>, origin: Vec3) -> Self {
        Self::Hittable(hittable_pdf::HittablePDF::new(obj, origin))
    }

    /// Mix together two PDFs with a 50/50 split.
    pub fn mixture(pdf1: &Self, pdf2: &Self) -> Self {
        Self::Mixture(mixture_pdf::MixturePDF(pdf1.box_clone(), pdf2.box_clone()))
    }

    /// Generate a float for some vector.
    /// Calls the `value()` method on the underlying PDF struct.
    pub fn value(&self, direction: &Vec3) -> f32 {
        match self {
            Self::Cosine(c) => c.value(direction),
            Self::Hittable(h) => h.value(direction),
            Self::Mixture(m) => m.value(direction),
        }
    }

    /// Generate a vector.
    /// Calls the `generate()` method on the underlying PDF struct.
    pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        match self {
            Self::Cosine(c) => c.generate(rng),
            Self::Hittable(h) => h.generate(rng),
            Self::Mixture(m) => m.generate(rng),
        }
    }

    /// Clone the PDF into a `Box<PDF>`.
    pub fn box_clone(&self) -> Box<PDF> {
        Box::new(self.clone())
    }
}
