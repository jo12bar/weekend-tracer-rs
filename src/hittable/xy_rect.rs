//! For axis-aligned rectangles. I can't figure out rotation yet 😅
//!
//! Note that these axis-aligned rectangles have infinitely-thin sides. This can be a
//! problem when dividing the world into our axis-aligned bounding volume
//! hierarchy (`BVH`). To counter this, all hittable objects should get a
//! bounding box that has finite width alonge very dimension. For our
//! rectangles, we'll just pad the box a bit on the infinitely-thin side.

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3,
    vec3::{Axis::*, Vec3},
};
use std::sync::Arc;

/// An axis-aligned rectangle.
#[derive(Debug, Clone)]
pub struct XYRect {
    pub material: Arc<Material>,
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,

    /// The height of the plane that the rectangle exists on.
    pub k: f32,
}

impl XYRect {
    /// Create a new, infinitely-thin, axis-aligned rectangle.
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Material) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material: Arc::new(material),
        }
    }
}

impl Hittable for XYRect {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin[Z]) / ray.direction[Z];

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin[X] + t * ray.direction[X];
        let y = ray.origin[Y] + t * ray.direction[Y];

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let outward_normal = vec3!(0.0, 0.0, 1.0);
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        Some(HitRecord::new(
            ray,
            t,
            ray.at(t),
            outward_normal,
            self.material.clone(),
            (u, v),
        ))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad
        // the Z dimension by a small amount.
        Some(AABB::new(
            vec3!(self.x0, self.y0, self.k - 0.0001),
            vec3!(self.x1, self.y1, self.k + 0.0001),
        ))
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
