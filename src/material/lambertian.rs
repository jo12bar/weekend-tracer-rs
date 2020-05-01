//! A Lambertian diffuse material.

use crate::{hittable::HitRecord, material::Scatter, ray::Ray, vec3::Vec3};
use rand::Rng;

/// A Lambertian diffuse material. Attenuation is adjustable via the `abedo`
/// property.
#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    pub fn scatter<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        _ray_in: &Ray,
        rec: &HitRecord,
    ) -> Option<Scatter> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector(rng);
        let scattered = Ray::new(rec.hit_point, scatter_direction);
        let attenuation = self.albedo;

        Some(Scatter::new(attenuation, scattered))
    }
}
