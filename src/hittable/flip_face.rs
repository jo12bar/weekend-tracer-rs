//! Allows you to flip the outward face of a `Hittable`.

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

/// A "holder" that does nothing but hold a `Hittable` and flip its face.
#[derive(Debug, Clone)]
pub struct FlipFace(Box<dyn Hittable>);

impl FlipFace {
    /// Create a new holder for flipping a `Hittable`'s face.
    pub fn new(p: Box<dyn Hittable>) -> Self {
        Self(p)
    }
}

impl Hittable for FlipFace {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut rec) = self.0.hit(ray, t_min, t_max) {
            rec.front_face = !rec.front_face;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.0.bounding_box(t0, t1)
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
