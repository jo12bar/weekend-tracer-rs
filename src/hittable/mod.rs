//! Things that can be hit by rays, and some related functions and traits.

pub mod moving_sphere;
pub mod sphere;
pub mod world;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// A utility struct for recording that a ray hit a point on a `Hittable` object.
#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    /// The point that got hit.
    pub hit_point: Vec3,
    /// The normal vector to the surface that got hit.
    pub normal: Vec3,
    /// How far along the ray the surface was hit.
    pub t: f32,
    /// Whether or not the hit was on the front face of the surface.
    pub front_face: bool,
    /// The material that got hit.
    pub material: Material,
}

impl HitRecord {
    /// Create a new `HitRecord`. Sets the normal vector based on an
    /// `outward_normal` passed to it, such that it always points against the
    /// incident ray. The `outward_normal` should always point out from the
    /// surface.
    pub fn new(
        ray: &Ray,
        t: f32,
        hit_point: Vec3,
        outward_normal: Vec3,
        material: Material,
    ) -> HitRecord {
        let (front_face, normal) = if ray.direction.dot(&outward_normal) < 0.0 {
            // The ray hit the outside of the surface
            (true, outward_normal)
        } else {
            // The ray hit the inside of the surface
            (false, -outward_normal)
        };

        HitRecord {
            hit_point,
            t,
            normal,
            front_face,
            material,
        }
    }
}

/// This trait indicates that something can be hit by a ray. It also provides a
/// way to test if an object will be hit by a ray, and to return information
/// about things like the hit point, the normal vector to the object at the hit
/// point, and how far along the ray the hit point is located.
pub trait Hittable: Send + Sync {
    /// Test if a ray will hit something.
    ///
    /// Note that this ray tracer only counts valid hits if they are within the
    /// range t_(min) < t < t_(max).
    ///
    /// # Returns:
    ///
    /// - `None` if the surface didn't hit anything.
    /// - `Some(HitRecord)` if the surface *did* hit something.
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
