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

// A rectangle aligned with the X and Y axises.
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

// A rectangle aligned with the X and Z axises.
#[derive(Debug, Clone)]
pub struct XZRect {
    pub material: Arc<Material>,
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,

    /// The height of the plane that the rectangle exists on.
    pub k: f32,
}

impl XZRect {
    /// Create a new, infinitely-thin, axis-aligned rectangle.
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Material) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material: Arc::new(material),
        }
    }
}

impl Hittable for XZRect {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin[Y]) / ray.direction[Y];

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin[X] + t * ray.direction[X];
        let z = ray.origin[Z] + t * ray.direction[Z];

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = vec3!(0.0, 1.0, 0.0);
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);

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
        // the Y dimension by a small amount.
        Some(AABB::new(
            vec3!(self.x0, self.k - 0.0001, self.z0),
            vec3!(self.x1, self.k + 0.0001, self.z1),
        ))
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

// A rectangle aligned with the Y and Z axises.
#[derive(Debug, Clone)]
pub struct YZRect {
    pub material: Arc<Material>,
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,

    /// The height of the plane that the rectangle exists on.
    pub k: f32,
}

impl YZRect {
    /// Create a new, infinitely-thin, axis-aligned rectangle.
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Material) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material: Arc::new(material),
        }
    }
}

impl Hittable for YZRect {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin[X]) / ray.direction[X];

        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin[Y] + t * ray.direction[Y];
        let z = ray.origin[Z] + t * ray.direction[Z];

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = vec3!(1.0, 0.0, 0.0);
        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);

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
        // the X dimension by a small amount.
        Some(AABB::new(
            vec3!(self.k - 0.0001, self.y0, self.z0),
            vec3!(self.k + 0.0001, self.y1, self.z1),
        ))
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
