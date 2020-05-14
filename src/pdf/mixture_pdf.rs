//! A helper struct for evenly "mixing" together two PDFs.

use crate::{pdf::PDF, vec3::Vec3};
use rand::prelude::*;

/// A helper struct for evenly "mixing" together two PDFs. The method
/// `MixturePDF::value()` will average out the calls to `value()` for the two
/// PDFs, the the method `MixturePDF::generate()` has a 50% chance of calling
/// the `generate()` method on each PDF.
#[derive(Debug, Clone)]
pub struct MixturePDF(pub Box<PDF>, pub Box<PDF>);

impl MixturePDF {
    /// Mix together the values of the two PDFs. Each contributes half its value
    /// to the final result.
    pub fn value(&self, direction: &Vec3) -> f32 {
        0.5 * self.0.value(direction) + 0.5 * self.1.value(direction)
    }

    /// Randomly generate a vector using either the first or the second PDF, with
    /// a 50% chance of picking one or the other.
    pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        if rng.gen::<f32>() < 0.5 {
            self.0.generate(rng)
        } else {
            self.1.generate(rng)
        }
    }
}
