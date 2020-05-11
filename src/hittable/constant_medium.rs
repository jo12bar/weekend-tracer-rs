//! A *volume* (or *participating media*) with constant density throughout.

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    texture::Texture,
    vec3,
    vec3::Vec3,
};
use rand::prelude::*;
use std::sync::Arc;

/// A volume of constant density.
///
/// A ray going through this can either scatter inside the volume or make it all
/// the way through.
#[derive(Clone, Debug)]
pub struct ConstantMedium {
    /// Defines the shape of the volume.
    boundary: Box<dyn Hittable>,
    /// Defines how rays scatter throughout the material.
    phase_function: Arc<Material>,
    /// == -1/density
    neg_inv_density: f32,
}

impl ConstantMedium {
    /// Create a new constant medium. This is a volume of constant density.
    pub fn new(boundary: Box<dyn Hittable>, density: f32, albedo: Texture) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Material::isotropic(albedo)),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Not optimal to do this on every hit, but I don't feel like rewriting
        // everything right now.
        //
        // TODO: Rewrite Hittable::hit to take an rng parameter!
        let mut rng = thread_rng();

        if let Some(mut rec1) = self.boundary.hit(ray, std::f32::MIN, std::f32::MAX) {
            if let Some(mut rec2) = self.boundary.hit(ray, rec1.t + 0.0001, std::f32::MAX) {
                if rec1.t < t_min {
                    rec1.t = t_min
                }
                if rec2.t > t_max {
                    rec2.t = t_max
                }

                if rec1.t >= rec2.t {
                    return None;
                }

                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }

                let ray_length = ray.direction.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rng.gen::<f32>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                let hit_point = ray.at(t);
                let normal = vec3!(1.0); // arbritrary!
                let uv = rec1.uv; // also arbritrary!

                Some(HitRecord::new(
                    ray,
                    t,
                    hit_point,
                    normal,
                    self.phase_function.clone(),
                    uv,
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
