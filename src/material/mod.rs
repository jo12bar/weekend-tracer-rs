//! Materials. Allows for easy interchangibility between material types on
//! different objects.

pub mod lambertian;

use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};
use rand::Rng;

/// A scattered ray and its attenuation.
#[derive(Copy, Clone, Debug)]
pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

impl Scatter {
    pub fn new(attenuation: Vec3, scattered: Ray) -> Self {
        Self {
            attenuation,
            scattered,
        }
    }
}

/// A material. Each material will scatter incident light (and `Ray`'s) in
/// different ways.
#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian(lambertian::Lambertian),
}

impl Material {
    /// Create a new lambertian material.
    pub fn lambertian(albedo: Vec3) -> Material {
        Material::Lambertian(lambertian::Lambertian::new(albedo))
    }

    /// Scatter a ray off a material. Will delegate to the material's
    /// implementation of `scatter()`. Returns `Some(Scatter)` if the ray is
    /// scattered, `None` if it isn't.
    pub fn scatter<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        ray: &Ray,
        rec: &HitRecord,
    ) -> Option<Scatter> {
        match rec.material {
            Material::Lambertian(l) => l.scatter(rng, ray, rec),
        }
    }
}