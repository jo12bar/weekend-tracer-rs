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
        let target = rec.normal + Vec3::random_unit_vector(rng);
        let scattered = Ray::new(rec.hit_point, target.unit_vector(), ray_in.time);
        let albedo = self.albedo.0(rec.uv, &rec.hit_point);
        // let pdf = rec.normal.dot(&scattered.direction) / std::f32::consts::PI;
        let pdf = 0.5 / std::f32::consts::PI;

        Some(Scatter::new_with_pdf(albedo, scattered, pdf))
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
