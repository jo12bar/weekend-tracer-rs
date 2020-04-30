//! The world to be rendered.

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::sync::Arc;

/// The world that needs to be rendered, with all of its objects. Every object
/// needs to implement `Hittable`. Coincidentally, this struct *also* implements
/// `Hittable`.
#[derive(Default)]
pub struct World {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl World {
    /// Create a new `World`, filled with the passed-in `objects`.
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        World { objects }
    }

    /// Add an object to the `World`.
    pub fn add(&mut self, object: Arc<dyn Hittable>) -> &mut Self {
        self.objects.push(object);
        self
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // We want to keep track of the closest-hit object. So, we intialize the
        // closest value for `t` to `t_max`.
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(obj_hit_rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = obj_hit_rec.t;
                rec = Some(obj_hit_rec);
            }
        }

        rec
    }
}

/// A convenience macro for more easily building `World`'s.
#[macro_export]
macro_rules! create_world {
    ($($object:expr),* $(,)?) => {
        World::new(vec![
            $(Arc::new($object)),*
        ])
    };
}
