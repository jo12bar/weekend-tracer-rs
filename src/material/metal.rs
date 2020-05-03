//! A metallic, reflective material.

use crate::{hittable::HitRecord, material::Scatter, ray::Ray, vec3::Vec3};
use rand::Rng;

/// A basic, metallic, reflective material. Attenuation can be changed by
/// modifying the albedo property.
#[derive(Copy, Clone, Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    pub fn scatter<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        ray_in: &Ray,
        rec: &HitRecord,
    ) -> Option<Scatter> {
        let reflected = ray_in.direction.unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(
            rec.hit_point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(rng),
            ray_in.time,
        );
        let attenuation = self.albedo;

        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some(Scatter::new(attenuation, scattered))
        } else {
            None
        }
    }
}
