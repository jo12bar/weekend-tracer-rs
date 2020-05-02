//! A dielectric material that refracts/ reflects rays through it.

use crate::{hittable::HitRecord, material::Scatter, ray::Ray, vec3, vec3::Vec3};
use rand::Rng;

/// A dielectric material. Has some refraction index. Will refract or reflect
/// rays, based on Snell's law and the incident ray.
#[derive(Copy, Clone, Debug)]
pub struct Dielectric {
    /// The refractive index of the material. This affects how it refracts or
    /// reflects rays, based on Snell's law.
    pub refractive_index: f32,
    /// The albedo. Is `vec3!(1.0, 1.0, 1.0)` by default if you use
    /// `Dielectric::new()`. Can be changed with `Dielectric::new_with_albedo()`.
    pub albedo: Vec3,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self::new_with_albedo(vec3!(1.0, 1.0, 1.0), refractive_index)
    }

    pub fn new_with_albedo(albedo: Vec3, refractive_index: f32) -> Self {
        Self {
            albedo,
            refractive_index,
        }
    }

    pub fn scatter<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        ray_in: &Ray,
        rec: &HitRecord,
    ) -> Option<Scatter> {
        let attenuation = self.albedo;

        // Always assume that the bordering material is air, which has a
        // refractive index of 1.0. Here we find η/η′.
        let etai_over_etat = if rec.front_face {
            // The ray hit the outside surface of the object. So, η = 1.0 for
            // air, and η′ = self.refractive_index for the object.
            1.0 / self.refractive_index
        } else {
            // The ray hit the inside surface of the object. So,
            // η = self.refractive_index for the object, and η′ = 1.0 for air.
            self.refractive_index
        };

        let unit_direction = ray_in.direction.unit_vector();

        // We have to decide if the ray will refract or reflect. If
        // η/η′ * sin(θ) > 1.0, then the ray must reflect. Otherwise, it will
        // refract. We can solve for sin(θ) by the trig identity:
        // sin(θ) = sqrt(1 - cos^2(θ)).
        let cos_theta = rec.normal.dot(&(-unit_direction)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Also, real dielectrics (like glass) have a reflectivity that varies
        // with angle. We get the probability of reflection using the Schlick
        // approximation, and then compare it to a random f32.
        let reflect_prob = schlick(cos_theta, etai_over_etat);

        let scattered = if (etai_over_etat * sin_theta > 1.0) || (rng.gen::<f32>() < reflect_prob) {
            // Ray must reflect.
            let reflected = unit_direction.reflect(&rec.normal);
            Ray::new(rec.hit_point, reflected)
        } else {
            // Ray must refract.
            let refracted = unit_direction.refract(&rec.normal, etai_over_etat);
            Ray::new(rec.hit_point, refracted)
        };

        Some(Scatter::new(attenuation, scattered))
    }
}

/// Helps us get the angle at which the dielectric becomes a mirror.
/// Based on a polynomial approximation by Chirstophe Schlick.
fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
