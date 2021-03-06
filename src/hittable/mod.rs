//! Things that can be hit by rays, and some related functions and traits.

pub mod aa_rect;
pub mod block;
pub mod constant_medium;
pub mod flip_face;
pub mod moving_sphere;
pub mod rotate;
pub mod sphere;
pub mod translate;
pub mod world;

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Axis, Axis::*, Vec3};
use std::sync::Arc;

/// (u, v) surface coordinates for some `Hittable` that has a surface.
pub type UVCoord = (f32, f32);

/// Gets the (u, v) surface coordinates for a sphere, given a point on the
/// sphere's surface.
pub fn get_sphere_uv(point: Vec3) -> UVCoord {
    use std::f32::consts::PI;
    let phi = point[Z].atan2(point[X]);
    let theta = point[Y].asin();
    (
        // u:
        1.0 - (phi + PI) / (2.0 * PI),
        // v:
        (theta + PI / 2.0) / PI,
    )
}

/// A utility struct for recording that a ray hit a point on a `Hittable` object.
#[derive(Clone, Debug)]
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
    pub material: Arc<Material>,
    /// The (u, v) surface coordinates of the hit point.
    pub uv: UVCoord,
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
        material: Arc<Material>,
        uv: UVCoord,
    ) -> HitRecord {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            hit_point,
            t,
            normal,
            front_face,
            material,
            uv,
        }
    }
}

/// This trait indicates that something can be hit by a ray. It also provides a
/// way to test if an object will be hit by a ray, and to return information
/// about things like the hit point, the normal vector to the object at the hit
/// point, and how far along the ray the hit point is located.
pub trait Hittable: Send + Sync + core::fmt::Debug {
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

    /// Computes the bounding box of the object.
    ///
    /// # Returns:
    ///
    /// - `None` if the bounding box couldn't be computed.
    /// - `Some(AABB)` if the bounding box was successfully computed.
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;

    /// Clones the object into a Box<dyn Hittable>.
    fn box_clone(&self) -> Box<dyn Hittable>;

    /// Flips the face of the object, cloning it.
    fn flip_face(&self) -> flip_face::FlipFace {
        flip_face::FlipFace::new(self.box_clone())
    }

    /// Translates the object by some offset using a `Translate` instance, cloning it.
    fn translate(&self, displacement: Vec3) -> translate::Translate {
        translate::Translate::new(self.box_clone(), displacement)
    }

    /// Rotates the object by θ degrees counterclockwise about some `vec3::Axis`, cloning it.
    /// Uses a `Rotate` instance.
    fn rotate(&self, axis: Axis, angle: f32) -> rotate::Rotate {
        rotate::Rotate::new(self.box_clone(), axis, angle)
    }

    /// Get a value of the hittable's PDF given some origin and some vector.
    fn pdf_value(&self, _origin: &Vec3, _v: &Vec3) -> f32 {
        0.0
    }

    /// Get a random vector for PDF use.
    fn random(&self, _origin: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
