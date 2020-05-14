//! The world to be rendered.

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Vec3;
use rand::prelude::*;

/// The world that needs to be rendered, with all of its objects. Every object
/// needs to implement `Hittable`. Coincidentally, this struct *also* implements
/// `Hittable`.
#[derive(Default, Debug, Clone)]
pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    /// Create a new `World`, filled with the passed-in `objects`.
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        World { objects }
    }

    /// Add an object to the `World`.
    pub fn add(&mut self, object: Box<dyn Hittable>) -> &mut Self {
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

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut first_box = true;
        let mut temp_box: Option<AABB>;
        let mut output_box = AABB::new(vec3!(), vec3!());

        for object in &self.objects {
            temp_box = object.bounding_box(t0, t1);
            if let Some(temp_box) = temp_box {
                output_box = if first_box {
                    temp_box
                } else {
                    AABB::surrounding_box(output_box, temp_box)
                };
                first_box = false;
            } else {
                return None;
            }
        }

        Some(output_box)
    }

    fn pdf_value(&self, origin: &Vec3, v: &Vec3) -> f32 {
        let weight = 1.0 / self.objects.len() as f32;
        let mut sum = 0.0;

        for object in &self.objects {
            sum += object.pdf_value(origin, v);
        }

        sum * weight
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        // AAHHHH THIS IS BAD
        let mut rng = thread_rng();

        let index = rng.gen_range(0_u64, self.objects.len() as u64);

        self.objects[index as usize].random(origin)
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

/// A convenience macro for more easily building `World`'s.
#[macro_export]
macro_rules! create_world {
    ($($object:expr),* $(,)?) => {
        World::new(vec![
            $(Box::new($object)),*
        ])
    };
}
