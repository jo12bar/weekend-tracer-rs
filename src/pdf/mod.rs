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

/// A utility function for getting a special PDF-ready vector on a sphere.
pub fn random_to_sphere<R: Rng + ?Sized>(rng: &mut R, radius: f32, distance_squared: f32) -> Vec3 {
    let r1: f32 = rng.gen();
    let r2: f32 = rng.gen();

    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * std::f32::consts::PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}
