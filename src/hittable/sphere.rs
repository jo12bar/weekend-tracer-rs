//! Structs and methods for working with `Hittable` spheres.

use crate::aabb::AABB;
use crate::hittable::{get_sphere_uv, HitRecord, Hittable};
use crate::material::Material;
use crate::onb::ONB;
use crate::pdf::random_to_sphere;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Vec3;
use rand::prelude::*;
use std::sync::Arc;

/// A sphere. Can be hit with rays.
#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl Sphere {
    /// Create a new sphere.
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material: Arc::new(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // See the raytracing in one weekend book, chapter 6, for this formula.
        // We found a (modified) quadratic formula for hit-testing a sphere.
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);

        // The sphere is hit if the discriminant is greater than 0.
        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let solution_1 = (-half_b - root) / a;
            let solution_2 = (-half_b + root) / a;

            let t = if solution_1 < t_max && solution_1 > t_min {
                Some(solution_1)
            } else if solution_2 < t_max && solution_2 > t_min {
                Some(solution_2)
            } else {
                None
            };

            if let Some(t) = t {
                let hit_point = ray.at(t);
                Some(HitRecord::new(
                    ray,
                    t,
                    hit_point,
                    (hit_point - self.center) / self.radius,
                    self.material.clone(),
                    get_sphere_uv((hit_point - self.center) / self.radius),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            self.center - vec3!(self.radius, self.radius, self.radius),
            self.center + vec3!(self.radius, self.radius, self.radius),
        ))
    }

    fn pdf_value(&self, origin: &Vec3, v: &Vec3) -> f32 {
        if self
            .hit(&Ray::new(*origin, *v, 0.0), 0.001, std::f32::MAX)
            .is_some()
        {
            let cos_theta_max =
                (1.0 - self.radius * self.radius / (self.center - *origin).length_squared()).sqrt();
            let solid_angle = 2.0 * std::f32::consts::PI * (1.0 - cos_theta_max);

            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        // Not ideal, but eh...
        let mut rng = thread_rng();

        let direction = self.center - *origin;
        let distance_squared = direction.length_squared();
        let uvw = ONB::build_from_w(direction);

        uvw.local(&random_to_sphere(&mut rng, self.radius, distance_squared))
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
