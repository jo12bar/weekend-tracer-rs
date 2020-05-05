//! A Lambertian diffuse material.

use crate::{hittable::HitRecord, material::Scatter, ray::Ray, texture::Texture, vec3::Vec3};
use rand::Rng;

/// A Lambertian diffuse material. Attenuation is adjustable via the `abedo`
/// property.
#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Texture,
}

impl Lambertian {
    pub fn new(albedo: Texture) -> Self {
        Self { albedo }
    }

    pub fn scatter<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        ray_in: &Ray,
        rec: &HitRecord,
    ) -> Option<Scatter> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector(rng);
        let scattered = Ray::new(rec.hit_point, scatter_direction, ray_in.time);
        let attenuation = self.albedo.0(rec.uv, &rec.hit_point);

        Some(Scatter::new(attenuation, scattered))
    }
}
