//! Structs and methods for working with `Hittable` spheres.

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// A sphere. Can be hit with rays.
#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    /// Create a new sphere.
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
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
            } else if solution_2 < t_max && solution_2 > t_max {
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
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}
