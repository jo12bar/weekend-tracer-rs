//! An isotropic scattering function, to be used with a volume.

use crate::{hittable::HitRecord, material::Scatter, ray::Ray, texture::Texture, vec3::Vec3};
use rand::prelude::*;

/// An isotropic scattering function. Rays have a chance of scattering, and will
/// scatter in a uniform random direction.
#[derive(Clone, Debug)]
pub struct Isotropic {
    albedo: Texture,
}

impl Isotropic {
    /// Create a new isotropic material. Mainly useful as a isotropic scattering
    /// function.
    pub fn new(albedo: Texture) -> Self {
        Self { albedo }
    }

    /// Scatter a ray randomly in a uniform direction.
    pub fn scatter<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        ray: &Ray,
        rec: &HitRecord,
    ) -> Option<Scatter> {
        let scattered = Ray::new(rec.hit_point, Vec3::random_in_unit_sphere(rng), ray.time);
        let attenutation = self.albedo.0(rec.uv, &rec.hit_point);
        Some(Scatter::new(attenutation, scattered))
    }
}
