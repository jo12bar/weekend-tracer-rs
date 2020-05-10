//! Structs for translating `Hittable` objects.

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

/// A translation instance. Holds a `Hittable` object and translates it by some
/// displacement.
#[derive(Clone, Debug)]
pub struct Translate {
    obj: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    /// Create a new translation instance for some `Hittable` object. The object
    /// will be translated by the specified offset.
    pub fn new(obj: Box<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            obj,
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        if let Some(hit_record) = self.obj.hit(&moved_ray, t_min, t_max) {
            Some(HitRecord::new(
                &moved_ray,
                hit_record.t,
                hit_record.hit_point + self.offset,
                hit_record.normal,
                hit_record.material,
                hit_record.uv,
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(bb) = self.obj.bounding_box(t0, t1) {
            Some(AABB::new(bb.min + self.offset, bb.max + self.offset))
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
