//! Structs and methods for working with **moving** `Hittable` spheres.

use crate::{
    aabb::AABB,
    hittable::{get_sphere_uv, HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3,
    vec3::Vec3,
};
use std::sync::Arc;

/// A linearly-moving sphere. Will move from `center0` at `time0` to `center1`
/// at `time1`.
#[derive(Debug, Clone)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl MovingSphere {
    /// Create a new linearly-moving sphere.
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Material,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material: Arc::new(material),
        }
    }

    /// Get the center of the sphere at time `time`.
    ///
    /// # Usage
    ///
    /// ```
    /// use weekend_tracer_rs::hittable::moving_sphere::MovingSphere;
    /// use weekend_tracer_rs::vec3;
    /// use weekend_tracer_rs::vec3::Vec3;
    /// use weekend_tracer_rs::material::Material;
    ///
    /// let sphere = MovingSphere::new(
    ///     vec3!(-1.0, -1.0, -1.0),
    ///     vec3!(1.0, 1.0, 1.0),
    ///     0.0,
    ///     2.0,
    ///     0.5,
    ///     Material::dielectric(1.5, 0.0),
    /// );
    ///
    /// assert_eq!(sphere.center(-1.0), vec3!(-2.0, -2.0, -2.0));
    /// assert_eq!(sphere.center(0.0), vec3!(-1.0, -1.0, -1.0));
    /// assert_eq!(sphere.center(1.0), vec3!(0.0, 0.0, 0.0));
    /// assert_eq!(sphere.center(2.0), vec3!(1.0, 1.0, 1.0));
    /// assert_eq!(sphere.center(3.0), vec3!(2.0, 2.0, 2.0));
    /// ```
    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // See the raytracing in one weekend book, chapter 6, for this formula.
        // We found a (modified) quadratic formula for hit-testing a sphere.
        let oc = ray.origin - self.center(ray.time);
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
                    (hit_point - self.center(ray.time)) / self.radius,
                    self.material.clone(),
                    get_sphere_uv(hit_point - self.center(ray.time) / self.radius),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(t0) - vec3!(self.radius, self.radius, self.radius),
            self.center(t0) + vec3!(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(t1) - vec3!(self.radius, self.radius, self.radius),
            self.center(t1) + vec3!(self.radius, self.radius, self.radius),
        );
        Some(AABB::surrounding_box(box0, box1))
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
