//! A Lambertian diffuse material.

use crate::{
    hittable::HitRecord,
    material::{Scatter, ScatterType},
    pdf::PDF,
    ray::Ray,
    texture::Texture,
};
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
        _rng: &mut R,
        _ray_in: &Ray,
        rec: &HitRecord,
    ) -> Option<Scatter> {
        // Lambertian materials never scatter specular rays.
        let attenuation = self.albedo.0(rec.uv, &rec.hit_point);
        let scattered = ScatterType::PDF(PDF::cosine(rec.normal));

        Some(Scatter::new(attenuation, scattered))
    }

    pub fn scattering_pdf<R: Rng + ?Sized>(
        &self,
        _rng: &mut R,
        _ray_in: &Ray,
        rec: &HitRecord,
        scattered: &Ray,
    ) -> f32 {
        let cosine = rec.normal.dot(&scattered.direction.unit_vector());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f32::consts::PI
        }
    }
}
